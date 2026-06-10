//! 生成奖励系统
//!
//! 计算 3D 场景生成的金币奖励和惩罚

use serde::{Deserialize, Serialize};

/// 生成奖励配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRewardConfig {
    /// 成功生成基础奖励
    pub success_base_reward: f64,
    /// 失败惩罚金额
    pub failure_penalty: f64,
    /// 实体数量奖励系数（每个实体）
    pub entity_reward_per_unit: f64,
    /// 关系数量奖励系数（每个关系）
    pub relation_reward_per_unit: f64,
    /// 最大奖励上限
    pub max_reward: f64,
    /// 最小余额
    pub min_balance: f64,
    /// 每分钟最大生成次数
    pub max_generations_per_minute: usize,
    /// 相似度阈值（超过此值不奖励）
    pub similarity_threshold: f64,
}

impl Default for GenerationRewardConfig {
    fn default() -> Self {
        Self {
            success_base_reward: 10.0,
            failure_penalty: 5.0,
            entity_reward_per_unit: 2.0,
            relation_reward_per_unit: 1.5,
            max_reward: 100.0,
            min_balance: 0.0,
            max_generations_per_minute: 10,
            similarity_threshold: 0.8,
        }
    }
}

/// 金币变化记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinChange {
    /// 变化金额（正=获得，负=扣除）
    pub amount: f64,
    /// 变化类型
    pub change_type: String, // "reward" | "penalty"
    /// 变化后余额
    pub new_balance: f64,
    /// 原因说明
    pub reason: String,
    /// 相关实体数
    pub entity_count: usize,
    /// 相关关系数
    pub relation_count: usize,
}

/// 生成结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResult {
    /// 是否成功
    pub success: bool,
    /// 实体数量
    pub entity_count: usize,
    /// 关系数量
    pub relation_count: usize,
    /// 错误信息
    pub error: Option<String>,
}

impl GenerationRewardConfig {
    /// 创建新配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 计算生成奖励
    ///
    /// # 公式
    /// - 成功: R = base + (entity_count × entity_bonus) + (relation_count × relation_bonus)
    /// - 失败: R = -failure_penalty
    pub fn calculate_reward(&self, result: &GenerationResult) -> f64 {
        if !result.success {
            return -self.failure_penalty;
        }

        let entity_bonus = result.entity_count as f64 * self.entity_reward_per_unit;
        let relation_bonus = result.relation_count as f64 * self.relation_reward_per_unit;

        let reward = self.success_base_reward + entity_bonus + relation_bonus;
        reward.min(self.max_reward)
    }

    /// 检查用户是否可以支付失败惩罚
    pub fn can_afford_failure(&self, current_balance: f64) -> bool {
        current_balance >= self.failure_penalty
    }

    /// 检查余额是否有效
    pub fn is_valid_balance(&self, balance: f64) -> bool {
        balance >= self.min_balance
    }

    /// 创建金币变化记录
    pub fn create_coin_change(
        &self,
        result: &GenerationResult,
        current_balance: f64,
    ) -> CoinChange {
        let amount = self.calculate_reward(result);
        let new_balance = (current_balance + amount).max(self.min_balance);

        CoinChange {
            amount,
            change_type: if amount >= 0.0 { "reward".to_string() } else { "penalty".to_string() },
            new_balance,
            reason: if result.success {
                format!("生成成功: {} 个实体, {} 个关系", result.entity_count, result.relation_count)
            } else {
                format!("生成失败: {}", result.error.as_deref().unwrap_or("未知错误"))
            },
            entity_count: result.entity_count,
            relation_count: result.relation_count,
        }
    }
}

/// 用户生成历史记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationHistory {
    /// 用户ID
    pub user_id: String,
    /// 历史输入文本列表（用于相似度检测）
    pub history: Vec<HistoryEntry>,
    /// 最近一分钟生成次数
    pub recent_count: usize,
    /// 最近一分钟时间戳
    pub recent_timestamp: u64,
}

/// 历史记录条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// 输入文本
    pub text: String,
    /// 时间戳
    pub timestamp: u64,
    /// 是否成功
    pub success: bool,
    /// 金币变化
    pub coin_change: f64,
}

impl GenerationHistory {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            history: Vec::new(),
            recent_count: 0,
            recent_timestamp: 0,
        }
    }

    /// 添加记录
    pub fn add_entry(&mut self, text: &str, success: bool, coin_change: f64, timestamp: u64) {
        self.history.push(HistoryEntry {
            text: text.to_string(),
            timestamp,
            success,
            coin_change,
        });

        // 保持最近 100 条记录
        if self.history.len() > 100 {
            self.history.remove(0);
        }
    }

    /// 检查频率限制
    pub fn check_rate_limit(&mut self, config: &GenerationRewardConfig, current_timestamp: u64) -> bool {
        // 如果超过一分钟，重置计数
        if current_timestamp - self.recent_timestamp > 60 {
            self.recent_count = 0;
            self.recent_timestamp = current_timestamp;
        }

        self.recent_count < config.max_generations_per_minute
    }

    /// 计算与历史文本的相似度
    pub fn check_similarity(&self, text: &str, config: &GenerationRewardConfig) -> bool {
        for entry in self.history.iter().rev().take(10) {
            let similarity = calculate_text_similarity(&entry.text, text);
            if similarity >= config.similarity_threshold {
                return false; // 太相似，不奖励
            }
        }
        true // 可以奖励
    }
}

/// 计算两个文本的相似度（简单 Jaccard 相似度）
pub fn calculate_text_similarity(text1: &str, text2: &str) -> f64 {
    let words1: std::collections::HashSet<&str> = text1.split_whitespace().collect();
    let words2: std::collections::HashSet<&str> = text2.split_whitespace().collect();

    if words1.is_empty() && words2.is_empty() {
        return 1.0;
    }

    let intersection = words1.intersection(&words2).count();
    let union = words1.union(&words2).count();

    if union == 0 {
        return 0.0;
    }

    intersection as f64 / union as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_reward_success() {
        let config = GenerationRewardConfig::default();
        let result = GenerationResult {
            success: true,
            entity_count: 3,
            relation_count: 2,
            error: None,
        };

        let reward = config.calculate_reward(&result);
        // 基础 10 + 实体 3*2 + 关系 2*1.5 = 10 + 6 + 3 = 19
        assert_eq!(reward, 19.0);
    }

    #[test]
    fn test_calculate_reward_failure() {
        let config = GenerationRewardConfig::default();
        let result = GenerationResult {
            success: false,
            entity_count: 0,
            relation_count: 0,
            error: Some("解析失败".to_string()),
        };

        let reward = config.calculate_reward(&result);
        assert_eq!(reward, -5.0);
    }

    #[test]
    fn test_can_afford_failure() {
        let config = GenerationRewardConfig::default();
        assert!(config.can_afford_failure(10.0));
        assert!(!config.can_afford_failure(3.0));
    }

    #[test]
    fn test_text_similarity() {
        let sim = calculate_text_similarity("房子里有桌子", "房子里有桌子");
        assert_eq!(sim, 1.0);

        let sim = calculate_text_similarity("房子里有桌子", "房子里有椅子");
        assert!(sim > 0.5);

        let sim = calculate_text_similarity("房子里有桌子", "一棵大树");
        assert!(sim < 0.5);
    }
}
