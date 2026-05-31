# 蛊虫颜色遗传系统设计

## 概述

蛊虫智能体具有颜色属性，通过遗传机制传递给后代，使得页面显示更加丰富多彩。

## 颜色模型

### RGB 原色系统

一代蛊虫（原种）只有三种颜色：
- **红色种 (Red)**: RGB(255, 0, 0)
- **绿色种 (Green)**: RGB(0, 255, 0)
- **蓝色种 (Blue)**: RGB(0, 0, 255)

### 遗传规则

```
一代 (原种):
  - 红 (R)
  - 绿 (G)
  - 蓝 (B)

二代 (杂交):
  - R + G → 黄色 (Yellow)
  - R + B → 品红 (Magenta)
  - G + B → 青色 (Cyan)

三代+ (丰富色彩):
  - 混合遗传产生更丰富的颜色
  - 每次繁殖从父母中继承颜色基因
```

## 遗传算法

### 颜色混合公式

```rust
// 子代颜色 = 父母颜色的混合
fn mix_colors(parent1: RGB, parent2: RGB) -> RGB {
    // 带随机因子的混合
    let factor = random(0.3, 0.7);
    RGB {
        r: (parent1.r * factor + parent2.r * (1.0 - factor)) as u8,
        g: (parent1.g * factor + parent2.g * (1.0 - factor)) as u8,
        b: (parent1.b * factor + parent2.b * (1.0 - factor)) as u8,
    }
}
```

### 突变机制

```rust
// 小概率突变，产生新颜色
fn mutate(color: RGB, mutation_rate: f64) -> RGB {
    if random() < mutation_rate {
        // 颜色值微调
        RGB {
            r: (color.r as f64 + random(-30.0, 30.0)).clamp(0.0, 255.0) as u8,
            g: (color.g as f64 + random(-30.0, 30.0)).clamp(0.0, 255.0) as u8,
            b: (color.b as f64 + random(-30.0, 30.0)).clamp(0.0, 255.0) as u8,
        }
    } else {
        color
    }
}
```

## 数据结构

```rust
/// 蛊虫颜色
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GuColor {
    /// 红色通道 (0-255)
    pub r: u8,
    /// 绿色通道 (0-255)
    pub g: u8,
    /// 蓝色通道 (0-255)
    pub b: u8,
}

/// 颜色基因
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorGene {
    /// 主色基因
    pub primary: GuColor,
    /// 隐性基因（可能传递给后代）
    pub recessive: GuColor,
    /// 世代数
    pub generation: u32,
}

/// 原种类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimordialType {
    Red,    // 红色原种
    Green,  // 绿色原种
    Blue,   // 蓝色原种
}
```

## 遗传流程

```
1. 创建一代蛊虫（原种）
   - 随机选择一种原色：红/绿/蓝
   - primary = recessive = 原色
   - generation = 1

2. 繁殖新蛊虫
   - 从父母中选择一方继承主基因
   - 混合父母颜色产生新颜色
   - 小概率突变
   - generation = max(parent1.gen, parent2.gen) + 1

3. 颜色展示
   - 使用 primary 颜色渲染
   - 颜色丰富度随世代增加
```

## 预期效果

| 世代 | 颜色种类 | 示例颜色 |
|------|----------|----------|
| 1代 | 3种 | 红、绿、蓝 |
| 2代 | 6种 | +黄、品红、青 |
| 3代 | 12+种 | 橙、紫、粉、褐... |
| 4代+ | 数十种 | 各种过渡色 |

## 实现文件

- `src/world/color.rs` - 颜色系统和遗传算法
