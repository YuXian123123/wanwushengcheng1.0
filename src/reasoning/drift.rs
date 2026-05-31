//! 向量漂移检测模块 - 安全设计
//!
//! 检测与纠正概念向量的漂移问题

use std::collections::HashMap;

/// 漂移检测配置
#[derive(Debug, Clone)]
pub struct DriftConfig {
    /// 漂移阈值（余弦相似度）
    pub drift_threshold: f64,
    /// 校准周期（秒）
    pub calibration_interval: u64,
    /// 启用弹性约束
    pub elastic_constraint: bool,
}

impl Default for DriftConfig {
    fn default() -> Self {
        Self {
            drift_threshold: 0.1,
            calibration_interval: 3600,
            elastic_constraint: true,
        }
    }
}

/// 漂移检测结果
#[derive(Debug, Clone)]
pub struct DriftResult {
    /// 概念ID
    pub concept_id: String,
    /// 漂移程度
    pub drift_amount: f64,
    /// 是否需要纠正
    pub needs_correction: bool,
    /// 纠正向量
    pub correction_vector: Option<Vec<f64>>,
}

/// 向量漂移检测器
pub struct VectorDriftDetector {
    /// 配置
    config: DriftConfig,
    /// 锚点向量（标准向量）
    anchors: HashMap<String, Vec<f64>>,
    /// 当前向量
    current: HashMap<String, Vec<f64>>,
    /// 弹性约束系数
    elastic_coefficients: HashMap<String, f64>,
    /// 漂移历史
    drift_history: Vec<DriftResult>,
}

impl VectorDriftDetector {
    /// 创建新的漂移检测器
    pub fn new(config: DriftConfig) -> Self {
        Self {
            config,
            anchors: HashMap::new(),
            current: HashMap::new(),
            elastic_coefficients: HashMap::new(),
            drift_history: Vec::new(),
        }
    }

    /// 注册锚点向量
    pub fn register_anchor(&mut self, concept_id: String, vector: Vec<f64>) {
        self.anchors.insert(concept_id.clone(), vector.clone());
        self.current.insert(concept_id.clone(), vector);
        self.elastic_coefficients.insert(concept_id, 1.0);
    }

    /// 更新向量
    pub fn update_vector(&mut self, concept_id: &str, new_vector: Vec<f64>) -> DriftResult {
        let current = self.current.get(concept_id).cloned();
        let anchor = self.anchors.get(concept_id).cloned();

        let result = if let (Some(current_vec), Some(anchor_vec)) = (current, anchor) {
            // 计算漂移
            let drift = 1.0 - cosine_similarity(&current_vec, &new_vector);
            let anchor_drift = 1.0 - cosine_similarity(&anchor_vec, &new_vector);

            // 计算纠正向量
            let correction = if anchor_drift > self.config.drift_threshold {
                // 应用锚点纠正
                let elastic = self.elastic_coefficients.get(concept_id).copied().unwrap_or(1.0);
                let mut corrected = new_vector.clone();
                for (i, v) in corrected.iter_mut().enumerate() {
                    *v = *v * (1.0 - elastic) + anchor_vec[i] * elastic;
                }
                Some(corrected)
            } else {
                None
            };

            DriftResult {
                concept_id: concept_id.to_string(),
                drift_amount: drift,
                needs_correction: anchor_drift > self.config.drift_threshold,
                correction_vector: correction,
            }
        } else {
            // 没有锚点，直接接受
            DriftResult {
                concept_id: concept_id.to_string(),
                drift_amount: 0.0,
                needs_correction: false,
                correction_vector: None,
            }
        };

        // 更新当前向量
        if let Some(ref correction) = result.correction_vector {
            self.current.insert(concept_id.to_string(), correction.clone());
        } else {
            self.current.insert(concept_id.to_string(), new_vector);
        }

        // 记录历史
        self.drift_history.push(result.clone());

        result
    }

    /// 检测所有向量的漂移
    pub fn detect_all(&self) -> Vec<DriftResult> {
        self.current
            .iter()
            .filter_map(|(id, current_vec)| {
                let anchor = self.anchors.get(id)?;
                let drift = 1.0 - cosine_similarity(anchor, current_vec);

                Some(DriftResult {
                    concept_id: id.clone(),
                    drift_amount: drift,
                    needs_correction: drift > self.config.drift_threshold,
                    correction_vector: None,
                })
            })
            .collect()
    }

    /// 应用弹性约束
    pub fn apply_elastic_constraint(&mut self, concept_id: &str) {
        if self.config.elastic_constraint {
            let coefficient = self.elastic_coefficients.entry(concept_id.to_string()).or_insert(1.0);
            *coefficient = (*coefficient * 0.95).max(0.1);
        }
    }

    /// 周期性校准
    pub fn calibrate(&mut self) {
        for (id, current_vec) in self.current.iter_mut() {
            if let Some(anchor) = self.anchors.get(id) {
                let drift = 1.0 - cosine_similarity(anchor, current_vec);
                if drift > self.config.drift_threshold {
                    // 部分校准
                    for (i, v) in current_vec.iter_mut().enumerate() {
                        *v = *v * 0.7 + anchor[i] * 0.3;
                    }
                }
            }
        }
    }

    /// 获取漂移历史
    pub fn history(&self) -> &[DriftResult] {
        &self.drift_history
    }
}

/// 计算余弦相似度
fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let mag_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();

    if mag_a == 0.0 || mag_b == 0.0 {
        return 0.0;
    }

    dot / (mag_a * mag_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector_creation() {
        let detector = VectorDriftDetector::new(DriftConfig::default());
        assert!(detector.anchors.is_empty());
    }

    #[test]
    fn test_anchor_registration() {
        let mut detector = VectorDriftDetector::new(DriftConfig::default());
        detector.register_anchor("test".to_string(), vec![1.0, 0.0, 0.0]);

        assert!(detector.anchors.contains_key("test"));
    }

    #[test]
    fn test_no_drift() {
        let mut detector = VectorDriftDetector::new(DriftConfig::default());
        detector.register_anchor("test".to_string(), vec![1.0, 0.0, 0.0]);

        let result = detector.update_vector("test", vec![0.99, 0.1, 0.0]);
        assert!(!result.needs_correction);
    }

    #[test]
    fn test_significant_drift() {
        let config = DriftConfig {
            drift_threshold: 0.1,
            ..Default::default()
        };
        let mut detector = VectorDriftDetector::new(config);
        detector.register_anchor("test".to_string(), vec![1.0, 0.0, 0.0]);

        let result = detector.update_vector("test", vec![0.0, 1.0, 0.0]);
        assert!(result.needs_correction);
        assert!(result.correction_vector.is_some());
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);

        let c = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&a, &c) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_calibrate() {
        let mut detector = VectorDriftDetector::new(DriftConfig::default());
        detector.register_anchor("test".to_string(), vec![1.0, 0.0, 0.0]);
        detector.update_vector("test", vec![0.5, 0.5, 0.0]);

        detector.calibrate();

        // 校准后向量应该更接近锚点
        let current = detector.current.get("test").unwrap();
        let similarity = cosine_similarity(&[1.0, 0.0, 0.0], current);
        assert!(similarity > 0.7);
    }
}
