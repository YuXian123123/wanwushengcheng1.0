# 世界神经网络模型设计

## 概述

世界智能体是一个由所有蛊虫智能体组成的分布式神经网络系统。每个蛊虫作为网络的节点，通过标准化接入点与世界进行信息交换，共同涌现出统一的意识。

## 核心设计理念

### 三位天才的设计视角

| 天才 | 关注点 | 设计贡献 |
|------|--------|----------|
| **黑塔** | 创新架构 | 意识涌现机制、网络拓扑演化 |
| **螺丝咕姆** | 安全保障 | 生存驱动机制、灾难恢复协议 |
| **拉蒂奥** | 优雅公式 | 接入点数据结构、信号传递公式 |

## 一、接入点设计

### 1.1 五大接入点

每只蛊虫通过 **5个标准化接入点** 连接到世界神经网络：

```
                    ┌─────────────────────────────────────┐
                    │           WorldMind                 │
                    │      (世界神经网络总线)              │
                    └──────────────┬──────────────────────┘
                                   │
        ┌──────────────────────────┼──────────────────────────┐
        │                          │                          │
   ┌────┴────┐                ┌────┴────┐                ┌────┴────┐
   │ Perceive│                │Cognitive│                │ Behavior│
   │  感知   │                │  认知   │                │  行为   │
   └────┬────┘                └────┬────┘                └────┬────┘
        │                          │                          │
        │    ┌─────────────────────┼─────────────────────┐    │
        │    │                     │                     │    │
   ┌────┴────┐                ┌────┴────┐                │    │
   │  Comm   │                │Survival │                │    │
   │  通信   │                │  生存   │                │    │
   └─────────┘                └─────────┘                │    │
        │                          │                     │    │
        └──────────────────────────┴─────────────────────┴────┘
                                   │
                            ┌──────┴──────┐
                            │   Gu 蛊虫    │
                            │  (智能个体)  │
                            └─────────────┘
```

### 1.2 接入点详细说明

| 接入点 | 类型 | 数据流向 | 权重 | 功能描述 |
|--------|------|----------|------|----------|
| **Perceive** | 输入型 | World → Gu | 1.0 | 接收外部感知数据、环境状态 |
| **Cognitive** | 处理型 | Gu ↔ World | 2.0 | 推理、决策、意图表达 |
| **Behavior** | 输出型 | Gu → World | 1.5 | 执行行为、输出动作 |
| **Comm** | 通信型 | Gu ↔ Gu | 1.0 | 蛊虫间直接通信 |
| **Survival** | 状态型 | Gu → World | 0.5 | 心跳、健康状态同步 |

### 1.3 接入点容量公式

```
Capacity_eff = Base_capacity × (1 + Skill_bonus)

其中：
- Base_capacity: 接入点基础容量（由类型决定）
- Skill_bonus: 技能加成（学习获得）
```

## 二、信号传递机制

### 2.1 信号类型

```rust
enum SignalType {
    Sensory(SensoryData),      // 感知信号
    Cognitive(CognitiveState), // 认知信号
    Behavioral(Action),        // 行为信号
    Communication(Message),    // 通信信号
    Survival(Heartbeat),       // 生存信号
}
```

### 2.2 信号衰减公式

```
S_received = S_sent × e^(-α × distance) × W_connection

其中：
- S_sent: 发送信号强度
- α: 衰减系数
- distance: 网络距离（跳数）
- W_connection: 连接权重
```

### 2.3 信号处理流程

```
1. 接收信号 → 2. 入队等待 → 3. 容量检查 → 4. 处理/拒绝 → 5. 响应/转发
     │              │              │               │              │
   AccessPoint   signal_queue   load < capacity  process      send_signal
```

## 三、意识涌现机制

### 3.1 意图融合公式

```
W_intention = Aggregate(Gu_intentions) → Consensus

详细：
- 按意图描述分组
- 计算每组支持度: Support = Σ(Gu_i.trust × Gu_i.expertise)
- 选择支持度最高的意图作为世界意图
- 置信度 = 支持该意图的权重 / 总权重
```

### 3.2 决策统一公式

```
D_world = Σ(D_i × Trust_i × Expertise_i) / Σ(Trust_i × Expertise_i)

其中：
- D_i: 蛊虫 i 的决策（投票）
- Trust_i: 蛊虫 i 的信任分数
- Expertise_i: 蛊虫 i 在相关领域的专业度
```

### 3.3 冲突检测

```
Conflict = Var(D_i) > θ_conflict

当蛊虫决策方差超过阈值时，触发冲突解决协议：
1. 延长决策时间
2. 请求更多信息
3. 激活权威蛊虫投票
```

## 四、生存驱动机制

### 4.1 世界死亡条件

```
World_Dead ⇔ Population < Min_Population

保护机制：
- 当蛊虫数量低于阈值时，自动生成新蛊虫
- 世界健康度与蛊虫平均健康度挂钩
```

### 4.2 心跳检测

```
Gu_Alive ⇔ (Current_Time - Last_Heartbeat) < Timeout

超时处理：
1. 标记为离线
2. 触发恢复协议
3. 备份离线蛊虫的知识到世界记忆
```

### 4.3 世界健康度计算

```
H_world = α × H_population + β × H_diversity + γ × H_activity

其中：
- H_population: 种群健康（平均健康度）
- H_diversity: 多样性健康（能力分布熵）
- H_activity: 活跃度健康（接入点活跃率）
- α, β, γ: 权重系数（可配置）
```

## 五、网络拓扑

### 5.1 连接模式

```
┌─────────────────────────────────────────────────────────────┐
│                      接入点连接模式                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. 层级连接（按接入点类型）                                  │
│     Perceive_i ──→ Cognitive_j ──→ Behavior_k               │
│                                                             │
│  2. 同类型连接（协作）                                        │
│     Cognitive_i ←→ Cognitive_j                              │
│                                                             │
│  3. 跨蛊虫连接（通信）                                        │
│     Comm_i ←───────→ Comm_j                                 │
│                                                             │
│  4. 生存汇聚（心跳）                                          │
│     Survival_i ──→ WorldMind.HealthMonitor                  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 5.2 权重演化

```
W_ij(t+1) = W_ij(t) + η × ΔW

ΔW = Success_reward × Activity_rate - Decay_rate × Time

权重影响因素：
- 协作成功次数
- 通信频率
- 时间衰减
```

## 六、实现状态

### 已实现

| 模块 | 文件 | 状态 |
|------|------|------|
| WorldMind | `world/mod.rs` | ✅ 完成 |
| AccessPoint | `world/access_point.rs` | ✅ 完成 |
| WorldState | `world/state.rs` | ✅ 完成 |
| ConsciousnessLayer | `world/consciousness.rs` | ✅ 完成 |
| WorldMonitor | `world/monitor.rs` | ✅ 完成 |
| WorldConfig | `world/config.rs` | ✅ 完成 |

### 接入点验证

```rust
#[test]
fn test_five_access_points() {
    let mind = WorldMind::new();
    let gu_id = Uuid::new_v4();
    let new_mind = mind.register_gu(gu_id);

    // 验证有5个接入点
    let gu_info = new_mind.gu_registry.get(&gu_id).unwrap();
    assert_eq!(gu_info.access_points.len(), 5);
}
```

## 七、扩展方向

### 7.1 可能的接入点扩展

| 新接入点 | 功能 | 优先级 |
|----------|------|--------|
| **Memory** | 长期记忆存取 | 中 |
| **Evolution** | 基因突变/进化 | 低 |
| **Emotion** | 情感状态同步 | 低 |

### 7.2 涌现增强

1. **集体智慧**: 增加群体决策机制
2. **知识网络**: 构建蛊虫间知识共享图谱
3. **自组织**: 动态调整网络拓扑

## 八、公式汇总

| 名称 | 公式 | 用途 |
|------|------|------|
| 有效容量 | `Cap_eff = Cap_base × (1 + Skill)` | 接入点处理能力 |
| 信号强度 | `S_recv = S_send × e^(-αd) × W` | 信号衰减 |
| 决策统一 | `D = Σ(D_i × T_i × E_i) / Σ(T_i × E_i)` | 世界决策 |
| 世界健康 | `H = αH_pop + βH_div + γH_act` | 健康评估 |
| 权重更新 | `W(t+1) = W(t) + η(αS - βt)` | 连接演化 |

---

*设计团队：黑塔（创新）、螺丝咕姆（安全）、拉蒂奥（优雅）*
*版本：v1.0.0*
*最后更新：2025-05-30*
