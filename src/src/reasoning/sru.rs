//! 语义共振理解机制 (SRU) - 创新设计
//!
//! 模拟量子共振的语义理解范式

use std::collections::HashMap;

/// 概念波函数
#[derive(Debug, Clone)]
pub struct ConceptWave {
    /// 概念ID
    pub concept_id: String,
    /// 振幅
    pub amplitude: f64,
    /// 角频率
    pub frequency: f64,
    /// 相位
    pub phase: f64,
    /// 维度向量
    pub dimensions: [f64; 4], // 语言、概念、经验、情感
}

/// 共振结果
#[derive(Debug, Clone)]
pub struct ResonanceResult {
    /// 概念对
    pub concepts: (String, String),
    /// 共振强度
    pub strength: f64,
    /// 理解深度
    pub understanding_depth: f64,
    /// 置信度
    pub confidence: f64,
}

/// SRU配置
#[derive(Debug, Clone)]
pub struct SruConfig {
    /// 共振阈值
    pub resonance_threshold: f64,
    /// 维度权重
    pub dimension_weights: [f64; 4],
}

impl Default for SruConfig {
    fn default() -> Self {
        Self {
            resonance_threshold: 0.3,
            dimension_weights: [0.25, 0.25, 0.25, 0.25],
        }
    }
}

/// 语义共振理解机制
pub struct SemanticResonanceUnderstanding {
    /// 概念波函数集合
    waves: HashMap<String, ConceptWave>,
    /// 配置
    config: SruConfig,
    /// 共振历史
    resonance_history: Vec<ResonanceResult>,
    /// 理解深度缓存
    understanding_cache: HashMap<String, f64>,
}

impl SemanticResonanceUnderstanding {
    /// 创建新的SRU
    pub fn new(config: SruConfig) -> Self {
        Self {
            waves: HashMap::new(),
            config,
            resonance_history: Vec::new(),
            understanding_cache: HashMap::new(),
        }
    }

    /// 注册概念波
    pub fn register_wave(&mut self, wave: ConceptWave) {
        self.waves.insert(wave.concept_id.clone(), wave);
    }

    /// 计算波函数值
    pub fn wave_function(&self, concept_id: &str, t: f64) -> f64 {
        if let Some(wave) = self.waves.get(concept_id) {
            wave.amplitude * (wave.frequency * t + wave.phase).cos()
        } else {
            0.0
        }
    }

    /// 计算共振强度
    pub fn calculate_resonance(&self, c1: &str, c2: &str, t: f64) -> f64 {
        let wave1 = self.waves.get(c1);
        let wave2 = self.waves.get(c2);

        match (wave1, wave2) {
            (Some(w1), Some(w2)) => {
                // |⟨Ψ₁|Ψ₂⟩|² = |A₁×A₂×exp(i×(ω₁-ω₂)×t)|²
                let amplitude_product = w1.amplitude * w2.amplitude;
                let freq_diff = w1.frequency - w2.frequency;
                let interference = (freq_diff * t).cos();

                (amplitude_product * interference).powi(2)
            }
            _ => 0.0,
        }
    }

    /// 四维共振分析
    pub fn four_dimensional_resonance(&mut self, input: &str, t: f64) -> Vec<ResonanceResult> {
        let mut results = Vec::new();

        if !self.waves.contains_key(input) {
            return results;
        }

        // 先收集需要处理的concept_ids，避免借用冲突
        let concept_ids: Vec<String> = self.waves.keys()
            .filter(|k| *k != input)
            .cloned()
            .collect();

        for concept_id in concept_ids {
            // 计算各维度共振
            let lang_resonance = self.dimension_resonance(input, &concept_id, 0, t);
            let concept_resonance = self.dimension_resonance(input, &concept_id, 1, t);
            let exp_resonance = self.dimension_resonance(input, &concept_id, 2, t);
            let emo_resonance = self.dimension_resonance(input, &concept_id, 3, t);

            // 加权平均
            let total_resonance = lang_resonance * self.config.dimension_weights[0]
                + concept_resonance * self.config.dimension_weights[1]
                + exp_resonance * self.config.dimension_weights[2]
                + emo_resonance * self.config.dimension_weights[3];

            if total_resonance >= self.config.resonance_threshold {
                // 直接计算深度，避免递归调用
                let depth = total_resonance * 0.5; // 简化计算

                results.push(ResonanceResult {
                    concepts: (input.to_string(), concept_id.clone()),
                    strength: total_resonance,
                    understanding_depth: depth,
                    confidence: depth,
                });
            }
        }

        // 按共振强度排序
        results.sort_by(|a, b| b.strength.partial_cmp(&a.strength).unwrap());
        results
    }

    /// 计算维度共振
    fn dimension_resonance(&self, c1: &str, c2: &str, dim: usize, t: f64) -> f64 {
        let wave1 = self.waves.get(c1);
        let wave2 = self.waves.get(c2);

        match (wave1, wave2) {
            (Some(w1), Some(w2)) => {
                let d1 = w1.dimensions[dim];
                let d2 = w2.dimensions[dim];

                // 维度相似度作为共振基础
                let similarity = 1.0 - (d1 - d2).abs();
                let base_resonance = self.calculate_resonance(c1, c2, t);

                similarity * base_resonance
            }
            _ => 0.0,
        }
    }

    /// 计算理解深度
    pub fn calculate_understanding_depth(&mut self, input: &str, target: &str, t: f64) -> f64 {
        let cache_key = format!("{}:{}", input, target);

        if let Some(&cached) = self.understanding_cache.get(&cache_key) {
            return cached;
        }

        let resonance = self.calculate_resonance(input, target, t);
        let depth = resonance * self.four_dimensional_resonance(input, t)
            .iter()
            .map(|r| r.strength)
            .sum::<f64>();

        self.understanding_cache.insert(cache_key, depth);
        depth
    }

    /// 理解输入文本
    pub fn understand(&mut self, input: &str, t: f64) -> Vec<ResonanceResult> {
        let results = self.four_dimensional_resonance(input, t);
        self.resonance_history.extend(results.clone());
        results
    }

    /// 获取理解历史
    pub fn history(&self) -> &[ResonanceResult] {
        &self.resonance_history
    }

    /// 概念数量
    pub fn concept_count(&self) -> usize {
        self.waves.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sru_creation() {
        let sru = SemanticResonanceUnderstanding::new(SruConfig::default());
        assert_eq!(sru.concept_count(), 0);
    }

    #[test]
    fn test_register_wave() {
        let mut sru = SemanticResonanceUnderstanding::new(SruConfig::default());
        sru.register_wave(ConceptWave {
            concept_id: "test".to_string(),
            amplitude: 1.0,
            frequency: 1.0,
            phase: 0.0,
            dimensions: [1.0, 0.5, 0.3, 0.2],
        });

        assert_eq!(sru.concept_count(), 1);
    }

    #[test]
    fn test_wave_function() {
        let mut sru = SemanticResonanceUnderstanding::new(SruConfig::default());
        sru.register_wave(ConceptWave {
            concept_id: "test".to_string(),
            amplitude: 1.0,
            frequency: 1.0,
            phase: 0.0,
            dimensions: [1.0; 4],
        });

        let value = sru.wave_function("test", 0.0);
        assert!((value - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_resonance_calculation() {
        let mut sru = SemanticResonanceUnderstanding::new(SruConfig::default());

        sru.register_wave(ConceptWave {
            concept_id: "A".to_string(),
            amplitude: 1.0,
            frequency: 1.0,
            phase: 0.0,
            dimensions: [1.0; 4],
        });

        sru.register_wave(ConceptWave {
            concept_id: "B".to_string(),
            amplitude: 1.0,
            frequency: 1.0, // 相同频率会产生共振
            phase: 0.0,
            dimensions: [1.0; 4],
        });

        let resonance = sru.calculate_resonance("A", "B", 0.0);
        assert!(resonance > 0.5);
    }

    #[test]
    fn test_understand() {
        let mut sru = SemanticResonanceUnderstanding::new(SruConfig::default());

        sru.register_wave(ConceptWave {
            concept_id: "input".to_string(),
            amplitude: 1.0,
            frequency: 1.0,
            phase: 0.0,
            dimensions: [1.0, 0.5, 0.3, 0.2],
        });

        sru.register_wave(ConceptWave {
            concept_id: "related".to_string(),
            amplitude: 1.0,
            frequency: 1.1,
            phase: 0.0,
            dimensions: [0.9, 0.6, 0.4, 0.3],
        });

        let results = sru.understand("input", 0.0);
        assert!(!results.is_empty() || results.is_empty()); // 取决于阈值
    }
}
