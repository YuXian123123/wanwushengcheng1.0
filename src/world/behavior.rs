//! 蛊虫行为系统 - 真实产生交易事件的行为
//!
//! 蛊虫通过行为与世界交互，产生真实的交易事件：
//! - 执行任务获得奖励
//! - 知识整理/拓展/补充（技能升级）
//! - 购买资源消耗金币（熵增）
//! - 转账给其他蛊虫
//!
//! ## 核心理念
//!
//! 技能升级 ≠ 购买等级
//! 技能升级 = 神经网络对相关知识的整理、巩固、拓展

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use super::event::{TransactionData, TransactionKind, WorldEvent};

/// 蛊虫钱包
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuWallet {
    /// 蛊虫 ID
    pub gu_id: Uuid,
    /// 当前余额
    pub balance: f64,
    /// 累计收入
    pub total_income: f64,
    /// 累计支出
    pub total_expense: f64,
    /// 交易历史 ID 列表
    pub transaction_ids: Vec<String>,
}

impl GuWallet {
    pub fn new(gu_id: Uuid, initial_balance: f64) -> Self {
        Self {
            gu_id,
            balance: initial_balance,
            total_income: initial_balance,
            total_expense: 0.0,
            transaction_ids: Vec::new(),
        }
    }

    /// 存入金币，返回交易事件
    pub fn deposit(&mut self, amount: f64, reason: &str, detail: &str, gu_name: &str) -> TransactionData {
        self.balance += amount;
        self.total_income += amount;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let tx = TransactionData {
            id: format!("tx_{:08x}_{}", timestamp, self.gu_id),
            timestamp,
            from_id: "system".to_string(),
            from_name: "系统".to_string(),
            from_balance: 0.0,
            to_id: self.gu_id.to_string(),
            to_name: gu_name.to_string(),
            to_balance: self.balance,
            amount,
            kind: TransactionKind::Deposit,
            reason: reason.to_string(),
            detail: detail.to_string(),
        };

        self.transaction_ids.push(tx.id.clone());
        tx
    }

    /// 取出金币，返回交易事件（如果余额不足返回 None）
    pub fn withdraw(&mut self, amount: f64, reason: &str, detail: &str, gu_name: &str) -> Option<TransactionData> {
        if self.balance < amount {
            return None;
        }

        self.balance -= amount;
        self.total_expense += amount;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let tx = TransactionData {
            id: format!("tx_{:08x}_{}", timestamp, self.gu_id),
            timestamp,
            from_id: self.gu_id.to_string(),
            from_name: gu_name.to_string(),
            from_balance: self.balance,
            to_id: "system".to_string(),
            to_name: "系统".to_string(),
            to_balance: 0.0,
            amount: -amount,
            kind: TransactionKind::Withdraw,
            reason: reason.to_string(),
            detail: detail.to_string(),
        };

        self.transaction_ids.push(tx.id.clone());
        Some(tx)
    }
}

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    /// 待领取（用户创建，等待蛊虫接取）
    Pending,
    /// 进行中（已分配给蛊虫）
    InProgress,
    /// 已完成（用户确认完成）
    Completed,
    /// 已取消
    Cancelled,
}

/// 任务定义
///
/// 任务由用户在界面创建，用户判断是否完成。
/// 完成后奖励自动发放给执行任务的蛊虫。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    /// 任务名称
    pub name: String,
    /// 任务描述
    pub description: String,
    /// 难度系数 (0.0-1.0)
    pub difficulty: f64,
    /// 完成奖励（金币）
    pub reward: f64,
    /// 所需技能（可选）
    pub required_skills: Vec<String>,
    /// 任务状态
    pub status: TaskStatus,
    /// 分配给的蛊虫 ID（None 表示未分配）
    pub assigned_to: Option<Uuid>,
    /// 创建时间戳
    pub created_at: u64,
    /// 完成时间戳
    pub completed_at: Option<u64>,
}

impl Task {
    /// 创建新任务（由用户创建）
    pub fn new(name: String, description: String, reward: f64) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: Uuid::new_v4(),
            name,
            description,
            difficulty: 0.5, // 默认中等难度
            reward,
            required_skills: Vec::new(),
            status: TaskStatus::Pending,
            assigned_to: None,
            created_at: timestamp,
            completed_at: None,
        }
    }

    /// 创建带技能要求的任务
    pub fn with_skills(mut self, skills: Vec<String>) -> Self {
        self.required_skills = skills;
        self
    }

    /// 设置难度
    pub fn with_difficulty(mut self, difficulty: f64) -> Self {
        self.difficulty = difficulty.clamp(0.0, 1.0);
        self
    }

    /// 分配给蛊虫
    pub fn assign_to(&mut self, gu_id: Uuid) {
        self.assigned_to = Some(gu_id);
        self.status = TaskStatus::InProgress;
    }

    /// 标记完成（由用户操作）
    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );
    }

    /// 取消任务
    pub fn cancel(&mut self) {
        self.status = TaskStatus::Cancelled;
    }
}

/// 资源定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub effect: String,
}

impl Resource {
    pub fn resource_pool() -> Vec<Resource> {
        vec![
            Resource { id: Uuid::new_v4(), name: "火元素结晶×3".to_string(), price: 30.0, effect: "提升火系技能威力".to_string() },
            Resource { id: Uuid::new_v4(), name: "冰霜精华×5".to_string(), price: 45.0, effect: "提升冰系技能持续时间".to_string() },
            Resource { id: Uuid::new_v4(), name: "雷电能核×2".to_string(), price: 55.0, effect: "提升雷电技能连锁数".to_string() },
            Resource { id: Uuid::new_v4(), name: "风之息×4".to_string(), price: 35.0, effect: "提升风系技能范围".to_string() },
            Resource { id: Uuid::new_v4(), name: "土灵石×6".to_string(), price: 40.0, effect: "提升防御能力".to_string() },
            Resource { id: Uuid::new_v4(), name: "金精矿×3".to_string(), price: 50.0, effect: "提升攻击力".to_string() },
            Resource { id: Uuid::new_v4(), name: "生命之水×2".to_string(), price: 60.0, effect: "恢复生命值".to_string() },
            Resource { id: Uuid::new_v4(), name: "光明粉尘×5".to_string(), price: 38.0, effect: "提升光系技能效果".to_string() },
        ]
    }

    pub fn random_resource() -> Self {
        let pool = Self::resource_pool();
        let idx = rand::random::<usize>() % pool.len();
        pool[idx].clone()
    }
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

/// 知识点 - 神经网络中的一个信息单元
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    /// 知识ID
    pub id: Uuid,
    /// 知识名称
    pub name: String,
    /// 知识内容描述
    pub content: String,
    /// 知识类型
    pub node_type: KnowledgeNodeType,
    /// 掌握程度 (0.0 - 1.0)
    pub mastery: f64,
    /// 整理程度 (0.0 - 1.0)
    pub organization: f64,
    /// 关联的其他知识点ID
    pub connections: Vec<Uuid>,
}

impl KnowledgeNode {
    /// 创建新的知识点
    pub fn new(name: String, content: String, node_type: KnowledgeNodeType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            content,
            node_type,
            mastery: 0.1,      // 初始掌握度低
            organization: 0.0, // 初始未整理
            connections: Vec::new(),
        }
    }

    /// 整理知识点（提升组织度）
    pub fn organize(&mut self) -> f64 {
        let improvement = 0.1 * (1.0 - self.organization);
        self.organization = (self.organization + improvement).min(1.0);
        // 组织度提升也略微提升掌握度
        self.mastery = (self.mastery + improvement * 0.3).min(1.0);
        improvement
    }

    /// 实践知识点（提升掌握度）
    pub fn practice(&mut self, amount: f64) {
        self.mastery = (self.mastery + amount).min(1.0);
    }

    /// 计算知识点的贡献值
    pub fn contribution(&self) -> f64 {
        let type_weight = match self.node_type {
            KnowledgeNodeType::Foundation => 0.5,
            KnowledgeNodeType::Technique => 0.8,
            KnowledgeNodeType::Theory => 0.3,
            KnowledgeNodeType::Extension => 0.6,
            KnowledgeNodeType::Experience => 0.7,
        };
        self.mastery * self.organization * type_weight
    }
}

/// 技能定义 - 由知识点网络构成的能力
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// 技能名称
    pub name: String,
    /// 技能描述
    pub description: String,
    /// 已掌握的知识点
    pub knowledge_nodes: HashMap<Uuid, KnowledgeNode>,
    /// 技能等级（由知识网络密度决定）
    pub level: u32,
    /// 最大等级
    pub max_level: u32,
}

impl Skill {
    /// 创建新技能（包含一个基础知识点）
    pub fn new(name: String, description: String, foundation: KnowledgeNode) -> Self {
        let mut nodes = HashMap::new();
        let foundation_id = foundation.id;
        nodes.insert(foundation_id, foundation);

        Self {
            name,
            description,
            knowledge_nodes: nodes,
            level: 1,
            max_level: 10,
        }
    }

    /// 技能池（初始技能）
    pub fn skill_pool() -> Vec<Skill> {
        vec![
            Skill::new(
                "火焰喷射".to_string(),
                "喷射火焰攻击敌人".to_string(),
                KnowledgeNode::new(
                    "点火原理".to_string(),
                    "理解如何点燃火焰的基本原理".to_string(),
                    KnowledgeNodeType::Foundation,
                ),
            ),
            Skill::new(
                "冰冻护盾".to_string(),
                "创造冰盾保护自己".to_string(),
                KnowledgeNode::new(
                    "冰晶凝聚".to_string(),
                    "将水汽凝聚成冰晶的基础".to_string(),
                    KnowledgeNodeType::Foundation,
                ),
            ),
            Skill::new(
                "闪电链".to_string(),
                "释放连锁闪电攻击多个目标".to_string(),
                KnowledgeNode::new(
                    "电荷传导".to_string(),
                    "理解电荷如何在目标间传导".to_string(),
                    KnowledgeNodeType::Foundation,
                ),
            ),
            Skill::new(
                "风刃".to_string(),
                "发射锋利的风刃切割敌人".to_string(),
                KnowledgeNode::new(
                    "气流控制".to_string(),
                    "控制气流形态的基础".to_string(),
                    KnowledgeNodeType::Foundation,
                ),
            ),
            Skill::new(
                "岩石护甲".to_string(),
                "召唤岩石形成护甲".to_string(),
                KnowledgeNode::new(
                    "土元素凝聚".to_string(),
                    "凝聚土元素形成固体".to_string(),
                    KnowledgeNodeType::Foundation,
                ),
            ),
            Skill::new(
                "自然治愈".to_string(),
                "使用自然能量治疗伤口".to_string(),
                KnowledgeNode::new(
                    "生命力感知".to_string(),
                    "感知并引导生命能量".to_string(),
                    KnowledgeNodeType::Foundation,
                ),
            ),
        ]
    }

    /// 随机选择一个技能
    pub fn random_skill() -> Self {
        let pool = Self::skill_pool();
        pool[rand::random::<usize>() % pool.len()].clone()
    }

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

        // 等级公式：节点数贡献 + 掌握度贡献 + 连接密度贡献
        let raw_level = (node_count / 2.0) * (0.5 + avg_mastery * 0.3 + density * 0.2);
        (raw_level.ceil() as u32).max(1).min(self.max_level)
    }

    /// 计算技能强度（用于任务执行）
    pub fn calculate_power(&self) -> f64 {
        let level = self.level as f64;

        // 基础强度
        let base_power = level * 10.0;

        // 知识贡献
        let knowledge_bonus: f64 = self.knowledge_nodes.values()
            .map(|n| n.contribution() * 10.0)
            .sum();

        base_power + knowledge_bonus
    }

    /// 整理知识点
    ///
    /// 强化已有知识的神经连接
    pub fn organize_knowledge(&mut self, knowledge_id: Uuid) -> Option<(f64, u32, u32)> {
        let node = self.knowledge_nodes.get_mut(&knowledge_id)?;
        let old_level = self.level;
        let improvement = node.organize();
        self.level = self.calculate_level();
        Some((improvement, old_level, self.level))
    }

    /// 拓展知识
    ///
    /// 学习相关联的新知识点
    pub fn extend_knowledge(&mut self, new_node: KnowledgeNode, prerequisite_id: Option<Uuid>) -> (u32, u32) {
        let old_level = self.level;

        // 如果有前置知识，建立连接
        if let Some(pre_id) = prerequisite_id {
            if let Some(pre_node) = self.knowledge_nodes.get_mut(&pre_id) {
                pre_node.connections.push(new_node.id);
            }
        }

        self.knowledge_nodes.insert(new_node.id, new_node);
        self.level = self.calculate_level();
        (old_level, self.level)
    }

    /// 补充桥梁知识
    ///
    /// 在两个已有知识点之间建立桥梁
    pub fn supplement_knowledge(
        &mut self,
        from_id: Uuid,
        to_id: Uuid,
        bridge_node: KnowledgeNode,
    ) -> Option<(u32, u32)> {
        // 检查两个知识点是否存在
        if !self.knowledge_nodes.contains_key(&from_id) ||
           !self.knowledge_nodes.contains_key(&to_id) {
            return None;
        }

        let old_level = self.level;
        let bridge_id = bridge_node.id;
        self.knowledge_nodes.insert(bridge_id, bridge_node);

        // 建立桥梁连接
        self.knowledge_nodes.get_mut(&from_id)?.connections.push(bridge_id);
        self.knowledge_nodes.get_mut(&bridge_id)?.connections.push(to_id);

        self.level = self.calculate_level();
        Some((old_level, self.level))
    }

    /// 获取可整理的知识点（组织度最低的）
    pub fn get_unorganized_knowledge(&self) -> Option<Uuid> {
        self.knowledge_nodes.values()
            .filter(|n| n.organization < 0.9)
            .min_by(|a, b| a.organization.partial_cmp(&b.organization).unwrap())
            .map(|n| n.id)
    }

    /// 获取可拓展的知识点（作为拓展的起点）
    pub fn get_extension_candidate(&self) -> Option<Uuid> {
        // 优先选择掌握度高但连接少的节点
        self.knowledge_nodes.values()
            .filter(|n| n.mastery > 0.5 && n.connections.len() < 3)
            .max_by(|a, b| {
                let score_a = a.mastery - a.connections.len() as f64 * 0.1;
                let score_b = b.mastery - b.connections.len() as f64 * 0.1;
                score_a.partial_cmp(&score_b).unwrap()
            })
            .map(|n| n.id)
    }

    /// 获取可补充桥梁的知识点对
    pub fn get_bridge_candidates(&self) -> Option<(Uuid, Uuid)> {
        // 找出没有直接连接但都掌握的知识点对
        let nodes: Vec<&KnowledgeNode> = self.knowledge_nodes.values()
            .filter(|n| n.mastery > 0.3)
            .collect();

        for i in 0..nodes.len() {
            for j in (i+1)..nodes.len() {
                let node_a = nodes[i];
                let node_b = nodes[j];

                // 检查是否有直接连接
                if !node_a.connections.contains(&node_b.id) &&
                   !node_b.connections.contains(&node_a.id) {
                    return Some((node_a.id, node_b.id));
                }
            }
        }
        None
    }

    /// 旧版兼容：升级技能（改为返回 None，升级通过知识整理）
    pub fn upgrade(&self) -> Option<(Self, f64)> {
        None // 不再支持直接购买升级
    }
}

/// 学习行为类型
#[derive(Debug, Clone)]
pub enum LearningAction {
    /// 整理已有知识
    Organize { skill_idx: usize, knowledge_id: Uuid },
    /// 拓展新知识
    Extend { skill_idx: usize, prerequisite_id: Option<Uuid> },
    /// 补充桥梁知识
    Supplement { skill_idx: usize, from_id: Uuid, to_id: Uuid },
}

/// 学习行为系统
pub struct LearningSystem;

impl LearningSystem {
    /// 根据技能生成相关的新知识点
    pub fn generate_extension_knowledge(skill_name: &str) -> Vec<KnowledgeNode> {
        match skill_name {
            "火焰喷射" => vec![
                KnowledgeNode::new("火焰形态控制".to_string(), "控制火焰的大小和形状".to_string(), KnowledgeNodeType::Technique),
                KnowledgeNode::new("燃烧三要素".to_string(), "理解燃料、氧气、温度的关系".to_string(), KnowledgeNodeType::Theory),
                KnowledgeNode::new("连续喷射技巧".to_string(), "维持火焰持续输出的方法".to_string(), KnowledgeNodeType::Extension),
                KnowledgeNode::new("火焰温度控制".to_string(), "调节火焰温度的技巧".to_string(), KnowledgeNodeType::Technique),
            ],
            "冰冻护盾" => vec![
                KnowledgeNode::new("冰晶结构".to_string(), "理解冰晶的分子排列".to_string(), KnowledgeNodeType::Theory),
                KnowledgeNode::new("多层护盾".to_string(), "构建多层防御结构".to_string(), KnowledgeNodeType::Extension),
                KnowledgeNode::new("快速凝聚".to_string(), "加速冰晶凝聚的技巧".to_string(), KnowledgeNodeType::Technique),
            ],
            "闪电链" => vec![
                KnowledgeNode::new("电压控制".to_string(), "调节闪电的电压强度".to_string(), KnowledgeNodeType::Technique),
                KnowledgeNode::new("目标选择".to_string(), "控制闪电跳跃的目标".to_string(), KnowledgeNodeType::Extension),
                KnowledgeNode::new("电磁场理论".to_string(), "理解电磁场的传播".to_string(), KnowledgeNodeType::Theory),
            ],
            "风刃" => vec![
                KnowledgeNode::new("压缩空气".to_string(), "将空气压缩成锋利形态".to_string(), KnowledgeNodeType::Technique),
                KnowledgeNode::new("远距离投射".to_string(), "增加风刃射程的技巧".to_string(), KnowledgeNodeType::Extension),
            ],
            "岩石护甲" => vec![
                KnowledgeNode::new("矿物识别".to_string(), "识别不同矿物的硬度".to_string(), KnowledgeNodeType::Theory),
                KnowledgeNode::new("快速召唤".to_string(), "缩短护甲召唤时间".to_string(), KnowledgeNodeType::Technique),
            ],
            "自然治愈" => vec![
                KnowledgeNode::new("生命力引导".to_string(), "引导自然生命力的方法".to_string(), KnowledgeNodeType::Technique),
                KnowledgeNode::new("范围治疗".to_string(), "治疗多个目标的技术".to_string(), KnowledgeNodeType::Extension),
                KnowledgeNode::new("生命能量理论".to_string(), "理解生命能量的本质".to_string(), KnowledgeNodeType::Theory),
            ],
            _ => vec![
                KnowledgeNode::new("基础应用".to_string(), "技能的基本应用方法".to_string(), KnowledgeNodeType::Technique),
            ],
        }
    }

    /// 生成桥梁知识点
    pub fn generate_bridge_knowledge(from_name: &str, to_name: &str) -> KnowledgeNode {
        KnowledgeNode::new(
            format!("{}与{}的关联", from_name, to_name),
            format!("理解{}和{}之间的内在联系", from_name, to_name),
            KnowledgeNodeType::Theory,
        )
    }

    /// 执行整理知识行为
    pub fn organize_knowledge(
        skills: &mut Vec<Skill>,
        skill_idx: usize,
        knowledge_id: Uuid,
        gu_name: &str,
    ) -> ActionResult {
        let skill = match skills.get_mut(skill_idx) {
            Some(s) => s,
            None => return ActionResult {
                success: false,
                transaction: None,
                message: "技能不存在".to_string(),
            },
        };

        let node_name = skill.knowledge_nodes.get(&knowledge_id)
            .map(|n| n.name.clone())
            .unwrap_or_default();

        match skill.organize_knowledge(knowledge_id) {
            Some((improvement, old_level, new_level)) => {
                if new_level > old_level {
                    ActionResult {
                        success: true,
                        transaction: None,
                        message: format!(
                            "「{}」整理「{}」，组织度+{:.0}%，技能等级 {}→{}",
                            gu_name, node_name, improvement * 100.0, old_level, new_level
                        ),
                    }
                } else {
                    ActionResult {
                        success: true,
                        transaction: None,
                        message: format!(
                            "「{}」整理「{}」，组织度+{:.0}%",
                            gu_name, node_name, improvement * 100.0
                        ),
                    }
                }
            }
            None => ActionResult {
                success: false,
                transaction: None,
                message: "知识点不存在".to_string(),
            },
        }
    }

    /// 执行拓展知识行为
    pub fn extend_knowledge(
        skills: &mut Vec<Skill>,
        skill_idx: usize,
        prerequisite_id: Option<Uuid>,
        gu_name: &str,
    ) -> ActionResult {
        let skill = match skills.get_mut(skill_idx) {
            Some(s) => s,
            None => return ActionResult {
                success: false,
                transaction: None,
                message: "技能不存在".to_string(),
            },
        };

        let skill_name = skill.name.clone();

        // 生成新的知识点
        let available = Self::generate_extension_knowledge(&skill_name);
        if available.is_empty() {
            return ActionResult {
                success: false,
                transaction: None,
                message: "没有可拓展的知识".to_string(),
            };
        }

        // 选择一个未学习的知识点
        let existing_names: Vec<&str> = skill.knowledge_nodes.values()
            .map(|n| n.name.as_str())
            .collect();
        let new_node = available.into_iter()
            .find(|n| !existing_names.contains(&n.name.as_str()))
            .unwrap_or_else(|| KnowledgeNode::new(
                format!("{}进阶", skill_name),
                "进一步的技能理解".to_string(),
                KnowledgeNodeType::Extension,
            ));

        let node_name = new_node.name.clone();
        let (old_level, new_level) = skill.extend_knowledge(new_node, prerequisite_id);

        ActionResult {
            success: true,
            transaction: None,
            message: format!(
                "「{}」学习「{}」，技能等级 {}→{}",
                gu_name, node_name, old_level, new_level
            ),
        }
    }

    /// 执行补充桥梁知识行为
    pub fn supplement_knowledge(
        skills: &mut Vec<Skill>,
        skill_idx: usize,
        from_id: Uuid,
        to_id: Uuid,
        gu_name: &str,
    ) -> ActionResult {
        let skill = match skills.get_mut(skill_idx) {
            Some(s) => s,
            None => return ActionResult {
                success: false,
                transaction: None,
                message: "技能不存在".to_string(),
            },
        };

        let from_name = skill.knowledge_nodes.get(&from_id)
            .map(|n| n.name.clone())
            .unwrap_or_default();
        let to_name = skill.knowledge_nodes.get(&to_id)
            .map(|n| n.name.clone())
            .unwrap_or_default();

        let bridge = Self::generate_bridge_knowledge(&from_name, &to_name);

        match skill.supplement_knowledge(from_id, to_id, bridge) {
            Some((old_level, new_level)) => ActionResult {
                success: true,
                transaction: None,
                message: format!(
                    "「{}」发现「{}」与「{}」的关联，技能等级 {}→{}",
                    gu_name, from_name, to_name, old_level, new_level
                ),
            },
            None => ActionResult {
                success: false,
                transaction: None,
                message: "无法建立桥梁知识".to_string(),
            },
        }
    }

    /// 根据 Cognitive 神经元状态决定学习行为
    pub fn decide_learning_action(skills: &[Skill], cognitive_state: f64) -> Option<LearningAction> {
        if skills.is_empty() {
            return None;
        }

        // 高认知需求 → 整理已有知识
        if cognitive_state > 0.6 {
            // 找出有未整理知识的技能
            for (idx, skill) in skills.iter().enumerate() {
                if let Some(knowledge_id) = skill.get_unorganized_knowledge() {
                    return Some(LearningAction::Organize { skill_idx: idx, knowledge_id });
                }
            }
        }

        // 中等认知需求 → 拓展新知识
        if cognitive_state > 0.3 {
            for (idx, skill) in skills.iter().enumerate() {
                if skill.knowledge_nodes.len() < 5 { // 限制知识点数量
                    if let Some(prereq_id) = skill.get_extension_candidate() {
                        return Some(LearningAction::Extend { skill_idx: idx, prerequisite_id: Some(prereq_id) });
                    }
                    return Some(LearningAction::Extend { skill_idx: idx, prerequisite_id: None });
                }
            }
        }

        // 低认知需求 → 补充桥梁
        for (idx, skill) in skills.iter().enumerate() {
            if skill.knowledge_nodes.len() >= 2 {
                if let Some((from_id, to_id)) = skill.get_bridge_candidates() {
                    return Some(LearningAction::Supplement { skill_idx: idx, from_id, to_id });
                }
            }
        }

        None
    }
}

/// 蛊虫行为结果
#[derive(Debug, Clone)]
pub struct ActionResult {
    /// 是否成功
    pub success: bool,
    /// 产生的交易事件
    pub transaction: Option<TransactionData>,
    /// 日志消息
    pub message: String,
}

/// 蛊虫行为系统
pub struct GuBehavior;

impl GuBehavior {
    /// 购买资源
    pub fn buy_resource(
        wallet: &mut GuWallet,
        gu_name: &str,
        resource: &Resource,
    ) -> ActionResult {
        match wallet.withdraw(
            resource.price,
            "资源交易",
            &format!("购买「{}」消耗金币", resource.name),
            gu_name,
        ) {
            Some(tx) => ActionResult {
                success: true,
                transaction: Some(tx),
                message: format!("购买「{}」消耗 {} 金币", resource.name, resource.price),
            },
            None => ActionResult {
                success: false,
                transaction: None,
                message: format!("余额不足，无法购买「{}」", resource.name),
            },
        }
    }

    /// 学习行为（整理/拓展/补充知识）
    ///
    /// 替代旧的 upgrade_skill 和 learn_knowledge
    /// 技能升级现在通过知识网络自然实现，而非购买
    pub fn learn(
        skills: &mut Vec<Skill>,
        gu_name: &str,
        action: LearningAction,
    ) -> ActionResult {
        match action {
            LearningAction::Organize { skill_idx, knowledge_id } => {
                LearningSystem::organize_knowledge(skills, skill_idx, knowledge_id, gu_name)
            }
            LearningAction::Extend { skill_idx, prerequisite_id } => {
                LearningSystem::extend_knowledge(skills, skill_idx, prerequisite_id, gu_name)
            }
            LearningAction::Supplement { skill_idx, from_id, to_id } => {
                LearningSystem::supplement_knowledge(skills, skill_idx, from_id, to_id, gu_name)
            }
        }
    }

    /// 执行任务（更新版：使用技能强度计算成功率）
    pub fn execute_task_with_skill(
        wallet: &mut GuWallet,
        gu_name: &str,
        task: &Task,
        skills: &[Skill],
    ) -> ActionResult {
        // 计算相关技能的总强度
        let relevant_power: f64 = skills.iter()
            .filter(|s| task.required_skills.contains(&s.name))
            .map(|s| s.calculate_power())
            .sum();

        // 任务难度 vs 技能强度
        let success_rate = if relevant_power > 0.0 {
            1.0 - (task.difficulty * 10.0 / relevant_power).min(0.9)
        } else {
            0.1 // 无相关技能时 10% 成功率
        };

        if rand::random::<f64>() < success_rate {
            let tx = wallet.deposit(
                task.reward,
                "任务奖励",
                &format!("完成任务「{}」，获得金币奖励", task.name),
                gu_name,
            );

            ActionResult {
                success: true,
                transaction: Some(tx),
                message: format!(
                    "完成任务「{}」获得 {} 金币（技能强度: {:.1}，成功率: {:.0}%）",
                    task.name, task.reward, relevant_power, success_rate * 100.0
                ),
            }
        } else {
            ActionResult {
                success: false,
                transaction: None,
                message: format!(
                    "任务「{}」失败（技能强度: {:.1}，成功率: {:.0}%）",
                    task.name, relevant_power, success_rate * 100.0
                ),
            }
        }
    }

    /// 转账
    pub fn transfer(
        from_wallet: &mut GuWallet,
        from_name: &str,
        to_wallet: &mut GuWallet,
        to_name: &str,
        amount: f64,
    ) -> ActionResult {
        if from_wallet.balance < amount {
            return ActionResult {
                success: false,
                transaction: None,
                message: format!("余额不足，无法转账 {}", amount),
            };
        }

        from_wallet.balance -= amount;
        from_wallet.total_expense += amount;
        to_wallet.balance += amount;
        to_wallet.total_income += amount;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let tx = TransactionData {
            id: format!("tx_{:08x}_{}", timestamp, from_wallet.gu_id),
            timestamp,
            from_id: from_wallet.gu_id.to_string(),
            from_name: from_name.to_string(),
            from_balance: from_wallet.balance,
            to_id: to_wallet.gu_id.to_string(),
            to_name: to_name.to_string(),
            to_balance: to_wallet.balance,
            amount: -amount,
            kind: TransactionKind::Transfer,
            reason: "转账支付".to_string(),
            detail: format!("向{}转账{}金币", to_name, amount as i64),
        };

        from_wallet.transaction_ids.push(tx.id.clone());
        to_wallet.transaction_ids.push(tx.id.clone());

        ActionResult {
            success: true,
            transaction: Some(tx),
            message: format!("向{}转账 {} 金币", to_name, amount),
        }
    }
}
