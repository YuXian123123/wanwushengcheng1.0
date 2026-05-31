//! 信号共鸣编码模块
//!
//! 实现基于LNN共振的信号编码，而非传统编码方式

use uuid::Uuid;
use super::message::{GuId, MessageType};

/// 信号类型
#[derive(Debug, Clone)]
pub enum SignalType {
    /// 知识信号 - 携带知识内容
    Knowledge {
        content: String,
        confidence: f64,
        source_gu: GuId,
    },
    /// 情绪信号 - 携带情绪状态
    Emotion {
        pleasure: f64,
        arousal: f64,
        intensity: f64,
    },
    /// 交易信号 - 携带交易请求
    Trade {
        product_id: String,
        price: f64,
        action: TradeAction,
    },
    /// 状态信号 - 携带状态更新
    Status {
        health: f64,
        coins: f64,
    },
}

/// 交易行为
#[derive(Debug, Clone)]
pub enum TradeAction {
    Buy,
    Sell,
    Exchange,
}

/// 共振信号
#[derive(Debug, Clone)]
pub struct ResonanceSignal {
    /// 信号ID
    pub id: Uuid,
    /// 信号类型
    pub signal_type: SignalType,
    /// 发送者
    pub source: GuId,
    /// 共振模式（模拟LNN生成的模式）
    pub resonance_pattern: ResonancePattern,
    /// 时间戳
    pub timestamp: u64,
    /// 生存时间
    pub ttl: u64,
}

/// 共振模式（模拟LNN内部状态）
#[derive(Debug, Clone)]
pub struct ResonancePattern {
    /// 频率向量
    pub frequencies: Vec<f64>,
    /// 相位向量
    pub phases: Vec<f64>,
    /// 振幅向量
    pub amplitudes: Vec<f64>,
}

impl ResonancePattern {
    /// 创建新的共振模式
    pub fn new() -> Self {
        Self {
            frequencies: vec![0.0; 16],
            phases: vec![0.0; 16],
            amplitudes: vec![0.0; 16],
        }
    }

    /// 从消息类型生成共振模式
    pub fn from_message_type(msg_type: &MessageType) -> Self {
        let mut pattern = Self::new();

        match msg_type {
            MessageType::Knowledge { content, concepts, confidence } => {
                // 知识消息：基于内容长度和概念数量生成模式
                let base_freq = (content.len() as f64 / 100.0).min(1.0);
                let concept_factor = (concepts.len() as f64 / 10.0).min(1.0);

                for i in 0..16 {
                    pattern.frequencies[i] = base_freq * (i as f64 + 1.0) / 16.0;
                    pattern.amplitudes[i] = confidence * concept_factor;
                    pattern.phases[i] = (i as f64 * 0.1) % 1.0;
                }
            }
            MessageType::Trade { price, .. } => {
                // 交易消息：基于价格生成模式
                let price_factor = (price / 100.0).min(1.0);
                for i in 0..16 {
                    pattern.frequencies[i] = price_factor;
                    pattern.amplitudes[i] = 0.8;
                    pattern.phases[i] = 0.0;
                }
            }
            MessageType::Status { health, .. } => {
                // 状态消息：基于健康度生成模式
                for i in 0..16 {
                    pattern.frequencies[i] = *health;
                    pattern.amplitudes[i] = *health;
                    pattern.phases[i] = 0.5;
                }
            }
            MessageType::Emotion { pleasure, arousal } => {
                // 情绪消息：基于情绪维度生成模式
                for i in 0..16 {
                    pattern.frequencies[i] = *pleasure;
                    pattern.amplitudes[i] = *arousal;
                    pattern.phases[i] = (*pleasure + *arousal) / 2.0;
                }
            }
            MessageType::System { .. } => {
                // 系统消息：固定高频模式
                for i in 0..16 {
                    pattern.frequencies[i] = 1.0;
                    pattern.amplitudes[i] = 1.0;
                    pattern.phases[i] = 0.0;
                }
            }
        }

        pattern
    }

    /// 计算与另一个模式的共振相似度
    pub fn resonance_similarity(&self, other: &ResonancePattern) -> f64 {
        if self.frequencies.len() != other.frequencies.len() {
            return 0.0;
        }

        let mut total_sim = 0.0;
        let len = self.frequencies.len() as f64;

        for i in 0..self.frequencies.len() {
            let freq_sim = 1.0 - (self.frequencies[i] - other.frequencies[i]).abs();
            let amp_sim = 1.0 - (self.amplitudes[i] - other.amplitudes[i]).abs();
            let phase_sim = 1.0 - (self.phases[i] - other.phases[i]).abs();

            total_sim += (freq_sim + amp_sim + phase_sim) / 3.0;
        }

        total_sim / len
    }
}

/// 信号编码器
#[derive(Debug, Clone)]
pub struct SignalEncoder {
    /// 默认生存时间
    pub default_ttl: u64,
}

impl Default for SignalEncoder {
    fn default() -> Self {
        Self {
            default_ttl: 3600, // 1小时
        }
    }
}

impl SignalEncoder {
    /// 编码消息为共振信号
    pub fn encode(&self, msg_type: &MessageType, source: GuId) -> ResonanceSignal {
        let pattern = ResonancePattern::from_message_type(msg_type);

        let signal_type = match msg_type {
            MessageType::Knowledge { content, confidence, .. } => {
                SignalType::Knowledge {
                    content: content.clone(),
                    confidence: *confidence,
                    source_gu: source.clone(),
                }
            }
            MessageType::Emotion { pleasure, arousal, .. } => {
                SignalType::Emotion {
                    pleasure: *pleasure,
                    arousal: *arousal,
                    intensity: (pleasure + arousal) / 2.0,
                }
            }
            MessageType::Trade { product, price, .. } => {
                SignalType::Trade {
                    product_id: product.name.clone(),
                    price: *price,
                    action: TradeAction::Sell, // 默认
                }
            }
            MessageType::Status { health, .. } => {
                // 获取金币需要额外参数，这里简化
                SignalType::Status {
                    health: *health,
                    coins: 0.0,
                }
            }
            MessageType::System { .. } => {
                SignalType::Status {
                    health: 1.0,
                    coins: 0.0,
                }
            }
        };

        ResonanceSignal {
            id: Uuid::new_v4(),
            signal_type,
            source,
            resonance_pattern: pattern,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            ttl: self.default_ttl,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_resonance_pattern() {
        let msg_type = MessageType::Knowledge {
            content: "test content".to_string(),
            concepts: HashSet::from(["test".to_string()]),
            confidence: 0.8,
        };

        let pattern = ResonancePattern::from_message_type(&msg_type);
        assert_eq!(pattern.frequencies.len(), 16);
    }

    #[test]
    fn test_resonance_similarity() {
        let p1 = ResonancePattern::new();
        let p2 = ResonancePattern::new();

        // 相同模式应该相似度为1
        let sim = p1.resonance_similarity(&p2);
        assert!((sim - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_signal_encoding() {
        let encoder = SignalEncoder::default();
        let msg_type = MessageType::Knowledge {
            content: "test".to_string(),
            concepts: HashSet::new(),
            confidence: 0.5,
        };

        let signal = encoder.encode(&msg_type, GuId("test".to_string()));
        assert_eq!(signal.ttl, 3600);
    }
}
