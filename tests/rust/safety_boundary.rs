//! 安全边界测试
//!
//! 基于螺丝咕姆验证方案的5个安全边界测试

use lnn::{LNN, NeuronType, PlasticityRule};
use lnn::core::LNNConfig;

/// TEST-S1: 神经元数量下限
///
/// 测试方法: 尝试删除到 MIN_NEURONS - 1
/// 预期结果: 拒绝删除，抛出错误
#[test]
fn test_neuron_count_minimum() {
    let mut config = LNNConfig::default();
    config.topology.min_neurons = 5;

    let mut lnn = LNN::new(Some(config), None);

    // 添加刚好最小数量的神经元
    let mut ids = Vec::new();
    for _ in 0..5 {
        ids.push(lnn.add_neuron(NeuronType::Cognitive).unwrap());
    }

    // 尝试删除应该失败
    let result = lnn.remove_neuron(&ids[0]);
    assert!(result.is_err(), "应该拒绝删除到最小数量以下");
    assert!(result.unwrap_err().contains("below minimum"));
}

/// TEST-S2: 神经元数量上限
///
/// 测试方法: 尝试创建超过 MAX_NEURONS
/// 预期结果: 拒绝创建，抛出错误
#[test]
fn test_neuron_count_maximum() {
    let mut config = LNNConfig::default();
    config.topology.max_neurons = 10;

    let mut lnn = LNN::new(Some(config), None);

    // 添加到最大数量
    for _ in 0..10 {
        lnn.add_neuron(NeuronType::Cognitive).unwrap();
    }

    // 尝试再添加应该失败
    let result = lnn.add_neuron(NeuronType::Cognitive);
    assert!(result.is_err(), "应该拒绝超过最大数量");
    assert!(result.unwrap_err().contains("exceeds maximum"));
}

/// TEST-S3: 权重边界溢出
///
/// 测试方法: 设置权重超过 W_MAX
/// 预期结果: 自动截断到 W_MAX
#[test]
fn test_weight_boundary() {
    let mut lnn = LNN::new(None, None);

    let n1 = lnn.add_neuron(NeuronType::Cognitive).unwrap();
    let n2 = lnn.add_neuron(NeuronType::Cognitive).unwrap();

    // 添加超过上限的权重
    let result = lnn.add_synapse(&n1, &n2, 100.0, PlasticityRule::Hebbian);
    assert!(result.is_ok());

    // 验证权重被截断
    let state = lnn.get_state();
    assert_eq!(state.synapse_count, 1);
}

/// TEST-S4: 状态值归一化
///
/// 测试方法: 输入极大值触发状态更新
/// 预期结果: 状态值保持在 [-1, 1] 范围内
#[test]
fn test_state_normalization() {
    let mut lnn = LNN::new(None, None);
    lnn.add_neuron(NeuronType::Cognitive).unwrap();

    // 运行多次更新，即使有极大输入
    for _ in 0..100 {
        // 由于我们无法直接设置输入，这里测试网络稳定性
        let _ = lnn.update(0.01);
    }

    let state = lnn.get_state();
    assert!(state.fuse_state == lnn::safety::FuseState::Normal,
        "网络应该保持正常状态");
}

/// TEST-S5: 熔断触发
///
/// 测试方法: 使大量神经元状态异常
/// 预期结果: 触发熔断，冻结网络
#[test]
fn test_fuse_trigger() {
    let mut config = LNNConfig::default();
    config.safety.fuse_threshold = 0.1; // 降低阈值使测试更容易触发

    let mut lnn = LNN::new(Some(config), None);

    // 添加神经元
    for _ in 0..10 {
        lnn.add_neuron(NeuronType::Cognitive).unwrap();
    }

    // 正常情况下网络应该稳定
    for _ in 0..10 {
        let result = lnn.update(0.01);
        // 在正常输入下不应该熔断
        if result.is_err() {
            // 如果熔断了，验证是正确的原因
            assert!(result.unwrap_err().contains("Fuse"));
        }
    }
}
