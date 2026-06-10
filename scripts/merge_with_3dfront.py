#!/usr/bin/env python3
"""合并 3D-Front 数据与现有训练数据"""

import json
from pathlib import Path

def merge_with_3dfront():
    print("=" * 60)
    print("合并 3D-Front 训练数据")
    print("=" * 60)

    # 文件路径
    existing_path = Path("data/training/scenes_full.json")
    front_path = Path("data/training/scenes_3dfront.json")
    output_path = Path("data/training/scenes_complete.json")

    # 读取现有数据
    print("\n[1] 读取现有训练数据...")
    existing_data = []
    if existing_path.exists():
        with open(existing_path, 'r', encoding='utf-8') as f:
            existing = json.load(f)
            existing_data = existing.get('data', [])
        print(f"  现有样本: {len(existing_data)}")
    else:
        print("  未找到现有数据文件")

    # 读取 3D-Front 数据
    print("\n[2] 读取 3D-Front 训练数据...")
    front_data = []
    if front_path.exists():
        with open(front_path, 'r', encoding='utf-8') as f:
            front = json.load(f)
            front_data = front.get('data', [])
        print(f"  3D-Front 样本: {len(front_data)}")
    else:
        print("  未找到 3D-Front 数据文件，请先运行 process_3dfront.py")

    # 合并
    all_data = existing_data + front_data
    print(f"\n[3] 合并后总计: {len(all_data)} 样本")

    # 统计
    zh_count = sum(1 for d in all_data if d.get('language') == 'zh')
    en_count = sum(1 for d in all_data if d.get('language') == 'en')

    sources = {}
    for d in all_data:
        src = d.get('source', 'unknown')
        sources[src] = sources.get(src, 0) + 1

    print(f"  中文: {zh_count}, 英文: {en_count}")
    print(f"  来源: {sources}")

    # 保存
    output = {
        "version": "4.0",
        "description": "Combined training data with 3D-Front real scene data",
        "sources": {
            "existing": len(existing_data),
            "3d_front": len(front_data)
        },
        "data": all_data
    }

    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(output, f, ensure_ascii=False, indent=2)

    size_mb = output_path.stat().st_size / (1024 * 1024)
    print(f"\n[4] 保存到: {output_path} ({size_mb:.1f} MB)")

    return output_path


if __name__ == "__main__":
    merge_with_3dfront()
