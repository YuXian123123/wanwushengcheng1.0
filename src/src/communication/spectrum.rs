//! 动态频谱分配模块
//!
//! 实现通信资源的动态频谱管理

use std::collections::HashMap;
use super::message::ChannelType;

/// 频段类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrequencyBand {
    /// 低频 - 世界广播
    Low,
    /// 中低频 - 小组协作
    MediumLow,
    /// 中频 - 个人对话
    Medium,
    /// 中高频 - 紧急事件
    MediumHigh,
    /// 高频 - 知识流动
    High,
    /// 超高频 - 交易信号
    UltraHigh,
}

/// 频段配置
#[derive(Debug, Clone)]
pub struct BandConfig {
    /// 带宽容量
    pub capacity: u32,
    /// 当前使用量
    pub usage: u32,
    /// 成本系数
    pub cost_factor: f64,
    /// 优先级
    pub priority: u8,
}

impl BandConfig {
    pub fn new(capacity: u32, cost_factor: f64, priority: u8) -> Self {
        Self {
            capacity,
            usage: 0,
            cost_factor,
            priority,
        }
    }

    /// 使用率
    pub fn usage_ratio(&self) -> f64 {
        if self.capacity == 0 { return 0.0; }
        self.usage as f64 / self.capacity as f64
    }

    /// 是否可用
    pub fn is_available(&self) -> bool {
        self.usage < self.capacity
    }

    /// 分配带宽（返回新配置）
    pub fn with_allocated(&self, amount: u32) -> Option<Self> {
        if self.usage + amount > self.capacity {
            return None;
        }
        Some(Self {
            usage: self.usage + amount,
            ..self.clone()
        })
    }

    /// 释放带宽（返回新配置）
    pub fn with_released(&self, amount: u32) -> Self {
        Self {
            usage: self.usage.saturating_sub(amount),
            ..self.clone()
        }
    }
}

/// 频谱管理器
#[derive(Debug, Clone)]
pub struct SpectrumManager {
    /// 各频段配置
    bands: HashMap<FrequencyBand, BandConfig>,
}

impl SpectrumManager {
    /// 创建新的频谱管理器
    pub fn new() -> Self {
        let mut bands = HashMap::new();

        // 初始化各频段
        bands.insert(FrequencyBand::Low, BandConfig::new(1000, 5.0, 1));
        bands.insert(FrequencyBand::MediumLow, BandConfig::new(500, 2.0, 2));
        bands.insert(FrequencyBand::Medium, BandConfig::new(200, 1.0, 3));
        bands.insert(FrequencyBand::MediumHigh, BandConfig::new(100, 0.5, 0)); // 最高优先级
        bands.insert(FrequencyBand::High, BandConfig::new(300, 1.5, 2));
        bands.insert(FrequencyBand::UltraHigh, BandConfig::new(150, 3.0, 2));

        Self { bands }
    }

    /// 获取信道对应的频段
    pub fn get_band_for_channel(&self, channel: &ChannelType) -> FrequencyBand {
        match channel {
            ChannelType::World => FrequencyBand::Low,
            ChannelType::Group { .. } => FrequencyBand::MediumLow,
            ChannelType::Personal { .. } => FrequencyBand::Medium,
        }
    }

    /// 分配带宽
    pub fn allocate(&self, band: FrequencyBand, amount: u32) -> Option<(Self, f64)> {
        let band_config = self.bands.get(&band)?;
        let new_config = band_config.with_allocated(amount)?;

        let mut new_manager = self.clone();
        new_manager.bands.insert(band, new_config);

        // 返回成本
        let cost = band_config.cost_factor * amount as f64;

        Some((new_manager, cost))
    }

    /// 释放带宽
    pub fn release(&self, band: FrequencyBand, amount: u32) -> Self {
        let mut new_manager = self.clone();

        if let Some(band_config) = self.bands.get(&band) {
            new_manager.bands.insert(band, band_config.with_released(amount));
        }

        new_manager
    }

    /// 获取频段使用率
    pub fn get_band_usage(&self, band: FrequencyBand) -> f64 {
        self.bands.get(&band).map(|b| b.usage_ratio()).unwrap_or(0.0)
    }

    /// 获取频段成本系数
    pub fn get_band_cost_factor(&self, band: FrequencyBand) -> f64 {
        self.bands.get(&band).map(|b| b.cost_factor).unwrap_or(1.0)
    }

    /// 动态调整带宽（根据负载自动扩展/收缩）
    pub fn with_dynamic_adjustment(&self) -> Self {
        let mut new_manager = self.clone();

        for (band_type, band_config) in &self.bands {
            let usage_ratio = band_config.usage_ratio();

            // 高使用率：扩展容量
            let new_capacity = if usage_ratio > 0.8 {
                (band_config.capacity as f64 * 1.2) as u32
            } else if usage_ratio < 0.3 {
                // 低使用率：收缩容量
                (band_config.capacity as f64 * 0.9) as u32
            } else {
                band_config.capacity
            };

            new_manager.bands.insert(
                *band_type,
                BandConfig {
                    capacity: new_capacity,
                    ..band_config.clone()
                },
            );
        }

        new_manager
    }

    /// 获取总体频谱统计
    pub fn stats(&self) -> SpectrumStats {
        let total_capacity: u32 = self.bands.values().map(|b| b.capacity).sum();
        let total_usage: u32 = self.bands.values().map(|b| b.usage).sum();

        SpectrumStats {
            total_capacity,
            total_usage,
            overall_usage_ratio: if total_capacity > 0 {
                total_usage as f64 / total_capacity as f64
            } else {
                0.0
            },
        }
    }
}

/// 频谱统计
#[derive(Debug, Clone)]
pub struct SpectrumStats {
    pub total_capacity: u32,
    pub total_usage: u32,
    pub overall_usage_ratio: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectrum_manager_creation() {
        let manager = SpectrumManager::new();
        assert!(manager.bands.contains_key(&FrequencyBand::Low));
    }

    #[test]
    fn test_band_allocation() {
        let manager = SpectrumManager::new();
        let (new_manager, cost) = manager.allocate(FrequencyBand::Low, 10).unwrap();

        assert!(cost > 0.0);
        assert_ne!(new_manager.get_band_usage(FrequencyBand::Low), 0.0);
    }

    #[test]
    fn test_band_release() {
        let manager = SpectrumManager::new();
        let (allocated, _) = manager.allocate(FrequencyBand::Low, 10).unwrap();
        let released = allocated.release(FrequencyBand::Low, 5);

        assert!(released.get_band_usage(FrequencyBand::Low) < allocated.get_band_usage(FrequencyBand::Low));
    }

    #[test]
    fn test_dynamic_adjustment() {
        let mut manager = SpectrumManager::new();

        // 模拟高负载
        if let Some(band) = manager.bands.get_mut(&FrequencyBand::Low) {
            band.usage = (band.capacity as f64 * 0.9) as u32;
        }

        let adjusted = manager.with_dynamic_adjustment();
        let new_capacity = adjusted.bands.get(&FrequencyBand::Low).unwrap().capacity;

        // 高负载应该扩展容量
        assert!(new_capacity > 1000);
    }
}
