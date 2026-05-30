//! 意图解析

use serde::{Deserialize, Serialize};

/// 意图类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentType {
    /// 声明
    Statement,
    /// 提问
    Question,
    /// 请求
    Request,
    /// 回答
    Answer,
    /// 确认
    Confirmation,
    /// 否认
    Denial,
}

/// 意图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    /// 意图类型
    pub intent_type: IntentType,
    /// 置信度
    pub confidence: f64,
    /// 关键概念
    pub key_concepts: Vec<String>,
}

impl Intent {
    /// 创建新意图
    pub fn new(intent_type: IntentType, confidence: f64, key_concepts: Vec<String>) -> Self {
        Self { intent_type, confidence, key_concepts }
    }

    /// 从向量解析意图（简化版）
    pub fn from_vector(vector: &[f64]) -> Self {
        // 简化实现：基于向量统计特征
        let mean: f64 = vector.iter().sum::<f64>() / vector.len() as f64;

        let intent_type = if mean > 0.1 {
            IntentType::Statement
        } else if mean < -0.1 {
            IntentType::Question
        } else {
            IntentType::Statement
        };

        Self {
            intent_type,
            confidence: 0.5,
            key_concepts: Vec::new(),
        }
    }
}
