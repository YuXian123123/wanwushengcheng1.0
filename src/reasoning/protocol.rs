//! 安全协作协议模块 - 安全设计
//!
//! 多蛊虫协作：签名验证、共识机制、异常隔离

use std::collections::{HashMap, HashSet};

/// 蛊虫ID
pub type GuId = String;

/// 消息签名
#[derive(Debug, Clone)]
pub struct Signature {
    /// 签名者ID
    pub signer: GuId,
    /// 签名数据
    pub data: Vec<u8>,
    /// 时间戳
    pub timestamp: u64,
}

/// 协作消息
#[derive(Debug, Clone)]
pub struct CollaborationMessage {
    /// 发送者ID
    pub sender: GuId,
    /// 接收者ID（空为广播）
    pub receiver: Option<GuId>,
    /// 消息类型
    pub message_type: MessageType,
    /// 内容
    pub content: String,
    /// 签名
    pub signature: Option<Signature>,
}

/// 消息类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    /// 推理请求
    InferenceRequest,
    /// 推理响应
    InferenceResponse,
    /// 知识共享
    KnowledgeShare,
    /// 共识投票
    ConsensusVote,
    /// 心跳
    Heartbeat,
    /// 异常报告
    AnomalyReport,
}

/// 蛊虫档案
#[derive(Debug, Clone)]
pub struct GuProfile {
    /// ID
    pub id: GuId,
    /// 基础权重
    pub base_weight: f64,
    /// 历史准确率
    pub accuracy: f64,
    /// 领域专业度
    pub expertise: HashMap<String, f64>,
    /// 是否被隔离
    pub isolated: bool,
}

/// 共识请求
#[derive(Debug, Clone)]
pub struct ConsensusRequest {
    /// 请求ID
    pub request_id: String,
    /// 提议内容
    pub proposal: String,
    /// 发起者
    pub proposer: GuId,
    /// 投票截止时间
    pub deadline: u64,
    /// 已投票
    pub votes: HashMap<GuId, bool>,
}

impl ConsensusRequest {
    /// 创建新的共识请求
    pub fn new(proposal: String, proposer: GuId) -> Self {
        Self {
            request_id: format!("consensus_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()),
            proposal,
            proposer,
            deadline: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() + 300, // 5分钟截止
            votes: HashMap::new(),
        }
    }

    /// 投票
    pub fn vote(&mut self, voter: GuId, approve: bool) {
        self.votes.insert(voter, approve);
    }

    /// 检查是否达成共识
    pub fn check_consensus(&self, total_voters: usize, threshold: f64) -> bool {
        if self.votes.len() < total_voters {
            return false;
        }

        let approve_count = self.votes.values().filter(|&&v| v).count();
        let ratio = approve_count as f64 / total_voters as f64;
        ratio >= threshold
    }
}

/// 异常类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnomalyType {
    /// 置信度持续过低
    LowConfidence,
    /// 向量漂移异常
    VectorDrift,
    /// 投票模式偏离
    VoteDeviation,
    /// 通信延迟异常
    CommunicationDelay,
}

/// 异常记录
#[derive(Debug, Clone)]
pub struct AnomalyRecord {
    /// 蛊虫ID
    pub gu_id: GuId,
    /// 异常类型
    pub anomaly_type: AnomalyType,
    /// 时间戳
    pub timestamp: u64,
    /// 详情
    pub details: String,
}

/// 协作协议
pub struct CollaborationProtocol {
    /// 蛊虫注册表
    registry: HashMap<GuId, GuProfile>,
    /// 异常检测器
    anomaly_records: Vec<AnomalyRecord>,
    /// 隔离列表
    isolated: HashSet<GuId>,
    /// 共识阈值
    consensus_threshold: f64,
}

impl CollaborationProtocol {
    /// 创建新的协作协议
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            anomaly_records: Vec::new(),
            isolated: HashSet::new(),
            consensus_threshold: 0.6, // 60%通过
        }
    }

    /// 注册蛊虫
    pub fn register(&mut self, profile: GuProfile) {
        self.registry.insert(profile.id.clone(), profile);
    }

    /// 注销蛊虫
    pub fn unregister(&mut self, gu_id: &GuId) {
        self.registry.remove(gu_id);
        self.isolated.remove(gu_id);
    }

    /// 验证消息
    pub fn verify_message(&self, message: &CollaborationMessage) -> Result<(), String> {
        // 检查发送者是否已注册
        if !self.registry.contains_key(&message.sender) {
            return Err("发送者未注册".to_string());
        }

        // 检查发送者是否被隔离
        if self.isolated.contains(&message.sender) {
            return Err("发送者已被隔离".to_string());
        }

        // 验证签名（简化版）
        if message.signature.is_none() {
            return Err("消息缺少签名".to_string());
        }

        Ok(())
    }

    /// 报告异常
    pub fn report_anomaly(&mut self, record: AnomalyRecord) {
        // 记录异常
        self.anomaly_records.push(record.clone());

        // 检查是否需要隔离
        let recent_anomalies = self.anomaly_records.iter()
            .filter(|r| r.gu_id == record.gu_id)
            .count();

        if recent_anomalies >= 3 {
            self.isolate(&record.gu_id);
        }
    }

    /// 隔离蛊虫
    pub fn isolate(&mut self, gu_id: &GuId) {
        self.isolated.insert(gu_id.clone());
        if let Some(profile) = self.registry.get_mut(gu_id) {
            profile.isolated = true;
        }
    }

    /// 解除隔离
    pub fn release(&mut self, gu_id: &GuId) {
        self.isolated.remove(gu_id);
        if let Some(profile) = self.registry.get_mut(gu_id) {
            profile.isolated = false;
        }
    }

    /// 计算投票权重
    pub fn calculate_weight(&self, gu_id: &GuId) -> f64 {
        if let Some(profile) = self.registry.get(gu_id) {
            profile.base_weight * profile.accuracy
        } else {
            0.0
        }
    }

    /// 发起共识
    pub fn initiate_consensus(&self, proposal: String, proposer: GuId) -> ConsensusRequest {
        ConsensusRequest::new(proposal, proposer)
    }

    /// 执行共识检查
    pub fn check_consensus(&self, request: &ConsensusRequest) -> bool {
        let active_voters = self.registry.keys()
            .filter(|id| !self.isolated.contains(*id))
            .count();

        request.check_consensus(active_voters, self.consensus_threshold)
    }

    /// 获取活跃蛊虫数量
    pub fn active_count(&self) -> usize {
        self.registry.keys()
            .filter(|id| !self.isolated.contains(*id))
            .count()
    }

    /// 获取隔离蛊虫数量
    pub fn isolated_count(&self) -> usize {
        self.isolated.len()
    }
}

impl Default for CollaborationProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_creation() {
        let protocol = CollaborationProtocol::new();
        assert_eq!(protocol.active_count(), 0);
    }

    #[test]
    fn test_registration() {
        let mut protocol = CollaborationProtocol::new();
        protocol.register(GuProfile {
            id: "gu_1".to_string(),
            base_weight: 1.0,
            accuracy: 0.9,
            expertise: HashMap::new(),
            isolated: false,
        });

        assert_eq!(protocol.active_count(), 1);
    }

    #[test]
    fn test_isolation() {
        let mut protocol = CollaborationProtocol::new();
        protocol.register(GuProfile {
            id: "gu_1".to_string(),
            base_weight: 1.0,
            accuracy: 0.9,
            expertise: HashMap::new(),
            isolated: false,
        });

        protocol.isolate(&"gu_1".to_string());
        assert_eq!(protocol.isolated_count(), 1);
        assert_eq!(protocol.active_count(), 0);
    }

    #[test]
    fn test_message_verification() {
        let mut protocol = CollaborationProtocol::new();
        protocol.register(GuProfile {
            id: "gu_1".to_string(),
            base_weight: 1.0,
            accuracy: 0.9,
            expertise: HashMap::new(),
            isolated: false,
        });

        let message = CollaborationMessage {
            sender: "gu_1".to_string(),
            receiver: None,
            message_type: MessageType::Heartbeat,
            content: "ping".to_string(),
            signature: Some(Signature {
                signer: "gu_1".to_string(),
                data: vec![],
                timestamp: 0,
            }),
        };

        assert!(protocol.verify_message(&message).is_ok());
    }

    #[test]
    fn test_consensus() {
        let mut protocol = CollaborationProtocol::new();

        // 注册3个蛊虫
        for i in 1..=3 {
            protocol.register(GuProfile {
                id: format!("gu_{}", i),
                base_weight: 1.0,
                accuracy: 0.9,
                expertise: HashMap::new(),
                isolated: false,
            });
        }

        let mut request = protocol.initiate_consensus(
            "测试提议".to_string(),
            "gu_1".to_string(),
        );

        request.vote("gu_1".to_string(), true);
        request.vote("gu_2".to_string(), true);
        request.vote("gu_3".to_string(), false);

        // 2/3 > 60%
        assert!(protocol.check_consensus(&request));
    }

    #[test]
    fn test_anomaly_isolation() {
        let mut protocol = CollaborationProtocol::new();
        protocol.register(GuProfile {
            id: "gu_1".to_string(),
            base_weight: 1.0,
            accuracy: 0.9,
            expertise: HashMap::new(),
            isolated: false,
        });

        // 报告3次异常
        for _ in 0..3 {
            protocol.report_anomaly(AnomalyRecord {
                gu_id: "gu_1".to_string(),
                anomaly_type: AnomalyType::LowConfidence,
                timestamp: 0,
                details: "".to_string(),
            });
        }

        assert_eq!(protocol.isolated_count(), 1);
    }
}
