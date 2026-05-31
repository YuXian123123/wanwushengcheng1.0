# 硬编码检测报告

## 检测时间
2026-05-31

## 检测方法
使用 ripgrep 搜索以下模式：
- 浮点数: `\b\d+\.\d+\b`
- 整数常量: `const.*=\s*\d+;`
- 魔术数字比较: `[><=]=?\s*[1-9][0-9]*\s*\{`

---

## 统计摘要

| 类型 | 数量 | 严重程度 |
|------|------|----------|
| 浮点数硬编码 | 1274 | ⚠️ 中等 |
| 整数常量 | 1 | 🔴 高 |
| 魔术数字比较 | 15 | 🔴 高 |

---

## 🔴 高优先级问题

### 1. 整数常量 (必须修复)

**文件**: `src/language/concept/types.rs:28`
```rust
pub const VECTOR_DIM: usize = 256;  // ❌ 硬编码
```

**修复方案**: 已标记为 deprecated，使用 `ConceptConfig::vector_dim()`

---

### 2. 魔术数字比较 (必须修复)

| 文件 | 行号 | 代码 | 问题 |
|------|------|------|------|
| `world/state.rs` | 152 | `> 10000` | 世界记忆大小限制 |
| `world/monitor.rs` | 155 | `< 5` | 最小种群警告阈值 |
| `world/ethics/mod.rs` | 529 | `< 9` | 伦理规则数量 |
| `world/knowledge/validation.rs` | 241 | `< 5` | 知识内容最小长度 |
| `reasoning/protocol.rs` | 208 | `>= 3` | 异常检测阈值 |
| `economy/security/behavior.rs` | 67,76 | `> 100` | 行为历史大小 |
| `economy/security/quality.rs` | 84-88 | `< 10/50/100` | 内容质量阈值 |

---

### 3. 配置默认值硬编码

**文件**: `src/core/types.rs`

```rust
// 学习率配置
impl Default for LearningRateConfig {
    fn default() -> Self {
        Self {
            initial_rate: 0.01,    // ❌ 硬编码
            min_rate: 0.001,       // ❌ 硬编码
            max_rate: 0.1,         // ❌ 硬编码
            ...
        }
    }
}

// 时间步配置
impl Default for TimeConfig {
    fn default() -> Self {
        Self {
            dt: 0.01,              // ❌ 硬编码
            tau_range: (0.1, 10.0), // ❌ 硬编码
        }
    }
}

// 熔断配置
impl Default for FuseConfig {
    fn default() -> Self {
        Self {
            fuse_threshold: 0.3,   // ❌ 硬编码
        }
    }
}
```

---

### 4. 通信频段硬编码

**文件**: `src/communication/spectrum.rs:92-97`

```rust
bands.insert(FrequencyBand::Low, BandConfig::new(1000, 5.0, 1));      // ❌
bands.insert(FrequencyBand::MediumLow, BandConfig::new(500, 2.0, 2)); // ❌
bands.insert(FrequencyBand::Medium, BandConfig::new(200, 1.0, 3));    // ❌
bands.insert(FrequencyBand::MediumHigh, BandConfig::new(100, 0.5, 0)); // ❌
bands.insert(FrequencyBand::High, BandConfig::new(300, 1.5, 2));      // ❌
bands.insert(FrequencyBand::UltraHigh, BandConfig::new(150, 3.0, 2)); // ❌
```

---

## ⚠️ 中等优先级问题

### 浮点数硬编码分布

| 模块 | 数量 | 主要类型 |
|------|------|----------|
| `embedding/` | ~150 | 训练参数、向量计算 |
| `core/lnn.rs` | ~80 | 神经网络参数 |
| `communication/` | ~60 | 信号强度、衰减率 |
| `world/` | ~200 | 阈值、权重 |
| `economy/` | ~100 | 奖励、惩罚系数 |

---

## ✅ 已正确使用配置的示例

### 1. 世界配置 (`world/config.rs`)

```rust
pub struct WorldConfig {
    pub survival: SurvivalConfig,
    pub access_points_per_gu: usize,
    pub heartbeat_interval: u64,
    // ...
}

impl WorldConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.min_population == 0 {
            return Err("min_population must be > 0".to_string());
        }
        Ok(())
    }
}
```

### 2. 概念配置 (`config/concept.rs`)

```rust
pub struct ConceptConfig {
    pub vector_dim: usize,
    pub normalization_threshold: f64,
}

impl ConceptConfig {
    pub fn vector_dim(&self) -> usize {
        self.vector_dim
    }
}
```

---

## 修复建议

### 优先级 1: 立即修复

1. **`VECTOR_DIM` 常量** - 已标记 deprecated，需完成迁移
2. **魔术数字比较** - 移至配置文件

```rust
// 修复示例: world/state.rs
// 之前:
if new_state.world_memory.len() > 10000 {

// 之后:
if new_state.world_memory.len() > config.max_memory_size {
```

### 优先级 2: 计划修复

1. **配置默认值** - 移至 `config/default.toml`
2. **通信频段** - 创建 `SpectrumConfig`

### 优先级 3: 渐进修复

1. **训练参数** - 创建 `TrainingConfig`
2. **阈值参数** - 统一到配置系统

---

## 配置文件建议

```toml
# config/default.toml

[world]
max_memory_size = 10000
min_population_warning = 5

[world.ethics]
max_rules = 10

[world.knowledge]
min_content_length = 5

[reasoning]
anomaly_threshold = 3

[economy.security]
max_behavior_history = 100
quality_threshold_low = 10
quality_threshold_medium = 50
quality_threshold_high = 100

[communication.spectrum]
low_capacity = 1000
low_cost = 5.0
medium_capacity = 200
# ...
```

---

## 结论

当前代码库存在一定程度的硬编码问题，主要集中在：

1. **配置默认值** - 需要移至配置文件
2. **阈值比较** - 需要使用配置参数
3. **魔术数字** - 需要添加语义化常量

建议按照优先级逐步修复，确保所有参数可配置、可验证、可测试。

---

*检测工具: ripgrep*
*检测日期: 2026-05-31*
