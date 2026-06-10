# 知识共享与共识机制设计

## 问题分析

### 当前设计的问题

1. **知识分散** - 每个蛊虫独立学习，知识存到私有目录
2. **重复学习** - 同样的知识被多个蛊虫重复学习
3. **版本混乱** - 同一知识有多个版本，无法确定哪个最好
4. **无法共享** - 私有技能无法被其他蛊虫使用
5. **学习质量低** - 没有验证机制，错误知识也会被存储

### 目标设计

```
┌─────────────────────────────────────────────────────────────────────┐
│                      知识共享与共识流程                               │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  蛊虫 A ──┐                                                         │
│           │                                                         │
│  蛊虫 B ──┼──→ 候选知识池 ──→ 共识验证 ──→ 共享知识库               │
│           │       │              │           │                       │
│  蛊虫 C ──┘       │              │           │                       │
│                   │              │           │                       │
│                   ▼              ▼           ▼                       │
│              ┌─────────┐   ┌─────────┐   ┌─────────┐                │
│              │ 候选版本 │   │ 投票裁决 │   │ 最终版本 │                │
│              │ 暂存区   │   │ 三天才   │   │ 入库     │                │
│              └─────────┘   └─────────┘   └─────────┘                │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 新架构设计

### 1. 目录结构

```
knowledge/
├── skills/
│   ├── shared/              # 共享知识库（共识通过）
│   │   ├── html-basics.html # 统一版本
│   │   └── css-layout.html
│   └── candidates/          # 候选知识池（待验证）
│       ├── html-basics/
│       │   ├── gu_a_v1.html # 蛊虫 A 的候选版本
│       │   ├── gu_b_v1.html # 蛊虫 B 的候选版本
│       │   └── meta.json    # 候选元数据（投票状态）
│       └── css-layout/
│           └── ...
├── gus/                     # 蛊虫索引（只存引用）
│   ├── gu_a.html           # 引用 shared/ 中的技能
│   └── gu_b.html
└── votes/                   # 投票记录
    └── html-basics.json     # 知识投票历史
```

### 2. 知识学习流程

```rust
/// 新的知识学习流程
pub fn learn_knowledge(&mut self, content: &str) -> LearningResult {
    // 1. 认知素分解
    let particles = self.parse_cognis(content);
    let topic = particles.main_topic.clone();

    // 2. 检查共享知识库是否已存在
    if self.shared_knowledge.exists(&topic) {
        // 已存在 → 增强学习（更新引用计数）
        return self.reinforce_existing(&topic);
    }

    // 3. 检查候选池是否已有其他蛊虫提交
    if self.candidates.has_candidates(&topic) {
        // 有候选 → 加入竞争
        return self.compete_for_topic(&topic, particles);
    }

    // 4. 首次学习 → 提交候选版本
    self.submit_candidate(&topic, particles)
}
```

### 3. 共识验证机制

```rust
/// 知识共识验证
pub struct KnowledgeConsensus {
    /// 候选版本
    candidates: Vec<CandidateKnowledge>,
    /// 投票记录
    votes: HashMap<Uuid, Vote>,
    /// 共识阈值（需要多少蛊虫同意）
    threshold: usize,
}

impl KnowledgeConsensus {
    /// 提交候选版本
    pub fn submit(&mut self, gu_id: Uuid, knowledge: SkillDocument) {
        self.candidates.push(CandidateKnowledge {
            submitted_by: gu_id,
            knowledge,
            score: 0.0,
            votes: 0,
        });
    }

    /// 投票（三天才裁决）
    pub fn vote(&mut self, voter_id: Uuid, candidate_idx: usize, vote: Vote) {
        // 记录投票
        self.votes.insert(voter_id, vote.clone());

        // 更新候选分数
        if vote.approve {
            self.candidates[candidate_idx].votes += 1;
            self.candidates[candidate_idx].score += vote.confidence;
        }

        // 检查是否达到共识
        if self.candidates[candidate_idx].votes >= self.threshold {
            self.promote_to_shared(candidate_idx);
        }
    }

    /// 提升到共享知识库
    fn promote_to_shared(&mut self, candidate_idx: usize) {
        let winner = self.candidates.remove(candidate_idx);
        // 存储到 shared/
        // 清理候选池
        // 通知所有蛊虫
    }
}
```

### 4. 投票策略（三天才裁决）

```rust
/// 对候选知识进行投票
pub fn evaluate_candidate(candidate: &SkillDocument) -> Vote {
    // 黑塔：创新性评估
    let innovation_score = Self::evaluate_innovation(candidate);

    // 螺丝咕姆：准确性评估
    let accuracy_score = Self::evaluate_accuracy(candidate);

    // 拉蒂奥：优雅度评估
    let elegance_score = Self::evaluate_elegance(candidate);

    // 综合裁决
    let total = innovation_score * 0.3
              + accuracy_score * 0.5   // 准确性权重最高
              + elegance_score * 0.2;

    Vote {
        approve: total > 0.6,
        confidence: total,
        reason: format!("创新:{:.2} 准确:{:.2} 优雅:{:.2}",
                       innovation_score, accuracy_score, elegance_score),
    }
}
```

---

## 5. 实现方案

### Phase 1: 候选知识池

```rust
// src/world/knowledge_pool.rs

/// 候选知识池
pub struct KnowledgePool {
    /// 候选知识（按主题分组）
    candidates: HashMap<String, Vec<CandidateKnowledge>>,
    /// 共识阈值
    consensus_threshold: usize,
    /// 投票超时（秒）
    vote_timeout: u64,
}

/// 候选知识
pub struct CandidateKnowledge {
    /// 提交者
    submitted_by: Uuid,
    /// 提交时间
    submitted_at: u64,
    /// 知识内容
    knowledge: SkillDocument,
    /// 投票记录
    votes: Vec<VoteRecord>,
    /// 状态
    status: CandidateStatus,
}

#[derive(Debug, Clone)]
pub enum CandidateStatus {
    /// 等待投票
    Pending,
    /// 达成共识，待提升
    ConsensusReached,
    /// 已提升到共享库
    Promoted,
    /// 被拒绝
    Rejected,
}
```

### Phase 2: 集成到学习流程

```rust
// 修改 receive_knowledge_file

pub fn receive_knowledge_file(&mut self, file_event: &KnowledgeFileEvent) -> FileDigestResult {
    // ... 解析内容 ...

    // 检查共享知识库
    if self.knowledge_pool.shared_exists(&topic) {
        // 已存在 → 增加引用计数，强化学习
        self.knowledge_pool.increment_ref(&topic);
        self.reinforce_learning(gu_id, &topic);
        return FileDigestResult { success: true, ... };
    }

    // 提交候选版本
    self.knowledge_pool.submit_candidate(gu_id, skill_doc);

    // 触发其他蛊虫投票
    self.trigger_voting(&topic);

    // 检查是否达成共识
    if self.knowledge_pool.check_consensus(&topic) {
        // 提升到共享库
        self.knowledge_pool.promote_to_shared(&topic);
    }

    FileDigestResult { success: true, ... }
}
```

### Phase 3: 蛊虫间通信

```rust
// 蛊虫收到投票请求
pub fn on_vote_request(&mut self, topic: &str, candidate: &SkillDocument) {
    // 三天才评估
    let vote = self.trinity_evaluate(candidate);

    // 提交投票
    self.world.submit_vote(self.id, topic, vote);
}
```

---

## 6. 效果对比

### 改进前

| 指标 | 值 |
|------|-----|
| 26 个文件学习 | 产生 26 个独立版本 |
| 存储空间 | 26 × 文件大小 |
| 知识一致性 | ❌ 每个蛊虫版本不同 |
| 知识共享 | ❌ 私有目录无法共享 |
| 验证机制 | ❌ 无 |

### 改进后

| 指标 | 值 |
|------|-----|
| 26 个文件学习 | 产生 1 个统一版本（共识后） |
| 存储空间 | 1 × 文件大小 |
| 知识一致性 | ✅ 所有蛊虫使用同一版本 |
| 知识共享 | ✅ 共享目录，所有蛊虫可用 |
| 验证机制 | ✅ 三天才共识验证 |

---

## 7. 下一步

1. **创建 `KnowledgePool` 模块** - 候选知识池管理
2. **改造 `receive_knowledge_file`** - 集成共识流程
3. **实现投票机制** - 蛊虫间通信
4. **测试验证** - 多蛊虫竞争学习

---

*设计版本: v2.0*
*设计日期: 2026-05-31*
