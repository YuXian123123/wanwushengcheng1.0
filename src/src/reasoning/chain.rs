//! 推理链模块 - 优雅设计
//!
//! 形式化的推理链表示和验证

use std::collections::VecDeque;

/// 推理步骤
#[derive(Debug, Clone)]
pub struct ReasoningStep {
    /// 步骤ID
    pub id: usize,
    /// 前提概念
    pub premises: Vec<String>,
    /// 结论概念
    pub conclusion: String,
    /// 推理规则
    pub rule: ReasoningRule,
    /// 置信度
    pub confidence: f64,
}

/// 推理规则
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReasoningRule {
    /// Modus Ponens: A→B, A ⊢ B
    ModusPonens,
    /// Modus Tollens: A→B, ¬B ⊢ ¬A
    ModusTollens,
    /// Hypothetical Syllogism: A→B, B→C ⊢ A→C
    HypotheticalSyllogism,
    /// Disjunctive Syllogism: A∨B, ¬A ⊢ B
    DisjunctiveSyllogism,
    /// Generalization: A ⊢ ∀x A(x)
    Generalization,
    /// Instantiation: ∀x A(x) ⊢ A(c)
    Instantiation,
    /// Analogy: A~B, A has P ⊢ B has P
    Analogy,
    /// Causal: A causes B
    CausalInference,
}

/// 推理链
#[derive(Debug, Clone)]
pub struct ReasoningChain {
    /// 链ID
    pub id: String,
    /// 起始概念
    pub start: String,
    /// 目标概念
    pub target: String,
    /// 步骤序列
    pub steps: VecDeque<ReasoningStep>,
    /// 总置信度
    pub total_confidence: f64,
    /// 是否已验证
    pub verified: bool,
}

impl ReasoningChain {
    /// 创建新的推理链
    pub fn new(id: String, start: String, target: String) -> Self {
        Self {
            id,
            start,
            target,
            steps: VecDeque::new(),
            total_confidence: 1.0,
            verified: false,
        }
    }

    /// 添加步骤
    pub fn add_step(&mut self, step: ReasoningStep) {
        // 更新总置信度：链式乘法
        self.total_confidence *= step.confidence;
        self.steps.push_back(step);
    }

    /// 验证推理链
    pub fn verify(&mut self) -> bool {
        // 检查步骤连贯性
        for i in 1..self.steps.len() {
            let prev = &self.steps[i - 1];
            let curr = &self.steps[i];

            // 前一步骤的结论应该是当前步骤的前提之一
            if !curr.premises.contains(&prev.conclusion) {
                // 检查是否是独立步骤
                if curr.premises.is_empty() {
                    continue;
                }
                self.verified = false;
                return false;
            }
        }

        self.verified = true;
        true
    }

    /// 获取链长度
    pub fn length(&self) -> usize {
        self.steps.len()
    }

    /// 是否为空链
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// 格式化输出
    pub fn format(&self) -> String {
        let mut output = format!("推理链 [{}]: {} → {}\n", self.id, self.start, self.target);
        output.push_str(&format!("总置信度: {:.2}%\n", self.total_confidence * 100.0));
        output.push_str("步骤:\n");

        for step in &self.steps {
            output.push_str(&format!(
                "  {}. {} ⊢ {} [{:?}] ({:.0}%)\n",
                step.id,
                step.premises.join(", "),
                step.conclusion,
                step.rule,
                step.confidence * 100.0
            ));
        }

        output
    }
}

/// 推理链构建器
pub struct ChainBuilder {
    chain: ReasoningChain,
    step_counter: usize,
}

impl ChainBuilder {
    /// 创建构建器
    pub fn new(id: String, start: String, target: String) -> Self {
        Self {
            chain: ReasoningChain::new(id, start, target),
            step_counter: 0,
        }
    }

    /// 添加步骤
    pub fn step(mut self, premises: Vec<String>, conclusion: String, rule: ReasoningRule, confidence: f64) -> Self {
        self.step_counter += 1;
        self.chain.add_step(ReasoningStep {
            id: self.step_counter,
            premises,
            conclusion,
            rule,
            confidence,
        });
        self
    }

    /// 构建推理链
    pub fn build(mut self) -> ReasoningChain {
        self.chain.verify();
        self.chain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_creation() {
        let chain = ReasoningChain::new(
            "test_chain".to_string(),
            "A".to_string(),
            "C".to_string(),
        );
        assert!(chain.is_empty());
        assert_eq!(chain.total_confidence, 1.0);
    }

    #[test]
    fn test_chain_building() {
        let chain = ChainBuilder::new(
            "test".to_string(),
            "A".to_string(),
            "C".to_string(),
        )
        .step(
            vec!["A".to_string()],
            "B".to_string(),
            ReasoningRule::ModusPonens,
            0.9,
        )
        .step(
            vec!["B".to_string()],
            "C".to_string(),
            ReasoningRule::ModusPonens,
            0.85,
        )
        .build();

        assert_eq!(chain.length(), 2);
        assert!((chain.total_confidence - 0.765).abs() < 0.001);
    }

    #[test]
    fn test_chain_verification() {
        let mut chain = ChainBuilder::new(
            "test".to_string(),
            "A".to_string(),
            "B".to_string(),
        )
        .step(
            vec!["A".to_string()],
            "B".to_string(),
            ReasoningRule::ModusPonens,
            0.9,
        )
        .build();

        assert!(chain.verify());
        assert!(chain.verified);
    }
}
