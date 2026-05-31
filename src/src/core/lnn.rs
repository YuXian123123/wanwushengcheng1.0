//! LNN网络实现
//!
//! 液体神经网络主体

use crate::core::{Neuron, NeuronType, Synapse, PlasticityRule, LNNConfig, TopologyDynamics};
use crate::safety::{SafetyMonitor, FuseState};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::Instant;

/// 审计日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub event: String,
    pub data: HashMap<String, String>,
}

/// LNN 持久化状态（可序列化）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LNNSnapshot {
    /// 版本号
    pub version: u32,
    /// 创建时间戳
    pub created_at: u64,
    /// 神经元状态
    pub neurons: Vec<crate::core::neuron::NeuronState>,
    /// 突触状态
    pub synapses: Vec<crate::core::synapse::SynapseState>,
    /// 当前时间
    pub current_time: f64,
    /// 配置
    pub config: LNNConfig,
    /// 拓扑动态配置
    pub topology_dynamics: TopologyDynamics,
}

/// LNN网络
pub struct LNN {
    /// 神经元集合
    neurons: HashMap<String, Neuron>,
    /// 突触集合
    synapses: HashMap<String, Synapse>,
    /// 配置
    config: LNNConfig,
    /// 动态拓扑配置
    topology_dynamics: TopologyDynamics,
    /// 当前时间
    current_time: f64,
    /// 安全监控器
    safety_monitor: SafetyMonitor,
    /// 审计日志
    audit_log: Vec<AuditEntry>,
    /// 上次生长时间
    last_growth_time: Option<Instant>,
    /// 上次修剪时间
    last_prune_time: Option<Instant>,
    /// 任务复杂度
    task_complexity: f64,
}

impl LNN {
    /// 创建新的LNN网络
    pub fn new(config: Option<LNNConfig>, topology_dynamics: Option<TopologyDynamics>) -> Self {
        Self {
            neurons: HashMap::new(),
            synapses: HashMap::new(),
            config: config.unwrap_or_default(),
            topology_dynamics: topology_dynamics.unwrap_or_default(),
            current_time: 0.0,
            safety_monitor: SafetyMonitor::new(),
            audit_log: Vec::new(),
            last_growth_time: None,
            last_prune_time: None,
            task_complexity: 0.0,
        }
    }

    /// 添加神经元
    ///
    /// # Errors
    /// 超过最大神经元数时返回错误
    pub fn add_neuron(&mut self, neuron_type: NeuronType) -> Result<String, String> {
        // 安全检查
        if self.neurons.len() >= self.config.topology.max_neurons {
            return Err("Neuron count exceeds maximum".to_string());
        }

        let id = format!("neuron_{}_{}",
            chrono::Utc::now().timestamp_millis(),
            uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or("x")
        );

        let neuron = Neuron::new(id.clone(), neuron_type);
        self.neurons.insert(id.clone(), neuron);

        // 审计日志
        if self.config.safety.audit_topology {
            let mut data = HashMap::new();
            data.insert("id".to_string(), id.clone());
            data.insert("type".to_string(), format!("{:?}", neuron_type));
            self.audit_log.push(AuditEntry {
                timestamp: self.current_time as u64,
                event: "neuron_added".to_string(),
                data,
            });
        }

        Ok(id)
    }

    /// 移除神经元
    ///
    /// # Errors
    /// 低于最小神经元数时返回错误
    pub fn remove_neuron(&mut self, id: &str) -> Result<(), String> {
        if self.neurons.len() <= self.config.topology.min_neurons {
            return Err("Neuron count below minimum".to_string());
        }

        if self.neurons.remove(id).is_none() {
            return Err(format!("Neuron {} not found", id));
        }

        // 删除相关突触
        self.synapses.retain(|_, syn| {
            syn.from() != id && syn.to() != id
        });

        // 审计日志
        if self.config.safety.audit_topology {
            let mut data = HashMap::new();
            data.insert("id".to_string(), id.to_string());
            self.audit_log.push(AuditEntry {
                timestamp: self.current_time as u64,
                event: "neuron_removed".to_string(),
                data,
            });
        }

        Ok(())
    }

    /// 添加突触
    ///
    /// # Errors
    /// 神经元不存在或超过连接数限制时返回错误
    pub fn add_synapse(
        &mut self,
        from_id: &str,
        to_id: &str,
        weight: f64,
        rule: PlasticityRule,
    ) -> Result<String, String> {
        // 检查神经元存在
        if !self.neurons.contains_key(from_id) || !self.neurons.contains_key(to_id) {
            return Err("Neuron not found".to_string());
        }

        // 检查连接数限制
        let from_connections = self.count_connections(from_id);
        if from_connections >= self.config.topology.max_connections {
            return Err("Connection count exceeds maximum".to_string());
        }

        let synapse = Synapse::new(
            from_id.to_string(),
            to_id.to_string(),
            weight,
            rule,
        );
        let id = synapse.id().to_string();
        self.synapses.insert(id.clone(), synapse);

        // 审计日志
        if self.config.safety.audit_topology {
            let mut data = HashMap::new();
            data.insert("from".to_string(), from_id.to_string());
            data.insert("to".to_string(), to_id.to_string());
            data.insert("weight".to_string(), weight.to_string());
            self.audit_log.push(AuditEntry {
                timestamp: self.current_time as u64,
                event: "synapse_added".to_string(),
                data,
            });
        }

        Ok(id)
    }

    /// 移除突触
    pub fn remove_synapse(&mut self, id: &str) -> bool {
        self.synapses.remove(id).is_some()
    }

    /// 时间步更新
    ///
    /// 执行一次完整的时间步更新：
    /// 1. 计算每个神经元的输入
    /// 2. 更新神经元状态
    /// 3. 更新突触权重（局部学习）
    /// 4. 安全检查
    /// 5. 动态拓扑管理
    pub fn update(&mut self, dt: f64) -> Result<(), String> {
        // 检查熔断状态
        if self.safety_monitor.is_fused() {
            return Err("Network is fused".to_string());
        }

        self.current_time += dt;

        // 1. 计算每个神经元的输入
        let mut inputs: HashMap<String, f64> = HashMap::new();
        for id in self.neurons.keys() {
            inputs.insert(id.clone(), 0.0);
        }

        // 累加突触输入
        for synapse in self.synapses.values() {
            if let Some(pre_neuron) = self.neurons.get(synapse.from()) {
                let pre_state = pre_neuron.state();
                let input = inputs.get_mut(synapse.to()).unwrap();
                *input += synapse.weight() * pre_state;
            }
        }

        // 2. 更新神经元状态
        for (id, neuron) in self.neurons.iter_mut() {
            let input = inputs.get(id).copied().unwrap_or(0.0);
            neuron.update(input, dt);
        }

        // 3. 更新突触权重
        self.update_synapses();

        // 4. 安全检查
        if self.config.safety.enable_monitoring {
            if let Err(reason) = self.check_safety() {
                let reason_clone = reason.clone();
                self.safety_monitor.trigger_fuse(reason);
                return Err(format!("Fuse triggered: {}", reason_clone));
            }
        }

        // 5. 动态拓扑管理
        self.manage_topology()?;

        Ok(())
    }

    /// 更新突触权重（局部学习）
    fn update_synapses(&mut self) {
        let lr = self.config.learning.initial_rate;

        // 收集需要更新的突触信息
        let updates: Vec<(String, f64, f64)> = self.synapses.iter()
            .filter_map(|(id, synapse)| {
                let pre_state = self.neurons.get(synapse.from())?.state();
                let post_state = self.neurons.get(synapse.to())?.state();
                Some((id.clone(), pre_state, post_state))
            })
            .collect();

        // 应用更新
        for (id, pre_state, post_state) in updates {
            if let Some(synapse) = self.synapses.get_mut(&id) {
                synapse.update_weight(pre_state, post_state, lr);
            }
        }
    }

    /// 安全检查
    fn check_safety(&mut self) -> Result<(), String> {
        let abnormal_count = self.neurons.values()
            .filter(|n| n.is_abnormal())
            .count();

        let abnormal_rate = abnormal_count as f64 / self.neurons.len().max(1) as f64;

        if abnormal_rate > self.config.safety.fuse_threshold {
            return Err(format!("Abnormal neuron rate: {:.2}", abnormal_rate));
        }

        Ok(())
    }

    /// 动态拓扑管理
    fn manage_topology(&mut self) -> Result<(), String> {
        let now = Instant::now();

        // 检查是否可以修剪
        if let Some(last) = self.last_prune_time {
            if now.duration_since(last).as_millis() < self.topology_dynamics.prune_cooldown_ms as u128 {
                return Ok(());
            }
        }

        // 修剪不活跃的突触
        self.prune_inactive_synapses()?;

        // 修剪不活跃的神经元
        self.prune_inactive_neurons()?;

        self.last_prune_time = Some(now);

        Ok(())
    }

    /// 修剪不活跃的突触
    fn prune_inactive_synapses(&mut self) -> Result<(), String> {
        let threshold = self.topology_dynamics.synapse_weight_threshold;
        let max_prune = self.topology_dynamics.max_prune_per_cycle;

        let to_remove: Vec<String> = self.synapses.iter()
            .filter(|(_, syn)| !syn.is_active(threshold))
            .take(max_prune)
            .map(|(id, _)| id.clone())
            .collect();

        for id in to_remove {
            self.synapses.remove(&id);
        }

        Ok(())
    }

    /// 修剪不活跃的神经元
    fn prune_inactive_neurons(&mut self) -> Result<(), String> {
        let threshold = self.topology_dynamics.neuron_activity_threshold;
        let max_prune = self.topology_dynamics.max_prune_per_cycle;

        let to_remove: Vec<String> = self.neurons.iter()
            .filter(|(_, neuron)| !neuron.is_active(threshold))
            .take(max_prune)
            .map(|(id, _)| id.clone())
            .collect();

        for id in to_remove {
            if self.neurons.len() > self.config.topology.min_neurons {
                self.remove_neuron(&id)?;
            }
        }

        Ok(())
    }

    /// 获取连接数
    fn count_connections(&self, neuron_id: &str) -> usize {
        self.synapses.values()
            .filter(|syn| syn.from() == neuron_id)
            .count()
    }

    /// 获取网络状态
    pub fn get_state(&self) -> LNNState {
        LNNState {
            neuron_count: self.neurons.len(),
            synapse_count: self.synapses.len(),
            current_time: self.current_time,
            fuse_state: self.safety_monitor.state(),
        }
    }

    /// 获取审计日志
    pub fn get_audit_log(&self) -> &[AuditEntry] {
        &self.audit_log
    }

    // ========================================================================
    // 持久化功能
    // ========================================================================

    /// 创建网络快照（用于保存）
    pub fn create_snapshot(&self) -> LNNSnapshot {
        LNNSnapshot {
            version: 1,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0),
            neurons: self.neurons.values().map(|n| n.to_state()).collect(),
            synapses: self.synapses.values().map(|s| s.to_state()).collect(),
            current_time: self.current_time,
            config: self.config.clone(),
            topology_dynamics: self.topology_dynamics.clone(),
        }
    }

    /// 从快照恢复网络
    pub fn from_snapshot(snapshot: LNNSnapshot) -> Self {
        let mut neurons = HashMap::new();
        for state in snapshot.neurons {
            let id = state.id.clone();
            neurons.insert(id, Neuron::from_state(state));
        }

        let mut synapses = HashMap::new();
        for state in snapshot.synapses {
            let id = state.id.clone();
            synapses.insert(id, Synapse::from_state(state));
        }

        Self {
            neurons,
            synapses,
            config: snapshot.config,
            topology_dynamics: snapshot.topology_dynamics,
            current_time: snapshot.current_time,
            safety_monitor: SafetyMonitor::new(),
            audit_log: Vec::new(),
            last_growth_time: None,
            last_prune_time: None,
            task_complexity: 0.0,
        }
    }

    /// 保存网络到文件
    ///
    /// # 格式
    /// JSON 格式，可读性强，便于调试
    ///
    /// # Errors
    /// 文件写入失败时返回错误
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), LNNError> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let snapshot = self.create_snapshot();
        serde_json::to_writer_pretty(writer, &snapshot)?;
        Ok(())
    }

    /// 从文件加载网络
    ///
    /// # Errors
    /// 文件读取或解析失败时返回错误
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, LNNError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let snapshot: LNNSnapshot = serde_json::from_reader(reader)?;
        Ok(Self::from_snapshot(snapshot))
    }

    /// 保存为二进制格式（更紧凑）
    pub fn save_binary<P: AsRef<Path>>(&self, path: P) -> Result<(), LNNError> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let snapshot = self.create_snapshot();
        bincode::serialize_into(writer, &snapshot)?;
        Ok(())
    }

    /// 从二进制格式加载
    pub fn load_binary<P: AsRef<Path>>(path: P) -> Result<Self, LNNError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let snapshot: LNNSnapshot = bincode::deserialize_from(reader)?;
        Ok(Self::from_snapshot(snapshot))
    }

    /// 获取网络统计信息
    pub fn statistics(&self) -> NetworkStatistics {
        let neuron_count = self.neurons.len();
        let synapse_count = self.synapses.len();

        let avg_activity = if neuron_count > 0 {
            self.neurons.values().map(|n| n.activity()).sum::<f64>() / neuron_count as f64
        } else {
            0.0
        };

        let avg_weight = if synapse_count > 0 {
            self.synapses.values().map(|s| s.weight()).sum::<f64>() / synapse_count as f64
        } else {
            0.0
        };

        let type_distribution = self.neurons.values()
            .fold(HashMap::new(), |mut acc, n| {
                *acc.entry(n.neuron_type()).or_insert(0) += 1;
                acc
            });

        NetworkStatistics {
            neuron_count,
            synapse_count,
            avg_activity,
            avg_weight,
            current_time: self.current_time,
            type_distribution,
        }
    }
}

/// 网络统计信息
#[derive(Debug, Clone)]
pub struct NetworkStatistics {
    pub neuron_count: usize,
    pub synapse_count: usize,
    pub avg_activity: f64,
    pub avg_weight: f64,
    pub current_time: f64,
    pub type_distribution: HashMap<NeuronType, usize>,
}

/// LNN 错误类型
#[derive(Debug)]
pub enum LNNError {
    IoError(std::io::Error),
    SerializeError(serde_json::Error),
    BincodeError(bincode::Error),
}

impl From<std::io::Error> for LNNError {
    fn from(e: std::io::Error) -> Self {
        LNNError::IoError(e)
    }
}

impl From<serde_json::Error> for LNNError {
    fn from(e: serde_json::Error) -> Self {
        LNNError::SerializeError(e)
    }
}

impl From<bincode::Error> for LNNError {
    fn from(e: bincode::Error) -> Self {
        LNNError::BincodeError(e)
    }
}

/// LNN状态
#[derive(Debug, Clone)]
pub struct LNNState {
    pub neuron_count: usize,
    pub synapse_count: usize,
    pub current_time: f64,
    pub fuse_state: FuseState,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lnn_creation() {
        let lnn = LNN::new(None, None);
        let state = lnn.get_state();
        assert_eq!(state.neuron_count, 0);
        assert_eq!(state.synapse_count, 0);
    }

    #[test]
    fn test_add_neuron() {
        let mut lnn = LNN::new(None, None);
        let _id = lnn.add_neuron(NeuronType::Cognitive).unwrap();
        assert!(lnn.get_state().neuron_count == 1);
    }

    #[test]
    fn test_neuron_boundary_min() {
        let mut config = LNNConfig::default();
        config.topology.min_neurons = 5;

        let mut lnn = LNN::new(Some(config), None);

        // 添加几个神经元
        for _ in 0..5 {
            lnn.add_neuron(NeuronType::Cognitive).unwrap();
        }

        // 尝试删除应该失败
        let id = lnn.neurons.keys().next().unwrap().clone();
        assert!(lnn.remove_neuron(&id).is_err());
    }

    #[test]
    fn test_add_synapse() {
        let mut lnn = LNN::new(None, None);
        let n1 = lnn.add_neuron(NeuronType::Cognitive).unwrap();
        let n2 = lnn.add_neuron(NeuronType::Cognitive).unwrap();

        let _syn_id = lnn.add_synapse(&n1, &n2, 0.5, PlasticityRule::Hebbian).unwrap();
        assert!(lnn.get_state().synapse_count == 1);
    }

    #[test]
    fn test_state_propagation() {
        let mut lnn = LNN::new(None, None);

        // 创建三个神经元的链式连接 A -> B -> C
        let a = lnn.add_neuron(NeuronType::Perception).unwrap();
        let b = lnn.add_neuron(NeuronType::Cognitive).unwrap();
        let c = lnn.add_neuron(NeuronType::Behavior).unwrap();

        lnn.add_synapse(&a, &b, 0.5, PlasticityRule::Hebbian).unwrap();
        lnn.add_synapse(&b, &c, 0.5, PlasticityRule::Hebbian).unwrap();

        // 手动设置A的状态（模拟输入）
        if let Some(neuron_a) = lnn.neurons.get_mut(&a) {
            neuron_a.update(1.0, 0.01); // 强输入
        }

        // 运行几步更新
        for _ in 0..100 {
            lnn.update(0.01).unwrap();
        }

        // C的状态应该有变化
        let state_c = lnn.neurons.get(&c).unwrap().state();
        assert!(state_c.abs() > 0.0);
    }

    #[test]
    fn test_save_load_json() {
        let mut lnn = LNN::new(None, None);

        // 创建网络结构
        let n1 = lnn.add_neuron(NeuronType::Perception).unwrap();
        let n2 = lnn.add_neuron(NeuronType::Cognitive).unwrap();
        let n3 = lnn.add_neuron(NeuronType::Behavior).unwrap();
        lnn.add_synapse(&n1, &n2, 0.5, PlasticityRule::Hebbian).unwrap();
        lnn.add_synapse(&n2, &n3, 0.3, PlasticityRule::Oja).unwrap();

        // 运行一些更新
        for _ in 0..10 {
            lnn.update(0.01).unwrap();
        }

        // 保存
        let temp_path = std::env::temp_dir().join("test_lnn_save.json");
        lnn.save(&temp_path).unwrap();

        // 加载
        let loaded = LNN::load(&temp_path).unwrap();

        // 验证
        assert_eq!(loaded.neurons.len(), 3);
        assert_eq!(loaded.synapses.len(), 2);
        assert!((loaded.current_time - lnn.current_time).abs() < 1e-10);

        // 清理
        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_save_load_binary() {
        let mut lnn = LNN::new(None, None);

        let n1 = lnn.add_neuron(NeuronType::Perception).unwrap();
        let n2 = lnn.add_neuron(NeuronType::Cognitive).unwrap();
        lnn.add_synapse(&n1, &n2, 0.7, PlasticityRule::Stdp).unwrap();

        for _ in 0..10 {
            lnn.update(0.01).unwrap();
        }

        // 保存二进制
        let temp_path = std::env::temp_dir().join("test_lnn_save.bin");
        lnn.save_binary(&temp_path).unwrap();

        // 加载
        let loaded = LNN::load_binary(&temp_path).unwrap();

        assert_eq!(loaded.neurons.len(), 2);
        assert_eq!(loaded.synapses.len(), 1);

        // 清理
        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_snapshot_restore() {
        let mut lnn = LNN::new(None, None);

        let n1 = lnn.add_neuron(NeuronType::Cognitive).unwrap();
        let n2 = lnn.add_neuron(NeuronType::Cognitive).unwrap();
        lnn.add_synapse(&n1, &n2, 0.5, PlasticityRule::Hebbian).unwrap();

        // 运行更新
        for _ in 0..50 {
            lnn.update(0.01).unwrap();
        }

        // 创建快照
        let snapshot = lnn.create_snapshot();

        // 从快照恢复
        let restored = LNN::from_snapshot(snapshot);

        // 验证状态一致
        assert_eq!(restored.neurons.len(), lnn.neurons.len());
        assert_eq!(restored.synapses.len(), lnn.synapses.len());
    }

    #[test]
    fn test_statistics() {
        let mut lnn = LNN::new(None, None);

        lnn.add_neuron(NeuronType::Perception).unwrap();
        lnn.add_neuron(NeuronType::Cognitive).unwrap();
        lnn.add_neuron(NeuronType::Cognitive).unwrap();
        lnn.add_neuron(NeuronType::Behavior).unwrap();

        let stats = lnn.statistics();
        assert_eq!(stats.neuron_count, 4);
        assert_eq!(stats.synapse_count, 0);
        assert_eq!(*stats.type_distribution.get(&NeuronType::Perception).unwrap_or(&0), 1);
        assert_eq!(*stats.type_distribution.get(&NeuronType::Cognitive).unwrap_or(&0), 2);
        assert_eq!(*stats.type_distribution.get(&NeuronType::Behavior).unwrap_or(&0), 1);
    }
}
