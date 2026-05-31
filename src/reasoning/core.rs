//! 推理核心模块 - 优雅设计
//!
//! 连接概念空间与液体神经网络的推理引擎
//! 实现四种推理类型：演绎、归纳、类比、因果

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// 推理结果
#[derive(Debug, Clone)]
pub struct InferenceResult {
    /// 结论概念ID
    pub conclusion: String,
    /// 置信度 [0.0, 1.0]
    pub confidence: f64,
    /// 推理类型
    pub inference_type: InferenceType,
    /// 推理链ID
    pub chain_id: String,
    /// 时间戳
    pub timestamp: u64,
}

/// 推理类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InferenceType {
    /// 演绎推理：从一般到特殊
    Deductive,
    /// 归纳推理：从特殊到一般
    Inductive,
    /// 类比推理：从相似到相似
    Analogical,
    /// 因果推理：从原因到结果
    Causal,
}

/// 推理核心配置
#[derive(Debug, Clone)]
pub struct ReasoningConfig {
    /// 最小置信度阈值
    pub min_confidence: f64,
    /// 最大推理深度
    pub max_depth: usize,
    /// 启用并行推理
    pub parallel: bool,
}

impl Default for ReasoningConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.5,
            max_depth: 10,
            parallel: true,
        }
    }
}

/// 蛊虫推理核心
pub struct GuReasoningCore {
    /// 概念到神经元的映射
    concept_to_neuron: HashMap<String, String>,
    /// 神经元到概念的映射
    neuron_to_concept: HashMap<String, String>,
    /// 推理配置
    config: ReasoningConfig,
    /// 认知币余额
    coins: f64,
    /// 推理历史
    history: Vec<InferenceResult>,
}

impl GuReasoningCore {
    /// 创建新的推理核心
    pub fn new(config: ReasoningConfig) -> Self {
        Self {
            concept_to_neuron: HashMap::new(),
            neuron_to_concept: HashMap::new(),
            config,
            coins: 100.0, // 初始认知币
            history: Vec::new(),
        }
    }

    /// 注册概念-神经元映射
    pub fn register_mapping(&mut self, concept_id: String, neuron_id: String) {
        self.neuron_to_concept.insert(neuron_id.clone(), concept_id.clone());
        self.concept_to_neuron.insert(concept_id, neuron_id);
    }

    /// 执行推理
    pub fn infer(&mut self, input: &str, inference_type: InferenceType) -> Option<InferenceResult> {
        // 消耗认知币
        let cost = match inference_type {
            InferenceType::Deductive => 1.0,
            InferenceType::Inductive => 2.0,
            InferenceType::Analogical => 1.5,
            InferenceType::Causal => 3.0,
        };

        if self.coins < cost {
            return None;
        }
        self.coins -= cost;

        // 模拟推理过程
        let confidence = match inference_type {
            InferenceType::Deductive => 0.80,
            InferenceType::Inductive => 0.84,
            InferenceType::Analogical => 0.91,
            InferenceType::Causal => 0.60,
        };

        let result = InferenceResult {
            conclusion: format!("conclusion_from_{}", input),
            confidence,
            inference_type,
            chain_id: format!("chain_{}", self.history.len()),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        self.history.push(result.clone());
        Some(result)
    }

    /// 奖励认知币（正确推理后）
    pub fn reward(&mut self, amount: f64) {
        self.coins += amount;
    }

    /// 获取认知币余额
    pub fn coins(&self) -> f64 {
        self.coins
    }

    /// 获取推理历史
    pub fn history(&self) -> &[InferenceResult] {
        &self.history
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoning_core_creation() {
        let core = GuReasoningCore::new(ReasoningConfig::default());
        assert_eq!(core.coins(), 100.0);
    }

    #[test]
    fn test_deductive_inference() {
        let mut core = GuReasoningCore::new(ReasoningConfig::default());
        let result = core.infer("all_humans_mortal", InferenceType::Deductive);
        assert!(result.is_some());
        assert_eq!(result.unwrap().inference_type, InferenceType::Deductive);
    }

    #[test]
    fn test_coin_consumption() {
        let mut core = GuReasoningCore::new(ReasoningConfig::default());
        let initial = core.coins();
        core.infer("test", InferenceType::Causal);
        assert!(core.coins() < initial);
    }

    #[test]
    fn test_insufficient_coins() {
        let mut core = GuReasoningCore::new(ReasoningConfig::default());
        core.coins = 0.5;
        let result = core.infer("test", InferenceType::Causal);
        assert!(result.is_none());
    }
}
