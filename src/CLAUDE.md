# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

万物生成器 - 基于蛊虫智能体的世界模型系统

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

#### 为什么禁止？

1. **可维护性**: 参数集中管理，易于调整
2. **可测试性**: 可以注入不同配置进行测试
3. **安全性**: 配置验证可以捕获无效值
4. **灵活性**: 支持运行时动态调整

#### 正确做法

所有数值参数必须：
1. 定义在 `src/config/` 模块中
2. 通过配置结构体访问
3. 包含验证逻辑
4. 提供合理的默认值

```rust
// 配置定义 (src/config/concept.rs)
pub struct ConceptConfig {
    pub vector_dim: usize,
    // ...
}

impl ConceptConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.vector_dim == 0 {
            return Err("vector_dim 必须大于0".to_string());
        }
        Ok(())
    }
}

// 使用 (src/language/concept/types.rs)
pub struct ConceptVector {
    data: Vec<f64>,
    config: Arc<ConceptConfig>, // 通过配置获取维度
}
```

### 检测硬编码

运行以下命令检测潜在的硬编码：

```bash
# 检测数字字面量
rg -n '\b\d+\.\d+\b' src/ --type rust | grep -v "test" | grep -v "config"

# 检测整数常量
rg -n 'const.*=\s*\d+;' src/ --type rust | grep -v "test" | grep -v "config"

# 检测硬编码比较
rg -n '>\s*\d+\s*{' src/ --type rust | grep -v "test"
```

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
```

## 架构概览

```
src/
├── config/          # 配置系统（所有参数定义）
│   ├── mod.rs       # 全局配置
│   ├── concept.rs   # 概念配置
│   ├── learning.rs  # 学习配置
│   ├── consensus.rs # 共识配置
│   ├── context.rs   # 上下文配置
│   └── tokenizer.rs # 分词器配置
│
├── core/            # LNN核心
│   ├── neuron.rs    # 神经元
│   ├── synapse.rs   # 突触
│   └── topology.rs  # 拓扑管理
│
├── learning/        # 学习规则
│   └── rules.rs     # 局部学习规则
│
├── safety/          # 安全机制
│   └── fuse.rs      # 熔断器
│
└── language/        # 自然语言对齐
    ├── concept/     # 概念空间
    ├── encoder/     # 编码器
    ├── decoder/     # 解码器
    ├── consensus/   # 共识机制
    └── context/     # 上下文管理
```

## 编码风格

- 使用不可变模式，避免突变
- 小函数（<50行），小文件（<800行）
- 完整的错误处理
- 单元测试覆盖（80%+）

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

## 安全检查清单

在提交前确保：
- [ ] 没有硬编码数值
- [ ] 所有参数通过配置管理
- [ ] 配置有验证逻辑
- [ ] 测试覆盖配置验证
