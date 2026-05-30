//! 蛊虫推理核心
//!
//! 天才理事会综合设计
//!
//! # 设计理念
//!
//! 根据"万物生成器"设计：
//! - 概念向量驱动神经元活动
//! - LNN进行连续时间推理
//! - 经济系统训练推理能力
//!
//! # 天才理事会分工
//!
//! - 黑塔：创新架构（DCWN, SRU, CMS）
//! - 螺丝咕姆：安全验证
//! - 拉蒂奥：优雅数据结构

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// 重导出配置和类型
pub use crate::core::{LNN, LNNState, NeuronType, PlasticityRule};
pub use crate::language::concept::{ConceptSpace, ConceptLevel};
pub use crate::config::GlobalConfig;

// ============== 黑塔创新模块 ==============
pub mod dcwn;
pub mod sru;
pub mod cms;

pub use dcwn::{DynamicConceptWeavingNetwork, ConceptNode, WeaveConnection, WeaveConfig};
pub use sru::{SemanticResonanceUnderstanding, ConceptWave, ResonanceResult, SruConfig};
pub use cms::{
    CognitiveMarketSystem, MarketConfig, KnowledgeProduct,
    InferenceTask, Transaction, TransactionType,
};

// ============== 拉蒂奥优雅结构模块 ==============
pub mod core_elegant;
pub mod chain;
pub mod hierarchy;

pub use core_elegant::{GuReasoningCore as ElegantReasoningCore, InferenceResult, InferenceType, ReasoningConfig};
pub use chain::{ReasoningChain, ReasoningStep, ReasoningRule, ChainBuilder};
pub use hierarchy::{KnowledgeHierarchy, KnowledgeNode, KnowledgeLevel as KLevel, KnowledgeSource};

// ============== 螺丝咕姆安全模块 ==============
pub mod validation;
pub mod drift;
pub mod protocol;

pub use validation::{InferenceValidator, ValidationResult, ValidationLevel};
pub use drift::{VectorDriftDetector, DriftConfig, DriftResult};
pub use protocol::{
    CollaborationProtocol, CollaborationMessage, ConsensusRequest,
    GuProfile, GuId, AnomalyRecord, AnomalyType,
};

/// 推理结果（兼容旧版）
#[derive(Debug, Clone)]
pub struct LegacyInferenceResult {
    /// 推理结论
    pub conclusion: String,
    /// 置信度
    pub confidence: f64,
    /// 推理路径
    pub reasoning_path: Vec<String>,
    /// 激活的概念
    pub activated_concepts: Vec<String>,
}

/// 蛊虫推理核心
///
/// 连接概念空间和LNN，实现真正的推理
pub struct GuReasoningCore {
    /// 概念空间
    concept_space: Arc<RwLock<ConceptSpace>>,
    /// LNN网络
    lnn: LNN,
    /// 概念ID到神经元ID的映射
    concept_to_neuron: HashMap<String, String>,
    /// 神经元ID到概念ID的映射
    neuron_to_concept: HashMap<String, String>,
    /// 配置
    config: GlobalConfig,
    /// 金币（用于经济系统）
    coins: f64,
    /// 推理历史
    reasoning_history: Vec<LegacyInferenceResult>,
    /// 验证器
    validator: InferenceValidator,
    /// 认知市场
    market: CognitiveMarketSystem,
}

impl GuReasoningCore {
    /// 创建新的推理核心
    pub fn new() -> Self {
        Self {
            concept_space: Arc::new(RwLock::new(ConceptSpace::new())),
            lnn: LNN::new(None, None),
            concept_to_neuron: HashMap::new(),
            neuron_to_concept: HashMap::new(),
            config: GlobalConfig::new(),
            coins: 0.0,
            reasoning_history: Vec::new(),
            validator: InferenceValidator::new(),
            market: CognitiveMarketSystem::new(MarketConfig::default()),
        }
    }

    /// 从概念空间创建推理核心
    pub fn from_concept_space(space: ConceptSpace) -> Self {
        let mut core = Self::new();
        core.concept_space = Arc::new(RwLock::new(space));
        core.build_neural_network();
        core
    }

    /// 构建神经网络
    pub fn build_neural_network(&mut self) {
        let space = self.concept_space.read().unwrap();

        for (concept_id, concept) in space.all_concepts() {
            let neuron_type = match concept.level {
                ConceptLevel::SystemCore => NeuronType::Perception,
                ConceptLevel::Basic => NeuronType::Cognitive,
                ConceptLevel::Common => NeuronType::Cognitive,
                ConceptLevel::Domain => NeuronType::Behavior,
                ConceptLevel::Temporary => NeuronType::Behavior,
            };

            if let Ok(neuron_id) = self.lnn.add_neuron(neuron_type) {
                self.concept_to_neuron.insert(concept_id.clone(), neuron_id.clone());
                self.neuron_to_concept.insert(neuron_id, concept_id.clone());
            }
        }

        for (concept_id, concept) in space.all_concepts() {
            if let Some(parent_id) = &concept.parent_id {
                if let (Some(from_neuron), Some(to_neuron)) = (
                    self.concept_to_neuron.get(parent_id),
                    self.concept_to_neuron.get(concept_id),
                ) {
                    let _ = self.lnn.add_synapse(
                        from_neuron,
                        to_neuron,
                        0.5,
                        PlasticityRule::Hebbian,
                    );
                }
            }
        }
    }

    /// 执行推理（带验证）
    pub fn reason(
        &mut self,
        input_concepts: &[&str],
        inference_type: InferenceType,
        steps: usize,
    ) -> Option<LegacyInferenceResult> {
        let mut reasoning_path = Vec::new();
        let mut activated_concepts = Vec::new();

        for concept_id in input_concepts {
            if let Some(_neuron_id) = self.concept_to_neuron.get(*concept_id) {
                reasoning_path.push(format!("激活概念: {}", concept_id));
                activated_concepts.push(concept_id.to_string());
            }
        }

        for step in 0..steps {
            if self.lnn.update(0.01).is_err() {
                break;
            }
            reasoning_path.push(format!("推理步 {}/{}", step + 1, steps));
        }

        let space = self.concept_space.read().unwrap();
        for concept_id in input_concepts {
            if space.get_concept(concept_id).is_some() {
                if let Some(similar) = space.find_similar(concept_id, 3) {
                    for (sim_id, sim_score) in similar {
                        if !activated_concepts.contains(&sim_id) && sim_score > 0.5 {
                            activated_concepts.push(sim_id.clone());
                            reasoning_path.push(format!("关联激活: {} (相似度: {:.3})", sim_id, sim_score));
                        }
                    }
                }
            }
        }

        let (conclusion, confidence) = match inference_type {
            InferenceType::Deductive => self.deductive_inference(input_concepts, &activated_concepts),
            InferenceType::Inductive => self.inductive_inference(input_concepts, &activated_concepts),
            InferenceType::Analogical => self.analogical_inference(input_concepts, &activated_concepts),
            InferenceType::Causal => self.causal_inference(input_concepts, &activated_concepts),
        };

        // 四层验证
        let validation_results = self.validator.validate(
            &input_concepts.join(","),
            &conclusion,
            confidence,
            &reasoning_path,
        );

        // 如果验证失败，降低置信度
        let final_confidence = if self.validator.is_valid(&validation_results) {
            confidence
        } else {
            confidence * 0.5
        };

        let result = LegacyInferenceResult {
            conclusion,
            confidence: final_confidence,
            reasoning_path,
            activated_concepts,
        };

        self.reasoning_history.push(result.clone());
        Some(result)
    }

    fn deductive_inference(&self, inputs: &[&str], _activated: &[String]) -> (String, f64) {
        let space = self.concept_space.read().unwrap();

        for input in inputs {
            if let Some(concept) = space.get_concept(input) {
                let children: Vec<_> = space.all_concepts()
                    .filter(|(_, c)| c.parent_id.as_deref() == Some(*input))
                    .collect();

                if !children.is_empty() {
                    let child_names: Vec<String> = children.iter()
                        .map(|(_, c)| c.name.clone())
                        .collect();
                    return (
                        format!("根据'{}'可以推导出: {}", concept.name, child_names.join(", ")),
                        0.8,
                    );
                }
            }
        }

        ("无法进行演绎推理，缺少足够的规则".to_string(), 0.3)
    }

    fn inductive_inference(&self, inputs: &[&str], _activated: &[String]) -> (String, f64) {
        let space = self.concept_space.read().unwrap();

        if inputs.len() < 2 {
            return ("归纳推理需要至少两个输入".to_string(), 0.0);
        }

        let mut similarities = Vec::new();
        for i in 0..inputs.len() {
            for j in (i + 1)..inputs.len() {
                if let Some(sim) = space.similarity(inputs[i], inputs[j]) {
                    similarities.push(sim);
                }
            }
        }

        if !similarities.is_empty() {
            let avg_sim: f64 = similarities.iter().sum::<f64>() / similarities.len() as f64;
            return (
                format!("这些概念具有 {:.0}% 的共性，可能属于同一类别", avg_sim * 100.0),
                avg_sim,
            );
        }

        ("无法找到共同特征".to_string(), 0.2)
    }

    fn analogical_inference(&self, inputs: &[&str], _activated: &[String]) -> (String, f64) {
        let space = self.concept_space.read().unwrap();

        if inputs.is_empty() {
            return ("类比推理需要输入".to_string(), 0.0);
        }

        if let Some(input) = inputs.first() {
            if let Some(similar) = space.find_similar(input, 1) {
                if let Some((sim_id, sim_score)) = similar.first() {
                    if *sim_score > 0.5 {
                        return (
                            format!("'{}' 与 '{}' 相似，可以借鉴其特性", input, sim_id),
                            *sim_score,
                        );
                    }
                }
            }
        }

        ("未找到足够的相似概念进行类比".to_string(), 0.2)
    }

    fn causal_inference(&self, inputs: &[&str], activated: &[String]) -> (String, f64) {
        if activated.len() > inputs.len() * 2 {
            return (
                format!("输入概念'{}'可能引发连锁反应，涉及 {} 个相关概念",
                    inputs.join("', '"), activated.len()),
                0.6,
            );
        }

        ("因果关系不明显".to_string(), 0.3)
    }

    /// 学习新知识（获得金币）
    pub fn learn_knowledge(&mut self, _concept_id: &str, content: &str) -> Result<f64, String> {
        let reward = content.len() as f64 * 0.01;
        self.coins += reward;
        Ok(reward)
    }

    /// 执行任务（消费金币，训练推理）
    pub fn execute_task(&mut self, task: &str, reward: f64) -> Option<LegacyInferenceResult> {
        let result = self.reason(&[task], InferenceType::Deductive, 10);

        if let Some(ref r) = result {
            if r.confidence > 0.5 {
                self.coins += reward;
            }
        }

        result
    }

    /// 获取金币数量
    pub fn get_coins(&self) -> f64 {
        self.coins
    }

    /// 获取概念空间
    pub fn concept_space(&self) -> Arc<RwLock<ConceptSpace>> {
        Arc::clone(&self.concept_space)
    }

    /// 获取推理历史
    pub fn reasoning_history(&self) -> &[LegacyInferenceResult] {
        &self.reasoning_history
    }

    /// 获取网络状态
    pub fn network_state(&self) -> LNNState {
        self.lnn.get_state()
    }
}

impl Default for GuReasoningCore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_reasoning_core() {
        let core = GuReasoningCore::new();
        assert_eq!(core.get_coins(), 0.0);
    }

    #[test]
    fn test_build_network() {
        let mut space = ConceptSpace::new();
        space.create_concept("parent".to_string(), "父概念".to_string(), ConceptLevel::Basic).unwrap();
        space.create_child_concept("parent", "child".to_string(), "子概念".to_string()).unwrap();

        let mut core = GuReasoningCore::from_concept_space(space);
        let state = core.network_state();

        assert_eq!(state.neuron_count, 2);
        assert_eq!(state.synapse_count, 1);
    }

    #[test]
    fn test_deductive_inference() {
        let mut space = ConceptSpace::new();
        space.create_concept("fruit".to_string(), "水果".to_string(), ConceptLevel::Basic).unwrap();
        space.create_child_concept("fruit", "apple".to_string(), "苹果".to_string()).unwrap();

        let mut core = GuReasoningCore::from_concept_space(space);

        let result = core.reason(&["fruit"], InferenceType::Deductive, 5);
        assert!(result.is_some());

        let r = result.unwrap();
        assert!(r.conclusion.contains("苹果"));
    }

    #[test]
    fn test_economic_system() {
        let mut core = GuReasoningCore::new();

        let reward = core.learn_knowledge("test", "这是一段知识内容").unwrap();
        assert!(reward > 0.0);
        assert!(core.get_coins() > 0.0);
    }

    #[test]
    fn test_validation() {
        let core = GuReasoningCore::new();
        let validator = InferenceValidator::new();

        let results = validator.validate("input", "output", 0.9, &[]);
        assert!(validator.is_valid(&results));
    }

    #[test]
    fn test_cognitive_market() {
        let mut market = CognitiveMarketSystem::new(MarketConfig::default());
        market.register_gu("test_gu".to_string());
        assert_eq!(market.balance("test_gu"), 100.0);
    }
}
