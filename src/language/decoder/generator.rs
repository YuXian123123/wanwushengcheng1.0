//! 语言生成器

use serde::{Deserialize, Serialize};
use super::intent::Intent;

/// 表达风格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    /// 正式程度 [0, 1]
    pub formality: f64,
    /// 热情程度 [0, 1]
    pub warmth: f64,
    /// 简洁程度 [0, 1]
    pub conciseness: f64,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            formality: 0.5,
            warmth: 0.5,
            conciseness: 0.5,
        }
    }
}

/// 解码结果
#[derive(Debug, Clone)]
pub struct DecodedResult {
    /// 生成的文本
    pub text: String,
    /// 意图
    pub intent: Intent,
    /// 置信度
    pub confidence: f64,
}

/// 语言生成器
pub struct Generator {
    style: Style,
}

impl Generator {
    /// 创建新生成器
    pub fn new() -> Self {
        Self { style: Style::default() }
    }

    /// 设置风格
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// 从意图生成文本
    pub fn generate(&self, intent: &Intent) -> DecodedResult {
        let text = self.intent_to_text(intent);

        DecodedResult {
            text,
            intent: intent.clone(),
            confidence: intent.confidence,
        }
    }

    /// 意图转文本（简化版）
    fn intent_to_text(&self, intent: &Intent) -> String {
        match intent.intent_type {
            super::intent::IntentType::Statement => {
                format!("我理解了：{}", intent.key_concepts.join("、"))
            }
            super::intent::IntentType::Question => {
                format!("你是在问关于{}的问题吗？", intent.key_concepts.join("和"))
            }
            super::intent::IntentType::Request => {
                format!("好的，我来处理：{}", intent.key_concepts.join("、"))
            }
            super::intent::IntentType::Answer => {
                format!("关于{}，我可以告诉你...", intent.key_concepts.join("、"))
            }
            super::intent::IntentType::Confirmation => {
                "是的，我确认。".to_string()
            }
            super::intent::IntentType::Denial => {
                "不，我不这么认为。".to_string()
            }
        }
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::intent::IntentType;

    #[test]
    fn test_generate_statement() {
        let generator = Generator::new();
        let intent = Intent::new(IntentType::Statement, 0.9, vec!["苹果".to_string()]);
        let result = generator.generate(&intent);

        assert!(result.text.contains("苹果"));
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_generate_question() {
        let generator = Generator::new();
        let intent = Intent::new(IntentType::Question, 0.8, vec!["价格".to_string()]);
        let result = generator.generate(&intent);

        assert!(result.text.contains("问") || result.text.contains("?"));
    }
}
