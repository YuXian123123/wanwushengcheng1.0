//! 分词器配置

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// 分词器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizerConfig {
    /// 是否过滤空白符
    pub filter_whitespace: bool,

    /// 是否过滤标点符号
    pub filter_punctuation: bool,

    /// 是否识别数字
    pub recognize_numbers: bool,

    /// 数字小数点识别
    pub recognize_decimals: bool,

    /// 中文字符范围起始
    ///
    /// Unicode CJK统一表意文字范围
    pub chinese_char_start: u32,

    /// 中文字符范围结束
    pub chinese_char_end: u32,

    /// 最大token长度
    ///
    /// 防止超长token
    pub max_token_length: usize,

    /// 自定义分隔符
    #[serde(default)]
    pub custom_separators: HashSet<char>,
}

impl TokenizerConfig {
    /// 创建默认配置
    pub fn new() -> Self {
        Self {
            filter_whitespace: false,
            filter_punctuation: false,
            recognize_numbers: true,
            recognize_decimals: true,
            chinese_char_start: 0x4E00, // CJK统一表意文字起始
            chinese_char_end: 0x9FFF,   // CJK统一表意文字结束
            max_token_length: 100,
            custom_separators: HashSet::new(),
        }
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), String> {
        if self.chinese_char_start >= self.chinese_char_end {
            return Err("chinese_char_start 必须小于 chinese_char_end".to_string());
        }
        if self.max_token_length == 0 {
            return Err("max_token_length 必须大于0".to_string());
        }
        Ok(())
    }

    /// 检查是否为中文字符
    pub fn is_chinese(&self, ch: char) -> bool {
        let code = ch as u32;
        (self.chinese_char_start..=self.chinese_char_end).contains(&code)
    }

    /// 获取标点符号集合
    ///
    /// 注意：这是配置驱动的，不是硬编码
    pub fn punctuation_set(&self) -> HashSet<char> {
        let mut punctuation = HashSet::new();

        // ASCII标点
        for ch in "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".chars() {
            punctuation.insert(ch);
        }

        // 中文标点
        for ch in "，。！？、：；「」『』（）【】《》".chars() {
            punctuation.insert(ch);
        }

        // 合并自定义分隔符
        punctuation.extend(&self.custom_separators);

        punctuation
    }
}

impl Default for TokenizerConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_valid() {
        let config = TokenizerConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_chinese_detection() {
        let config = TokenizerConfig::new();
        assert!(config.is_chinese('中'));
        assert!(config.is_chinese('文'));
        assert!(!config.is_chinese('a'));
        assert!(!config.is_chinese('1'));
    }

    #[test]
    fn test_punctuation_set() {
        let config = TokenizerConfig::new();
        let punctuation = config.punctuation_set();

        assert!(punctuation.contains(&'!'));
        assert!(punctuation.contains(&'，'));
        assert!(punctuation.contains(&'。'));
        assert!(!punctuation.contains(&'a'));
    }

    #[test]
    fn test_custom_separators() {
        let mut config = TokenizerConfig::new();
        config.custom_separators.insert('|');
        let punctuation = config.punctuation_set();
        assert!(punctuation.contains(&'|'));
    }
}
