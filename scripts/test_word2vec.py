# -*- coding: utf-8 -*-
"""
测试腾讯词向量加载
用于验证词向量是否可用
"""
import gensim
from gensim.models import KeyedVectors
import os
import sys

# 设置输出编码
sys.stdout.reconfigure(encoding='utf-8')

# 词向量文件路径
model_path = r"C:\Users\admin\text2vec-word2vec-tencent-chinese\light_Tencent_AILab_ChineseEmbedding.bin"

print("=== 腾讯词向量测试 ===\n")

# 加载模型
print(f"加载模型: {model_path}")
print("请稍候...")

try:
    # 加载二进制格式的词向量
    model = KeyedVectors.load_word2vec_format(model_path, binary=True)
    print(f"[OK] 模型加载成功!")
    print(f"   词汇量: {len(model):,}")
    print(f"   向量维度: {model.vector_size}")
except Exception as e:
    print(f"[FAIL] 加载失败: {e}")
    exit(1)

# 测试一些词汇
print("\n=== 测试词汇相似度 ===\n")

test_words = ["房子", "桌子", "椅子", "树", "人", "建筑", "房间"]

for word in test_words:
    if word in model:
        # 找最相似的词
        similar = model.most_similar(word, topn=5)
        print(f"【{word}】最相似的词:")
        for w, score in similar:
            print(f"   - {w}: {score:.4f}")
    else:
        print(f"【{word}】不在词表中")
    print()

# 测试语义关系
print("=== 测试语义关系 ===\n")

# 类比测试：房子 - 建筑 = 桌子 - ?
if "房子" in model and "建筑" in model and "桌子" in model:
    result = model.most_similar(positive=["房子", "桌子"], negative=["建筑"], topn=3)
    print("类比: 房子 - 建筑 = 桌子 - ?")
    for w, score in result:
        print(f"   - {w}: {score:.4f}")

print("\n=== 测试实体识别场景 ===\n")

# 测试我们的场景词汇
scene_words = [
    ("房子", "建筑"),
    ("桌子", "家具"),
    ("椅子", "家具"),
    ("树", "植物"),
    ("花", "植物"),
    ("人", "生物"),
    ("红色", "颜色"),
    ("蓝色", "颜色"),
]

print("词汇类型相似度测试:")
for word, category in scene_words:
    if word in model and category in model:
        sim = model.similarity(word, category)
        print(f"   {word} <-> {category}: {sim:.4f}")
    else:
        missing = word if word not in model else category
        print(f"   {word} <-> {category}: [{missing} 不在词表中]")

print("\n[OK] 词向量模型可用于实体识别改进!")
