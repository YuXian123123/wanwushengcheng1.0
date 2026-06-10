"""
合并训练数据集

合并中文场景描述数据与英文场景图数据，
创建综合训练数据集。
"""

import json
from pathlib import Path


def load_json(path):
    """加载 JSON 文件"""
    with open(path, 'r', encoding='utf-8') as f:
        return json.load(f)


def save_json(data, path):
    """保存 JSON 文件"""
    with open(path, 'w', encoding='utf-8') as f:
        json.dump(data, f, indent=2, ensure_ascii=False)


def main():
    print("=== 合并训练数据集 ===\n")

    # 加载中文训练数据
    zh_path = Path("data/training/scenes_basic.json")
    zh_data = load_json(zh_path)
    print(f"中文数据: {len(zh_data['data'])} 条样本")

    # 加载英文场景图数据
    en_path = Path("data/training/scene_graphs_converted.json")
    en_data = load_json(en_path)
    print(f"英文数据: {len(en_data['data'])} 条样本")

    # 合并数据
    merged_data = {
        "version": "2.0",
        "description": "Combined Chinese scene descriptions and English scene graphs",
        "sources": {
            "chinese": {
                "source": "scenes_basic.json",
                "count": len(zh_data['data']),
                "language": "zh-CN"
            },
            "english": {
                "source": "openvocab-scene-graphs",
                "count": len(en_data['data']),
                "language": "en"
            }
        },
        "data": zh_data['data'] + en_data['data']
    }

    # 保存合并数据
    output_path = Path("data/training/scenes_combined.json")
    save_json(merged_data, output_path)

    print(f"\n合并完成: {len(merged_data['data'])} 条样本")
    print(f"保存到: {output_path}")
    print(f"文件大小: {output_path.stat().st_size / 1024 / 1024:.2f} MB")

    # 统计信息
    print("\n统计信息:")

    # 语言分布
    lang_count = {"zh-CN": 0, "en": 0}
    for sample in merged_data['data']:
        lang = sample.get('language', 'zh-CN')
        lang_count[lang] = lang_count.get(lang, 0) + 1
    print(f"  语言分布: {lang_count}")

    # 实体类型统计
    from collections import Counter
    entity_types = Counter()
    relation_types = Counter()

    for sample in merged_data['data']:
        for entity in sample.get('entities', []):
            entity_types[entity.get('type', 'unknown')] += 1
        for relation in sample.get('relations', []):
            relation_types[relation.get('type', 'unknown')] += 1

    print(f"\n  实体类型 (前10):")
    for etype, count in entity_types.most_common(10):
        print(f"    {etype}: {count}")

    print(f"\n  关系类型 (前10):")
    for rtype, count in relation_types.most_common(10):
        print(f"    {rtype}: {count}")

    # 打印示例
    print("\n示例数据:")
    for sample in merged_data['data'][:3]:
        print(f"\n  [{sample.get('language', 'zh-CN')}] {sample['text']}")
        print(f"    实体: {[e['name'] for e in sample['entities'][:5]]}")
        print(f"    关系数: {len(sample['relations'])}")


if __name__ == "__main__":
    main()
