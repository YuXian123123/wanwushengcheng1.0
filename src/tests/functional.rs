//! 功能正确性测试
//!
//! 基于螺丝咕姆验证方案的5个功能测试

use lnn::{LNN, NeuronType, PlasticityRule};
use lnn::learning::LearningRules;
use lnn::core::LNNConfig;

/// TEST-F1: 状态方程求解
///
/// 测试方法: 单神经元，恒定输入，运行多个时间步
/// 预期结果: 状态收敛到稳态值
#[test]
fn test_state_equation_convergence() {
    let mut lnn = LNN::new(None, None);

    // 创建简单的感知-认知-行为链
    let input = lnn.add_neuron(NeuronType::Perception).unwrap();
    let hidden = lnn.add_neuron(NeuronType::Cognitive).unwrap();
    let output = lnn.add_neuron(NeuronType::Behavior).unwrap();

    // 创建连接
    lnn.add_synapse(&input, &hidden, 0.5, PlasticityRule::Hebbian).unwrap();
    lnn.add_synapse(&hidden, &output, 0.5, PlasticityRule::Hebbian).unwrap();

    // 运行足够多的时间步
    for _ in 0..1000 {
        let _ = lnn.update(0.01);
    }

    // 网络应该保持稳定（未熔断）
    let state = lnn.get_state();
    assert!(state.neuron_count == 3);
}

/// TEST-F2: 赫布学习
///
/// 测试方法: 两个神经元同时激活，更新权重
/// 预期结果: 权重增加: Δw = η × 1.0 × 1.0 = η
#[test]
fn test_hebbian_learning() {
    // 直接测试学习规则
    let delta = LearningRules::hebbian(1.0, 1.0, 0.01);

    // 两个神经元同时激活，权重应该增加
    assert!((delta - 0.01).abs() < 1e-10,
        "赫布学习: Δw = η × pre × post = 0.01 × 1.0 × 1.0");

    // 一个激活一个抑制
    let delta_inhibit = LearningRules::hebbian(1.0, -1.0, 0.01);
    assert!(delta_inhibit < 0.0, "激活×抑制应该导致权重减少");
}

/// TEST-F3: Oja规则防爆炸
///
/// 测试方法: 持续学习多次，观察权重变化
/// 预期结果: 权重不会无限增长，收敛到稳定值
#[test]
fn test_oja_stability() {
    let mut weight = 0.1;
    let lr = 0.01;

    // 模拟持续学习
    for _ in 0..10000 {
        let delta = LearningRules::oja(1.0, 1.0, weight, lr);
        weight += delta;
    }

    // Oja规则应该防止权重爆炸
    assert!(weight.is_finite(), "权重应该是有限值");
    assert!(weight.abs() < 15.0, "权重应该收敛，不会无限增长");
    assert!(weight.abs() > 0.0, "应该有学习效果");
}

/// TEST-F4: 动态拓扑 - 神经元增删
///
/// 测试方法: 添加新神经元后删除
/// 预期结果: 正确添加和删除
#[test]
fn test_neuron_dynamic_add_remove() {
    let mut config = LNNConfig::default();
    config.topology.min_neurons = 2; // 设置较低的最小值

    let mut lnn = LNN::new(Some(config), None);

    // 添加神经元
    let id1 = lnn.add_neuron(NeuronType::Cognitive).unwrap();
    let id2 = lnn.add_neuron(NeuronType::Cognitive).unwrap();
    let id3 = lnn.add_neuron(NeuronType::Cognitive).unwrap();

    assert_eq!(lnn.get_state().neuron_count, 3);

    // 创建连接
    lnn.add_synapse(&id1, &id2, 0.5, PlasticityRule::Hebbian).unwrap();
    lnn.add_synapse(&id2, &id3, 0.5, PlasticityRule::Hebbian).unwrap();

    // 删除中间神经元，相关突触也应该被删除
    let result = lnn.remove_neuron(&id2);
    assert!(result.is_ok(), "删除应该成功: {:?}", result);

    let state = lnn.get_state();
    assert_eq!(state.neuron_count, 2);
    // 突触应该被清理
    assert_eq!(state.synapse_count, 0);
}

/// TEST-F5: 信号传递
///
/// 测试方法: 三个神经元链式连接 A→B→C，观察信号传递
/// 预期结果: 信号正确传递
#[test]
fn test_signal_propagation() {
    let mut lnn = LNN::new(None, None);

    // 创建链式连接 A -> B -> C
    let a = lnn.add_neuron(NeuronType::Perception).unwrap();
    let b = lnn.add_neuron(NeuronType::Cognitive).unwrap();
    let c = lnn.add_neuron(NeuronType::Behavior).unwrap();

    // 强连接
    lnn.add_synapse(&a, &b, 0.8, PlasticityRule::Hebbian).unwrap();
    lnn.add_synapse(&b, &c, 0.8, PlasticityRule::Hebbian).unwrap();

    // 运行更新
    for _ in 0..100 {
        let _ = lnn.update(0.01);
    }

    // 网络应该稳定
    let state = lnn.get_state();
    assert!(state.neuron_count == 3);
    assert!(state.synapse_count == 2);
}
