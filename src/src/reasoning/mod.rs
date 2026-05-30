//! 蛊虫推理核心
//!
//! 将概念空间与LNN结合，实现真正的推理能力
//!
//! # 设计理念
//!
//! 根据"万物生成器"设计：
//! - 概念向量驱动神经元活动
//! - LNN进行连续时间推理
//! - 经济系统训练推理能力

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::core::{LNN, LNNState, NeuronType, PlasticityRule};
use crate::language::concept::{ConceptSpace, ConceptLevel};
use crate::config::GlobalConfig;

/// 推理结果
#[derive(Debug, Clone)]
pub struct InferenceResult {
    /// 推理结论
    pub conclusion: String,
    /// 置信度
    pub confidence: f64,
    /// 推理路径
    pub reasoning_path: Vec<String>,
    /// 激活的概念
    pub activated_concepts: Vec<String>,
}

/// 推理类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InferenceType {
    /// 演绎推理 - 从一般到特殊
    Deductive,
    /// 归纳推理 - 从特殊到一般
    Inductive,
    /// 类比推理 - 相似性迁移
    Analogical,
    /// 因果推理 - 因果关系推断
    Causal,
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
    reasoning_history: Vec<InferenceResult>,
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
    ///
    /// 为每个概念创建对应的神经元
    pub fn build_neural_network(&mut self) {
        let space = self.concept_space.read().unwrap();

        // 为每个概念创建神经元
        for (concept_id, concept) in space.all_concepts() {
            // 根据概念层级选择神经元类型
            let neuron_type = match concept.level {
                ConceptLevel::SystemCore => NeuronType::Perception,
                ConceptLevel::Basic => NeuronType::Cognitive,
                ConceptLevel::Common => NeuronType::Cognitive,
                ConceptLevel::Domain => NeuronType::Behavior,
                ConceptLevel::Temporary => NeuronType::Behavior,
            };

            // 创建神经元
            if let Ok(neuron_id) = self.lnn.add_neuron(neuron_type) {
                self.concept_to_neuron.insert(concept_id.clone(), neuron_id.clone());
                self.neuron_to_concept.insert(neuron_id, concept_id.clone());
            }
        }

        // 为有父子关系的概念创建突触连接
        for (concept_id, concept) in space.all_concepts() {
            if let Some(parent_id) = &concept.parent_id {
                // 父概念 -> 子概念 连接
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

    /// 执行推理
    ///
    /// # 参数
    /// - `input_concepts`: 输入概念列表
    /// - `inference_type`: 推理类型
    /// - `steps`: 推理步数
    pub fn reason(
        &mut self,
        input_concepts: &[&str],
        inference_type: InferenceType,
        steps: usize,
    ) -> Option<InferenceResult> {
        let mut reasoning_path = Vec::new();
        let mut activated_concepts = Vec::new();

        // 1. 激活输入概念对应的神经元
        for concept_id in input_concepts {
            if let Some(neuron_id) = self.concept_to_neuron.get(*concept_id) {
                reasoning_path.push(format!("激活概念: {}", concept_id));
                activated_concepts.push(concept_id.to_string());
                // TODO: 设置神经元状态为激活
            }
        }

        // 2. 运行LNN推理
        for step in 0..steps {
            if self.lnn.update(0.01).is_err() {
                break;
            }
            reasoning_path.push(format!("推理步 {}/{}", step + 1, steps));
        }

        // 3. 收集激活的概念
        let space = self.concept_space.read().unwrap();
        for concept_id in input_concepts {
            if space.get_concept(concept_id).is_some() {
                // 寻找相似概念
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

        // 4. 根据推理类型生成结论
        let (conclusion, confidence) = match inference_type {
            InferenceType::Deductive => {
                self.deductive_inference(input_concepts, &activated_concepts)
            }
            InferenceType::Inductive => {
                self.inductive_inference(input_concepts, &activated_concepts)
            }
            InferenceType::Analogical => {
                self.analogical_inference(input_concepts, &activated_concepts)
            }
            InferenceType::Causal => {
                self.causal_inference(input_concepts, &activated_concepts)
            }
        };

        let result = InferenceResult {
            conclusion,
            confidence,
            reasoning_path,
            activated_concepts,
        };

        self.reasoning_history.push(result.clone());
        Some(result)
    }

    /// 演绎推理
    fn deductive_inference(
        &self,
        inputs: &[&str],
        activated: &[String],
    ) -> (String, f64) {
        // 从一般到特殊：如果输入包含父概念，推导子概念
        let space = self.concept_space.read().unwrap();

        for input in inputs {
            if let Some(concept) = space.get_concept(input) {
                // 检查是否有子概念
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

    /// 归纳推理
    fn inductive_inference(
        &self,
        inputs: &[&str],
        activated: &[String],
    ) -> (String, f64) {
        // 从特殊到一般：寻找共同特征
        let space = self.concept_space.read().unwrap();

        if inputs.len() < 2 {
            return ("归纳推理需要至少两个输入".to_string(), 0.0);
        }

        // 计算输入概念的相似度
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

    /// 类比推理
    fn analogical_inference(
        &self,
        inputs: &[&str],
        activated: &[String],
    ) -> (String, f64) {
        // 基于相似性进行迁移
        let space = self.concept_space.read().unwrap();

        if inputs.is_empty() {
            return ("类比推理需要输入".to_string(), 0.0);
        }

        // 寻找最相似的概念
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

    /// 因果推理
    fn causal_inference(
        &self,
        inputs: &[&str],
        activated: &[String],
    ) -> (String, f64) {
        // 基于概念关联进行因果推断
        let activated_set: std::collections::HashSet<_> = activated.iter().cloned().collect();

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
    pub fn learn_knowledge(&mut self, concept_id: &str, content: &str) -> Result<f64, String> {
        // 学习知识获得金币奖励
        let reward = content.len() as f64 * 0.01;
        self.coins += reward;

        Ok(reward)
    }

    /// 执行任务（消费金币，训练推理）
    pub fn execute_task(&mut self, task: &str, reward: f64) -> Option<InferenceResult> {
        // 执行任务需要消耗推理能力
        // 成功则获得金币

        let result = self.reason(
            &[task],
            InferenceType::Deductive,
            10,
        );

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
    pub fn reasoning_history(&self) -> &[InferenceResult] {
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
    fn test_analogical_inference() {
        let mut space = ConceptSpace::new();
        space.create_concept("apple".to_string(), "苹果".to_string(), ConceptLevel::Common).unwrap();
        space.create_concept("banana".to_string(), "香蕉".to_string(), ConceptLevel::Common).unwrap();

        // 建立关联
        let _ = space.learn_association("apple", "banana", 1.0);

        let mut core = GuReasoningCore::from_concept_space(space);

        let result = core.reason(&["apple"], InferenceType::Analogical, 5);
        assert!(result.is_some());
    }

    #[test]
    fn test_economic_system() {
        let mut core = GuReasoningCore::new();

        // 学习获得金币
        let reward = core.learn_knowledge("test", "这是一段知识内容").unwrap();
        assert!(reward > 0.0);
        assert!(core.get_coins() > 0.0);
    }
}
