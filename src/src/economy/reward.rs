//! 奖励系统 - 拉蒂奥优雅公式设计

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::config::RewardConfig;
use super::security::SecurityCheckResult;

/// 奖励类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RewardType {
    /// 任务奖励
    Task,
    /// 知识奖励
    Knowledge,
    /// 创新奖励
    Innovation,
}

/// 奖励计算结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardResult {
    /// 奖励金额
    pub amount: f64,
    /// 奖励类型
    pub kind: RewardType,
    /// 基础奖励
    pub base: f64,
    /// 质量加成
    pub quality_bonus: f64,
    /// 新颖性加成
    pub novelty_bonus: f64,
    /// 安全检查是否通过
    pub security_passed: bool,
}

impl RewardResult {
    pub fn new(
        kind: RewardType,
        base: f64,
        quality: f64,
        novelty: f64,
        security_passed: bool,
    ) -> Self {
        let amount = if security_passed {
            base + quality + novelty
        } else {
            0.0 // 安全检查未通过，无奖励
        };

        Self {
            amount,
            kind,
            base,
            quality_bonus: quality,
            novelty_bonus: novelty,
            security_passed,
        }
    }
}

/// 奖励系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardSystem {
    /// 配置
    config: RewardConfig,
    /// 奖励历史
    history: Vec<(Uuid, RewardResult)>,
}

impl RewardSystem {
    pub fn new(config: RewardConfig) -> Self {
        Self {
            config,
            history: Vec::new(),
        }
    }

    /// 计算任务奖励: R = base × difficulty × urgency
    pub fn calculate_task_reward(&self, difficulty: f64, urgency: f64) -> f64 {
        self.config.base_task_reward * difficulty * urgency
    }

    /// 计算知识奖励: R = value × quality
    pub fn calculate_knowledge_reward(&self, value: f64, quality: f64) -> f64 {
        self.config.base_knowledge_reward + value * quality * self.config.quality_weight
    }

    /// 计算创新奖励: R = base × novelty_factor
    pub fn calculate_innovation_reward(&self, novelty_factor: f64) -> f64 {
        self.config.base_innovation_reward * novelty_factor
    }

    /// 发放奖励（带安全检查）
    pub fn grant(
        &self,
        recipient: Uuid,
        kind: RewardType,
        security_result: Option<&SecurityCheckResult>,
        params: RewardParams,
    ) -> Self {
        let mut new_system = self.clone();

        // 安全检查
        let security_passed = security_result.map(|r| r.passed).unwrap_or(true);

        // 计算奖励
        let (base, quality, novelty) = match kind {
            RewardType::Task => {
                let base = self.calculate_task_reward(params.difficulty, params.urgency);
                (base, 0.0, 0.0)
            }
            RewardType::Knowledge => {
                let base = self.calculate_knowledge_reward(params.value, params.quality);
                let quality_bonus = params.quality * self.config.quality_weight;
                (base, quality_bonus, 0.0)
            }
            RewardType::Innovation => {
                let base = self.calculate_innovation_reward(params.novelty);
                let novelty_bonus = params.novelty * self.config.novelty_weight;
                (base, 0.0, novelty_bonus)
            }
        };

        let result = RewardResult::new(kind, base, quality, novelty, security_passed);
        new_system.history.push((recipient, result));

        new_system
    }

    /// 获取用户总奖励
    pub fn total_rewards(&self, user: &Uuid) -> f64 {
        self.history
            .iter()
            .filter(|(id, _)| id == user)
            .map(|(_, r)| r.amount)
            .sum()
    }

    /// 获取用户奖励历史
    pub fn get_user_history(&self, user: &Uuid) -> Vec<&RewardResult> {
        self.history
            .iter()
            .filter(|(id, _)| id == user)
            .map(|(_, r)| r)
            .collect()
    }
}

/// 奖励参数
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RewardParams {
    /// 难度
    pub difficulty: f64,
    /// 紧急程度
    pub urgency: f64,
    /// 价值
    pub value: f64,
    /// 质量
    pub quality: f64,
    /// 新颖性
    pub novelty: f64,
}

impl RewardParams {
    pub fn new() -> Self {
        Self {
            difficulty: 1.0,
            urgency: 1.0,
            value: 1.0,
            quality: 1.0,
            novelty: 1.0,
        }
    }

    pub fn with_difficulty(mut self, difficulty: f64) -> Self {
        self.difficulty = difficulty;
        self
    }

    pub fn with_urgency(mut self, urgency: f64) -> Self {
        self.urgency = urgency;
        self
    }

    pub fn with_quality(mut self, quality: f64) -> Self {
        self.quality = quality;
        self
    }

    pub fn with_novelty(mut self, novelty: f64) -> Self {
        self.novelty = novelty;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_reward() {
        let config = RewardConfig::default();
        let system = RewardSystem::new(config);

        let reward = system.calculate_task_reward(2.0, 1.5);
        assert!(reward > system.config.base_task_reward);
    }

    #[test]
    fn test_knowledge_reward() {
        let config = RewardConfig::default();
        let system = RewardSystem::new(config);

        let reward = system.calculate_knowledge_reward(10.0, 0.8);
        assert!(reward > 0.0);
    }

    #[test]
    fn test_innovation_reward() {
        let config = RewardConfig::default();
        let system = RewardSystem::new(config);

        let reward = system.calculate_innovation_reward(2.0);
        assert_eq!(reward, system.config.base_innovation_reward * 2.0);
    }

    #[test]
    fn test_security_blocked_reward() {
        let config = RewardConfig::default();
        let system = RewardSystem::new(config);
        let user = Uuid::new_v4();

        // 模拟安全检查失败
        let security_result = SecurityCheckResult {
            passed: false,
            quality: super::super::security::QualityScore::new(0.1, 0.1, 0.1, 0.1),
            similarity: super::super::security::SimilarityResult {
                score: 0.9,
                threshold: 0.8,
                is_violation: true,
                max_historical_similarity: 0.9,
            },
            behavior: super::super::security::BehaviorPattern {
                frequency_factor: 0.5,
                timing_uniformity: 0.5,
                content_diversity: 0.5,
                is_anomaly: false,
                anomaly_type: None,
            },
            trust: super::super::security::TrustScore::new(),
            risk_score: 0.8,
        };

        let system = system.grant(user, RewardType::Knowledge, Some(&security_result), RewardParams::new());

        // 安全检查未通过，奖励应为0
        let total = system.total_rewards(&user);
        assert_eq!(total, 0.0);
    }
}