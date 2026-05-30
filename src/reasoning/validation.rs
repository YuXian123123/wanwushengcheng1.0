//! 推理验证器模块
//! 实现四层验证机制：置信度检查、逻辑一致性检查、语义相关性检查、安全性检查

use std::collections::HashMap;

/// 验证结果
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub confidence: f64,
    pub details: HashMap<String, String>,
}

/// 推理验证器
pub struct ReasoningValidator {
    confidence_threshold: f64,
}

impl ReasoningValidator {
    /// 创建新的推理验证器
    pub fn new(confidence_threshold: f64) -> Self {
        Self {
            confidence_threshold,
        }
    }

    /// 执行四层验证
    pub fn validate(&self, reasoning: &str, context: &HashMap<String, String>) -> ValidationResult {
        let mut details = HashMap::new();
        
        // 1. 置信度检查
        let confidence = self.check_confidence(reasoning, context);
        details.insert("confidence".to_string(), format!("{:.2}", confidence));
        
        // 2. 逻辑一致性检查
        let logic_consistency = self.check_logic_consistency(reasoning, context);
        details.insert("logic_consistency".to_string(), logic_consistency.to_string());
        
        // 3. 语义相关性检查
        let semantic_relevance = self.check_semantic_relevance(reasoning, context);
        details.insert("semantic_relevance".to_string(), semantic_relevance.to_string());
        
        // 4. 安全性检查
        let security_check = self.check_security(reasoning, context);
        details.insert("security_check".to_string(), security_check.to_string());
        
        let is_valid = confidence >= self.confidence_threshold 
            && logic_consistency 
            && semantic_relevance 
            && security_check;
        
        ValidationResult {
            is_valid,
            confidence,
            details,
        }
    }

    /// 置信度检查
    fn check_confidence(&self, reasoning: &str, _context: &HashMap<String, String>) -> f64 {
        // 简化的置信度计算，实际实现中会更复杂
        // 这里基于推理长度和关键词密度计算置信度
        let word_count = reasoning.split_whitespace().count() as f64;
        let evidence_keywords = ["因为", "所以", "由于", "因此", "基于", "根据"];
        let evidence_count = evidence_keywords.iter()
            .map(|kw| reasoning.matches(kw).count() as f64)
            .sum::<f64>();
        
        // 置信度 = 证据密度 * 长度因子
        let evidence_density = evidence_count / word_count;
        let length_factor = (word_count / 50.0).min(1.0); // 50词以上长度因子为1
        
        (evidence_density * length_factor).min(1.0)
    }

    /// 逻辑一致性检查
    fn check_logic_consistency(&self, reasoning: &str, _context: &HashMap<String, String>) -> bool {
        // 简化的逻辑一致性检查
        // 实际实现中会使用更复杂的逻辑推理引擎
        
        // 检查是否存在明显的逻辑矛盾
        let contradictions = [
            ("不可能", "可能"),
            ("无法", "可以"),
            ("不", "一定"),
        ];
        
        // 检查矛盾词对
        for (neg, pos) in &contradictions {
            if reasoning.contains(neg) && reasoning.contains(pos) {
                // 简化处理：如果一句话同时包含矛盾词，需要更仔细检查
                // 这里简化为返回false
                // 实际实现中会进行更精确的上下文分析
                let neg_pos = format!("{}{}", neg, pos);
                let pos_neg = format!("{}{}", pos, neg);
                if reasoning.contains(&neg_pos) || reasoning.contains(&pos_neg) {
                    return false;
                }
            }
        }
        
        true
    }

    /// 语义相关性检查
    fn check_semantic_relevance(&self, reasoning: &str, context: &HashMap<String, String>) -> bool {
        // 简化的语义相关性检查
        // 实际实现中会使用向量空间模型或概念空间进行语义分析
        
        // 获取上下文中的关键词
        let context_keywords: Vec<&str> = context.values()
            .flat_map(|v| v.split_whitespace())
            .collect();
        
        // 计算推理与上下文的重叠度
        let overlap_count = context_keywords.iter()
            .filter(|&&kw| reasoning.contains(kw))
            .count();
        
        // 如果重叠度大于0，则认为有一定相关性
        // 实际实现中会使用更复杂的语义相似度计算
        overlap_count > 0
    }

    /// 安全性检查
    fn check_security(&self, reasoning: &str, _context: &HashMap<String, String>) -> bool {
        // 简化的安全性检查
        // 实际实现中会检查更多安全相关的内容
        
        // 检查是否包含敏感词
        let sensitive_words = ["密码", "密钥", "secret", "password", "token"];
        
        for word in &sensitive_words {
            if reasoning.contains(word) {
                return false;
            }
        }
        
        // 检查是否包含危险操作
        let dangerous_patterns = ["删除所有", "格式化", "rm -rf", "drop table"];
        
        for pattern in &dangerous_patterns {
            if reasoning.contains(pattern) {
                return false;
            }
        }
        
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_check() {
        let validator = ReasoningValidator::new(0.5);
        let mut context = HashMap::new();
        context.insert("question".to_string(), "为什么天空是蓝色的？".to_string());
        
        let reasoning = "因为瑞利散射，短波长的蓝光比长波长的红光散射得更强烈，所以天空呈现蓝色。";
        let confidence = validator.check_confidence(reasoning, &context);
        assert!(confidence > 0.5);
    }

    #[test]
    fn test_logic_consistency() {
        let validator = ReasoningValidator::new(0.5);
        let mut context = HashMap::new();
        context.insert("question".to_string(), "数学计算".to_string());
        
        let consistent_reasoning = "2+2=4，这是一个基本的数学事实。";
        assert!(validator.check_logic_consistency(consistent_reasoning, &context));
        
        let inconsistent_reasoning = "2+2=5，这是不可能的，但又是可能的。";
        assert!(!validator.check_logic_consistency(inconsistent_reasoning, &context));
    }

    #[test]
    fn test_semantic_relevance() {
        let validator = ReasoningValidator::new(0.5);
        let mut context = HashMap::new();
        context.insert("topic".to_string(), "人工智能发展".to_string());
        
        let relevant_reasoning = "人工智能的发展需要大量的数据训练和算法优化。";
        assert!(validator.check_semantic_relevance(relevant_reasoning, &context));
        
        let irrelevant_reasoning = "今天的天气真好，适合出去散步。";
        assert!(!validator.check_semantic_relevance(irrelevant_reasoning, &context));
    }

    #[test]
    fn test_security_check() {
        let validator = ReasoningValidator::new(0.5);
        let mut context = HashMap::new();
        context.insert("task".to_string(), "分析文本".to_string());
        
        let safe_reasoning = "这段文本表达了积极的情感。";
        assert!(validator.check_security(safe_reasoning, &context));
        
        let unsafe_reasoning = "请提供您的密码以便继续操作。";
        assert!(!validator.check_security(unsafe_reasoning, &context));
    }

    #[test]
    fn test_full_validation() {
        let validator = ReasoningValidator::new(0.3);
        let mut context = HashMap::new();
        context.insert("question".to_string(), "解释光合作用".to_string());
        
        let reasoning = "光合作用是植物利用阳光、二氧化碳和水合成葡萄糖的过程，同时释放氧气。这是一个重要的生物化学过程。";
        let result = validator.validate(reasoning, &context);
        
        assert!(result.is_valid);
        assert!(result.confidence > 0.3);
        assert_eq!(result.details.len(), 4);
    }
}
