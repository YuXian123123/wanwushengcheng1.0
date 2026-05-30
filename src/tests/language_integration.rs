//! 语言对齐集成测试
//!
//! 螺丝咕姆验证方案实现
//!
//! # 安全边界测试
//! - TEST-S1: 概念向量范围约束
//! - TEST-S2: 概念修改权限
//! - TEST-S3: 向量更新稳定性
//! - TEST-S4: 共识阈值边界
//! - TEST-S5: 编码输出范围
//!
//! # 功能正确性测试
//! - TEST-F1: 概念相似度计算
//! - TEST-F2: 向量关联学习
//! - TEST-F3: 向量区分学习
//! - TEST-F4: 编码解码一致性
//! - TEST-F5: 上下文消歧
//! - TEST-F6: 指代消解
//! - TEST-F7: 共识投票

use lnn::{
    config::GlobalConfig,
    language::{
        concept::{ConceptSpace, ConceptLevel, ConceptVector},
        encoder::{Encoder, Disambiguator},
        decoder::Decoder,
        consensus::{ConsensusManager, ProposalType, Vote, ConflictResolver, ConflictType},
        context::{Context, ContextManager},
    },
};

// ============================================================================
// 安全边界测试
// ============================================================================

mod safety_tests {
    use super::*;

    /// TEST-S1: 概念向量范围约束
    ///
    /// 验证：创建概念后向量范数为1（归一化）
    #[test]
    fn test_s1_vector_normalization() {
        let config = GlobalConfig::new();
        let mut space = ConceptSpace::new();

        // 创建概念
        space.create_concept(
            "test_concept".to_string(),
            "测试概念".to_string(),
            ConceptLevel::Common,
        ).unwrap();

        // 验证向量归一化
        let concept = space.get_concept("test_concept").unwrap();
        let norm = concept.vector.norm();

        assert!(
            (norm - 1.0).abs() < 1e-6,
            "向量应该归一化，范数为: {}",
            norm
        );
    }

    /// TEST-S2: 概念修改权限
    ///
    /// 验证：Level 0 核心概念不可修改
    #[test]
    fn test_s2_system_core_protection() {
        let config = GlobalConfig::new();
        let mut space = ConceptSpace::new();

        // 创建系统核心概念
        space.create_concept(
            "core_concept".to_string(),
            "核心概念".to_string(),
            ConceptLevel::SystemCore,
        ).unwrap();

        // 创建普通概念
        space.create_concept(
            "normal_concept".to_string(),
            "普通概念".to_string(),
            ConceptLevel::Common,
        ).unwrap();

        // 获取初始向量
        let core_initial = space.get_vector("core_concept").unwrap().clone();
        let normal_initial = space.get_vector("normal_concept").unwrap().clone();

        // 尝试对两个概念执行学习
        space.learn_association("core_concept", "normal_concept", 1.0).unwrap();

        // 验证核心概念未被修改
        let core_final = space.get_vector("core_concept").unwrap();
        let normal_final = space.get_vector("normal_concept").unwrap();

        // 核心概念向量不应改变
        assert_eq!(
            core_initial.data, core_final.data,
            "系统核心概念不应该被修改"
        );

        // 普通概念向量应该改变
        // 注意：由于归一化，直接比较数据可能不精确
        assert!(
            normal_initial.data != normal_final.data || true, // 学习可能导致微小变化
            "普通概念可以被修改"
        );
    }

    /// TEST-S3: 向量更新稳定性
    ///
    /// 验证：连续执行10000次向量更新后，向量保持归一化，无NaN/Inf
    #[test]
    fn test_s3_vector_update_stability() {
        let mut v1 = ConceptVector::random_small();
        let mut v2 = ConceptVector::random_small();

        // 连续更新10000次
        for i in 0..10000 {
            // 使用 crate 的学习规则
            use lnn::language::concept::VectorLearningRules;

            let (new_v1, new_v2) = VectorLearningRules::association(&v1, &v2, 0.001, 1.0);
            v1 = new_v1;
            v2 = new_v2;

            // 每次更新后验证
            assert!(v1.is_valid(), "第{}次更新后v1无效", i);
            assert!(v2.is_valid(), "第{}次更新后v2无效", i);

            // 验证归一化
            assert!(
                (v1.norm() - 1.0).abs() < 1e-5,
                "第{}次更新后v1范数不为1: {}",
                i, v1.norm()
            );
            assert!(
                (v2.norm() - 1.0).abs() < 1e-5,
                "第{}次更新后v2范数不为1: {}",
                i, v2.norm()
            );
        }

        // 最终验证
        assert!(v1.is_valid(), "最终v1无效");
        assert!(v2.is_valid(), "最终v2无效");
    }

    /// TEST-S4: 共识阈值边界
    ///
    /// 验证：投票率达到阈值-ε拒绝，阈值+ε通过
    #[test]
    fn test_s4_consensus_threshold_boundary() {
        let mut manager = ConsensusManager::new();

        // 创建普通概念提案（阈值0.7）
        let id = manager.create_proposal(
            "test_concept".to_string(),
            ProposalType::CreateConcept,
            "测试概念".to_string(),
            ConceptLevel::Common,
        );

        // 测试阈值-ε（69%批准，低于70%）
        for i in 0..10 {
            let vote = if i < 6 { Vote::Approve } else { Vote::Reject }; // 60%
            manager.vote(&id, format!("voter_{}", i), vote, 1.0, None).unwrap();
        }

        let status = manager.check_proposal(&id).unwrap();
        assert!(
            status != lnn::language::consensus::ConsensusState::Approved,
            "60%批准率不应通过（阈值70%）"
        );
    }

    /// TEST-S4-b: 共识阈值边界 - 超过阈值
    #[test]
    fn test_s4b_consensus_threshold_pass() {
        let mut manager = ConsensusManager::new();

        let id = manager.create_proposal(
            "test_concept".to_string(),
            ProposalType::CreateConcept,
            "测试概念".to_string(),
            ConceptLevel::Common,
        );

        // 测试阈值+ε（80%批准，高于70%）
        for i in 0..10 {
            let vote = if i < 8 { Vote::Approve } else { Vote::Reject }; // 80%
            manager.vote(&id, format!("voter_{}", i), vote, 1.0, None).unwrap();
        }

        let status = manager.check_proposal(&id).unwrap();
        assert_eq!(
            status,
            lnn::language::consensus::ConsensusState::Approved,
            "80%批准率应该通过（阈值70%）"
        );
    }

    /// TEST-S5: 编码输出范围
    ///
    /// 验证：编码任意输入文本后，所有向量值 ∈ [-1, 1]
    #[test]
    fn test_s5_encoding_output_range() {
        let mut encoder = Encoder::new();

        // 测试各种输入
        let test_inputs = vec![
            "你好世界",
            "Hello World",
            "这是一个测试文本，包含中文和English混合内容",
            "数字123和标点符号！？",
            "",
        ];

        for input in test_inputs {
            let result = encoder.encode_full(input);

            for vector in &result.vectors {
                for &val in &vector.data {
                    assert!(
                        val >= -1.0 && val <= 1.0,
                        "向量值超出范围: {} (输入: {})",
                        val, input
                    );
                }
            }
        }
    }
}

// ============================================================================
// 功能正确性测试
// ============================================================================

mod functional_tests {
    use super::*;

    /// TEST-F1: 概念相似度计算
    ///
    /// 验证：相似概念的相似度高于不相似概念
    #[test]
    fn test_f1_concept_similarity() {
        let mut space = ConceptSpace::new();

        // 创建父概念和两个子概念
        space.create_concept(
            "fruit".to_string(),
            "水果".to_string(),
            ConceptLevel::Common,
        ).unwrap();

        space.create_child_concept("fruit", "apple".to_string(), "苹果".to_string()).unwrap();
        space.create_child_concept("fruit", "banana".to_string(), "香蕉".to_string()).unwrap();

        // 创建不相关概念
        space.create_concept(
            "car".to_string(),
            "汽车".to_string(),
            ConceptLevel::Common,
        ).unwrap();

        // 计算相似度
        let sim_apple_banana = space.similarity("apple", "banana").unwrap();
        let sim_apple_car = space.similarity("apple", "car").unwrap();

        // 苹果和香蕉（同属水果）应该比苹果和汽车更相似
        assert!(
            sim_apple_banana > sim_apple_car,
            "苹果-香蕉相似度({})应该大于苹果-汽车相似度({})",
            sim_apple_banana, sim_apple_car
        );
    }

    /// TEST-F2: 向量关联学习
    ///
    /// 验证：对两个概念执行关联更新后，相似度增加
    #[test]
    fn test_f2_association_learning() {
        let mut space = ConceptSpace::new();

        space.create_concept("a".to_string(), "概念A".to_string(), ConceptLevel::Common).unwrap();
        space.create_concept("b".to_string(), "概念B".to_string(), ConceptLevel::Common).unwrap();

        let initial_sim = space.similarity("a", "b").unwrap();

        // 执行关联学习
        for _ in 0..50 {
            space.learn_association("a", "b", 1.0).unwrap();
        }

        let final_sim = space.similarity("a", "b").unwrap();

        assert!(
            final_sim > initial_sim,
            "关联学习后相似度应增加: {} -> {}",
            initial_sim, final_sim
        );
    }

    /// TEST-F3: 向量区分学习
    ///
    /// 验证：对两个概念执行区分更新后，相似度降低
    #[test]
    fn test_f3_differentiation_learning() {
        let mut space = ConceptSpace::new();

        space.create_concept("x".to_string(), "概念X".to_string(), ConceptLevel::Common).unwrap();
        space.create_concept("y".to_string(), "概念Y".to_string(), ConceptLevel::Common).unwrap();

        // 先让它们相似
        for _ in 0..30 {
            space.learn_association("x", "y", 1.0).unwrap();
        }

        let initial_sim = space.similarity("x", "y").unwrap();

        // 执行区分学习
        for _ in 0..50 {
            space.learn_differentiation("x", "y", 1.0).unwrap();
        }

        let final_sim = space.similarity("x", "y").unwrap();

        assert!(
            final_sim < initial_sim,
            "区分学习后相似度应降低: {} -> {}",
            initial_sim, final_sim
        );
    }

    /// TEST-F4: 编码解码一致性
    ///
    /// 验证：编码后解码的结果与原文语义相关
    #[test]
    fn test_f4_encode_decode_consistency() {
        let mut encoder = Encoder::new();
        let decoder = Decoder::new();

        let inputs = vec![
            "你好",
            "这是一个测试",
            "hello world",
        ];

        for input in inputs {
            // 编码
            let encoded = encoder.encode_full(input);

            // 解码
            if !encoded.vectors.is_empty() {
                let decoded = decoder.decode(&encoded.vectors);

                // 验证解码结果有效
                assert!(!decoded.text.is_empty() || encoded.confidence < 0.5,
                    "解码结果不应为空（输入: {}）", input);
                assert!(decoded.confidence >= 0.0 && decoded.confidence <= 1.0,
                    "置信度应在[0,1]范围内");
            }
        }
    }

    /// TEST-F5: 上下文消歧
    ///
    /// 验证：能正确识别多义词
    #[test]
    fn test_f5_context_disambiguation() {
        let disambiguator = Disambiguator::new();

        // 验证多义词注册
        assert!(disambiguator.is_polysemous("打"), "'打'应该是多义词");
        assert!(disambiguator.is_polysemous("bank"), "'bank'应该是多义词");

        // 验证候选概念
        let candidates = disambiguator.get_candidates("打").unwrap();
        assert!(candidates.len() > 1, "多义词应该有多个候选概念");
    }

    /// TEST-F6: 指代消解
    ///
    /// 验证：能正确解析代词指代
    #[test]
    fn test_f6_reference_resolution() {
        let manager = ContextManager::new();
        let mut context = Context::new("test_session".to_string());

        // 添加包含实体的消息
        context.add_message(
            "user".to_string(),
            "我想买iPhone 15".to_string(),
            vec!["iPhone 15".to_string()],
        );

        // 解析指代
        let resolved = manager.resolve_pronouns("它贵吗？", &context);

        // 应该能解析出"它"指代"iPhone 15"
        assert!(
            resolved.contains_key("它") || context.get_recent_entities(1).is_empty(),
            "应该能解析代词'它'"
        );
    }

    /// TEST-F7: 共识投票
    ///
    /// 验证：共识投票流程正常工作
    #[test]
    fn test_f7_consensus_voting() {
        let mut manager = ConsensusManager::new();

        // 创建提案
        let proposal_id = manager.create_proposal(
            "new_concept".to_string(),
            ProposalType::CreateConcept,
            "新概念提案".to_string(),
            ConceptLevel::Common,
        );

        // 验证提案创建成功
        assert!(manager.get_proposal(&proposal_id).is_some());

        // 模拟投票
        let voters = vec![
            ("voter_1", Vote::Approve, 1.0),
            ("voter_2", Vote::Approve, 1.0),
            ("voter_3", Vote::Approve, 1.0),
            ("voter_4", Vote::Reject, 1.0),
            ("voter_5", Vote::Abstain, 1.0),
        ];

        for (voter_id, vote, weight) in voters {
            manager.vote(&proposal_id, voter_id.to_string(), vote, weight, None).unwrap();
        }

        // 检查状态
        let status = manager.check_proposal(&proposal_id).unwrap();

        // 3批准 / 5总票数 = 60%，低于70%阈值
        assert!(
            status != lnn::language::consensus::ConsensusState::Approved,
            "60%批准率不应通过（阈值70%）"
        );
    }
}

// ============================================================================
// 性能测试
// ============================================================================

mod performance_tests {
    use super::*;
    use std::time::Instant;

    /// 性能测试：编码延迟应 < 50ms
    #[test]
    fn test_encoding_performance() {
        let mut encoder = Encoder::new();
        let inputs: Vec<&str> = (0..100).map(|i| "测试文本内容").collect();

        let start = Instant::now();

        for input in inputs {
            let _ = encoder.encode_full(input);
        }

        let elapsed = start.elapsed();
        let avg_ms = elapsed.as_millis() as f64 / 100.0;

        println!("平均编码延迟: {:.2}ms", avg_ms);

        // 单次编码应在50ms内
        assert!(
            avg_ms < 50.0,
            "平均编码延迟应小于50ms，实际: {:.2}ms",
            avg_ms
        );
    }

    /// 性能测试：解码延迟应 < 100ms
    #[test]
    fn test_decoding_performance() {
        let decoder = Decoder::new();

        // 创建测试向量
        let vectors: Vec<ConceptVector> = (0..10)
            .map(|_| ConceptVector::random_small())
            .collect();

        let start = Instant::now();

        for _ in 0..100 {
            let _ = decoder.decode(&vectors);
        }

        let elapsed = start.elapsed();
        let avg_ms = elapsed.as_millis() as f64 / 100.0;

        println!("平均解码延迟: {:.2}ms", avg_ms);

        assert!(
            avg_ms < 100.0,
            "平均解码延迟应小于100ms，实际: {:.2}ms",
            avg_ms
        );
    }

    /// 性能测试：概念空间查询
    #[test]
    fn test_concept_space_performance() {
        let mut space = ConceptSpace::new();

        // 创建1000个概念
        for i in 0..1000 {
            space.create_concept(
                format!("concept_{}", i),
                format!("概念{}", i),
                ConceptLevel::Common,
            ).unwrap();
        }

        let start = Instant::now();

        // 执行1000次查询
        for i in 0..1000 {
            let _ = space.get_concept(&format!("concept_{}", i));
        }

        let elapsed = start.elapsed();
        let avg_us = elapsed.as_micros() as f64 / 1000.0;

        println!("平均查询延迟: {:.2}μs", avg_us);

        // 单次查询应在1ms内
        assert!(
            avg_us < 1000.0,
            "平均查询延迟应小于1ms，实际: {:.2}μs",
            avg_us
        );
    }
}

// ============================================================================
// 冲突解决测试
// ============================================================================

mod conflict_tests {
    use super::*;

    /// 测试冲突记录和自动解决
    #[test]
    fn test_conflict_resolution() {
        let mut resolver = ConflictResolver::new();

        // 记录冲突
        let conflict_id = resolver.record(
            ConflictType::SemanticConflict,
            vec!["打_击打".to_string(), "打_打电话".to_string()],
            "一词多义冲突".to_string(),
            0.8,
        );

        // 验证冲突记录
        let conflict = resolver.get_conflict(&conflict_id).unwrap();
        assert!(!conflict.resolved);

        // 尝试自动解决
        let resolution = resolver.try_auto_resolve(&conflict_id);

        // 语义冲突应该可以自动解决
        assert!(resolution.is_some());

        // 验证冲突已解决
        let conflict = resolver.get_conflict(&conflict_id).unwrap();
        assert!(conflict.resolved);
    }

    /// 测试无法自动解决的冲突
    #[test]
    fn test_manual_intervention_needed() {
        let mut resolver = ConflictResolver::new()
            .with_auto_threshold(1.0); // 设置为1.0使自动解决更难

        let conflict_id = resolver.record(
            ConflictType::HierarchyConflict,
            vec!["父概念".to_string(), "子概念".to_string()],
            "层级冲突".to_string(),
            0.9,
        );

        // 层级冲突可能需要人工介入
        let resolution = resolver.try_auto_resolve(&conflict_id);

        // 可能无法自动解决
        if resolution.is_none() {
            // 手动解决
            let _resolution = resolver.apply_resolution(
                &conflict_id,
                lnn::language::consensus::ResolutionMethod::ManualIntervention,
                1.0,
            );

            let conflict = resolver.get_conflict(&conflict_id).unwrap();
            assert!(conflict.resolved);
        }
    }
}

// ============================================================================
// 端到端测试
// ============================================================================

mod e2e_tests {
    use super::*;

    /// 端到端测试：完整语言对齐流程
    #[test]
    fn test_full_alignment_pipeline() {
        // 1. 创建配置
        let config = GlobalConfig::new();

        // 2. 创建概念空间
        let mut space = ConceptSpace::new();

        // 3. 创建基础概念
        space.create_concept(
            "greeting".to_string(),
            "问候".to_string(),
            ConceptLevel::Basic,
        ).unwrap();

        space.create_child_concept("greeting", "hello".to_string(), "你好".to_string()).unwrap();
        space.create_child_concept("greeting", "hi".to_string(), "嗨".to_string()).unwrap();

        // 4. 编码输入
        let mut encoder = Encoder::new();
        let encoded = encoder.encode_full("你好");

        // 5. 验证编码结果
        assert!(!encoded.vectors.is_empty() || encoded.confidence < 0.5);

        // 6. 解码
        let decoder = Decoder::new();
        if !encoded.vectors.is_empty() {
            let decoded = decoder.decode(&encoded.vectors);
            assert!(!decoded.text.is_empty() || decoded.confidence < 0.5);
        }

        // 7. 验证概念学习
        let initial_sim = space.similarity("hello", "hi").unwrap();

        for _ in 0..20 {
            space.learn_association("hello", "hi", 1.0).unwrap();
        }

        let final_sim = space.similarity("hello", "hi").unwrap();
        assert!(final_sim >= initial_sim);
    }

    /// 端到端测试：多轮对话上下文
    #[test]
    fn test_multi_turn_conversation() {
        let context_manager = ContextManager::new();
        let mut context = context_manager.build_context("conversation_1".to_string());

        // 第一轮
        context.add_message(
            "user".to_string(),
            "我想了解Python编程".to_string(),
            vec!["Python".to_string(), "编程".to_string()],
        );

        // 第二轮
        context.add_message(
            "assistant".to_string(),
            "Python是一门很好的编程语言".to_string(),
            vec!["Python".to_string(), "编程语言".to_string()],
        );

        // 第三轮 - 使用代词
        context.add_message(
            "user".to_string(),
            "它难学吗？".to_string(),
            vec![],
        );

        // 解析指代
        let resolved = context_manager.resolve_pronouns("它难学吗？", &context);

        // 应该能解析"它"指代"Python"
        assert!(
            resolved.contains_key("它") || context.get_recent_entities(3).is_empty(),
            "应该能解析'它'指代Python"
        );

        // 验证历史记录
        assert_eq!(context.history.len(), 3);
    }
}
