# LNN 神经网络需求分析

## 概述

本文档分析 LNN（液体神经网络）的完整需求，以及与词向量数据库的集成方案。

---

## 一、LNN 已实现功能

### 1.1 核心组件 ✅

| 组件 | 文件 | 功能 |
|------|------|------|
| 神经元 | `core/neuron.rs` | 连续时间状态更新，状态归一化 |
| 突触 | `core/synapse.rs` | 局部学习规则（Hebbian, Oja, STDP）|
| LNN 网络 | `core/lnn.rs` | 拓扑管理，安全熔断，动态修剪 |
| 学习规则 | `learning/rules.rs` | 情绪调节学习率 |

### 1.2 持久化功能 ✅（刚完成）

```rust
// LNN 权重保存
lnn.save("network.json")?;      // JSON 格式（可读）
lnn.save_binary("network.bin")?; // Binary 格式（紧凑）

// LNN 权重加载
let loaded = LNN::load("network.json")?;
let loaded = LNN::load_binary("network.bin")?;

// 创建快照（内存）
let snapshot = lnn.create_snapshot();
let restored = LNN::from_snapshot(snapshot);
```

### 1.3 词向量支持 ✅

```rust
// 加载词向量
let embedding = WordEmbedding::from_fasttext_vec("data/embeddings/test.vec", config)?;

// 相似度计算
let sim = embedding.similarity("猫", "狗"); // 0.998

// 最相似词查找
let similar = embedding.most_similar("猫", 5);

// 类比推理
let result = embedding.analogy("王", "男", "女", 5); // 王 - 男 + 女 = 后
```

---

## 二、LNN 还需要什么？

### 2.1 词向量与概念空间桥接 🔴 待实现

**需求**：将词向量映射到 LNN 的概念空间

```rust
pub struct ConceptEmbedding {
    /// 词向量模型
    word_embedding: WordEmbedding,
    /// 概念空间维度
    concept_dim: usize,
    /// 映射矩阵（词向量 -> 概念向量）
    projection: Vec<f64>,
}

impl ConceptEmbedding {
    /// 将词编码为概念向量
    pub fn encode_word(&self, word: &str) -> ConceptVector {
        let word_vec = self.word_embedding.get_vector(word);
        // 投影到概念空间
        self.project(&word_vec)
    }

    /// 初始化神经元状态
    pub fn init_neuron(&self, word: &str) -> f64 {
        let concept = self.encode_word(word);
        concept.magnitude() // 作为神经元初始状态
    }
}
```

### 2.2 语义学习训练 🔴 待实现

**需求**：让 LNN 学习词向量中的语义关系

```rust
impl LNN {
    /// 从词向量学习语义关系
    pub fn learn_semantics(&mut self, embedding: &WordEmbedding) {
        // 1. 为每个词创建神经元
        for word in embedding.vocabulary() {
            let vec = embedding.get_vector(word);
            // 创建神经元，初始状态基于词向量
            self.add_semantic_neuron(word, vec);
        }

        // 2. 根据相似度创建突触
        for (w1, w2) in embedding.word_pairs() {
            let sim = embedding.similarity(w1, w2);
            if sim > 0.7 {
                // 高相似度词之间建立强连接
                self.add_semantic_synapse(w1, w2, sim);
            }
        }
    }
}
```

### 2.3 训练数据生成 🔴 待实现

**需求**：从词向量生成训练数据

```rust
pub struct TrainingDataGenerator {
    embedding: WordEmbedding,
}

impl TrainingDataGenerator {
    /// 生成语义关系训练数据
    pub fn generate(&self) -> Vec<TrainingSample> {
        let mut samples = Vec::new();

        // 正样本：相似词对
        for (w1, w2) in self.similar_pairs(0.7) {
            samples.push(TrainingSample {
                input: w1,
                target: w2,
                label: 1.0,
            });
        }

        // 负样本：不相似词对
        for (w1, w2) in self.dissimilar_pairs(0.3) {
            samples.push(TrainingSample {
                input: w1,
                target: w2,
                label: 0.0,
            });
        }

        samples
    }
}
```

---

## 三、完整训练流程设计

### 3.1 训练流程

```
┌─────────────────────────────────────────────────────────────┐
│                    LNN 训练流程                               │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  1. 下载词向量数据库                                          │
│     python scripts/download_embeddings.py --model fasttext-zh│
│                                                               │
│  2. 加载词向量                                                │
│     WordEmbedding::from_fasttext_vec("cc.zh.300.vec", config)│
│                                                               │
│  3. 初始化 LNN 网络                                           │
│     LNN::new(config, topology)                               │
│                                                               │
│  4. 语义学习                                                  │
│     lnn.learn_semantics(&embedding)                          │
│                                                               │
│  5. 训练迭代                                                  │
│     for epoch in 0..epochs {                                 │
│         lnn.update(dt)?;                                     │
│         // 评估、调整                                         │
│     }                                                         │
│                                                               │
│  6. 保存权重                                                  │
│     lnn.save("models/trained.json")?                         │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 权重文件格式

**JSON 格式示例**：
```json
{
  "version": 1,
  "created_at": 1717123456789,
  "neurons": [
    {
      "id": "neuron_001",
      "neuron_type": "Cognitive",
      "state": 0.5,
      "tau": 1.0,
      "bias": 0.0,
      "activity": 0.23,
      "importance": 0.8
    }
  ],
  "synapses": [
    {
      "id": "neuron_001->neuron_002",
      "from_neuron_id": "neuron_001",
      "to_neuron_id": "neuron_002",
      "weight": 0.85,
      "plasticity_rule": "Hebbian"
    }
  ],
  "current_time": 10.5,
  "config": { ... }
}
```

---

## 四、下载完整词向量

### 4.1 fastText 中文词向量

```bash
# 下载完整版（约 4GB）
python scripts/download_embeddings.py --model fasttext-zh

# 文件位置
data/embeddings/cc.zh.300.vec
```

### 4.2 测试词向量（已存在）

```bash
# 测试版（32 词，10 维）
data/embeddings/test.vec
```

---

## 五、权重保存 - 已实现

### 5.1 为什么需要保存权重？

1. **训练成本**：LNN 训练需要大量时间步更新
2. **知识积累**：突触权重编码了学习到的语义关系
3. **部署需求**：生产环境需要快速加载训练好的模型
4. **版本管理**：不同阶段的模型需要版本控制

### 5.2 保存频率建议

| 场景 | 频率 | 格式 |
|------|------|------|
| 训练检查点 | 每 1000 epoch | Binary |
| 训练完成 | 最终 | JSON + Binary |
| 生产部署 | 发布时 | Binary |

---

## 六、后续任务

1. **Task #63**: 实现 LNN 语义学习功能
2. **Task #64**: 设计词向量与 LNN 概念空间的桥接层

---

## 附录：关键公式

### LNN 状态更新

```
τ·dx/dt = -x + input + bias

欧拉法：x(t+dt) = x(t) + dt/τ · (-x + input + bias)
```

### 突触学习规则

| 规则 | 公式 | 特点 |
|------|------|------|
| Hebbian | Δw = η·xᵢ·xⱼ | 相关性学习 |
| Oja | Δw = η·y·(x-y·w) | 防止权重爆炸 |
| STDP | Δw = A₊·exp(-Δt/τ₊) | 时序依赖 |
