# -*- coding: utf-8 -*-
"""
导出常用词汇向量到JSON格式
供 Rust 项目使用
"""
import json
from gensim.models import KeyedVectors
import sys

sys.stdout.reconfigure(encoding='utf-8')

# 加载模型
model_path = r"C:\Users\admin\text2vec-word2vec-tencent-chinese\light_Tencent_AILab_ChineseEmbedding.bin"
print("加载词向量模型...")
model = KeyedVectors.load_word2vec_format(model_path, binary=True)
print(f"词汇量: {len(model)}")

# 定义需要导出的词汇类别
word_categories = {
    "building": ["房子", "建筑", "楼房", "房屋", "屋子", "房间", "卧室", "客厅", "厨房", "卫生间", "门", "窗", "屋顶", "墙壁"],
    "furniture": ["桌子", "椅子", "沙发", "床", "柜子", "书架", "茶几", "凳子", "衣柜", "书桌"],
    "plant": ["树", "花", "草", "森林", "树木", "花朵", "叶子", "树枝"],
    "person": ["人", "男人", "女人", "孩子", "老人", "男孩", "女孩", "人们"],
    "location": ["村庄", "城市", "街道", "公园", "学校", "医院", "商店", "超市"],
    "color": ["红色", "蓝色", "绿色", "黄色", "白色", "黑色", "橙色", "紫色", "粉色"],
    "material": ["木头", "金属", "玻璃", "塑料", "石头", "布"],
}

# 导出词向量
output = {
    "dimension": model.vector_size,
    "words": {},
    "categories": word_categories
}

# 导出所有在词表中的词汇
all_words = set()
for words in word_categories.values():
    all_words.update(words)

# 添加相似词
for word in list(all_words):
    if word in model:
        similar = model.most_similar(word, topn=10)
        for w, _ in similar:
            all_words.add(w)

# 导出向量
for word in all_words:
    if word in model:
        output["words"][word] = model[word].tolist()

print(f"导出词汇数: {len(output['words'])}")

# 保存为JSON
output_path = "data/word_vectors.json"
import os
os.makedirs("data", exist_ok=True)

with open(output_path, "w", encoding="utf-8") as f:
    json.dump(output, f, ensure_ascii=False, indent=2)

print(f"保存到: {output_path}")

# 同时保存一个小的测试集
test_output = {
    "dimension": model.vector_size,
    "words": {},
    "test_cases": []
}

# 测试用例
test_cases = [
    ("房子", "building"),
    ("桌子", "furniture"),
    ("椅子", "furniture"),
    ("树", "plant"),
    ("人", "person"),
    ("红色", "color"),
]

for word, category in test_cases:
    if word in model:
        test_output["words"][word] = model[word].tolist()
        test_output["test_cases"].append({
            "word": word,
            "category": category,
            "vector_sample": model[word][:5].tolist()
        })

# 添加类型关键词
for category, words in word_categories.items():
    for word in words[:3]:  # 每个类别只取前3个
        if word in model and word not in test_output["words"]:
            test_output["words"][word] = model[word].tolist()

test_path = "data/word_vectors_test.json"
with open(test_path, "w", encoding="utf-8") as f:
    json.dump(test_output, f, ensure_ascii=False, indent=2)

print(f"测试集保存到: {test_path}")
print("\n[OK] 词向量导出完成!")
