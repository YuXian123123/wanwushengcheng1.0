//! 创造力系统
//!
//! 实现世界智能体的创造能力：
//! - 创造涌现机制
//! - 创造安全沙盒
//! - 创造测度
//!
//! 核心公式：
//! Creativity = Diversity × Combination_Rate × Novelty
//! Safe_Create = Idea × Risk_Score^(-1) × Approval_Rate

pub mod sandbox;

pub use sandbox::{
    CreationSandbox, SandboxConfig, SandboxStats,
    CreationIdea, CreationType, CreationLevel,
    RiskAssessment, RiskCategory, PotentialImpact,
    SimulationResult, ApprovalResult, Vote,
    CreateResult, ExecutionRecord, RollbackRecord,
};

use serde::{Deserialize, Serialize};

// ============================================================================
// 创造测度
// ============================================================================

/// 创造测度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativityMeasure {
    /// 新颖度
    pub novelty: f64,
    /// 实用性
    pub utility: f64,
    /// 可行性
    pub feasibility: f64,
    /// 综合创造分数
    pub score: f64,
}

impl CreativityMeasure {
    pub fn new(novelty: f64, utility: f64, feasibility: f64) -> Self {
        let score = novelty * utility * feasibility;
        Self {
            novelty,
            utility,
            feasibility,
            score,
        }
    }
}

/// 创造统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativityStats {
    /// 创造总数
    pub total_creations: usize,
    /// 成功创造数
    pub successful_creations: usize,
    /// 回滚次数
    pub rollbacks: usize,
    /// 平均新颖度
    pub avg_novelty: f64,
    /// 平均创造分数
    pub avg_score: f64,
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creativity_measure() {
        let measure = CreativityMeasure::new(0.8, 0.7, 0.9);
        assert!((measure.score - 0.504).abs() < 0.01);
    }
}
