//! 自然语言对齐模块
//!
//! 蛊虫通信的基础设施，确保蛊虫之间对自然语言有一致的理解
//!
//! # 核心组件
//! - `concept`: 概念空间，概念的向量表示
//! - `encoder`: 自然语言编码器
//! - `decoder`: 内部表示解码器
//! - `consensus`: 知识共识机制
//! - `context`: 上下文管理
//!
//! # Example
//!
//! ```
//! use lnn::language::{ConceptSpace, ConceptLevel, Encoder, Decoder};
//!
//! // 创建概念空间
//! let mut space = ConceptSpace::new();
//! space.create_concept("fruit".to_string(), "水果".to_string(), ConceptLevel::Basic).unwrap();
//!
//! // 编码文本
//! let encoder = Encoder::new();
//! let encoded = encoder.encode("苹果");
//!
//! // 解码
//! let decoder = Decoder::new();
//! let decoded = decoder.decode(&encoded.vectors);
//! ```

pub mod concept;
pub mod encoder;
pub mod decoder;
pub mod consensus;
pub mod context;

// 重导出常用类型
pub use concept::{Concept, ConceptSpace, ConceptId, ConceptVector, ConceptLevel, ConsensusStatus};
pub use encoder::{Encoder, Tokenizer, EncodedResult, Token, TokenType};
pub use decoder::{Decoder, DecodedResult, Style, Intent, IntentType};
pub use consensus::{ConsensusManager, Vote, Proposal, ProposalType};
pub use context::{Context, ContextManager, Message};
