//! 分词器

use serde::{Deserialize, Serialize};

/// Token类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    /// 普通词汇
    Word,
    /// 数字
    Number,
    /// 标点符号
    Punctuation,
    /// 空格
    Whitespace,
}

/// Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    /// 文本内容
    pub text: String,
    /// 起始位置
    pub start: usize,
    /// 结束位置
    pub end: usize,
    /// 类型
    pub token_type: TokenType,
}

impl Token {
    /// 创建新Token
    pub fn new(text: String, start: usize, end: usize, token_type: TokenType) -> Self {
        Self { text, start, end, token_type }
    }
}

/// 分词器
pub struct Tokenizer {
    // 可以添加词典等资源
}

impl Tokenizer {
    /// 创建新分词器
    pub fn new() -> Self {
        Self {}
    }

    /// 分词
    pub fn tokenize(&self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut pos = 0;

        while pos < chars.len() {
            let ch = chars[pos];

            // 空格
            if ch.is_whitespace() {
                let start = pos;
                while pos < chars.len() && chars[pos].is_whitespace() {
                    pos += 1;
                }
                tokens.push(Token::new(
                    chars[start..pos].iter().collect(),
                    start,
                    pos,
                    TokenType::Whitespace,
                ));
                continue;
            }

            // 数字
            if ch.is_ascii_digit() {
                let start = pos;
                while pos < chars.len() && (chars[pos].is_ascii_digit() || chars[pos] == '.') {
                    pos += 1;
                }
                tokens.push(Token::new(
                    chars[start..pos].iter().collect(),
                    start,
                    pos,
                    TokenType::Number,
                ));
                continue;
            }

            // 标点符号
            if is_punctuation(ch) {
                tokens.push(Token::new(
                    ch.to_string(),
                    pos,
                    pos + 1,
                    TokenType::Punctuation,
                ));
                pos += 1;
                continue;
            }

            // 中文或英文词汇
            let start = pos;
            if is_chinese(ch) {
                // 中文：按字符分割（简化版，实际应使用分词算法）
                tokens.push(Token::new(
                    ch.to_string(),
                    start,
                    start + 1,
                    TokenType::Word,
                ));
                pos += 1;
            } else {
                // 英文：连续字母
                while pos < chars.len() && chars[pos].is_ascii_alphabetic() {
                    pos += 1;
                }
                if pos > start {
                    tokens.push(Token::new(
                        chars[start..pos].iter().collect(),
                        start,
                        pos,
                        TokenType::Word,
                    ));
                } else {
                    pos += 1;
                }
            }
        }

        tokens
    }

    /// 分词（过滤空格）
    pub fn tokenize_no_whitespace(&self, text: &str) -> Vec<Token> {
        self.tokenize(text)
            .into_iter()
            .filter(|t| t.token_type != TokenType::Whitespace)
            .collect()
    }
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

/// 判断是否为中文字符
fn is_chinese(ch: char) -> bool {
    ('\u{4E00}'..='\u{9FFF}').contains(&ch)
}

/// 判断是否为标点符号
fn is_punctuation(ch: char) -> bool {
    matches!(ch, '!' | '"' | '#' | '$' | '%' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | '-' | '.' | '/' | ':' | ';' | '<' | '=' | '>' | '?' | '@' | '[' | '\\' | ']' | '^' | '_' | '`' | '{' | '|' | '}' | '~' | '，' | '。' | '！' | '？' | '、' | '：' | '；' | '"' | '"' | '\'' | '\'' | '（' | '）' | '【' | '】' | '《' | '》')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_english() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize_no_whitespace("hello world");

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "hello");
        assert_eq!(tokens[1].text, "world");
    }

    #[test]
    fn test_tokenize_chinese() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize_no_whitespace("你好世界");

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].text, "你");
        assert_eq!(tokens[1].text, "好");
    }

    #[test]
    fn test_tokenize_mixed() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize_no_whitespace("我在吃apple");

        assert!(tokens.len() >= 3);
        assert_eq!(tokens[0].text, "我");
        assert_eq!(tokens[3].text, "apple");
    }

    #[test]
    fn test_tokenize_numbers() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize_no_whitespace("价格100元");

        let number_token = tokens.iter().find(|t| t.token_type == TokenType::Number);
        assert!(number_token.is_some());
        assert_eq!(number_token.unwrap().text, "100");
    }
}
