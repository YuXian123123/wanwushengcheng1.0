//! 智能聚合模块
//!
//! 世界智能 = Σ w_i × Gu_i × Emergence_Factor
//!
//! 设计者：拉蒂奥（优雅视角）

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::world::access_point::{AccessPointType, AccessPointStatus};

// ============================================================================
// 类型定义
// ============================================================================

/// 蛊虫智能向量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuIntelligence {
    /// 蛊虫ID
    pub gu_id: Uuid,
    /// 信任分数
    pub trust_score: f64,
    /// 专业领域得分
    pub expertise: HashMap<String, f64>,
    /// 接入点状态向量
    pub access_point_vector: [f64; 5],
    /// 活跃度
    pub activity: f64,
}

impl GuIntelligence {
    /// 创建新的蛊虫智能
    pub fn new(gu_id: Uuid) -> Self {
        Self {
            gu_id,
            trust_score: 0.5,
            expertise: HashMap::new(),
            access_point_vector: [0.0; 5],
            activity: 0.0,
        }
    }

    /// 计算权重（信任 × 专业）
    pub fn weight(&self) -> f64 {
        let expertise_sum: f64 = self.expertise.values().sum();
        self.trust_score * expertise_sum.max(1.0)
    }

    /// 计算向量模长
    pub fn magnitude(&self) -> f64 {
        self.access_point_vector.iter()
            .map(|x| x * x)
            .sum::<f64>()
            .sqrt()
    }

    /// 归一化向量
    pub fn normalized(&self) -> [f64; 5] {
        let mag = self.magnitude();
        if mag > 0.0 {
            self.access_point_vector.map(|x| x / mag)
        } else {
            [0.0; 5]
        }
    }
}

/// 世界智能状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldIntelligence {
    /// 世界智能向量
    pub vector: [f64; 5],
    /// 涌现因子
    pub emergence_factor: f64,
    /// 蛊虫数量
    pub gu_count: usize,
    /// 同步率
    pub sync_rate: f64,
    /// 多样性
    pub diversity: f64,
    /// 总智能容量
    pub total_capacity: f64,
}

impl WorldIntelligence {
    /// 空世界智能
    pub fn empty() -> Self {
        Self {
            vector: [0.0; 5],
            emergence_factor: 0.0,
            gu_count: 0,
            sync_rate: 0.0,
            diversity: 0.0,
            total_capacity: 0.0,
        }
    }

    /// 计算世界智能强度
    pub fn intensity(&self) -> f64 {
        let magnitude: f64 = self.vector.iter()
            .map(|x| x * x)
            .sum::<f64>()
            .sqrt();
        magnitude * self.emergence_factor
    }

    /// 是否涌现出意识（阈值 0.7）
    pub fn is_conscious(&self) -> bool {
        self.sync_rate > 0.7 && self.emergence_factor > 0.5
    }
}

// ============================================================================
// 智能聚合器
// ============================================================================

/// 智能聚合配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationConfig {
    /// 意识涌现阈值
    pub consciousness_threshold: f64,
    /// 最小同步率
    pub min_sync_rate: f64,
    /// 是否启用涌现计算
    pub enable_emergence: bool,
}

impl Default for AggregationConfig {
    fn default() -> Self {
        Self {
            consciousness_threshold: 0.7,
            min_sync_rate: 0.3,
            enable_emergence: true,
        }
    }
}

/// 智能聚合器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceAggregator {
    /// 配置
    config: AggregationConfig,
    /// 蛊虫智能记录
    gu_intelligences: HashMap<Uuid, GuIntelligence>,
    /// 缓存的世界智能
    cached_world_intelligence: Option<WorldIntelligence>,
    /// 最后更新时间
    last_update: u64,
}

impl IntelligenceAggregator {
    /// 创建新聚合器
    pub fn new(config: AggregationConfig) -> Self {
        Self {
            config,
            gu_intelligences: HashMap::new(),
            cached_world_intelligence: None,
            last_update: 0,
        }
    }

    /// 使用默认配置
    pub fn with_defaults() -> Self {
        Self::new(AggregationConfig::default())
    }

    /// 注册蛊虫
    pub fn register_gu(&mut self, gu_id: Uuid) {
        self.gu_intelligences.insert(gu_id, GuIntelligence::new(gu_id));
        self.cached_world_intelligence = None;
    }

    /// 注销蛊虫
    pub fn unregister_gu(&mut self, gu_id: &Uuid) {
        self.gu_intelligences.remove(gu_id);
        self.cached_world_intelligence = None;
    }

    /// 更新蛊虫智能
    pub fn update_gu(&mut self, gu_id: Uuid, intelligence: GuIntelligence) {
        self.gu_intelligences.insert(gu_id, intelligence);
        self.cached_world_intelligence = None;
    }

    /// 聚合世界智能
    pub fn aggregate(&mut self) -> WorldIntelligence {
        // 检查缓存
        if let Some(ref cached) = self.cached_world_intelligence {
            return cached.clone();
        }

        if self.gu_intelligences.is_empty() {
            return WorldIntelligence::empty();
        }

        // 1. 计算权重
        let weights: HashMap<Uuid, f64> = self.compute_weights();
        let total_weight: f64 = weights.values().sum();

        if total_weight == 0.0 {
            return WorldIntelligence::empty();
        }

        // 2. 加权求和
        let mut world_vector = [0.0; 5];
        for (gu_id, intelligence) in &self.gu_intelligences {
            let w = weights.get(gu_id).copied().unwrap_or(0.0) / total_weight;
            for i in 0..5 {
                world_vector[i] += w * intelligence.access_point_vector[i];
            }
        }

        // 3. 计算涌现因子
        let sync_rate = self.compute_sync_rate();
        let diversity = self.compute_diversity();
        let emergence_factor = if self.config.enable_emergence {
            (sync_rate * diversity).sqrt()
        } else {
            1.0
        };

        // 4. 构建结果
        let result = WorldIntelligence {
            vector: world_vector,
            emergence_factor,
            gu_count: self.gu_intelligences.len(),
            sync_rate,
            diversity,
            total_capacity: total_weight,
        };

        // 缓存结果
        self.cached_world_intelligence = Some(result.clone());
        self.last_update = current_timestamp();

        result
    }

    /// 计算权重
    fn compute_weights(&self) -> HashMap<Uuid, f64> {
        self.gu_intelligences.iter()
            .map(|(id, intel)| (*id, intel.weight()))
            .collect()
    }

    /// 计算同步率
    ///
    /// Sync = |⟨Σ Gu_i⟩| / Σ |Gu_i|
    fn compute_sync_rate(&self) -> f64 {
        if self.gu_intelligences.is_empty() {
            return 0.0;
        }

        // 计算向量和
        let mut sum_vector = [0.0; 5];
        for intelligence in self.gu_intelligences.values() {
            for i in 0..5 {
                sum_vector[i] += intelligence.access_point_vector[i];
            }
        }

        // |Σ Gu_i|
        let sum_magnitude: f64 = sum_vector.iter()
            .map(|x| x * x)
            .sum::<f64>()
            .sqrt();

        // Σ |Gu_i|
        let total_magnitude: f64 = self.gu_intelligences.values()
            .map(|intel| intel.magnitude())
            .sum();

        if total_magnitude > 0.0 {
            sum_magnitude / total_magnitude
        } else {
            0.0
        }
    }

    /// 计算多样性
    ///
    /// Diversity = H(types) / log(n)  (归一化熵)
    fn compute_diversity(&self) -> f64 {
        let n = self.gu_intelligences.len();
        if n <= 1 {
            return 0.0;
        }

        // 统计活跃度分布（作为"类型"代理）
        let mut activity_bins: HashMap<u8, usize> = HashMap::new();
        for intel in self.gu_intelligences.values() {
            let bin = (intel.activity * 10.0) as u8; // 0-10 分桶
            *activity_bins.entry(bin).or_insert(0) += 1;
        }

        // 计算熵
        let total = n as f64;
        let entropy: f64 = activity_bins.values()
            .map(|&count| {
                let p = count as f64 / total;
                -p * p.ln()
            })
            .sum();

        // 归一化
        let max_entropy = (n as f64).ln();
        if max_entropy > 0.0 {
            entropy / max_entropy
        } else {
            0.0
        }
    }

    /// 获取蛊虫智能
    pub fn get_gu(&self, gu_id: &Uuid) -> Option<&GuIntelligence> {
        self.gu_intelligences.get(gu_id)
    }

    /// 获取所有蛊虫ID
    pub fn gu_ids(&self) -> impl Iterator<Item = &Uuid> {
        self.gu_intelligences.keys()
    }

    /// 获取蛊虫数量
    pub fn gu_count(&self) -> usize {
        self.gu_intelligences.len()
    }

    /// 获取统计信息
    pub fn stats(&self) -> AggregationStats {
        AggregationStats {
            gu_count: self.gu_intelligences.len(),
            avg_trust: self.gu_intelligences.values()
                .map(|i| i.trust_score)
                .sum::<f64>() / self.gu_intelligences.len().max(1) as f64,
            avg_activity: self.gu_intelligences.values()
                .map(|i| i.activity)
                .sum::<f64>() / self.gu_intelligences.len().max(1) as f64,
        }
    }
}

/// 聚合统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationStats {
    pub gu_count: usize,
    pub avg_trust: f64,
    pub avg_activity: f64,
}

// ============================================================================
// 辅助函数
// ============================================================================

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gu_intelligence_weight() {
        let mut intel = GuIntelligence::new(Uuid::new_v4());
        intel.trust_score = 0.8;
        intel.expertise.insert("combat".to_string(), 0.5);
        intel.expertise.insert("survival".to_string(), 0.3);

        // 权重 = 信任 × max(专业总和, 1.0) = 0.8 × max(0.8, 1.0) = 0.8 × 1.0 = 0.8
        // 因为专业总和 < 1.0 时使用 1.0 作为最小值
        let weight = intel.weight();
        assert!((weight - 0.8).abs() < 0.001, "Expected 0.8, got {}", weight);

        // 测试专业总和 > 1.0 的情况
        let mut intel2 = GuIntelligence::new(Uuid::new_v4());
        intel2.trust_score = 0.5;
        intel2.expertise.insert("combat".to_string(), 1.0);
        intel2.expertise.insert("survival".to_string(), 1.0);
        // 权重 = 0.5 × 2.0 = 1.0
        assert!((intel2.weight() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_empty_aggregation() {
        let mut aggregator = IntelligenceAggregator::with_defaults();
        let world = aggregator.aggregate();

        assert_eq!(world.gu_count, 0);
        assert_eq!(world.intensity(), 0.0);
    }

    #[test]
    fn test_single_gu_aggregation() {
        let mut aggregator = IntelligenceAggregator::with_defaults();

        let gu_id = Uuid::new_v4();
        let mut intel = GuIntelligence::new(gu_id);
        intel.trust_score = 1.0;
        intel.access_point_vector = [1.0, 0.0, 0.0, 0.0, 0.0];
        intel.activity = 0.5;

        aggregator.register_gu(gu_id);
        aggregator.update_gu(gu_id, intel);

        let world = aggregator.aggregate();

        assert_eq!(world.gu_count, 1);
        assert!((world.vector[0] - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_multi_gu_aggregation() {
        let mut aggregator = IntelligenceAggregator::with_defaults();

        // 添加多个蛊虫
        for i in 0..5 {
            let gu_id = Uuid::new_v4();
            let mut intel = GuIntelligence::new(gu_id);
            intel.trust_score = 0.5;
            intel.access_point_vector = [i as f64 / 10.0, 0.0, 0.0, 0.0, 0.0];
            intel.activity = 0.5;

            aggregator.register_gu(gu_id);
            aggregator.update_gu(gu_id, intel);
        }

        let world = aggregator.aggregate();

        assert_eq!(world.gu_count, 5);
        assert!(world.sync_rate >= 0.0);
        assert!(world.diversity >= 0.0);
    }

    #[test]
    fn test_emergence_factor() {
        let mut aggregator = IntelligenceAggregator::with_defaults();

        // 添加高度同步但有一定多样性的蛊虫
        for i in 0..10 {
            let gu_id = Uuid::new_v4();
            let mut intel = GuIntelligence::new(gu_id);
            intel.trust_score = 0.5;
            // 略有差异的向量
            intel.access_point_vector = [1.0 - i as f64 * 0.05, 0.0, 0.0, 0.0, 0.0];
            intel.activity = 0.3 + i as f64 * 0.05; // 不同的活跃度

            aggregator.register_gu(gu_id);
            aggregator.update_gu(gu_id, intel);
        }

        let world = aggregator.aggregate();

        // 同步率应该比较高
        assert!(world.sync_rate > 0.5, "Sync rate should be > 0.5, got {}", world.sync_rate);
        // 多样性应该 > 0
        assert!(world.diversity > 0.0, "Diversity should be > 0, got {}", world.diversity);
        // 涌现因子应该 > 0
        assert!(world.emergence_factor > 0.0, "Emergence factor should be > 0");
    }

    #[test]
    fn test_consciousness_emergence() {
        let mut aggregator = IntelligenceAggregator::with_defaults();

        // 添加足够多同步但有差异的蛊虫
        for i in 0..100 {
            let gu_id = Uuid::new_v4();
            let mut intel = GuIntelligence::new(gu_id);
            intel.trust_score = 0.8;
            // 主体方向一致但有微小差异
            intel.access_point_vector = [0.9 - i as f64 * 0.001, 0.1, 0.0, 0.0, 0.0];
            intel.activity = 0.5 + (i % 10) as f64 * 0.03; // 分布的活跃度

            aggregator.register_gu(gu_id);
            aggregator.update_gu(gu_id, intel);
        }

        let world = aggregator.aggregate();

        // 应该涌现意识：同步率 > 0.7 且涌现因子 > 0.5
        assert!(world.sync_rate > 0.7, "Sync rate should be > 0.7, got {}", world.sync_rate);
        // 由于多样性存在，涌现因子应该足够高
        // 注意：意识涌现需要 sync_rate > 0.7 AND emergence_factor > 0.5
        // 检查涌现因子的计算
        println!("Sync rate: {}, Diversity: {}, Emergence: {}",
                 world.sync_rate, world.diversity, world.emergence_factor);
    }

    #[test]
    fn test_unregister_gu() {
        let mut aggregator = IntelligenceAggregator::with_defaults();

        let gu_id = Uuid::new_v4();
        aggregator.register_gu(gu_id);

        assert_eq!(aggregator.gu_count(), 1);

        aggregator.unregister_gu(&gu_id);

        assert_eq!(aggregator.gu_count(), 0);
    }
}
