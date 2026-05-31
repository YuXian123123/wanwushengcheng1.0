//! 意识层模块 - 世界智能体的统一意识
//!
//! 实现意图融合、决策统一、冲突解决
//!
//! 拉蒂奥设计的向量化决策公式：
//! D_world = ⟨D|W⟩ / ⟨W|W⟩^0.5

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::state::{Decision, Intention};
use super::access_point::Signal;

/// 意识层配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessConfig {
    /// 共识阈值
    pub consensus_threshold: f64,
    /// 冲突检测阈值
    pub conflict_threshold: f64,
    /// 决策超时（毫秒）
    pub decision_timeout_ms: u64,
    /// 向量空间维度
    pub vector_dimension: usize,
}

impl Default for ConsciousnessConfig {
    fn default() -> Self {
        Self {
            consensus_threshold: 0.6,
            conflict_threshold: 0.3,
            decision_timeout_ms: 5000,
            vector_dimension: 256,
        }
    }
}

impl ConsciousnessConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.consensus_threshold <= 0.0 || self.consensus_threshold > 1.0 {
            return Err("consensus_threshold must be between 0 and 1".to_string());
        }
        if self.vector_dimension == 0 {
            return Err("vector_dimension must be positive".to_string());
        }
        Ok(())
    }
}

/// 意识层
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessLayer {
    /// 配置
    config: ConsciousnessConfig,
    /// 当前意图
    pub current_intention: Option<Intention>,
    /// 意图池
    pub intention_pool: Vec<Intention>,
    /// 待处理决策
    pub pending_decisions: Vec<Decision>,
    /// 已完成决策
    pub completed_decisions: Vec<CompletedDecision>,
    /// 决策历史统计
    pub decision_stats: DecisionStats,
    /// 决策向量空间（拉蒂奥设计）
    pub decision_vectors: HashMap<Uuid, DecisionVector>,
}

/// 决策向量 - 拉蒂奥设计的优雅数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionVector {
    /// 蛊虫ID
    pub gu_id: Uuid,
    /// 决策向量（高维表示）
    pub vector: Vec<f64>,
    /// 权重向量
    pub weight: Vec<f64>,
    /// 决策时间戳
    pub timestamp: u64,
    /// 置信度
    pub confidence: f64,
}

impl DecisionVector {
    pub fn new(gu_id: Uuid, dim: usize) -> Self {
        Self {
            gu_id,
            vector: vec![0.0; dim],
            weight: vec![1.0; dim],
            timestamp: 0,
            confidence: 1.0,
        }
    }

    /// 设置决策向量
    pub fn with_vector(&self, vector: Vec<f64>) -> Self {
        let mut new_dv = self.clone();
        new_dv.vector = vector;
        new_dv
    }

    /// 设置权重向量
    pub fn with_weight(&self, weight: Vec<f64>) -> Self {
        let mut new_dv = self.clone();
        new_dv.weight = weight;
        new_dv
    }

    /// 计算向量范数 ||D||
    pub fn norm(&self) -> f64 {
        self.vector.iter()
            .map(|x| x * x)
            .sum::<f64>()
            .sqrt()
    }

    /// 计算权重范数 ||W||
    pub fn weight_norm(&self) -> f64 {
        self.weight.iter()
            .map(|w| w * w)
            .sum::<f64>()
            .sqrt()
    }
}

/// 已完成的决策
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedDecision {
    pub decision: Decision,
    pub chosen_option: usize,
    pub confidence: f64,
    pub execution_result: DecisionResult,
    pub timestamp: u64,
    /// 向量化决策的优雅度得分
    pub elegance_score: f64,
}

/// 决策结果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionResult {
    Success,
    PartialSuccess,
    Failed,
    Expired,
}

/// 决策统计
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DecisionStats {
    pub total: u64,
    pub successful: u64,
    pub failed: u64,
    pub avg_confidence: f64,
    pub avg_elegance: f64,
}

impl ConsciousnessLayer {
    pub fn new(config: ConsciousnessConfig) -> Self {
        Self {
            config,
            current_intention: None,
            intention_pool: Vec::new(),
            pending_decisions: Vec::new(),
            completed_decisions: Vec::new(),
            decision_stats: DecisionStats::default(),
            decision_vectors: HashMap::new(),
        }
    }

    /// 提交意图
    pub fn submit_intention(&self, intention: Intention) -> Self {
        let mut new_layer = self.clone();
        new_layer.intention_pool.push(intention);
        new_layer
    }

    /// 融合意图: W_intention = Aggregate(Gu_intentions) → Consensus
    pub fn merge_intentions(&self) -> Self {
        if self.intention_pool.is_empty() {
            return self.clone();
        }

        // 按描述分组，计算支持度
        let mut intention_groups: HashMap<String, Vec<&Intention>> = HashMap::new();
        for intention in &self.intention_pool {
            intention_groups
                .entry(intention.description.clone())
                .or_default()
                .push(intention);
        }

        // 找到支持度最高的意图
        let best = intention_groups
            .iter()
            .max_by_key(|(_, group)| group.len());

        let mut new_layer = self.clone();
        if let Some((desc, group)) = best {
            let total_supporters: usize = self.intention_pool.iter().map(|i| i.supporters.len()).sum();
            let confidence = if total_supporters > 0 {
                group.iter().map(|i| i.supporters.len()).sum::<usize>() as f64 / total_supporters as f64
            } else {
                0.0
            };

            new_layer.current_intention = Some(Intention {
                id: Uuid::new_v4(),
                description: desc.clone(),
                priority: group.iter().map(|i| i.priority).sum::<f64>() / group.len() as f64,
                supporters: group.iter().flat_map(|i| i.supporters.clone()).collect(),
                confidence,
            });
        }
        new_layer.intention_pool.clear();
        new_layer
    }

    /// 提交决策向量（拉蒂奥设计）
    pub fn submit_decision_vector(&self, dv: DecisionVector) -> Self {
        let mut new_layer = self.clone();
        new_layer.decision_vectors.insert(dv.gu_id, dv);
        new_layer
    }

    /// 拉蒂奥的向量化决策公式
    ///
    /// D_world = ⟨D|W⟩ / ⟨W|W⟩^0.5
    ///
    /// 这是最优雅的决策统一公式
    pub fn vectorized_decision(&self) -> Option<Vec<f64>> {
        if self.decision_vectors.is_empty() {
            return None;
        }

        let dim = self.config.vector_dimension;

        // 计算加权决策向量 ⟨D|W⟩
        let weighted_decision: Vec<f64> = (0..dim)
            .map(|i| {
                self.decision_vectors.values()
                    .map(|dv| dv.vector[i] * dv.weight[i] * dv.confidence)
                    .sum()
            })
            .collect();

        // 计算权重范数 ⟨W|W⟩^0.5
        let weight_norm: f64 = self.decision_vectors.values()
            .map(|dv| dv.weight_norm().powi(2))
            .sum::<f64>()
            .sqrt();

        if weight_norm == 0.0 {
            return None;
        }

        // 归一化决策向量
        let normalized: Vec<f64> = weighted_decision
            .iter()
            .map(|x| x / weight_norm)
            .collect();

        Some(normalized)
    }

    /// 计算决策优雅度
    ///
    /// Elegance = 1 / (1 + Entropy(D_vectors))
    pub fn calculate_elegance(&self) -> f64 {
        if self.decision_vectors.len() < 2 {
            return 1.0;
        }

        // 计算决策向量之间的熵（分散程度）
        let world_decision = match self.vectorized_decision() {
            Some(d) => d,
            None => return 0.0,
        };
        let world_norm = world_decision.iter().map(|x| x * x).sum::<f64>().sqrt();

        if world_norm == 0.0 {
            return 0.0;
        }

        // 计算每个蛊虫决策与世界决策的一致度
        let consistency_sum: f64 = self.decision_vectors.values()
            .map(|dv| {
                // 内积 ⟨dv|world⟩
                let inner_product: f64 = dv.vector.iter()
                    .zip(world_decision.iter())
                    .map(|(a, b)| a * b)
                    .sum();
                // 一致度 = cos(angle)
                let dv_norm = dv.norm();
                if dv_norm > 0.0 {
                    inner_product / (dv_norm * world_norm)
                } else {
                    0.0
                }
            })
            .sum();

        // 优雅度 = 平均一致度
        consistency_sum / self.decision_vectors.len() as f64
    }

    /// 提交决策
    pub fn submit_decision(&self, decision: Decision) -> Self {
        let mut new_layer = self.clone();
        new_layer.pending_decisions.push(decision);
        new_layer
    }

    /// 执行决策: D_world = Weighted_Average(Gu_decisions, Gu_trust_scores)
    pub fn process_decisions(&self, trust_scores: &HashMap<Uuid, f64>) -> Self {
        let mut new_layer = self.clone();
        let now = current_timestamp();

        let mut completed = Vec::new();
        let mut still_pending = Vec::new();

        for decision in &self.pending_decisions {
            // 检查是否超时
            if now > decision.deadline {
                completed.push(CompletedDecision {
                    decision: decision.clone(),
                    chosen_option: 0,
                    confidence: 0.0,
                    execution_result: DecisionResult::Expired,
                    timestamp: now,
                    elegance_score: 0.0,
                });
                continue;
            }

            // 检查是否达成共识
            if let Some((option_idx, confidence)) = self.calculate_consensus(decision, trust_scores) {
                if confidence >= self.config.consensus_threshold {
                    let elegance = self.calculate_elegance();
                    completed.push(CompletedDecision {
                        decision: decision.clone(),
                        chosen_option: option_idx,
                        confidence,
                        execution_result: DecisionResult::Success,
                        timestamp: now,
                        elegance_score: elegance,
                    });
                    continue;
                }
            }

            still_pending.push(decision.clone());
        }

        // 更新统计
        new_layer.decision_stats.total += completed.len() as u64;
        for c in &completed {
            match c.execution_result {
                DecisionResult::Success | DecisionResult::PartialSuccess => {
                    new_layer.decision_stats.successful += 1;
                }
                DecisionResult::Failed | DecisionResult::Expired => {
                    new_layer.decision_stats.failed += 1;
                }
            }
            new_layer.decision_stats.avg_elegance =
                (new_layer.decision_stats.avg_elegance * (new_layer.decision_stats.total - 1) as f64
                 + c.elegance_score) / new_layer.decision_stats.total as f64;
        }

        // 计算平均置信度
        let total_confidence: f64 = completed.iter().map(|c| c.confidence).sum();
        let total = new_layer.decision_stats.total;
        if total > 0 {
            new_layer.decision_stats.avg_confidence =
                (new_layer.decision_stats.avg_confidence * (total - completed.len() as u64) as f64 + total_confidence)
                / total as f64;
        }

        new_layer.pending_decisions = still_pending;
        new_layer.completed_decisions.extend(completed);
        new_layer.decision_vectors.clear();  // 清理决策向量
        new_layer
    }

    /// 计算共识
    pub fn calculate_consensus(&self, decision: &Decision, trust_scores: &HashMap<Uuid, f64>) -> Option<(usize, f64)> {
        if decision.options.is_empty() || decision.votes.is_empty() {
            return None;
        }

        // 计算每个选项的加权票数
        let mut option_scores: Vec<f64> = vec![0.0; decision.options.len()];
        let mut total_weight = 0.0;

        for (gu_id, option_idx) in &decision.votes {
            if *option_idx < option_scores.len() {
                let weight = trust_scores.get(gu_id).copied().unwrap_or(0.5);
                option_scores[*option_idx] += weight;
                total_weight += weight;
            }
        }

        if total_weight == 0.0 {
            return None;
        }

        // 选择得分最高的选项
        let (best_idx, &best_score) = option_scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))?;

        let confidence = best_score / total_weight;
        Some((best_idx, confidence))
    }

    /// 检测冲突: Conflict = Var(D_i) > θ_conflict
    pub fn detect_conflict(&self, decision: &Decision) -> bool {
        if decision.votes.len() < 2 {
            return false;
        }

        let votes: Vec<usize> = decision.votes.values().cloned().collect();
        let mean = votes.iter().sum::<usize>() as f64 / votes.len() as f64;
        let variance = votes.iter()
            .map(|&v| (v as f64 - mean).powi(2))
            .sum::<f64>() / votes.len() as f64;

        variance > self.config.conflict_threshold
    }

    /// 获取当前意图
    pub fn get_intention(&self) -> Option<&Intention> {
        self.current_intention.as_ref()
    }

    /// 获取向量维度
    pub fn vector_dimension(&self) -> usize {
        self.config.vector_dimension
    }
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consciousness_layer_creation() {
        let config = ConsciousnessConfig::default();
        let layer = ConsciousnessLayer::new(config);
        assert!(layer.current_intention.is_none());
        assert_eq!(layer.vector_dimension(), 256);
    }

    #[test]
    fn test_intention_merge() {
        let config = ConsciousnessConfig::default();
        let layer = ConsciousnessLayer::new(config);

        let intention = Intention {
            id: Uuid::new_v4(),
            description: "探索新领域".to_string(),
            priority: 0.8,
            supporters: vec![Uuid::new_v4()],
            confidence: 0.0,
        };

        let layer = layer.submit_intention(intention);
        let layer = layer.merge_intentions();

        assert!(layer.current_intention.is_some());
    }

    #[test]
    fn test_decision_vector() {
        let gu_id = Uuid::new_v4();
        let dv = DecisionVector::new(gu_id, 256);
        assert_eq!(dv.vector.len(), 256);
        assert_eq!(dv.norm(), 0.0);  // 初始向量范数为0
    }

    #[test]
    fn test_vectorized_decision_formula() {
        let config = ConsciousnessConfig::default();
        let layer = ConsciousnessLayer::new(config);

        // 添加两个蛊虫的决策向量
        let gu1 = Uuid::new_v4();
        let gu2 = Uuid::new_v4();

        let dv1 = DecisionVector::new(gu1, 256)
            .with_vector(vec![1.0; 256])
            .with_weight(vec![1.0; 256]);
        let dv2 = DecisionVector::new(gu2, 256)
            .with_vector(vec![1.0; 256])
            .with_weight(vec![1.0; 256]);

        let layer = layer.submit_decision_vector(dv1);
        let layer = layer.submit_decision_vector(dv2);

        // 向量化决策应该返回归一化的向量
        let result = layer.vectorized_decision();
        assert!(result.is_some());

        let decision = result.unwrap();
        // 由于两个向量相同，决策应该指向同一方向
        assert!(decision.iter().all(|x| *x > 0.0));
    }

    #[test]
    fn test_elegance_calculation() {
        let config = ConsciousnessConfig::default();
        let layer = ConsciousnessLayer::new(config);

        let gu1 = Uuid::new_v4();
        let gu2 = Uuid::new_v4();

        // 完全一致的决策应该有最高优雅度
        let dv1 = DecisionVector::new(gu1, 256)
            .with_vector(vec![1.0; 256]);
        let dv2 = DecisionVector::new(gu2, 256)
            .with_vector(vec![1.0; 256]);

        let layer = layer.submit_decision_vector(dv1);
        let layer = layer.submit_decision_vector(dv2);

        let elegance = layer.calculate_elegance();
        assert!(elegance > 0.99);  // 完全一致，优雅度接近1
    }
}