# 知识整理与技能升级系统

## 核心理念

**技能升级 ≠ 购买等级**
**技能升级 = 神经网络对相关知识的整理、巩固、拓展**

这符合人脑学习的原理：
- 学习新知识 → 神经连接形成
- 整理知识 → 神经连接强化
- 拓展关联 → 神经网络扩大

## 当前问题

```
Skill {
    name: "火焰喷射",
    current_level: 1,      // 只是数字
    max_level: 5,
    upgrade_cost: 100.0,   // 金币购买等级
}

Knowledge {
    name: "火焰操控进阶技巧",
    price: 150.0,          // 知识和技能没有关联
}
```

**问题**：
1. 技能和知识是割裂的
2. 升级只是"花钱买等级"
3. 知识学习后没有实际作用

## 新设计：知识图谱驱动的技能

### 1. 知识点定义

```rust
/// 知识点 - 神经网络中的一个信息单元
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    /// 知识ID
    pub id: Uuid,
    /// 知识名称
    pub name: String,
    /// 知识内容
    pub content: String,
    /// 关联的技能
    pub related_skill: String,
    /// 知识类型
    pub node_type: KnowledgeNodeType,
    /// 掌握程度 (0.0 - 1.0)
    pub mastery: f64,
    /// 整理程度 (0.0 - 1.0)
    pub organization: f64,
    /// 关联的其他知识点
    pub connections: Vec<Uuid>,
}

/// 知识点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeNodeType {
    /// 基础概念 - 构成技能的基础
    Foundation,
    /// 技巧方法 - 实际应用的方法
    Technique,
    /// 原理理论 - 深层理解
    Theory,
    /// 拓展应用 - 衍生应用
    Extension,
    /// 经验总结 - 从实践获得
    Experience,
}
```

### 2. 技能定义（重新设计）

```rust
/// 技能 - 由知识点网络构成的能力
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// 技能名称
    pub name: String,
    /// 技能描述
    pub description: String,
    /// 已掌握的知识点
    pub knowledge_nodes: HashMap<Uuid, KnowledgeNode>,
    /// 技能等级（由知识网络密度决定，而非购买）
    pub level: u32,
    /// 最大等级
    pub max_level: u32,
}

impl Skill {
    /// 计算技能等级（基于知识网络）
    ///
    /// 等级 = 知识点数量 × 平均掌握度 × 网络连接密度
    pub fn calculate_level(&self) -> u32 {
        if self.knowledge_nodes.is_empty() {
            return 1;
        }

        let node_count = self.knowledge_nodes.len() as f64;

        // 平均掌握度
        let avg_mastery: f64 = self.knowledge_nodes.values()
            .map(|n| n.mastery)
            .sum::<f64>() / node_count;

        // 网络连接密度
        let total_connections: usize = self.knowledge_nodes.values()
            .map(|n| n.connections.len())
            .sum();
        let max_connections = node_count * (node_count - 1.0);
        let density = if max_connections > 0.0 {
            total_connections as f64 / max_connections
        } else {
            0.0
        };

        // 等级公式
        let raw_level = (node_count / 3.0) * avg_mastery * (1.0 + density);
        (raw_level.ceil() as u32).min(self.max_level)
    }

    /// 计算技能强度（用于任务执行）
    pub fn calculate_power(&self) -> f64 {
        let level = self.calculate_level();

        // 基础强度
        let base_power = level as f64 * 10.0;

        // 知识类型加成
        let type_bonus: f64 = self.knowledge_nodes.values()
            .map(|n| match n.node_type {
                KnowledgeNodeType::Foundation => n.mastery * 5.0,
                KnowledgeNodeType::Technique => n.mastery * 8.0,
                KnowledgeNodeType::Theory => n.mastery * 3.0,
                KnowledgeNodeType::Extension => n.mastery * 6.0,
                KnowledgeNodeType::Experience => n.mastery * 7.0,
            })
            .sum();

        base_power + type_bonus
    }
}
```

### 3. 行为：知识整理

```rust
/// 知识整理行为 - 强化已有知识的神经连接
pub fn organize_knowledge(
    skill: &mut Skill,
    knowledge_id: Uuid,
) -> ActionResult {
    let node = match skill.knowledge_nodes.get_mut(&knowledge_id) {
        Some(n) => n,
        None => return ActionResult {
            success: false,
            message: "知识点不存在".to_string(),
            transaction: None,
        },
    };

    // 整理提升组织度
    let improvement = 0.1 * (1.0 - node.organization); // 越接近1提升越慢
    node.organization = (node.organization + improvement).min(1.0);

    // 组织度提升也略微提升掌握度
    node.mastery = (node.mastery + improvement * 0.3).min(1.0);

    // 重新计算技能等级
    let old_level = skill.level;
    skill.level = skill.calculate_level();

    ActionResult {
        success: true,
        message: format!(
            "整理「{}」，组织度 {:.0}%→{:.0}%，技能等级 {}→{}",
            node.name,
            (node.organization - improvement) * 100.0,
            node.organization * 100.0,
            old_level,
            skill.level
        ),
        transaction: None, // 整理不消耗金币，只消耗时间
    }
}
```

### 4. 行为：知识拓展

```rust
/// 知识拓展行为 - 学习相关联的新知识点
pub fn extend_knowledge(
    skill: &mut Skill,
    new_knowledge: KnowledgeNode,
    prerequisite_id: Option<Uuid>,
) -> ActionResult {
    // 如果有前置知识，建立连接
    if let Some(pre_id) = prerequisite_id {
        if let Some(pre_node) = skill.knowledge_nodes.get_mut(&pre_id) {
            pre_node.connections.push(new_knowledge.id);
        }
    }

    let knowledge_name = new_knowledge.name.clone();
    skill.knowledge_nodes.insert(new_knowledge.id, new_knowledge);

    // 重新计算技能等级
    let old_level = skill.level;
    skill.level = skill.calculate_level();

    ActionResult {
        success: true,
        message: format!(
            "学习「{}」，技能等级 {}→{}",
            knowledge_name,
            old_level,
            skill.level
        ),
        transaction: None,
    }
}
```

### 5. 行为：知识补充

```rust
/// 知识补充行为 - 填补知识网络中的空白
pub fn supplement_knowledge(
    skill: &mut Skill,
    from_id: Uuid,
    to_id: Uuid,
    bridge_knowledge: KnowledgeNode,
) -> ActionResult {
    // 检查两个知识点是否存在
    if !skill.knowledge_nodes.contains_key(&from_id) ||
       !skill.knowledge_nodes.contains_key(&to_id) {
        return ActionResult {
            success: false,
            message: "前置知识点不存在".to_string(),
            transaction: None,
        };
    }

    // 建立桥梁连接
    let bridge_id = bridge_knowledge.id;
    skill.knowledge_nodes.insert(bridge_id, bridge_knowledge);

    // 连接三个节点
    skill.knowledge_nodes.get_mut(&from_id).unwrap().connections.push(bridge_id);
    skill.knowledge_nodes.get_mut(&bridge_id).unwrap().connections.push(to_id);

    // 重新计算技能等级
    let old_level = skill.level;
    skill.level = skill.calculate_level();

    ActionResult {
        success: true,
        message: format!("补充桥梁知识，网络连通性提升，技能等级 {}→{}", old_level, skill.level),
        transaction: None,
    }
}
```

## 神经网络驱动的学习决策

### 1. Cognitive 神经元驱动学习

```rust
/// 根据 Cognitive 神经元状态决定学习行为
pub fn decide_learning(lnn: &GuLNN, skill: &Skill) -> LearningAction {
    let cognitive_state = lnn.get_neuron_state(NeuronType::Cognitive);
    let behavior_state = lnn.get_neuron_state(NeuronType::Behavior);

    // 认知需求高 → 需要整理已有知识
    if cognitive_state > 0.6 {
        // 找出组织度最低的知识点进行整理
        let unorganized = skill.knowledge_nodes.values()
            .filter(|n| n.organization < 0.7)
            .min_by(|a, b| a.organization.partial_cmp(&b.organization).unwrap())
            .map(|n| n.id);

        if let Some(id) = unorganized {
            return LearningAction::Organize(id);
        }
    }

    // 行为需求高 + 认知适中 → 拓展新知识
    if behavior_state > 0.5 && cognitive_state > 0.3 {
        return LearningAction::Extend;
    }

    // 默认：补充空白
    LearningAction::Supplement
}

pub enum LearningAction {
    /// 整理已有知识
    Organize(Uuid),
    /// 拓展新知识
    Extend,
    /// 补充桥梁知识
    Supplement,
}
```

### 2. 完整的学习循环

```
┌─────────────────────────────────────────────────────────────┐
│                    知识驱动学习循环                          │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Cognitive 神经元状态                                     │
│     └─▶ 高认知需求 = 需要整理/理解                           │
│     └─▶ 低认知需求 = 可以拓展                               │
│                                                              │
│  2. 知识网络状态                                             │
│     └─▶ 组织度低 → 整理行为                                 │
│     └─▶ 连接稀疏 → 补充行为                                 │
│     └─▶ 节点不足 → 拓展行为                                 │
│                                                              │
│  3. 技能等级自动提升                                         │
│     └─▶ level = f(节点数, 掌握度, 连接密度)                  │
│                                                              │
│  4. 技能强度用于任务                                         │
│     └─▶ power = f(等级, 知识类型加成)                        │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## 示例场景

### 场景：火焰喷射技能的升级

```
初始状态：
Skill: 火焰喷射 Lv.1
├── 知识点：
│   └── [基础] 点火原理 (mastery: 0.8, organization: 0.5)
│
技能等级 = 1 (只有1个节点)

学习行为1：拓展知识
├── 学习：[技巧] 火焰形态控制
├── 连接：点火原理 → 火焰形态控制
│
技能等级 = 2 (2个节点，有连接)

学习行为2：整理知识
├── 整理：点火原理 (organization: 0.5 → 0.9)
├── 掌握度提升：mastery: 0.8 → 0.85
│
技能等级 = 2 (节点未增，但组织度提升)

学习行为3：补充桥梁
├── 补充：[理论] 燃烧三要素
├── 连接：点火原理 → 燃烧三要素 → 火焰形态控制
│
技能等级 = 3 (网络连通性大幅提升)

学习行为4：拓展应用
├── 学习：[拓展] 连续火焰喷射
├── 连接：火焰形态控制 → 连续火焰喷射
│
技能等级 = 4 (节点数和连接密度都提升)
```

## 与需求系统的结合

```
知识学习/整理的动力来源：

1. Survival 需求
   └─▶ 任务需要更高技能强度 → 提升技能等级
   └─▶ 技能等级需要知识网络支撑 → 学习/整理知识

2. Cognitive 需求
   └─▶ 认知神经元状态过低 → 需要整理知识提升理解
   └─▶ 认知神经元状态过高 → 需要实践/拓展

3. Behavior 需求
   └─▶ 行为神经元激活 → 执行任务获得金币
   └─▶ 任务难度 > 技能强度 → 需要提升技能
```

## 实现步骤

1. **重构 KnowledgeNode** - 添加掌握度、组织度、连接
2. **重构 Skill** - 改为知识网络结构
3. **实现学习行为** - 整理、拓展、补充
4. **修改升级逻辑** - 等级由网络计算而非购买
5. **连接神经网络** - Cognitive 状态驱动学习决策
