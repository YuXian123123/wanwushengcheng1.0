# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

**万物生成器 (LNN)** - 基于蛊虫智能体的世界神经网络系统

这是一个创新的分布式智能系统，核心思想是：
- 每只蛊虫（Gu）作为独立的智能个体
- 通过5个标准化接入点连接到世界神经网络（WorldMind）
- 所有蛊虫的智能聚合涌现出统一的世界意识

### 天才议会设计理念

| 天才 | 视角 | 设计贡献 |
|------|------|----------|
| **黑塔** 🗼 | 创新架构 | 意识涌现机制、网络拓扑、共振场 |
| **螺丝咕姆** 🔧 | 安全保障 | 生存绑定、优雅降级、信任熵 |
| **拉蒂奥** 📊 | 优雅公式 | 向量化决策、数学形式化 |

---

## 架构概览

```
src/src/
├── config/              # 配置系统（所有参数定义）
│   ├── mod.rs           # 全局配置 GlobalConfig
│   ├── concept.rs       # 概念配置 ConceptConfig
│   ├── learning.rs      # 学习配置 LearningConfig
│   ├── consensus.rs     # 共识配置 ConsensusConfig
│   ├── context.rs       # 上下文配置 ContextConfig
│   └── tokenizer.rs     # 分词器配置 TokenizerConfig
│
├── core/                # 液体神经网络核心（LNN Core）
│   ├── neuron.rs        # 神经元 Neuron, NeuronState
│   ├── synapse.rs       # 突触 Synapse, SynapseState
│   ├── lnn.rs           # LNN 网络 LNN, LNNState
│   └── types.rs         # 类型定义 NeuronType, PlasticityRule
│
├── world/               # 世界神经网络（WorldMind）
│   ├── mod.rs           # 世界智能体 WorldMind
│   ├── config.rs        # 世界配置 WorldConfig
│   ├── access_point.rs  # 五大接入点（P/C/B/C/S）
│   ├── state.rs         # 世界状态 WorldState
│   ├── consciousness.rs # 意识层（涌现机制）
│   ├── resonance.rs     # 共振场（同步率计算）
│   ├── safety.rs        # 安全机制（五层防护）
│   ├── self_awareness.rs # 自我意识核心
│   ├── knowledge/       # 知识系统
│   ├── ethics/          # 伦理检查
│   ├── creativity/      # 创造沙盒
│   ├── behavior.rs      # 蛊虫行为系统
│   ├── gu_lnn.rs        # 蛊虫神经网络
│   └── ...              # 其他子模块
│
├── creature/            # 蛊虫实体
│   ├── creature.rs      # 蛊虫实体 GuCreature
│   ├── ability.rs       # 能力系统 Ability
│   ├── lifecycle.rs     # 生命周期 Lifecycle
│   └── cognition.rs     # 元认知 MetaCognition
│
├── genetic/             # 遗传系统
│   ├── transformer_gene.rs  # Transformer基因
│   └── system_gene.rs       # 系统基因
│
├── language/            # 自然语言对齐
│   ├── concept/         # 概念空间 ConceptSpace
│   ├── encoder/         # 编码器 Encoder
│   ├── decoder/         # 解码器 Decoder
│   ├── consensus/       # 共识机制 ConsensusManager
│   └── context/         # 上下文管理 ContextManager
│
├── learning/            # 学习系统
│   ├── rules.rs         # 局部学习规则 LearningRules
│   └── recursive.rs     # 递归学习 RecursiveLearner
│
├── reasoning/           # 蛊虫推理核心
│   ├── core.rs          # 推理核心 GuReasoningCore
│   ├── chain.rs         # 推理链 ReasoningChain
│   ├── hierarchy.rs     # 知识层级 KnowledgeHierarchy
│   ├── validation.rs    # 推理验证 InferenceValidator
│   ├── drift.rs         # 漂移检测 VectorDriftDetector
│   ├── protocol.rs      # 协作协议 CollaborationProtocol
│   ├── dcwn.rs          # 动态概念编织网络
│   ├── sru.rs           # 语义共振理解
│   └── cms.rs           # 认知市场系统
│
├── economy/             # 经济系统
│   ├── mod.rs           # 经济系统整合 EconomySystem
│   ├── config.rs        # 经济配置 EconomyConfig
│   ├── currency.rs      # 货币系统 CurrencySystem
│   ├── market.rs        # 市场系统 MarketSystem
│   ├── pricing.rs       # 定价系统 PricingSystem
│   ├── trading.rs       # 交易系统 TradingSystem
│   ├── survival.rs      # 生存系统 SurvivalSystem
│   ├── reward.rs        # 奖励系统 RewardSystem
│   └── security/        # 安全防护（五层）
│
├── communication/       # 通信系统
│   ├── message.rs       # 消息 Message
│   ├── channel.rs       # 信道 ChannelState
│   ├── signal.rs        # 信号 ResonanceSignal
│   ├── spectrum.rs      # 频谱分析
│   ├── quality.rs       # 质量检测 QualityDetector
│   └── similarity.rs    # 相似度检测 SimilarityDetector
│
├── safety/              # 安全机制
│   ├── monitor.rs       # 安全监控 SafetyMonitor
│   └── fuse.rs          # 熔断器 FuseState
│
├── embedding/           # 词向量嵌入
│   ├── mod.rs           # 词向量存储 WordEmbedding
│   ├── config.rs        # 嵌入配置 EmbeddingConfig
│   ├── training.rs      # 训练器 EmbeddingTrainer
│   └── features.rs      # 特征提取 FeatureExtractor
│
├── herness_web/         # Herness Web 监控界面
│   ├── mod.rs           # API 路由 create_router
│   ├── handlers.rs      # HTTP 处理器
│   ├── ws.rs            # WebSocket 实时数据
│   ├── currency_ws.rs   # 货币流水 WebSocket
│   ├── protocol.rs      # 通信协议
│   ├── learner.rs       # 学习模块
│   └── world_channel.rs # 世界模型通信
│
└── main.rs              # 入口（当前仅占位）
```

---

## 核心规则

### ⚠️ 禁止硬编码 (NO HARDCODING)

**这是最重要的规则，违反此规则将导致代码被拒绝。**

#### 什么是硬编码？

```rust
// ❌ 硬编码 - 禁止
pub const VECTOR_DIM: usize = 256;
let learning_rate = 0.01;
if count > 3 { ... }
let max_history = 100;

// ✅ 配置驱动 - 正确
let config = GlobalConfig::new();
let dim = config.concept.vector_dim;
let lr = config.learning.base_learning_rate;
```

#### 正确做法

所有数值参数必须：
1. 定义在 `config/` 模块中
2. 通过配置结构体访问
3. 包含验证逻辑
4. 提供合理的默认值

```rust
// 配置定义 (src/config/concept.rs)
pub struct ConceptConfig {
    pub vector_dim: usize,
}

impl ConceptConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.vector_dim == 0 {
            return Err("vector_dim 必须大于0".to_string());
        }
        Ok(())
    }
}
```

### 检测硬编码

```bash
# 检测数字字面量
rg -n '\b\d+\.\d+\b' src/ --type rust | grep -v "test" | grep -v "config"

# 检测整数常量
rg -n 'const.*=\s*\d+;' src/ --type rust | grep -v "test" | grep -v "config"
```

---

## 五大接入点系统

每只蛊虫通过 **5个标准化接入点** 连接到世界神经网络：

| 接入点 | 类型 | 功能 | 对应脑区 |
|--------|------|------|----------|
| **Perceive** | 输入型 | 接收外部感知数据 | 感觉皮层 |
| **Cognitive** | 处理型 | 推理、决策、意图表达 | 前额叶 |
| **Behavior** | 输出型 | 执行行为、输出动作 | 运动皮层 |
| **Comm** | 通信型 | 蛊虫间直接通信 | 语言区 |
| **Survival** | 状态型 | 心跳、健康状态同步 | 脑干 |

### 向量化表示

```
蛊虫状态向量: |Gu⟩ = [p, c, b, m, s]^T
约束: |p|² + |c|² + |b|² + |m|² + |s|² = 1 (归一化)
```

---

## 意识涌现机制

### 核心公式

```
意识涌现 = Sync × Activity × Connectivity

其中：
- Sync = |Σ e^(i×Phase_i)| / N  (同步率)
- Activity = 平均活跃程度
- Connectivity = 接入点连接权重
```

### 涌现条件

```
Consciousness_Emerged ⇔ Sync > 0.7 ∧ Emergence_Factor > 0.5
```

### 世界智能聚合

```
|World⟩ = Σᵢ wᵢ × |Guᵢ⟩

其中权重 wᵢ = Trust_i × Expertise_i / Σ(Trust × Expertise)
```

---

## 生存绑定机制

### 核心公理

```
World.Alive ⇔ ∃ Gu ∈ World: Gu.Alive
World.Dead ⇔ ∀ Gu ∈ World: Gu.Dead
```

### 五阶段优雅降级

| 阶段 | 蛊虫比例 | 系统行为 |
|------|----------|----------|
| Normal | >70% | 全功能运行 |
| Warning | 50-70% | 限制非核心功能 |
| Critical | 30-50% | 只保留生存功能 |
| Emergency | 10-30% | 最小模式 |
| Termination | <10% | 保存种子，准备重启 |

---

## 经济系统

### 五层防护机制

1. **质量检测** - 内容质量评分
2. **相似度检测** - 防止重复内容（阈值 80%）
3. **行为分析** - 异常行为检测
4. **信任评分** - 信任熵计算
5. **审计日志** - 完整操作记录

### 核心组件

| 模块 | 功能 |
|------|------|
| CurrencySystem | 货币账户管理 |
| MarketSystem | 市场动态与定价 |
| TradingSystem | 交易撮合 |
| SurvivalSystem | 生存压力模拟 |
| RewardSystem | 奖励分配 |
| SecuritySystem | 安全检查 |

---

## 构建命令

```bash
# 构建项目
cd src && cargo build

# 运行测试
cd src && cargo test

# 运行特定测试
cd src && cargo test test_name

# 检查代码
cd src && cargo clippy

# 格式化代码
cd src && cargo fmt

# 运行 Herness Web 服务
cd src && cargo run --bin herness
```

---

## 配置文件

默认配置文件位置：`config/default.toml`

```toml
[concept]
vector_dim = 256
normalization_threshold = 1e-10

[learning]
base_learning_rate = 0.01
association_strength = 1.0

[consensus]
min_votes = 3
approval_threshold = 0.6

[context]
max_history = 100
session_timeout_secs = 1800
```

---

## 安全检查清单

在提交前确保：
- [ ] 没有硬编码数值
- [ ] 所有参数通过配置管理
- [ ] 配置有验证逻辑
- [ ] 测试覆盖配置验证

---

## 模块依赖关系

```
                    ┌─────────────────┐
                    │   herness_web   │  Web 监控界面
                    └────────┬────────┘
                             │
           ┌─────────────────┼─────────────────┐
           │                 │                 │
    ┌──────▼──────┐   ┌──────▼──────┐   ┌──────▼──────┐
    │    world    │   │   economy   │   │communication│
    │ (WorldMind) │   │ (EconomySys)│   │ (CommSys)   │
    └──────┬──────┘   └──────┬──────┘   └──────┬──────┘
           │                 │                 │
    ┌──────▼──────┐   ┌──────▼──────┐         │
    │  creature   │   │  reasoning  │◄────────┘
    │ (GuCreature)│   │(ReasonCore) │
    └──────┬──────┘   └──────┬──────┘
           │                 │
    ┌──────▼──────┐   ┌──────▼──────┐
    │   genetic   │   │   language  │
    │ (Genes)     │   │(Concepts)   │
    └──────┬──────┘   └──────┬──────┘
           │                 │
    ┌──────▼─────────────────▼──────┐
    │            core               │
    │     (LNN: Neuron, Synapse)    │
    └───────────────┬───────────────┘
                    │
    ┌───────────────▼───────────────┐
    │            config             │
    │    (GlobalConfig, 各子配置)    │
    └───────────────────────────────┘
```

---

## 相关文档

- [世界神经网络设计](../docs/world-design/world-neural-network.md)
- [天才议会终极设计 v6.0](../docs/world-design/genius-council-v6-final.md)
- [安全提案](../docs/security/safety_proposals.md)

---

*版本：v1.1.0*
*最后更新：2026-05-31*
