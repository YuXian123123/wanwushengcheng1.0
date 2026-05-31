//! 推理验证模块 - 安全设计
//!
//! 四层验证机制：置信度、逻辑一致性、语义相关性、历史一致性

use std::collections::HashMap;

/// 验证级别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationLevel {
    /// 通过
    Pass,
    /// 警告（可接受但需注意）
    Warning,
    /// 失败
    Fail,
}

/// 验证结果
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// 验证级别
    pub level: ValidationLevel,
    /// 验证项名称
    pub check_name: String,
    /// 详细消息
    pub message: String,
    /// 置信度分数
    pub score: f64,
}

/// 四层验证器
pub struct InferenceValidator {
    /// 最小置信度阈值
    min_confidence: f64,
    /// 历史结果缓存
    history: HashMap<String, Vec<String>>,
    /// 逻辑矛盾检测
    contradictions: Vec<(String, String)>,
}

impl InferenceValidator {
    /// 创建新的验证器
    pub fn new() -> Self {
        Self {
            min_confidence: 0.5,
            history: HashMap::new(),
            contradictions: Vec::new(),
        }
    }

    /// 设置最小置信度
    pub fn with_min_confidence(mut self, threshold: f64) -> Self {
        self.min_confidence = threshold;
        self
    }

    /// 执行四层验证
    pub fn validate(
        &self,
        input: &str,
        output: &str,
        confidence: f64,
        reasoning_chain: &[String],
    ) -> Vec<ValidationResult> {
        let mut results = Vec::new();

        // 第一层：置信度验证
        results.push(self.validate_confidence(confidence));

        // 第二层：逻辑一致性验证
        results.push(self.validate_logic(reasoning_chain));

        // 第三层：语义相关性验证
        results.push(self.validate_semantic(input, output));

        // 第四层：历史一致性验证
        results.push(self.validate_history(output));

        results
    }

    /// 第一层：置信度验证
    fn validate_confidence(&self, confidence: f64) -> ValidationResult {
        if confidence >= self.min_confidence {
            ValidationResult {
                level: ValidationLevel::Pass,
                check_name: "置信度验证".to_string(),
                message: format!("置信度 {:.2}% >= 阈值 {:.2}%", confidence * 100.0, self.min_confidence * 100.0),
                score: confidence,
            }
        } else {
            ValidationResult {
                level: ValidationLevel::Fail,
                check_name: "置信度验证".to_string(),
                message: format!("置信度 {:.2}% < 阈值 {:.2}%", confidence * 100.0, self.min_confidence * 100.0),
                score: confidence,
            }
        }
    }

    /// 第二层：逻辑一致性验证
    fn validate_logic(&self, reasoning_chain: &[String]) -> ValidationResult {
        // 检查推理链是否为空
        if reasoning_chain.is_empty() {
            return ValidationResult {
                level: ValidationLevel::Warning,
                check_name: "逻辑一致性验证".to_string(),
                message: "推理链为空".to_string(),
                score: 0.5,
            };
        }

        // 检查矛盾
        let mut detected_contradictions = 0;
        for (a, b) in &self.contradictions {
            if reasoning_chain.contains(&a.to_string()) && reasoning_chain.contains(&b.to_string()) {
                detected_contradictions += 1;
            }
        }

        if detected_contradictions > 0 {
            ValidationResult {
                level: ValidationLevel::Fail,
                check_name: "逻辑一致性验证".to_string(),
                message: format!("检测到 {} 个逻辑矛盾", detected_contradictions),
                score: 0.0,
            }
        } else {
            ValidationResult {
                level: ValidationLevel::Pass,
                check_name: "逻辑一致性验证".to_string(),
                message: "无逻辑矛盾".to_string(),
                score: 1.0,
            }
        }
    }

    /// 第三层：语义相关性验证
    fn validate_semantic(&self, input: &str, output: &str) -> ValidationResult {
        // 简化的语义相关性检查
        let input_words: std::collections::HashSet<&str> = input.split_whitespace().collect();
        let output_words: std::collections::HashSet<&str> = output.split_whitespace().collect();

        let intersection = input_words.intersection(&output_words).count();
        let union = input_words.union(&output_words).count();

        let relevance = if union > 0 {
            intersection as f64 / union as f64
        } else {
            0.0
        };

        if relevance >= 0.1 {
            ValidationResult {
                level: ValidationLevel::Pass,
                check_name: "语义相关性验证".to_string(),
                message: format!("语义相关度 {:.2}%", relevance * 100.0),
                score: relevance,
            }
        } else {
            ValidationResult {
                level: ValidationLevel::Warning,
                check_name: "语义相关性验证".to_string(),
                message: format!("语义相关度较低: {:.2}%", relevance * 100.0),
                score: relevance,
            }
        }
    }

    /// 第四层：历史一致性验证
    fn validate_history(&self, output: &str) -> ValidationResult {
        // 检查是否与历史结论冲突
        let mut conflicts = 0;

        for (key, values) in &self.history {
            if output.contains(key) {
                for value in values {
                    if output.contains(value) && key != value {
                        conflicts += 1;
                    }
                }
            }
        }

        if conflicts > 0 {
            ValidationResult {
                level: ValidationLevel::Warning,
                check_name: "历史一致性验证".to_string(),
                message: format!("发现 {} 个潜在历史冲突", conflicts),
                score: 0.7,
            }
        } else {
            ValidationResult {
                level: ValidationLevel::Pass,
                check_name: "历史一致性验证".to_string(),
                message: "与历史结论一致".to_string(),
                score: 1.0,
            }
        }
    }

    /// 添加已知矛盾对
    pub fn add_contradiction(&mut self, a: String, b: String) {
        self.contradictions.push((a, b));
    }

    /// 记录历史结论
    pub fn record_history(&mut self, key: String, value: String) {
        self.history.entry(key).or_default().push(value);
    }

    /// 判断是否通过所有验证
    pub fn is_valid(&self, results: &[ValidationResult]) -> bool {
        results.iter().all(|r| r.level != ValidationLevel::Fail)
    }
}

impl Default for InferenceValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = InferenceValidator::new();
        assert_eq!(validator.min_confidence, 0.5);
    }

    #[test]
    fn test_confidence_validation_pass() {
        let validator = InferenceValidator::new();
        let result = validator.validate_confidence(0.8);
        assert_eq!(result.level, ValidationLevel::Pass);
    }

    #[test]
    fn test_confidence_validation_fail() {
        let validator = InferenceValidator::new();
        let result = validator.validate_confidence(0.3);
        assert_eq!(result.level, ValidationLevel::Fail);
    }

    #[test]
    fn test_four_layer_validation() {
        let validator = InferenceValidator::new();
        let results = validator.validate(
            "所有人类都会死",
            "苏格拉底会死",
            0.9,
            &["苏格拉底是人".to_string(), "所有人类都会死".to_string()],
        );

        assert_eq!(results.len(), 4);
        assert!(validator.is_valid(&results));
    }

    #[test]
    fn test_contradiction_detection() {
        let mut validator = InferenceValidator::new();
        validator.add_contradiction("A".to_string(), "非A".to_string());

        let results = validator.validate(
            "test",
            "output",
            0.8,
            &["A".to_string(), "非A".to_string()],
        );

        let logic_result = results.iter().find(|r| r.check_name == "逻辑一致性验证").unwrap();
        assert_eq!(logic_result.level, ValidationLevel::Fail);
    }
}
