#!/usr/bin/env python3
"""
从 3D-FRONT-RENDER 数据提取训练样本

从目录名和 meta.json 提取房间类型和家具信息
"""

import os
import json
import tarfile
import re
from pathlib import Path
from collections import defaultdict
from typing import Dict, List, Set

TAR_PATH = Path("E:/ai_006_data/3d_front_hf/3D-FRONT-RENDER.tar.gz")
OUTPUT_PATH = Path("D:/ai_006/data/training/scenes_3dfront_render.json")

# 房间类型映射
ROOM_MAPPING = {
    "masterbedroom": "主卧室",
    "secondbedroom": "次卧室",
    "bedroom": "卧室",
    "livingroom": "客厅",
    "living_room": "客厅",
    "kitchen": "厨房",
    "bathroom": "浴室",
    "diningroom": "餐厅",
    "dining_room": "餐厅",
    "study": "书房",
    "office": "办公室",
    "childrenroom": "儿童房",
    "children_room": "儿童房",
    "guestroom": "客房",
    "guest_room": "客房",
    "storage": "储藏室",
    "closet": "衣帽间",
    "wardrobe": "衣帽间",
    "balcony": "阳台",
    "corridor": "走廊",
    "hallway": "门厅",
    "lobby": "大厅",
    "garage": "车库",
    "gym": "健身房",
    "library": "图书馆",
    "laundry": "洗衣房",
    "utility": "杂物间",
}

# 根据房间类型推断可能包含的家具
ROOM_FURNITURE = {
    "主卧室": ["床", "床头柜", "衣柜", "梳妆台", "椅子"],
    "次卧室": ["床", "床头柜", "衣柜", "书桌"],
    "卧室": ["床", "床头柜", "衣柜", "椅子"],
    "客厅": ["沙发", "茶几", "电视柜", "电视", "落地灯"],
    "厨房": ["冰箱", "橱柜", "灶台", "微波炉", "餐桌"],
    "浴室": ["洗手台", "马桶", "浴缸", "淋浴", "镜子"],
    "餐厅": ["餐桌", "餐椅", "餐边柜", "吊灯"],
    "书房": ["书桌", "书架", "椅子", "台灯"],
    "办公室": ["办公桌", "办公椅", "文件柜", "书架"],
    "儿童房": ["床", "书桌", "衣柜", "玩具柜"],
    "客房": ["床", "床头柜", "衣柜"],
    "储藏室": ["储物柜", "架子"],
    "衣帽间": ["衣柜", "鞋柜", "穿衣镜"],
    "阳台": ["椅子", "小桌子", "植物"],
    "走廊": ["鞋柜", "装饰画"],
    "门厅": ["鞋柜", "衣架", "镜子"],
    "健身房": ["跑步机", "哑铃", "瑜伽垫"],
    "洗衣房": ["洗衣机", "烘干机", "储物柜"],
    "杂物间": ["储物架", "工具柜"],
}


def extract_room_type(room_name: str) -> str:
    """从房间名提取房间类型"""
    room_lower = room_name.lower().replace("-", "").replace("_", "").replace(" ", "")

    for eng, cn in ROOM_MAPPING.items():
        if eng.replace("_", "").replace("-", "") in room_lower:
            return cn

    return "房间"


def generate_training_sample(scene_id: str, room_name: str, meta: dict = None) -> dict:
    """生成训练样本"""
    room_type = extract_room_type(room_name)
    furniture = ROOM_FURNITURE.get(room_type, ["家具"])

    # 随机选择 2-4 个家具
    import random
    max_furniture = min(4, len(furniture))
    min_furniture = min(2, len(furniture))
    num_furniture = random.randint(min_furniture, max_furniture)
    selected_furniture = random.sample(furniture, num_furniture)

    # 生成描述
    furniture_str = "和".join(selected_furniture)
    text = f"{room_type}里有{furniture_str}。"

    # 构建实体列表
    entities = [(room_type, "building")]
    for f in selected_furniture:
        entities.append((f, "furniture"))

    # 构建关系
    relations = []
    for f in selected_furniture:
        relations.append(f"{room_type}包含{f}")

    return {
        "id": f"3dfront-{scene_id}-{room_name}",
        "text": text,
        "language": "zh",
        "source": "3d-front-render",
        "entities": entities,
        "relations": relations,
        "raw": {
            "scene_id": scene_id,
            "room_name": room_name,
            "room_type": room_type
        }
    }


def process_tar_gz():
    """处理 tar.gz 文件"""
    print("=" * 60)
    print("从 3D-FRONT-RENDER 提取训练数据")
    print("=" * 60)

    samples = []
    scene_rooms = defaultdict(set)

    print(f"\n读取: {TAR_PATH}")

    with tarfile.open(TAR_PATH, 'r:gz') as tar:
        members = tar.getmembers()

        # 提取场景ID和房间名
        for member in members:
            parts = member.name.split('/')
            if len(parts) >= 3:
                # 格式: 3D-FRONT-RENDER/scene_id/RoomName-xxx/...
                scene_id = parts[1]
                room_part = parts[2]

                # 房间名格式: RoomType-number
                if '-' in room_part:
                    room_name = room_part.split('-')[0]
                    scene_rooms[scene_id].add(room_name)

        print(f"发现 {len(scene_rooms)} 个场景")

        # 生成训练样本
        for scene_id, rooms in scene_rooms.items():
            for room_name in rooms:
                sample = generate_training_sample(scene_id, room_name)
                samples.append(sample)

    print(f"生成 {len(samples)} 个训练样本")

    # 保存
    output_data = {
        "version": "3d-front-render-1.0",
        "description": "Extracted from 3D-FRONT-RENDER.tar.gz",
        "stats": {
            "scenes": len(scene_rooms),
            "samples": len(samples),
            "room_types": len(set(s["raw"]["room_type"] for s in samples))
        },
        "data": samples
    }

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT_PATH, 'w', encoding='utf-8') as f:
        json.dump(output_data, f, ensure_ascii=False, indent=2)

    size_mb = OUTPUT_PATH.stat().st_size / (1024 * 1024)
    print(f"\n保存到: {OUTPUT_PATH} ({size_mb:.1f} MB)")

    # 统计房间类型
    room_type_counts = defaultdict(int)
    for s in samples:
        room_type_counts[s["raw"]["room_type"]] += 1

    print("\n房间类型分布:")
    for rt, count in sorted(room_type_counts.items(), key=lambda x: -x[1])[:15]:
        print(f"  {rt}: {count}")

    return OUTPUT_PATH


if __name__ == "__main__":
    process_tar_gz()
