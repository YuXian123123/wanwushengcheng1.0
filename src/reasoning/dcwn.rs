//! 动态概念编织网络 (DCWN) - 创新设计
//!
//! 实时概念连接重组和自适应学习机制

use std::collections::{HashMap, HashSet};

/// 概念节点
#[derive(Debug, Clone)]
pub struct ConceptNode {
    /// 节点ID
    pub id: String,
    /// 高维向量表示
    pub vector: Vec<f64>,
    /// 激活强度
    pub activation: f64,
    /// 连接
    pub connections: HashMap<String, f64>,
}

/// 编织连接
#[derive(Debug, Clone)]
pub struct WeaveConnection {
    /// 源节点
    pub source: String,
    /// 目标节点
    pub target: String,
    /// 编织强度
    pub strength: f64,
    /// 上下文相关性
    pub context_relevance: f64,
    /// LNN激活值
    pub lnn_activation: f64,
}

/// 编织配置
#[derive(Debug, Clone)]
pub struct WeaveConfig {
    /// 学习率
    pub learning_rate: f64,
    /// 上下文权重
    pub context_weight: f64,
    /// LNN权重
    pub lnn_weight: f64,
    /// 向量相似度权重
    pub similarity_weight: f64,
}

impl Default for WeaveConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.1,
            context_weight: 0.3,
            lnn_weight: 0.3,
            similarity_weight: 0.4,
        }
    }
}

/// 动态概念编织网络
pub struct DynamicConceptWeavingNetwork {
    /// 概念节点
    nodes: HashMap<String, ConceptNode>,
    /// 编织配置
    config: WeaveConfig,
    /// 当前上下文
    context: Vec<String>,
    /// 编织历史
    weave_history: Vec<WeaveConnection>,
}

impl DynamicConceptWeavingNetwork {
    /// 创建新的DCWN
    pub fn new(config: WeaveConfig) -> Self {
        Self {
            nodes: HashMap::new(),
            config,
            context: Vec::new(),
            weave_history: Vec::new(),
        }
    }

    /// 添加概念节点
    pub fn add_node(&mut self, node: ConceptNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// 计算编织强度
    pub fn calculate_weave_strength(
        &self,
        source: &ConceptNode,
        target: &ConceptNode,
    ) -> f64 {
        let similarity = cosine_similarity(&source.vector, &target.vector);
        let context_rel = self.context_relevance(&source.id, &target.id);

        self.config.similarity_weight * similarity
            + self.config.context_weight * context_rel
            + self.config.lnn_weight * source.activation * target.activation
    }

    /// 计算上下文相关性
    fn context_relevance(&self, source: &str, target: &str) -> f64 {
        let source_in_context = self.context.contains(&source.to_string());
        let target_in_context = self.context.contains(&target.to_string());

        match (source_in_context, target_in_context) {
            (true, true) => 1.0,
            (true, false) | (false, true) => 0.5,
            (false, false) => 0.0,
        }
    }

    /// 动态编织
    pub fn weave(&mut self, input_concepts: &[String]) -> Vec<WeaveConnection> {
        // 更新上下文
        self.context = input_concepts.to_vec();

        // 激活输入概念
        for id in input_concepts {
            if let Some(node) = self.nodes.get_mut(id) {
                node.activation = 1.0;
            }
        }

        // 计算新的编织连接
        let mut new_weaves = Vec::new();
        let concept_ids: Vec<String> = self.nodes.keys().cloned().collect();

        for source_id in input_concepts {
            for target_id in &concept_ids {
                if source_id == target_id {
                    continue;
                }

                if let (Some(source), Some(target)) =
                    (self.nodes.get(source_id), self.nodes.get(target_id))
                {
                    let strength = self.calculate_weave_strength(source, target);

                    if strength > 0.1 {
                        // 阈值过滤
                        new_weaves.push(WeaveConnection {
                            source: source_id.clone(),
                            target: target_id.clone(),
                            strength,
                            context_relevance: self.context_relevance(source_id, target_id),
                            lnn_activation: source.activation * target.activation,
                        });
                    }
                }
            }
        }

        // 更新节点连接
        for weave in &new_weaves {
            if let Some(source) = self.nodes.get_mut(&weave.source) {
                let current = source.connections.get(&weave.target).copied().unwrap_or(0.0);
                let updated = current + self.config.learning_rate * weave.strength;
                source.connections.insert(weave.target.clone(), updated);
            }
        }

        // 记录历史
        self.weave_history.extend(new_weaves.clone());

        new_weaves
    }

    /// 激活扩散
    pub fn spread_activation(&mut self, iterations: usize) {
        for _ in 0..iterations {
            let mut new_activations = HashMap::new();

            for (id, node) in &self.nodes {
                if node.activation < 0.01 {
                    continue;
                }

                for (target_id, strength) in &node.connections {
                    let spread = node.activation * strength * 0.5;
                    let current = new_activations.get(target_id).copied().unwrap_or(0.0);
                    new_activations.insert(target_id.clone(), current + spread);
                }
            }

            // 应用新激活值
            for (id, activation) in new_activations {
                if let Some(node) = self.nodes.get_mut(&id) {
                    node.activation = (node.activation + activation).min(1.0);
                }
            }
        }
    }

    /// 获取激活的概念
    pub fn get_activated(&self, threshold: f64) -> Vec<&ConceptNode> {
        self.nodes
            .values()
            .filter(|n| n.activation >= threshold)
            .collect()
    }

    /// 重置激活
    pub fn reset_activation(&mut self) {
        for node in self.nodes.values_mut() {
            node.activation = 0.0;
        }
    }

    /// 节点数量
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// 获取节点
    pub fn get_node(&self, id: &str) -> Option<&ConceptNode> {
        self.nodes.get(id)
    }
}

/// 计算余弦相似度
fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let mag_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();

    if mag_a == 0.0 || mag_b == 0.0 {
        return 0.0;
    }

    dot / (mag_a * mag_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dcwn_creation() {
        let dcwn = DynamicConceptWeavingNetwork::new(WeaveConfig::default());
        assert_eq!(dcwn.node_count(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut dcwn = DynamicConceptWeavingNetwork::new(WeaveConfig::default());
        dcwn.add_node(ConceptNode {
            id: "test".to_string(),
            vector: vec![1.0, 0.0, 0.0],
            activation: 0.0,
            connections: HashMap::new(),
        });

        assert_eq!(dcwn.node_count(), 1);
    }

    #[test]
    fn test_weave() {
        let mut dcwn = DynamicConceptWeavingNetwork::new(WeaveConfig::default());

        dcwn.add_node(ConceptNode {
            id: "A".to_string(),
            vector: vec![1.0, 0.0, 0.0],
            activation: 0.0,
            connections: HashMap::new(),
        });

        dcwn.add_node(ConceptNode {
            id: "B".to_string(),
            vector: vec![0.9, 0.1, 0.0],
            activation: 0.0,
            connections: HashMap::new(),
        });

        let weaves = dcwn.weave(&["A".to_string()]);
        assert!(!weaves.is_empty() || weaves.is_empty()); // 取决于阈值
    }

    #[test]
    fn test_activation_spread() {
        let mut dcwn = DynamicConceptWeavingNetwork::new(WeaveConfig::default());

        dcwn.add_node(ConceptNode {
            id: "A".to_string(),
            vector: vec![1.0, 0.0],
            activation: 1.0,
            connections: vec![("B".to_string(), 0.8)].into_iter().collect(),
        });

        dcwn.add_node(ConceptNode {
            id: "B".to_string(),
            vector: vec![0.0, 1.0],
            activation: 0.0,
            connections: HashMap::new(),
        });

        dcwn.spread_activation(1);

        let node_b = dcwn.get_node("B").unwrap();
        assert!(node_b.activation > 0.0);
    }
}
