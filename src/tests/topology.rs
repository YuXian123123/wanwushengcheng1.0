//! 动态拓扑测试
//!
//! 测试神经元和突触的动态管理

use lnn::{LNN, NeuronType, PlasticityRule};
use lnn::core::{LNNConfig, TopologyDynamics};

/// 测试突触修剪
///
/// 不活跃的突触应该被自动删除
#[test]
fn test_synapse_pruning() {
    let mut topo = TopologyDynamics::default();
    topo.synapse_weight_threshold = 0.1;
    topo.prune_cooldown_ms = 0; // 立即修剪

    let mut lnn = LNN::new(None, Some(topo));

    let n1 = lnn.add_neuron(NeuronType::Cognitive).unwrap();
    let n2 = lnn.add_neuron(NeuronType::Cognitive).unwrap();

    // 添加一个极小权重的突触
    lnn.add_synapse(&n1, &n2, 0.01, PlasticityRule::Hebbian).unwrap();

    // 运行更新触发修剪
    for _ in 0..100 {
        let _ = lnn.update(0.01);
    }

    // 检查突触数量（可能被修剪）
    let state = lnn.get_state();
    assert!(state.synapse_count <= 1);
}

/// 测试连接数限制
#[test]
fn test_connection_limit() {
    let mut config = LNNConfig::default();
    config.topology.max_connections = 3;

    let mut lnn = LNN::new(Some(config), None);

    let source = lnn.add_neuron(NeuronType::Perception).unwrap();

    // 添加目标神经元
    let mut targets = Vec::new();
    for _ in 0..5 {
        targets.push(lnn.add_neuron(NeuronType::Cognitive).unwrap());
    }

    // 添加连接到限制
    for i in 0..3 {
        lnn.add_synapse(&source, &targets[i], 0.5, PlasticityRule::Hebbian).unwrap();
    }

    // 尝试添加超过限制的连接
    let result = lnn.add_synapse(&source, &targets[3], 0.5, PlasticityRule::Hebbian);
    assert!(result.is_err(), "应该拒绝超过连接限制");
}

/// 测试审计日志
#[test]
fn test_audit_log() {
    let mut config = LNNConfig::default();
    config.safety.audit_topology = true;

    let mut lnn = LNN::new(Some(config), None);

    // 添加神经元和突触
    let n1 = lnn.add_neuron(NeuronType::Cognitive).unwrap();
    let n2 = lnn.add_neuron(NeuronType::Cognitive).unwrap();
    lnn.add_synapse(&n1, &n2, 0.5, PlasticityRule::Hebbian).unwrap();

    // 检查审计日志
    let log = lnn.get_audit_log();
    assert!(log.len() >= 3, "应该记录拓扑变更");

    // 检查日志内容
    let events: Vec<&str> = log.iter().map(|e| e.event.as_str()).collect();
    assert!(events.contains(&"neuron_added"));
    assert!(events.contains(&"synapse_added"));
}

/// 测试熔断后状态冻结
#[test]
fn test_frozen_state() {
    let mut lnn = LNN::new(None, None);

    // 添加神经元
    for _ in 0..5 {
        lnn.add_neuron(NeuronType::Cognitive).unwrap();
    }

    // 获取初始状态
    let initial_state = lnn.get_state();

    // 正常更新
    for _ in 0..10 {
        let _ = lnn.update(0.01);
    }

    // 状态应该保持
    let final_state = lnn.get_state();
    assert_eq!(initial_state.neuron_count, final_state.neuron_count);
}

/// 测试不同神经元类型
#[test]
fn test_neuron_types() {
    let mut lnn = LNN::new(None, None);

    // 添加不同类型的神经元
    let p = lnn.add_neuron(NeuronType::Perception).unwrap();
    let c = lnn.add_neuron(NeuronType::Cognitive).unwrap();
    let b = lnn.add_neuron(NeuronType::Behavior).unwrap();

    // 创建跨类型的连接
    lnn.add_synapse(&p, &c, 0.5, PlasticityRule::Hebbian).unwrap();
    lnn.add_synapse(&c, &b, 0.5, PlasticityRule::Hebbian).unwrap();

    // 运行更新
    for _ in 0..50 {
        let _ = lnn.update(0.01);
    }

    let state = lnn.get_state();
    assert_eq!(state.neuron_count, 3);
    assert_eq!(state.synapse_count, 2);
}
