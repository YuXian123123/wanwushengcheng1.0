#!/usr/bin/env python3
"""
从 3D-FRONT-RENDER.tar.gz 提取真实家具信息

解压并读取 meta.json 文件，提取：
- 房间类型
- 家具列表
- 家具位置
"""

import os
import json
import tarfile
import tempfile
import shutil
from pathlib import Path
from collections import defaultdict
from typing import Dict, List, Set
import re

TAR_PATH = Path("E:/ai_006_data/3d_front_hf/3D-FRONT-RENDER.tar.gz")
OUTPUT_PATH = Path("D:/ai_006/data/training/scenes_3dfront_real.json")

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

# 家具类型映射（从英文到中文）
FURNITURE_MAPPING = {
    # 床类
    "bed": "床",
    "double_bed": "双人床",
    "single_bed": "单人床",
    "king_bed": "大床",
    "queen_bed": "双人床",
    "nightstand": "床头柜",
    "nightstands": "床头柜",
    "bedside": "床头柜",

    # 桌子类
    "table": "桌子",
    "dining_table": "餐桌",
    "coffee_table": "茶几",
    "desk": "书桌",
    "side_table": "边桌",
    "end_table": "边桌",
    "console": "玄关桌",
    "vanity": "梳妆台",

    # 椅子类
    "chair": "椅子",
    "armchair": "扶手椅",
    "dining_chair": "餐椅",
    "office_chair": "办公椅",
    "stool": "凳子",
    "sofa": "沙发",
    "couch": "长沙发",
    "loveseat": "双人沙发",
    "ottoman": "脚凳",
    "chaise": "躺椅",
    "bench": "长凳",
    "recliner": "躺椅",

    # 柜类
    "cabinet": "柜子",
    "wardrobe": "衣柜",
    "closet": "衣柜",
    "bookshelf": "书架",
    "bookcase": "书柜",
    "book_shelf": "书架",
    "tv_stand": "电视柜",
    "tv_cabinet": "电视柜",
    "drawer": "抽屉柜",
    "chest": "柜子",
    "dresser": "梳妆台",
    "sideboard": "餐边柜",
    "buffet": "餐边柜",
    "cupboard": "橱柜",
    "shelf": "架子",
    "shelves": "架子",
    "rack": "架子",
    "hanger": "衣架",
    "coat_rack": "衣架",

    # 家电类
    "tv": "电视",
    "television": "电视",
    "refrigerator": "冰箱",
    "fridge": "冰箱",
    "washing_machine": "洗衣机",
    "washer": "洗衣机",
    "dryer": "烘干机",
    "air_conditioner": "空调",
    "microwave": "微波炉",
    "oven": "烤箱",
    "stove": "灶台",
    "hood": "油烟机",
    "dishwasher": "洗碗机",

    # 灯具类
    "lamp": "灯",
    "floor_lamp": "落地灯",
    "table_lamp": "台灯",
    "ceiling_lamp": "吊灯",
    "chandelier": "吊灯",
    "pendant": "吊灯",
    "sconce": "壁灯",
    "light": "灯",

    # 床品类
    "pillow": "枕头",
    "mattress": "床垫",
    "blanket": "毯子",
    "quilt": "被子",

    # 其他
    "door": "门",
    "window": "窗户",
    "plant": "植物",
    "rug": "地毯",
    "carpet": "地毯",
    "curtain": "窗帘",
    "picture": "画",
    "painting": "画",
    "mirror": "镜子",
    "clock": "钟",
    "tv": "电视",
    "bathtub": "浴缸",
    "shower": "淋浴",
    "toilet": "马桶",
    "sink": "洗手台",
    "basin": "洗手台",
    "faucet": "水龙头",
}


def extract_room_type(room_name: str) -> str:
    """从房间名提取房间类型"""
    room_lower = room_name.lower().replace("-", "").replace("_", "").replace(" ", "")

    for eng, cn in ROOM_MAPPING.items():
        if eng.replace("_", "").replace("-", "") in room_lower:
            return cn

    return "房间"


def normalize_furniture_name(name: str) -> str:
    """标准化家具名称"""
    name_lower = name.lower().replace("-", "_").replace(" ", "_")

    # 直接匹配
    if name_lower in FURNITURE_MAPPING:
        return FURNITURE_MAPPING[name_lower]

    # 部分匹配
    for eng, cn in FURNITURE_MAPPING.items():
        if eng in name_lower or name_lower in eng:
            return cn

    # 返回原名（清理后）
    return name.replace("_", " ").replace("-", " ")


def process_tar_gz():
    """处理 tar.gz 文件"""
    print("=" * 60)
    print("从 3D-FRONT-RENDER 提取真实家具信息")
    print("=" * 60)

    samples = []
    stats = {
        "total_scenes": 0,
        "total_rooms": 0,
        "rooms_with_meta": 0,
        "furniture_count": defaultdict(int),
        "room_type_count": defaultdict(int),
    }

    # 场景信息缓存
    scene_rooms = defaultdict(dict)

    print(f"\n读取: {TAR_PATH}")
    print("这可能需要几分钟...")

    try:
        with tarfile.open(TAR_PATH, 'r:gz') as tar:
            # 第一遍：收集所有文件信息
            print("\n[1/2] 扫描文件结构...")
            all_members = tar.getmembers()
            print(f"  总文件数: {len(all_members)}")

            # 找出所有 meta.json 文件
            meta_files = [m for m in all_members if m.name.endswith('meta.json')]
            print(f"  meta.json 文件: {len(meta_files)}")

            # 第二遍：提取 meta.json 内容
            print("\n[2/2] 提取家具信息...")

            for i, member in enumerate(meta_files):
                if (i + 1) % 500 == 0:
                    print(f"  进度: {i+1}/{len(meta_files)}")

                try:
                    f = tar.extractfile(member)
                    if f is None:
                        continue

                    content = f.read().decode('utf-8')
                    meta = json.loads(content)

                    # 解析路径: 3D-FRONT-RENDER/scene_id/RoomName-xxx/meta.json
                    parts = member.name.split('/')
                    if len(parts) >= 3:
                        scene_id = parts[1]
                        room_folder = parts[2]

                        # 提取房间名
                        room_name = room_folder.split('-')[0] if '-' in room_folder else room_folder
                        room_type = extract_room_type(room_name)

                        stats["room_type_count"][room_type] += 1

                        # 提取家具信息
                        furniture_list = []

                        # meta.json 可能包含 furniture 字段或 model 字段
                        if isinstance(meta, dict):
                            # 尝试不同的字段名
                            for key in ['furniture', 'models', 'objects', 'items', 'instances']:
                                if key in meta:
                                    items = meta[key]
                                    if isinstance(items, list):
                                        for item in items:
                                            if isinstance(item, dict):
                                                # 提取家具名
                                                for name_key in ['name', 'type', 'category', 'model_name', 'label']:
                                                    if name_key in item:
                                                        furn_name = item[name_key]
                                                        if furn_name:
                                                            cn_name = normalize_furniture_name(str(furn_name))
                                                            if cn_name and cn_name not in furniture_list:
                                                                furniture_list.append(cn_name)
                                                                stats["furniture_count"][cn_name] += 1
                                                        break

                            # 如果没有家具列表，使用房间类型推断
                            if not furniture_list:
                                # 从 roomType 或 type 字段获取
                                room_type_from_meta = meta.get('roomType', meta.get('type', ''))
                                if room_type_from_meta:
                                    room_type = extract_room_type(room_type_from_meta)

                        # 生成训练样本
                        if furniture_list:
                            stats["rooms_with_meta"] += 1

                            # 生成描述
                            if len(furniture_list) == 1:
                                text = f"{room_type}里有{furniture_list[0]}。"
                            elif len(furniture_list) == 2:
                                text = f"{room_type}里有{furniture_list[0]}和{furniture_list[1]}。"
                            else:
                                text = f"{room_type}里有{'、'.join(furniture_list[:-1])}和{furniture_list[-1]}。"

                            # 构建实体列表
                            entities = [(room_type, "building")]
                            for f in furniture_list[:5]:  # 最多5个家具
                                entities.append((f, "furniture"))

                            # 构建关系
                            relations = []
                            for f in furniture_list[:5]:
                                relations.append(f"{room_type}包含{f}")

                            sample = {
                                "id": f"3dfront-{scene_id}-{room_folder}",
                                "text": text,
                                "language": "zh",
                                "source": "3d-front-render-real",
                                "entities": entities,
                                "relations": relations,
                                "furniture": furniture_list,
                                "raw": {
                                    "scene_id": scene_id,
                                    "room_name": room_name,
                                    "room_type": room_type
                                }
                            }
                            samples.append(sample)
                            stats["total_rooms"] += 1

                        stats["total_scenes"] = len(set(s["raw"]["scene_id"] for s in samples))

                except Exception as e:
                    pass  # 跳过解析错误的文件

    except Exception as e:
        print(f"读取错误: {e}")
        return None

    # 保存结果
    print(f"\n提取完成!")
    print(f"  场景数: {stats['total_scenes']}")
    print(f"  房间数: {stats['total_rooms']}")
    print(f"  有效样本: {len(samples)}")

    output_data = {
        "version": "3d-front-render-real-1.0",
        "description": "Real furniture data extracted from 3D-FRONT-RENDER.tar.gz",
        "stats": {
            "scenes": stats["total_scenes"],
            "rooms": stats["total_rooms"],
            "samples": len(samples),
            "room_types": len(stats["room_type_count"]),
            "furniture_types": len(stats["furniture_count"])
        },
        "furniture_stats": dict(sorted(stats["furniture_count"].items(), key=lambda x: -x[1])[:50]),
        "room_type_stats": dict(sorted(stats["room_type_count"].items(), key=lambda x: -x[1])),
        "data": samples
    }

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT_PATH, 'w', encoding='utf-8') as f:
        json.dump(output_data, f, ensure_ascii=False, indent=2)

    size_mb = OUTPUT_PATH.stat().st_size / (1024 * 1024)
    print(f"\n保存到: {OUTPUT_PATH} ({size_mb:.1f} MB)")

    # 打印统计
    print("\n房间类型分布:")
    for rt, count in sorted(stats["room_type_count"].items(), key=lambda x: -x[1])[:10]:
        print(f"  {rt}: {count}")

    print("\n家具类型分布 (Top 20):")
    for fn, count in sorted(stats["furniture_count"].items(), key=lambda x: -x[1])[:20]:
        print(f"  {fn}: {count}")

    return OUTPUT_PATH


if __name__ == "__main__":
    process_tar_gz()
