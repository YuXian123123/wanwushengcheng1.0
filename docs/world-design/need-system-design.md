# 蛊虫需求系统设计

## 问题分析

当前行为系统有**结果**但没有**需求**：
- 购买资源 → 资源只是消耗金币，没有实际作用
- 升级技能 → 技能升级后没有实际意义
- 执行任务 → 获得金币，但金币只用于上述无意义消费

**核心缺失**：行为没有服务于生存目标。

## 设计原则

### 三维天才约束

| 天才 | 约束 | 设计要求 |
|------|------|----------|
| 黑塔 | 涌现 | 需求从神经网络状态自然涌现，不硬编码 |
| 螺丝咕姆 | 生存 | 需求必须与生存绑定，满足生存约束 |
| 拉蒂奥 | 优雅 | 需求系统数学简洁，满足归一化约束 |

## 核心设计：需求 = 神经网络状态偏离

### 1. 需求的定义

**需求不是外部赋予的，而是神经网络状态偏离理想状态时产生的张力。**

```
需求 = |理想状态 - 当前状态|

其中：
- 理想状态：各神经元的最优激活水平
- 当前状态：当前神经网络状态向量
- 张力：驱动行为以恢复平衡
```

### 2. 五维需求映射

每个神经元对应一种需求类型：

| 神经元 | 状态过低时 | 状态过高时 | 理想状态 |
|--------|-----------|-----------|----------|
| **Perception** | 渴望刺激/信息过载 | 感官疲劳/需要休息 | 0.3-0.7 |
| **Cognitive** | 困惑/需要学习 | 过度思考/需要行动 | 0.4-0.6 |
| **Behavior** | 被动/需要执行任务 | 冲动/需要抑制 | 0.2-0.5 |
| **Comm** | 孤独/需要交流 | 社交过度/需要独处 | 0.3-0.5 |
| **Survival** | **死亡风险/紧急生存需求** | 过度安全/浪费资源 | 0.6-0.8 |

### 3. Survival 神经元的核心作用

```
Survival 状态决定蛊虫的"生存压力"：

Survival > 0.6  →  安全状态，可以探索、学习、社交
Survival < 0.6  →  生存压力，优先执行任务获取金币
Survival < 0.3  →  危机状态，紧急购买资源恢复
Survival < 0.1  →  濒死状态，无法行动
```

**Survival 状态的影响因素**：
1. 金币余额：`balance_factor = sigmoid(balance / 1000)`
2. 技能等级：`skill_factor = avg(skill_levels) / max_level`
3. 资源持有：`resource_factor = count(resources) / 10`

```
Survival = w1 × balance_factor + w2 × skill_factor + w3 × resource_factor + bias

其中 bias = 0.5 (基础生存偏置)
```

## 需求驱动的行为系统

### 1. 资源系统重构

```rust
pub struct Resource {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    /// 资源效果：影响哪个神经元
    pub target_neuron: NeuronType,
    /// 效果强度
    pub effect_strength: f64,
    /// 持续时间（毫秒）
    pub duration_ms: u64,
}

// 示例
Resource {
    name: "生命之水",
    target_neuron: NeuronType::Survival,
    effect_strength: 0.3,  // +0.3 Survival
    duration_ms: 0,        // 即时效果
    price: 60.0,
}

Resource {
    name: "智慧卷轴",
    target_neuron: NeuronType::Cognitive,
    effect_strength: 0.2,
    duration_ms: 3600000,  // 持续1小时
    price: 80.0,
}
```

### 2. 技能系统重构

```rust
pub struct Skill {
    pub name: String,
    pub level: u32,
    pub max_level: u32,
    /// 技能对生存的贡献
    pub survival_contribution: f64,
    /// 升级成本公式
    pub upgrade_cost: fn(u32) -> f64,
}

impl Skill {
    /// 技能等级直接影响 Survival 状态
    pub fn survival_boost(&self) -> f64 {
        self.survival_contribution * (self.level as f64 / self.max_level as f64)
    }
}
```

### 3. 任务系统重构

```rust
pub struct Task {
    pub name: String,
    pub reward: f64,
    /// 任务难度影响 Behavior 神经元消耗
    pub difficulty: f64,
    /// 完成任务需要消耗的神经元状态
    pub cost: NeuralCost,
}

pub struct NeuralCost {
    /// 消耗的 Behavior 激活
    pub behavior: f64,
    /// 消耗的 Cognitive 激活
    pub cognitive: f64,
    /// 时间成本（毫秒）
    pub time_ms: u64,
}
```

## 需求涌现的数学模型

### 1. 需求向量

```
需求向量 D = [d₁, d₂, d₃, d₄, d₅]

其中：
dᵢ = |idealᵢ - currentᵢ| × weightᵢ

理想状态向量 ideal = [0.5, 0.5, 0.35, 0.4, 0.7]
权重向量 weight = [0.5, 1.0, 0.8, 0.6, 2.0]  // Survival 权重最高
```

### 2. 行为选择公式

```
选择行为 a* 使得需求满足最大化：

a* = argmax_a Σᵢ (Δneed_satisfactionᵢ(a) / cost(a))

其中：
- Δneed_satisfactionᵢ(a) = 行为a对需求i的满足程度
- cost(a) = 行为a的金币/时间/神经消耗
```

### 3. 具体行为的需求满足

```
购买资源 r:
  ΔSurvival = r.effect_strength (如果 r.target_neuron == Survival)
  cost = r.price

升级技能 s:
  ΔSurvival = s.survival_boost() / 10
  cost = s.upgrade_cost()

执行任务 t:
  ΔSurvival = t.reward / 1000 × w_balance
  cost = t.cost.behavior + t.cost.cognitive

转账:
  ΔComm = 社交满足度（接收者的感激度）
  ΔSurvival = -amount / balance × 0.1 (负面影响)
```

## 完整需求循环

```
┌─────────────────────────────────────────────────────────────┐
│                    需求涌现循环                              │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. 世界状态变化                                             │
│     └─▶ 金币余额、技能等级、资源持有变化                      │
│                                                              │
│  2. Survival 神经元状态更新                                   │
│     └─▶ Survival = f(balance, skills, resources)             │
│                                                              │
│  3. 需求向量计算                                             │
│     └─▶ D = |ideal - current| × weight                       │
│                                                              │
│  4. 行为决策                                                 │
│     └─▶ a* = argmax Σ(Δneed_satisfaction / cost)             │
│                                                              │
│  5. 执行行为                                                 │
│     └─▶ 更新世界状态 → 返回步骤1                              │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## 示例场景

### 场景1：金币不足触发生存需求

```
当前状态：
- balance = 50 金币
- skills = 2级火焰喷射
- resources = []

计算：
- balance_factor = sigmoid(50/1000) = 0.512
- skill_factor = 2/10 = 0.2
- resource_factor = 0/10 = 0.0
- Survival = 0.4×0.512 + 0.3×0.2 + 0.3×0 + 0.5 = 0.705

理想 Survival = 0.7，当前 Survival = 0.705 → 需求满足

但如果 balance = 10：
- balance_factor = sigmoid(10/1000) = 0.502
- Survival = 0.4×0.502 + 0.3×0.2 + 0.3×0 + 0.5 = 0.661
- 需求 d_survival = |0.7 - 0.661| × 2.0 = 0.078

行为选择：
- 执行任务获得金币 → ΔSurvival = reward/1000 × 0.4
- 任务奖励50金币 → ΔSurvival = 0.02
- 成本：behavior消耗0.1，cognitive消耗0.05

决策：执行任务（需求满足/成本比最高）
```

### 场景2：技能升级需求

```
当前状态：
- balance = 500 金币
- skills = 1级火焰喷射（survival_contribution = 0.3）
- Survival = 0.75（良好）

但 Cognitive 状态：
- current Cognitive = 0.2（过低）
- ideal Cognitive = 0.5
- 需求 d_cognitive = |0.5 - 0.2| × 1.0 = 0.3

行为选择：
- 升级技能 → 提升能力 → 间接提升 Survival
- 学习知识 → 直接提升 Cognitive

决策：学习知识（直接满足 Cognitive 需求）
```

## 实现计划

1. **重构 Resource** - 添加 `target_neuron` 和 `effect_strength`
2. **重构 Skill** - 添加 `survival_contribution`
3. **重构 Task** - 添加 `NeuralCost`
4. **实现需求计算** - 在 GuLNN 中添加 `calculate_needs()`
5. **实现行为选择** - 基于需求满足最大化选择行为
