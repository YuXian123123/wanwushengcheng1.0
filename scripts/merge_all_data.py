#!/usr/bin/env python3
"""合并训练数据并重新训练"""

import json
from pathlib import Path

def merge_training_data():
    print("=" * 60)
    print("合并训练数据")
    print("=" * 60)

    # 读取现有数据
    combined_path = Path("data/training/scenes_combined.json")
    synthetic_path = Path("data/3d_front/synthetic_chinese_scenes.json")
    output_path = Path("data/training/scenes_full.json")

    # 读取现有数据
    print("\n读取现有训练数据...")
    with open(combined_path, 'r', encoding='utf-8') as f:
        existing = json.load(f)

    existing_data = existing.get('data', [])
    print(f"  现有样本: {len(existing_data)}")

    # 读取合成数据
    print("\n读取合成中文数据...")
    with open(synthetic_path, 'r', encoding='utf-8') as f:
        synthetic = json.load(f)

    synthetic_data = synthetic.get('data', [])
    print(f"  合成样本: {len(synthetic_data)}")

    # 合并
    all_data = existing_data + synthetic_data
    print(f"\n合并后总计: {len(all_data)} 样本")

    # 统计
    zh_count = sum(1 for d in all_data if d.get('language') == 'zh')
    en_count = sum(1 for d in all_data if d.get('language') == 'en')
    print(f"  中文: {zh_count}, 英文: {en_count}")

    # 保存
    output = {
        "version": "3.0",
        "description": "Combined training data with synthetic Chinese scenes",
        "sources": {
            "original": {
                "count": len(existing_data),
                "source": "scenes_combined.json"
            },
            "synthetic_chinese": {
                "count": len(synthetic_data),
                "source": "synthetic_chinese_scenes.json"
            }
        },
        "data": all_data
    }

    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(output, f, ensure_ascii=False, indent=2)

    size_mb = output_path.stat().st_size / (1024*1024)
    print(f"\n保存到: {output_path} ({size_mb:.1f} MB)")

    return output_path

if __name__ == "__main__":
    merge_training_data()
