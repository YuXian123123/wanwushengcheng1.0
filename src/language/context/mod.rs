//! 上下文管理模块
//!
//! 管理会话、任务、关系、世界四种上下文
//!
//! # 上下文类型
//!
//! - **会话上下文**: 当前对话的上下文
//! - **任务上下文**: 执行任务的上下文
//! - **关系上下文**: 实体间关系的上下文
//! - **世界上下文**: 全局世界状态的上下文

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::config::context::ContextConfig;

/// 上下文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    /// 会话ID
    pub session_id: String,
    /// 会话参与者
    pub participants: Vec<String>,
    /// 会话历史
    pub history: Vec<Message>,
    /// 当前焦点
    pub focus: Option<String>,
    /// 任务信息
    pub task: Option<TaskInfo>,
    /// 实体引用（指代消解用）
    pub entity_refs: HashMap<String, String>,
    /// 关系上下文
    pub relations: HashMap<String, f64>,
    /// 世界上下文
    pub world_state: HashMap<String, String>,
}

/// 消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// 发送者
    pub sender: String,
    /// 内容
    pub content: String,
    /// 时间戳
    pub timestamp: u64,
    /// 提取的实体
    pub entities: Vec<String>,
}

/// 任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    /// 任务ID
    pub id: String,
    /// 任务类型
    pub task_type: String,
    /// 目标
    pub goal: String,
    /// 进度
    pub progress: f64,
    /// 优先级
    pub priority: f64,
}

impl Context {
    /// 创建新上下文
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            participants: Vec::new(),
            history: Vec::new(),
            focus: None,
            task: None,
            entity_refs: HashMap::new(),
            relations: HashMap::new(),
            world_state: HashMap::new(),
        }
    }

    /// 添加消息
    pub fn add_message(&mut self, sender: String, content: String, entities: Vec<String>) {
        let timestamp = chrono::Utc::now().timestamp_millis() as u64;
        self.history.push(Message { sender, content, timestamp, entities });
    }

    /// 添加实体引用
    pub fn add_entity_ref(&mut self, reference: String, entity: String) {
        self.entity_refs.insert(reference, entity);
    }

    /// 解析指代
    pub fn resolve_reference(&self, reference: &str) -> Option<&String> {
        self.entity_refs.get(reference)
    }

    /// 获取最近的实体
    pub fn get_recent_entities(&self, n: usize) -> Vec<&String> {
        self.history.iter()
            .rev()
            .flat_map(|m| m.entities.iter())
            .take(n)
            .collect()
    }

    /// 添加关系
    pub fn add_relation(&mut self, entity1: &str, entity2: &str, strength: f64) {
        let key = format!("{}:{}", entity1, entity2);
        self.relations.insert(key, strength);
    }

    /// 获取关系强度
    pub fn get_relation(&self, entity1: &str, entity2: &str) -> Option<f64> {
        let key = format!("{}:{}", entity1, entity2);
        self.relations.get(&key).copied()
    }

    /// 设置世界状态
    pub fn set_world_state(&mut self, key: String, value: String) {
        self.world_state.insert(key, value);
    }

    /// 获取世界状态
    pub fn get_world_state(&self, key: &str) -> Option<&String> {
        self.world_state.get(key)
    }
}

/// 上下文管理器
pub struct ContextManager {
    /// 配置
    config: ContextConfig,
}

impl ContextManager {
    /// 创建新管理器
    pub fn new() -> Self {
        Self {
            config: ContextConfig::new(),
        }
    }

    /// 使用配置创建管理器
    pub fn with_config(config: ContextConfig) -> Self {
        Self { config }
    }

    /// 构建上下文
    pub fn build_context(&self, session_id: String) -> Context {
        Context::new(session_id)
    }

    /// 处理指代消解
    pub fn resolve_pronouns(&self, text: &str, context: &Context) -> HashMap<String, String> {
        let mut resolved = HashMap::new();

        // 中文代词
        let pronouns = ["它", "他", "她", "这", "那", "这个", "那个"];

        for pronoun in &pronouns {
            if text.contains(pronoun) {
                // 尝试从最近实体中解析
                if let Some(entity) = context.get_recent_entities(1).first() {
                    resolved.insert(pronoun.to_string(), (*entity).clone());
                }
            }
        }

        resolved
    }

    /// 清理过期历史
    pub fn cleanup_history(&self, context: &mut Context) {
        if context.history.len() > self.config.max_history {
            let excess = context.history.len() - self.config.max_history;
            context.history.drain(0..excess);
        }
    }

    /// 更新关系强度（使用配置）
    pub fn update_relation(&self, current: f64, delta: f64) -> f64 {
        self.config.update_relationship(current, delta)
    }

    /// 衰减任务优先级（使用配置）
    pub fn decay_priority(&self, priority: f64, time_elapsed: f64) -> f64 {
        self.config.decay_priority(priority, time_elapsed)
    }

    /// 创建注意力掩码
    ///
    /// 根据焦点和历史生成注意力权重
    pub fn create_attention_mask(&self, context: &Context, length: usize) -> Vec<f64> {
        let mut mask = vec![1.0; length];

        // 如果有焦点，增加焦点位置的权重
        if let Some(ref focus) = context.focus {
            for (i, msg) in context.history.iter().enumerate().rev() {
                if msg.content.contains(focus) && i < length {
                    mask[i] = 2.0;
                    break;
                }
            }
        }

        // 归一化
        let sum: f64 = mask.iter().sum();
        if sum > 0.0 {
            for m in &mut mask {
                *m /= sum;
            }
        }

        mask
    }

    /// 获取配置
    pub fn config(&self) -> &ContextConfig {
        &self.config
    }
}

impl Default for ContextManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let ctx = Context::new("session_1".to_string());
        assert_eq!(ctx.session_id, "session_1");
        assert!(ctx.history.is_empty());
    }

    #[test]
    fn test_add_message() {
        let mut ctx = Context::new("session_1".to_string());
        ctx.add_message("user".to_string(), "你好".to_string(), vec![]);

        assert_eq!(ctx.history.len(), 1);
        assert_eq!(ctx.history[0].content, "你好");
    }

    #[test]
    fn test_entity_reference() {
        let mut ctx = Context::new("session_1".to_string());
        ctx.add_entity_ref("它".to_string(), "苹果".to_string());

        assert_eq!(ctx.resolve_reference("它"), Some(&"苹果".to_string()));
    }

    #[test]
    fn test_pronoun_resolution() {
        let manager = ContextManager::new();
        let mut ctx = Context::new("session_1".to_string());
        ctx.add_message("user".to_string(), "我想买iPhone".to_string(), vec!["iPhone".to_string()]);

        let resolved = manager.resolve_pronouns("它贵吗？", &ctx);

        // 由于代词匹配，应该能解析出实体
        assert!(!resolved.is_empty() || ctx.get_recent_entities(1).is_empty());
    }

    #[test]
    fn test_relations() {
        let mut ctx = Context::new("session_1".to_string());
        ctx.add_relation("A", "B", 0.8);

        assert_eq!(ctx.get_relation("A", "B"), Some(0.8));
        assert_eq!(ctx.get_relation("B", "A"), None);
    }

    #[test]
    fn test_world_state() {
        let mut ctx = Context::new("session_1".to_string());
        ctx.set_world_state("location".to_string(), "北京".to_string());

        assert_eq!(ctx.get_world_state("location"), Some(&"北京".to_string()));
    }

    #[test]
    fn test_attention_mask() {
        let manager = ContextManager::new();
        let mut ctx = Context::new("session_1".to_string());
        ctx.focus = Some("重要".to_string());
        ctx.add_message("user".to_string(), "这是重要信息".to_string(), vec![]);

        let mask = manager.create_attention_mask(&ctx, 5);
        assert_eq!(mask.len(), 5);

        // 掩码应该归一化
        let sum: f64 = mask.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
    }
}
