//! 向量学习规则
//!
//! 局部学习规则（非梯度下降），用于概念向量的更新

use super::types::ConceptVector;

/// 向量学习规则
pub struct VectorLearningRules;

impl VectorLearningRules {
    /// 关联更新规则
    ///
    /// 当概念A和B在上下文中同时出现时，它们的向量应该相互靠近
    ///
    /// # 公式
    /// ΔV(A) = η · (V(B) - V(A)) · strength
    /// ΔV(B) = η · (V(A) - V(B)) · strength
    ///
    /// # Arguments
    /// * `v_a` - 概念A的向量
    /// * `v_b` - 概念B的向量
    /// * `learning_rate` - 学习率 η
    /// * `strength` - 关联强度
    ///
    /// # Returns
    /// (更新后的V(A), 更新后的V(B))
    pub fn association(
        v_a: &ConceptVector,
        v_b: &ConceptVector,
        learning_rate: f64,
        strength: f64,
    ) -> (ConceptVector, ConceptVector) {
        // 计算差异向量
        let diff: Vec<f64> = v_b.data.iter()
            .zip(v_a.data.iter())
            .map(|(b, a)| b - a)
            .collect();

        // 更新A：向B靠近
        let mut new_a_data: Vec<f64> = v_a.data.iter()
            .zip(diff.iter())
            .map(|(a, d)| a + learning_rate * d * strength)
            .collect();
        ConceptVector::normalize_in_place(&mut new_a_data);

        // 更新B：向A靠近
        let mut new_b_data: Vec<f64> = v_b.data.iter()
            .zip(diff.iter())
            .map(|(b, d)| b - learning_rate * d * strength)
            .collect();
        ConceptVector::normalize_in_place(&mut new_b_data);

        (
            ConceptVector::from_data_unnormalized(new_a_data),
            ConceptVector::from_data_unnormalized(new_b_data),
        )
    }

    /// 区分更新规则
    ///
    /// 当概念A和B需要被区分时，它们的向量应该相互远离
    ///
    /// # 公式
    /// ΔV(A) = η · (V(A) - V(B)) · strength
    /// ΔV(B) = η · (V(B) - V(A)) · strength
    ///
    /// # Arguments
    /// * `v_a` - 概念A的向量
    /// * `v_b` - 概念B的向量
    /// * `learning_rate` - 学习率 η
    /// * `strength` - 区分强度
    ///
    /// # Returns
    /// (更新后的V(A), 更新后的V(B))
    pub fn differentiation(
        v_a: &ConceptVector,
        v_b: &ConceptVector,
        learning_rate: f64,
        strength: f64,
    ) -> (ConceptVector, ConceptVector) {
        // 计算差异向量（A - B）
        let diff: Vec<f64> = v_a.data.iter()
            .zip(v_b.data.iter())
            .map(|(a, b)| a - b)
            .collect();

        // 更新A：远离B
        let mut new_a_data: Vec<f64> = v_a.data.iter()
            .zip(diff.iter())
            .map(|(a, d)| a + learning_rate * d * strength)
            .collect();
        ConceptVector::normalize_in_place(&mut new_a_data);

        // 更新B：远离A
        let mut new_b_data: Vec<f64> = v_b.data.iter()
            .zip(diff.iter())
            .map(|(b, d)| b - learning_rate * d * strength)
            .collect();
        ConceptVector::normalize_in_place(&mut new_b_data);

        (
            ConceptVector::from_data_unnormalized(new_a_data),
            ConceptVector::from_data_unnormalized(new_b_data),
        )
    }

    /// 继承更新规则
    ///
    /// 新概念创建时，从父概念继承向量
    ///
    /// # Arguments
    /// * `parent` - 父概念向量
    /// * `noise_scale` - 扰动强度
    pub fn inheritance(parent: &ConceptVector, noise_scale: f64) -> ConceptVector {
        ConceptVector::inherit_with_noise(parent, noise_scale)
    }

    /// 上下文推导更新
    ///
    /// 从上下文中提取语义，更新概念向量
    ///
    /// # 公式
    /// V'(概念) = V(概念) + η × Σ(w_i × V(上下文词_i))
    ///
    /// # Arguments
    /// * `concept_vector` - 当前概念向量
    /// * `context_vectors` - 上下文词向量及权重
    /// * `learning_rate` - 学习率
    pub fn context_inference(
        concept_vector: &ConceptVector,
        context_vectors: &[(ConceptVector, f64)],
        learning_rate: f64,
    ) -> ConceptVector {
        // 使用概念向量的维度（配置驱动）
        let dim = concept_vector.data.len();
        let mut weighted_sum = vec![0.0; dim];
        let mut total_weight = 0.0;

        for (vec, weight) in context_vectors {
            for (i, &v) in vec.data.iter().enumerate() {
                if i < dim {
                    weighted_sum[i] += v * weight;
                }
            }
            total_weight += weight;
        }

        if total_weight > 1e-10 {
            for v in &mut weighted_sum {
                *v /= total_weight;
            }
        }

        // 更新概念向量
        let mut new_data: Vec<f64> = concept_vector.data.iter()
            .zip(weighted_sum.iter())
            .map(|(c, ctx)| c + learning_rate * ctx)
            .collect();
        ConceptVector::normalize_in_place(&mut new_data);

        ConceptVector::from_data_unnormalized(new_data)
    }
}

/// ConceptVector的扩展方法
impl ConceptVector {
    /// 带指定噪声的继承
    pub fn inherit_with_noise(parent: &ConceptVector, noise_scale: f64) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut data: Vec<f64> = parent.data.iter()
            .map(|&v| v + rng.gen_range(-noise_scale..noise_scale))
            .collect();
        Self::normalize_in_place(&mut data);
        Self::from_data_unnormalized(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_association_increases_similarity() {
        let mut v_a = ConceptVector::random_small();
        let mut v_b = ConceptVector::random_small();

        let initial_sim = v_a.cosine_similarity(&v_b);

        // 执行关联更新
        for _ in 0..100 {
            let (new_a, new_b) = VectorLearningRules::association(&v_a, &v_b, 0.01, 1.0);
            v_a = new_a;
            v_b = new_b;
        }

        let final_sim = v_a.cosine_similarity(&v_b);
        assert!(final_sim > initial_sim, "关联更新应该增加相似度: {} -> {}", initial_sim, final_sim);
    }

    #[test]
    fn test_differentiation_decreases_similarity() {
        let mut v_a = ConceptVector::random_small();
        let mut v_b = ConceptVector::random_small();

        // 先让它们相似
        for _ in 0..50 {
            let (new_a, new_b) = VectorLearningRules::association(&v_a, &v_b, 0.01, 1.0);
            v_a = new_a;
            v_b = new_b;
        }

        let initial_sim = v_a.cosine_similarity(&v_b);

        // 执行区分更新
        for _ in 0..100 {
            let (new_a, new_b) = VectorLearningRules::differentiation(&v_a, &v_b, 0.01, 1.0);
            v_a = new_a;
            v_b = new_b;
        }

        let final_sim = v_a.cosine_similarity(&v_b);
        assert!(final_sim < initial_sim, "区分更新应该降低相似度: {} -> {}", initial_sim, final_sim);
    }

    #[test]
    fn test_vectors_remain_normalized() {
        let v_a = ConceptVector::random_small();
        let v_b = ConceptVector::random_small();

        let (new_a, new_b) = VectorLearningRules::association(&v_a, &v_b, 0.1, 1.0);

        assert!((new_a.norm() - 1.0).abs() < 1e-6, "更新后A应保持归一化");
        assert!((new_b.norm() - 1.0).abs() < 1e-6, "更新后B应保持归一化");
    }

    #[test]
    fn test_inheritance_similarity() {
        let parent = ConceptVector::random_small();
        let child = VectorLearningRules::inheritance(&parent, 0.1);

        let sim = parent.cosine_similarity(&child);
        // 考虑随机扰动，相似度应大于0.6
        assert!(sim > 0.6, "子概念应该与父概念相似: {}", sim);
    }
}
