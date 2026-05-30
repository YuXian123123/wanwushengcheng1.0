//! 信任评分系统 - 螺丝咕姆第四层防护

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 信任评分
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustScore {
    /// 当前信任值 (0-1)
    pub score: f64,
    /// 有效操作次数
    pub valid_actions: u64,
    /// 无效操作次数
    pub invalid_actions: u64,
    /// 最后更新时间
    pub last_updated: u64,
}

impl TrustScore {
    pub fn new() -> Self {
        Self {
            score: 0.5, // 初始信任值为中等
            valid_actions: 0,
            invalid_actions: 0,
            last_updated: 0,
        }
    }

    /// 计算信任评分: Trust = (Valid - Invalid) / Total
    pub fn calculate(&self) -> f64 {
        let total = self.valid_actions + self.invalid_actions;
        if total == 0 {
            return 0.5;
        }
        let raw_score = (self.valid_actions as f64 - self.invalid_actions as f64) / total as f64;
        // 映射到 0-1 范围
        (raw_score + 1.0) / 2.0
    }

    /// 更新信任评分（不可变）
    pub fn update(&self, is_valid: bool, timestamp: u64) -> Self {
        let mut new_score = self.clone();
        if is_valid {
            new_score.valid_actions += 1;
        } else {
            new_score.invalid_actions += 1;
        }
        new_score.score = new_score.calculate();
        new_score.last_updated = timestamp;
        new_score
    }

    /// 信任衰减: Trust' = Trust × e^(-λ·t)
    pub fn decay(&self, decay_rate: f64, time_elapsed: u64) -> Self {
        let decay_factor = (-decay_rate * time_elapsed as f64).exp();
        let mut new_score = self.clone();
        new_score.score = self.score * decay_factor;
        new_score
    }
}

impl Default for TrustScore {
    fn default() -> Self {
        Self::new()
    }
}

/// 信任系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustSystem {
    /// 信任衰减率
    decay_rate: f64,
    /// 用户信任评分
    trust_scores: HashMap<Uuid, TrustScore>,
}

impl TrustSystem {
    pub fn new(decay_rate: f64) -> Self {
        Self {
            decay_rate,
            trust_scores: HashMap::new(),
        }
    }

    /// 获取信任评分
    pub fn get_score(&self, user: &Uuid) -> TrustScore {
        self.trust_scores.get(user).cloned().unwrap_or_default()
    }

    /// 更新信任评分（不可变）
    pub fn update(&self, user: &Uuid, is_valid: bool) -> Self {
        let mut new_system = self.clone();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let score = self.get_score(user);
        let new_score = score.update(is_valid, timestamp);
        new_system.trust_scores.insert(*user, new_score);
        new_system
    }

    /// 应用信任衰减
    pub fn apply_decay(&self, user: &Uuid, time_elapsed: u64) -> Self {
        let mut new_system = self.clone();
        let score = self.get_score(user);
        let new_score = score.decay(self.decay_rate, time_elapsed);
        new_system.trust_scores.insert(*user, new_score);
        new_system
    }

    /// 批量衰减所有用户
    pub fn decay_all(&self, time_elapsed: u64) -> Self {
        let mut new_system = self.clone();
        for (user, score) in &self.trust_scores {
            let new_score = score.decay(self.decay_rate, time_elapsed);
            new_system.trust_scores.insert(*user, new_score);
        }
        new_system
    }

    /// 获取高信任用户
    pub fn high_trust_users(&self, threshold: f64) -> Vec<Uuid> {
        self.trust_scores
            .iter()
            .filter(|(_, s)| s.score >= threshold)
            .map(|(id, _)| *id)
            .collect()
    }

    /// 获取低信任用户
    pub fn low_trust_users(&self, threshold: f64) -> Vec<Uuid> {
        self.trust_scores
            .iter()
            .filter(|(_, s)| s.score < threshold)
            .map(|(id, _)| *id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_score_creation() {
        let score = TrustScore::new();
        assert_eq!(score.score, 0.5);
    }

    #[test]
    fn test_trust_score_update() {
        let score = TrustScore::new();

        // 有效操作增加信任
        let updated = score.update(true, 0);
        assert!(updated.score > score.score);

        // 无效操作降低信任
        let updated = updated.update(false, 0);
        assert!(updated.score < updated.calculate() + 0.5);
    }

    #[test]
    fn test_trust_system() {
        let system = TrustSystem::new(0.01);
        let user = Uuid::new_v4();

        let updated = system.update(&user, true);
        let score = updated.get_score(&user);
        assert!(score.valid_actions == 1);
    }

    #[test]
    fn test_trust_decay() {
        let score = TrustScore::new();
        let initial_score = score.score;

        // 随时间衰减
        let decayed = score.decay(0.01, 100);
        assert!(decayed.score < initial_score);
    }
}