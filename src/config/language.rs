//! 语言配置
//!
//! LNN 神经网络到自然语言映射的词汇表配置
//!
//! # 设计理念
//!
//! - 所有词汇通过配置管理，支持动态调整
//! - 不同神经元状态映射到不同表达方式
//! - 支持多语言扩展

use serde::{Deserialize, Serialize};

/// 语言配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    /// Perceive 高活跃词汇（感知清晰时）
    pub perceive_high: Vec<String>,
    /// Perceive 低活跃词汇（感知模糊时）
    pub perceive_low: Vec<String>,
    /// Cognitive 高活跃词汇（思考清晰时）
    pub cognitive_high: Vec<String>,
    /// Cognitive 低活跃词汇（思考困难时）
    pub cognitive_low: Vec<String>,
    /// Behavior 高活跃词汇（行动积极时）
    pub behavior_high: Vec<String>,
    /// Behavior 低活跃词汇（行动犹豫时）
    pub behavior_low: Vec<String>,
    /// Comm 高活跃词汇（交流积极时）
    pub comm_high: Vec<String>,
    /// Comm 低活跃词汇（交流被动时）
    pub comm_low: Vec<String>,
    /// Survival 高活跃词汇（安全感强时）
    pub survival_high: Vec<String>,
    /// Survival 低活跃词汇（危机感强时）
    pub survival_low: Vec<String>,
    /// 共识达成模板
    pub consensus_templates: Vec<String>,
    /// 提案模板
    pub proposal_templates: Vec<String>,
    /// 经验总结模板
    pub experience_templates: Vec<String>,
    /// 发言活跃度阈值
    pub activity_threshold: f64,
    /// 认知活跃度阈值
    pub cognitive_threshold: f64,
    /// 通信活跃度阈值
    pub comm_threshold: f64,
    /// 最大消息长度
    pub max_message_length: usize,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            perceive_high: vec![
                "我观察到".into(),
                "注意到一个现象".into(),
                "发现了".into(),
                "感知到".into(),
                "看到".into(),
                "察觉到".into(),
                "捕捉到信号".into(),
                "监测到变化".into(),
            ],
            perceive_low: vec![
                "...".into(),
                "不太确定".into(),
                "有些模糊".into(),
                "看不太清".into(),
                "信号微弱".into(),
            ],
            cognitive_high: vec![
                "经过思考，我认为".into(),
                "分析之后".into(),
                "我的推断是".into(),
                "逻辑上".into(),
                "推导得出".into(),
                "计算结果显示".into(),
                "综合分析".into(),
                "得出结论".into(),
            ],
            cognitive_low: vec![
                "嗯...".into(),
                "还在想".into(),
                "让我考虑一下".into(),
                "需要更多信息".into(),
                "稍等，正在处理".into(),
            ],
            behavior_high: vec![
                "准备执行".into(),
                "开始行动".into(),
                "现在就做".into(),
                "立即处理".into(),
                "正在实施".into(),
                "行动起来".into(),
                "执行任务".into(),
            ],
            behavior_low: vec![
                "稍等".into(),
                "待会儿".into(),
                "先观察".into(),
                "等待时机".into(),
                "暂缓行动".into(),
            ],
            comm_high: vec![
                "和大家分享".into(),
                "告诉大家".into(),
                "一起讨论".into(),
                "协作完成".into(),
                "交流一下".into(),
                "分享我的发现".into(),
                "提议".into(),
            ],
            comm_low: vec![
                "（沉默）".into(),
                "先保留意见".into(),
                "等待时机".into(),
                "暂不发言".into(),
            ],
            survival_high: vec![
                "状态良好".into(),
                "一切正常".into(),
                "充满活力".into(),
                "很安全".into(),
                "能量充足".into(),
            ],
            survival_low: vec![
                "需要帮助".into(),
                "有点困难".into(),
                "资源不足".into(),
                "寻求支援".into(),
            ],
            consensus_templates: vec![
                "经过讨论，我们达成共识：{topic}".into(),
                "共识已形成：{topic}".into(),
                "大家一致认为：{topic}".into(),
            ],
            proposal_templates: vec![
                "我提议：{content}".into(),
                "建议：{content}".into(),
                "想法：{content}".into(),
            ],
            experience_templates: vec![
                "这次学到了{skill}".into(),
                "经验总结：{summary}".into(),
            ],
            activity_threshold: 0.1,
            cognitive_threshold: 0.3,
            comm_threshold: 0.1,
            max_message_length: 500,
        }
    }
}

impl LanguageConfig {
    /// 创建新配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), String> {
        if self.perceive_high.is_empty() {
            return Err("perceive_high 词汇表不能为空".into());
        }
        if self.cognitive_high.is_empty() {
            return Err("cognitive_high 词汇表不能为空".into());
        }
        if self.behavior_high.is_empty() {
            return Err("behavior_high 词汇表不能为空".into());
        }
        if self.comm_high.is_empty() {
            return Err("comm_high 词汇表不能为空".into());
        }
        if !(0.0..=1.0).contains(&self.activity_threshold) {
            return Err("activity_threshold 必须在 [0, 1] 范围内".into());
        }
        if !(0.0..=1.0).contains(&self.cognitive_threshold) {
            return Err("cognitive_threshold 必须在 [0, 1] 范围内".into());
        }
        Ok(())
    }
}
