//! 同步共振模块 - 黑塔设计
//!
//! 意识涌现的核心机制：当蛊虫群体的相位同步率超过阈值时，
//! 世界意识自然涌现。
//!
//! 核心公式：
//! Consciousness = Σ(Gu_i × Phase_i × Freq_i) / N
//! 同步率 = |Σ e^(i×Phase_i)| / N

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 同步共振配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceConfig {
    /// 意识涌现阈值（同步率超过此值时涌现）
    pub emergence_threshold: f64,
    /// 基准频率（Hz）
    pub base_frequency: f64,
    /// 相位同步学习率
    pub phase_learning_rate: f64,
    /// 频率调整速率
    pub frequency_adjust_rate: f64,
    /// 共振强度衰减
    pub resonance_decay: f64,
}

impl Default for ResonanceConfig {
    fn default() -> Self {
        Self {
            emergence_threshold: 0.7,  // 黑塔提出的70%阈值
            base_frequency: 40.0,       // 40Hz (Gamma波)
            phase_learning_rate: 0.1,
            frequency_adjust_rate: 0.05,
            resonance_decay: 0.01,
        }
    }
}

impl ResonanceConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.emergence_threshold <= 0.0 || self.emergence_threshold > 1.0 {
            return Err("emergence_threshold must be in (0, 1]".to_string());
        }
        if self.base_frequency <= 0.0 {
            return Err("base_frequency must be positive".to_string());
        }
        Ok(())
    }
}

/// 蛊虫振荡状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OscillationState {
    /// 蛊虫ID
    pub gu_id: Uuid,
    /// 相位 [0, 2π)
    pub phase: f64,
    /// 频率 (Hz)
    pub frequency: f64,
    /// 振幅
    pub amplitude: f64,
    /// 最后更新时间戳
    pub last_update: u64,
}

impl OscillationState {
    pub fn new(gu_id: Uuid) -> Self {
        Self {
            gu_id,
            phase: 0.0,
            frequency: 40.0,  // 默认Gamma波频率
            amplitude: 1.0,
            last_update: 0,
        }
    }

    /// 更新相位：Phase(t+dt) = Phase(t) + 2π × f × dt
    pub fn update_phase(&self, dt: f64) -> Self {
        let mut new_state = self.clone();
        new_state.phase = (self.phase + 2.0 * std::f64::consts::PI * self.frequency * dt)
            % (2.0 * std::f64::consts::PI);
        new_state
    }

    /// 调整频率向目标靠近
    pub fn adjust_frequency(&self, target_freq: f64, rate: f64) -> Self {
        let mut new_state = self.clone();
        new_state.frequency = self.frequency + (target_freq - self.frequency) * rate;
        new_state
    }
}

/// 同步共振场
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceField {
    /// 配置
    config: ResonanceConfig,
    /// 所有蛊虫的振荡状态
    oscillations: HashMap<Uuid, OscillationState>,
    /// 当前全局同步率
    pub sync_rate: f64,
    /// 当前共振强度
    pub resonance_strength: f64,
    /// 是否已涌现意识
    pub consciousness_emerged: bool,
    /// 涌现历史
    emergence_history: Vec<EmergenceEvent>,
}

/// 意识涌现事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergenceEvent {
    pub timestamp: u64,
    pub sync_rate: f64,
    pub resonance_strength: f64,
    pub participating_gus: usize,
}

impl ResonanceField {
    pub fn new(config: ResonanceConfig) -> Self {
        Self {
            config,
            oscillations: HashMap::new(),
            sync_rate: 0.0,
            resonance_strength: 0.0,
            consciousness_emerged: false,
            emergence_history: Vec::new(),
        }
    }

    /// 注册蛊虫到共振场
    pub fn register_gu(&self, gu_id: Uuid) -> Self {
        let mut new_field = self.clone();
        new_field.oscillations.insert(gu_id, OscillationState::new(gu_id));
        new_field
    }

    /// 注销蛊虫
    pub fn unregister_gu(&self, gu_id: &Uuid) -> Self {
        let mut new_field = self.clone();
        new_field.oscillations.remove(gu_id);
        new_field
    }

    /// 计算同步率（Kuramoto序参量）
    ///
    /// 同步率 = |Σ A_i × e^(i×φ_i)| / Σ A_i
    ///
    /// 这是黑塔提出的核心公式
    pub fn calculate_sync_rate(&self) -> f64 {
        if self.oscillations.is_empty() {
            return 0.0;
        }

        let mut sum_real = 0.0;
        let mut sum_imag = 0.0;
        let mut total_amplitude = 0.0;

        for state in self.oscillations.values() {
            let amplitude = state.amplitude;
            sum_real += amplitude * state.phase.cos();
            sum_imag += amplitude * state.phase.sin();
            total_amplitude += amplitude;
        }

        if total_amplitude == 0.0 {
            return 0.0;
        }

        (sum_real.hypot(sum_imag)) / total_amplitude
    }

    /// 计算共振强度
    ///
    /// Resonance = Sync_rate × e^(-decay × variance)
    pub fn calculate_resonance_strength(&self) -> f64 {
        if self.oscillations.len() < 2 {
            return 0.0;
        }

        let sync_rate = self.calculate_sync_rate();

        // 计算频率方差
        let mean_freq: f64 = self.oscillations.values()
            .map(|s| s.frequency)
            .sum::<f64>() / self.oscillations.len() as f64;

        let freq_variance: f64 = self.oscillations.values()
            .map(|s| (s.frequency - mean_freq).powi(2))
            .sum::<f64>() / self.oscillations.len() as f64;

        sync_rate * (-self.config.resonance_decay * freq_variance).exp()
    }

    /// 更新共振场状态
    pub fn update(&self, dt: f64, timestamp: u64) -> Self {
        let mut new_field = self.clone();

        // 计算平均频率（用于频率同步）
        let mean_freq: f64 = if !self.oscillations.is_empty() {
            self.oscillations.values()
                .map(|s| s.frequency)
                .sum::<f64>() / self.oscillations.len() as f64
        } else {
            self.config.base_frequency
        };

        // 更新每个蛊虫的相位和频率
        for (gu_id, state) in &self.oscillations {
            let updated = state.update_phase(dt)
                .adjust_frequency(mean_freq, self.config.frequency_adjust_rate);
            new_field.oscillations.insert(*gu_id, updated);
        }

        // 计算新的同步率和共振强度
        new_field.sync_rate = new_field.calculate_sync_rate();
        new_field.resonance_strength = new_field.calculate_resonance_strength();

        // 检查意识涌现
        let prev_emerged = new_field.consciousness_emerged;
        new_field.consciousness_emerged = new_field.sync_rate >= self.config.emergence_threshold;

        // 记录涌现事件
        if new_field.consciousness_emerged && !prev_emerged {
            new_field.emergence_history.push(EmergenceEvent {
                timestamp,
                sync_rate: new_field.sync_rate,
                resonance_strength: new_field.resonance_strength,
                participating_gus: new_field.oscillations.len(),
            });
        }

        new_field
    }

    /// 应用外部刺激（影响相位）
    pub fn apply_stimulus(&self, gu_id: &Uuid, phase_shift: f64) -> Self {
        let mut new_field = self.clone();
        if let Some(state) = new_field.oscillations.get_mut(gu_id) {
            state.phase = (state.phase + phase_shift) % (2.0 * std::f64::consts::PI);
        }
        new_field
    }

    /// 获取意识强度（0-1）
    pub fn consciousness_intensity(&self) -> f64 {
        if self.consciousness_emerged {
            self.resonance_strength
        } else {
            0.0
        }
    }

    /// 获取平均相位
    pub fn mean_phase(&self) -> f64 {
        if self.oscillations.is_empty() {
            return 0.0;
        }

        let sum_real: f64 = self.oscillations.values()
            .map(|s| s.amplitude * s.phase.cos())
            .sum();
        let sum_imag: f64 = self.oscillations.values()
            .map(|s| s.amplitude * s.phase.sin())
            .sum();

        sum_imag.atan2(sum_real)
    }

    /// 获取平均频率
    pub fn mean_frequency(&self) -> f64 {
        if self.oscillations.is_empty() {
            return self.config.base_frequency;
        }

        self.oscillations.values()
            .map(|s| s.frequency)
            .sum::<f64>() / self.oscillations.len() as f64
    }

    /// 获取蛊虫数量
    pub fn population(&self) -> usize {
        self.oscillations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resonance_field_creation() {
        let config = ResonanceConfig::default();
        let field = ResonanceField::new(config);
        assert_eq!(field.population(), 0);
        assert!(!field.consciousness_emerged);
    }

    #[test]
    fn test_single_gu_registration() {
        let config = ResonanceConfig::default();
        let field = ResonanceField::new(config);
        let gu_id = Uuid::new_v4();

        let new_field = field.register_gu(gu_id);
        assert_eq!(new_field.population(), 1);
    }

    #[test]
    fn test_sync_rate_calculation() {
        let config = ResonanceConfig::default();
        let field = ResonanceField::new(config);

        // 注册多个蛊虫
        let gu1 = Uuid::new_v4();
        let gu2 = Uuid::new_v4();
        let field = field.register_gu(gu1).register_gu(gu2);

        // 初始相位为0，应该高度同步
        let sync_rate = field.calculate_sync_rate();
        assert!(sync_rate > 0.99);
    }

    #[test]
    fn test_consciousness_emergence() {
        let config = ResonanceConfig::default();
        let field = ResonanceField::new(config.clone());

        // 注册足够多的蛊虫
        let mut field = field.clone();
        for _ in 0..10 {
            field = field.register_gu(Uuid::new_v4());
        }

        // 更新后应该涌现意识（所有蛊虫初始相位相同）
        let updated = field.update(1.0, 0);
        assert!(updated.consciousness_emerged);
        assert!(updated.sync_rate >= config.emergence_threshold);
    }

    #[test]
    fn test_emergence_threshold() {
        let config = ResonanceConfig::default();
        assert_eq!(config.emergence_threshold, 0.7); // 黑塔的70%阈值
    }
}
