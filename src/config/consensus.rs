//! 共识配置

use serde::{Deserialize, Serialize};

/// 共识配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// 最小投票数
    ///
    /// 知识成为共识所需的最小投票数
    pub min_votes: u32,

    /// 批准阈值
    ///
    /// 批准率需要达到此阈值才能通过
    pub approval_threshold: f64,

    /// 投票超时（秒）
    ///
    /// 投票的有效时间
    pub vote_timeout_secs: u64,

    /// 最高级别阈值（系统核心）
    pub system_core_threshold: f64,

    /// 基础级别阈值
    pub basic_threshold: f64,

    /// 普通级别阈值
    pub common_threshold: f64,

    /// 领域级别阈值
    pub domain_threshold: f64,

    /// 临时级别阈值
    pub temporary_threshold: f64,
}

impl ConsensusConfig {
    /// 创建默认配置
    pub fn new() -> Self {
        Self {
            min_votes: 3,
            approval_threshold: 0.6,
            vote_timeout_secs: 3600, // 1小时
            system_core_threshold: 1.0,
            basic_threshold: 0.9,
            common_threshold: 0.7,
            domain_threshold: 0.6,
            temporary_threshold: 0.5,
        }
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), String> {
        if self.min_votes == 0 {
            return Err("min_votes 必须大于0".to_string());
        }
        if self.approval_threshold <= 0.0 || self.approval_threshold > 1.0 {
            return Err("approval_threshold 必须在 (0, 1] 范围内".to_string());
        }
        if self.vote_timeout_secs == 0 {
            return Err("vote_timeout_secs 必须大于0".to_string());
        }
        self.validate_thresholds()?;
        Ok(())
    }

    /// 验证阈值层级
    fn validate_thresholds(&self) -> Result<(), String> {
        if self.system_core_threshold < self.basic_threshold {
            return Err("system_core_threshold 应该 >= basic_threshold".to_string());
        }
        if self.basic_threshold < self.common_threshold {
            return Err("basic_threshold 应该 >= common_threshold".to_string());
        }
        if self.common_threshold < self.domain_threshold {
            return Err("common_threshold 应该 >= domain_threshold".to_string());
        }
        if self.domain_threshold < self.temporary_threshold {
            return Err("domain_threshold 应该 >= temporary_threshold".to_string());
        }
        Ok(())
    }

    /// 检查投票是否有效
    pub fn is_vote_valid(&self, vote_count: u32, approval_rate: f64, threshold: f64) -> bool {
        vote_count >= self.min_votes && approval_rate >= threshold
    }
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_valid() {
        let config = ConsensusConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_threshold_hierarchy() {
        let config = ConsensusConfig::new();
        assert!(config.system_core_threshold >= config.basic_threshold);
        assert!(config.basic_threshold >= config.common_threshold);
        assert!(config.common_threshold >= config.domain_threshold);
        assert!(config.domain_threshold >= config.temporary_threshold);
    }

    #[test]
    fn test_vote_validation() {
        let config = ConsensusConfig::new();
        assert!(config.is_vote_valid(3, 0.9, 0.7));
        assert!(!config.is_vote_valid(2, 0.9, 0.7)); // 投票数不足
        assert!(!config.is_vote_valid(3, 0.5, 0.7)); // 批准率不足
    }
}
