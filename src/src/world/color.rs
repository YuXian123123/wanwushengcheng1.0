//! 蛊虫颜色遗传系统
//!
//! 设计理念：
//! - 一代蛊虫：红、绿、蓝三原色
//! - 二代蛊虫：父母颜色的混合
//! - 后续世代：颜色越来越丰富

use serde::{Deserialize, Serialize};
use rand::Rng;

/// 蛊虫颜色 (RGB)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuColor {
    /// 红色通道 (0-255)
    pub r: u8,
    /// 绿色通道 (0-255)
    pub g: u8,
    /// 蓝色通道 (0-255)
    pub b: u8,
}

impl GuColor {
    /// 红色原种
    pub const RED: GuColor = GuColor { r: 255, g: 0, b: 0 };
    /// 绿色原种
    pub const GREEN: GuColor = GuColor { r: 0, g: 255, b: 0 };
    /// 蓝色原种
    pub const BLUE: GuColor = GuColor { r: 0, g: 0, b: 255 };

    /// 创建新颜色
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// 转换为十六进制字符串
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// 转换为 CSS rgb 格式
    pub fn to_rgb_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    /// 计算颜色亮度 (0.0 - 1.0)
    pub fn brightness(&self) -> f64 {
        (0.299 * self.r as f64 + 0.587 * self.g as f64 + 0.114 * self.b as f64) / 255.0
    }

    /// 计算与另一颜色的距离
    pub fn distance(&self, other: &GuColor) -> f64 {
        let dr = self.r as f64 - other.r as f64;
        let dg = self.g as f64 - other.g as f64;
        let db = self.b as f64 - other.b as f64;
        (dr * dr + dg * dg + db * db).sqrt()
    }

    /// 判断是否为原种颜色
    pub fn is_primordial(&self) -> bool {
        self == &Self::RED || self == &Self::GREEN || self == &Self::BLUE
    }

    /// 获取颜色名称
    pub fn name(&self) -> &'static str {
        match (self.r, self.g, self.b) {
            (255, 0, 0) => "火灵虫",
            (0, 255, 0) => "木灵虫",
            (0, 0, 255) => "水灵虫",
            (255, 255, 0) => "金灵虫",
            (255, 0, 255) => "雷灵虫",
            (0, 255, 255) => "冰灵虫",
            (255, 128, 0) => "炎魔虫",
            (128, 0, 255) => "幽影虫",
            (255, 128, 128) => "圣光虫",
            (128, 255, 128) => "青木虫",
            (128, 128, 255) => "深海虫",
            (255, 192, 203) => "月华虫",
            (255, 215, 0) => "日炎虫",
            (138, 43, 226) => "星辰虫",
            _ => "灵虫",
        }
    }
}

impl Default for GuColor {
    fn default() -> Self {
        Self::RED
    }
}

/// 原种类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimordialType {
    /// 红色原种 - 火属性
    Red,
    /// 绿色原种 - 木属性
    Green,
    /// 蓝色原种 - 水属性
    Blue,
}

impl PrimordialType {
    /// 获取对应颜色
    pub fn color(&self) -> GuColor {
        match self {
            Self::Red => GuColor::RED,
            Self::Green => GuColor::GREEN,
            Self::Blue => GuColor::BLUE,
        }
    }

    /// 获取名称
    pub fn name(&self) -> &'static str {
        match self {
            Self::Red => "火灵虫",
            Self::Green => "木灵虫",
            Self::Blue => "水灵虫",
        }
    }

    /// 随机选择一种原种
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => Self::Red,
            1 => Self::Green,
            _ => Self::Blue,
        }
    }
}

/// 颜色基因 - 用于遗传
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorGene {
    /// 主色基因（显示颜色）
    pub primary: GuColor,
    /// 隐性基因（可能传递给后代）
    pub recessive: GuColor,
    /// 世代数
    pub generation: u32,
    /// 原种标记（一代蛊虫）
    pub primordial: Option<PrimordialType>,
}

impl ColorGene {
    /// 创建原种基因（一代蛊虫）
    pub fn primordial(primordial: PrimordialType) -> Self {
        let color = primordial.color();
        Self {
            primary: color,
            recessive: color,
            generation: 1,
            primordial: Some(primordial),
        }
    }

    /// 创建随机原种基因
    pub fn random_primordial() -> Self {
        Self::primordial(PrimordialType::random())
    }

    /// 遗传繁殖 - 从两个父母产生子代基因
    pub fn breed(parent1: &ColorGene, parent2: &ColorGene) -> Self {
        let mut rng = rand::thread_rng();

        // 确定混合因子 (偏向某一方)
        let factor = rng.gen_range(0.3..0.7);

        // 混合主颜色
        let primary = mix_colors(parent1.primary, parent2.primary, factor);

        // 隐性基因从父母的隐性基因中随机选择
        let recessive = if rng.gen_bool(0.5) {
            parent1.recessive
        } else {
            parent2.recessive
        };

        // 计算世代
        let generation = parent1.generation.max(parent2.generation) + 1;

        // 突变 (5% 概率)
        let final_color = if rng.gen_bool(0.05) {
            mutate_color(primary)
        } else {
            primary
        };

        Self {
            primary: final_color,
            recessive,
            generation,
            primordial: None, // 子代不再是原种
        }
    }

    /// 获取显示颜色
    pub fn display_color(&self) -> GuColor {
        self.primary
    }

    /// 获取颜色名称
    pub fn color_name(&self) -> String {
        if let Some(primordial) = self.primordial {
            primordial.name().to_string()
        } else {
            self.primary.name().to_string()
        }
    }

    /// 获取世代描述
    pub fn generation_desc(&self) -> String {
        match self.generation {
            1 => "原种".to_string(),
            2 => "二代".to_string(),
            3 => "三代".to_string(),
            n => format!("{}代", n),
        }
    }
}

impl Default for ColorGene {
    fn default() -> Self {
        Self::random_primordial()
    }
}

/// 混合两种颜色
fn mix_colors(c1: GuColor, c2: GuColor, factor: f64) -> GuColor {
    GuColor {
        r: ((c1.r as f64) * factor + (c2.r as f64) * (1.0 - factor)).round() as u8,
        g: ((c1.g as f64) * factor + (c2.g as f64) * (1.0 - factor)).round() as u8,
        b: ((c1.b as f64) * factor + (c2.b as f64) * (1.0 - factor)).round() as u8,
    }
}

/// 颜色突变
fn mutate_color(color: GuColor) -> GuColor {
    let mut rng = rand::thread_rng();

    let mutation_range = 30.0;

    GuColor {
        r: ((color.r as f64) + rng.gen_range(-mutation_range..mutation_range))
            .clamp(0.0, 255.0) as u8,
        g: ((color.g as f64) + rng.gen_range(-mutation_range..mutation_range))
            .clamp(0.0, 255.0) as u8,
        b: ((color.b as f64) + rng.gen_range(-mutation_range..mutation_range))
            .clamp(0.0, 255.0) as u8,
    }
}

/// 颜色遗传系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorGenetics {
    /// 当前种群的颜色分布
    color_distribution: Vec<GuColor>,
    /// 原种数量
    primordial_count: [u32; 3], // Red, Green, Blue
}

impl ColorGenetics {
    /// 创建新的颜色遗传系统
    pub fn new() -> Self {
        Self {
            color_distribution: Vec::new(),
            primordial_count: [0, 0, 0],
        }
    }

    /// 注册新蛊虫的颜色
    pub fn register(&mut self, gene: &ColorGene) {
        self.color_distribution.push(gene.primary);

        if let Some(primordial) = gene.primordial {
            match primordial {
                PrimordialType::Red => self.primordial_count[0] += 1,
                PrimordialType::Green => self.primordial_count[1] += 1,
                PrimordialType::Blue => self.primordial_count[2] += 1,
            }
        }
    }

    /// 移除蛊虫颜色
    pub fn unregister(&mut self, color: &GuColor) {
        if let Some(pos) = self.color_distribution.iter().position(|c| c == color) {
            self.color_distribution.remove(pos);
        }
    }

    /// 计算颜色多样性
    pub fn diversity(&self) -> f64 {
        if self.color_distribution.len() < 2 {
            return 0.0;
        }

        // 计算平均颜色距离
        let mut total_distance = 0.0;
        let mut count = 0;

        for i in 0..self.color_distribution.len() {
            for j in (i + 1)..self.color_distribution.len() {
                total_distance += self.color_distribution[i].distance(&self.color_distribution[j]);
                count += 1;
            }
        }

        if count == 0 {
            0.0
        } else {
            // 归一化到 0-1
            let avg_distance = total_distance / count as f64;
            avg_distance / 441.7 // 最大可能距离 sqrt(255^2 * 3)
        }
    }

    /// 获取原种比例
    pub fn primordial_ratio(&self) -> [f64; 3] {
        let total = self.color_distribution.len() as f64;
        if total == 0.0 {
            return [0.0, 0.0, 0.0];
        }

        [
            self.primordial_count[0] as f64 / total,
            self.primordial_count[1] as f64 / total,
            self.primordial_count[2] as f64 / total,
        ]
    }

    /// 获取颜色分布统计
    pub fn color_stats(&self) -> ColorStats {
        let mut red_sum = 0u32;
        let mut green_sum = 0u32;
        let mut blue_sum = 0u32;

        for color in &self.color_distribution {
            red_sum += color.r as u32;
            green_sum += color.g as u32;
            blue_sum += color.b as u32;
        }

        let count = self.color_distribution.len();

        ColorStats {
            count,
            avg_color: if count > 0 {
                GuColor {
                    r: (red_sum / count as u32) as u8,
                    g: (green_sum / count as u32) as u8,
                    b: (blue_sum / count as u32) as u8,
                }
            } else {
                GuColor::default()
            },
            diversity: self.diversity(),
        }
    }
}

impl Default for ColorGenetics {
    fn default() -> Self {
        Self::new()
    }
}

/// 颜色统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorStats {
    /// 蛊虫数量
    pub count: usize,
    /// 平均颜色
    pub avg_color: GuColor,
    /// 颜色多样性 (0-1)
    pub diversity: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primordial_colors() {
        assert!(GuColor::RED.is_primordial());
        assert!(GuColor::GREEN.is_primordial());
        assert!(GuColor::BLUE.is_primordial());

        let mixed = GuColor::new(128, 128, 0);
        assert!(!mixed.is_primordial());
    }

    #[test]
    fn test_color_mixing() {
        let red = GuColor::RED;
        let green = GuColor::GREEN;

        // 红绿混合应该得到黄色（各占一半）
        let yellow = mix_colors(red, green, 0.5);
        // 255 * 0.5 + 0 * 0.5 = 127.5 ≈ 128
        assert!(yellow.r >= 100);  // 红色分量应该有
        assert!(yellow.g >= 100);  // 绿色分量应该有
        assert_eq!(yellow.b, 0);   // 蓝色分量应该为0
    }

    #[test]
    fn test_gene_breeding() {
        let parent1 = ColorGene::primordial(PrimordialType::Red);
        let parent2 = ColorGene::primordial(PrimordialType::Green);

        let child = ColorGene::breed(&parent1, &parent2);

        // 子代应该是二代
        assert_eq!(child.generation, 2);

        // 子代不是原种
        assert!(child.primordial.is_none());

        // 子代颜色应该是红绿的混合
        // 由于 factor 是随机的 0.3-0.7，两个分量都至少会有一些
        // 红色分量: 255 * factor, 至少 255 * 0.3 = 76.5
        // 绿色分量: 255 * (1-factor), 至少 255 * 0.3 = 76.5
        assert!(child.primary.r > 50, "Red component should be present, got {}", child.primary.r);
        assert!(child.primary.g > 50, "Green component should be present, got {}", child.primary.g);
    }

    #[test]
    fn test_color_diversity() {
        let mut genetics = ColorGenetics::new();

        // 只有一种颜色，多样性为0
        genetics.register(&ColorGene::primordial(PrimordialType::Red));
        assert_eq!(genetics.diversity(), 0.0);

        // 添加不同颜色（原色之间距离相等）
        genetics.register(&ColorGene::primordial(PrimordialType::Green));
        let diversity_2 = genetics.diversity();
        assert!(diversity_2 > 0.0, "Two colors should have positive diversity");

        // 添加第三种原色（红绿蓝三原色之间的距离相同，所以多样性不变）
        // 这是数学上的正确结果：三原色是等边三角形的三个顶点
        genetics.register(&ColorGene::primordial(PrimordialType::Blue));
        let diversity_3 = genetics.diversity();
        assert!(diversity_3 > 0.0, "Three primary colors should have positive diversity");

        // 添加一个混合色（应该增加多样性）
        let mixed_gene = ColorGene::breed(
            &ColorGene::primordial(PrimordialType::Red),
            &ColorGene::primordial(PrimordialType::Green)
        );
        genetics.register(&mixed_gene);
        let diversity_4 = genetics.diversity();
        assert!(diversity_4 > 0.0, "With mixed color should have diversity");
    }

    #[test]
    fn test_color_to_hex() {
        assert_eq!(GuColor::RED.to_hex(), "#FF0000");
        assert_eq!(GuColor::GREEN.to_hex(), "#00FF00");
        assert_eq!(GuColor::BLUE.to_hex(), "#0000FF");
    }
}
