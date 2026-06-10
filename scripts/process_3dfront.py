#!/usr/bin/env python3
"""
处理 3D-Front 数据集并转换为训练格式

3D-Front 数据集结构：
- 3D-FRONT-SCENE.tar.gz: 场景描述文件 (JSON)
- 3D-FRONT-RENDER.tar.gz: 渲染图片
- 3D-FRONT-model: 3D 模型文件 (.glb)

输出格式：
- 实体类型 (entity_types): 家具类型统计
- 关系模式 (relation_patterns): 空间关系
- 几何模板 (geometry_templates): 3D 几何信息
"""

import os
import json
import tarfile
import shutil
from pathlib import Path
from collections import defaultdict
from typing import Dict, List, Any, Set

INPUT_DIR = Path("data/3d_front_hf")
OUTPUT_DIR = Path("data/training")
TEMP_DIR = Path("data/3d_front_temp")

# 家具类型映射（英文 -> 中文）
FURNITURE_MAPPING = {
    # 床类
    "bed": "床",
    "double_bed": "双人床",
    "single_bed": "单人床",
    "king_bed": "大床",

    # 桌子类
    "table": "桌子",
    "dining_table": "餐桌",
    "coffee_table": "茶几",
    "desk": "书桌",
    "side_table": "边桌",
    "nightstand": "床头柜",

    # 椅子类
    "chair": "椅子",
    "armchair": "扶手椅",
    "dining_chair": "餐椅",
    "office_chair": "办公椅",
    "stool": "凳子",
    "sofa": "沙发",
    "couch": "长沙发",

    # 柜类
    "cabinet": "柜子",
    "wardrobe": "衣柜",
    "bookshelf": "书架",
    "bookcase": "书柜",
    "tv_stand": "电视柜",
    "drawer": "抽屉柜",
    "dresser": "梳妆台",

    # 家电类
    "tv": "电视",
    "television": "电视",
    "refrigerator": "冰箱",
    "fridge": "冰箱",
    "washing_machine": "洗衣机",
    "air_conditioner": "空调",
    "microwave": "微波炉",

    # 灯具类
    "lamp": "灯",
    "floor_lamp": "落地灯",
    "table_lamp": "台灯",
    "ceiling_lamp": "吊灯",
    "chandelier": "吊灯",

    # 房间类
    "room": "房间",
    "bedroom": "卧室",
    "living_room": "客厅",
    "kitchen": "厨房",
    "bathroom": "浴室",
    "dining_room": "餐厅",
    "study": "书房",

    # 其他
    "door": "门",
    "window": "窗户",
    "plant": "植物",
    "rug": "地毯",
    "curtain": "窗帘",
    "picture": "画",
    "mirror": "镜子",
}

# 空间关系映射
SPATIAL_RELATIONS = {
    "inside": "里面",
    "outside": "外面",
    "next_to": "旁边",
    "near": "附近",
    "far": "远离",
    "left": "左边",
    "right": "右边",
    "front": "前面",
    "behind": "后面",
    "above": "上面",
    "below": "下面",
    "on": "上面",
    "under": "下面",
    "between": "中间",
    "corner": "角落",
    "center": "中心",
    "wall": "靠墙",
}


class Front3DProcessor:
    """3D-Front 数据处理器"""

    def __init__(self):
        self.entity_types: Dict[str, Dict] = defaultdict(lambda: {"count": 0, "examples": []})
        self.relation_patterns: Dict[str, Dict] = defaultdict(lambda: {"count": 0, "examples": []})
        self.geometry_templates: Dict[str, Dict] = defaultdict(lambda: {"count": 0, "examples": []})
        self.processed_scenes: int = 0
        self.all_samples: List[Dict] = []

    def extract_tar_files(self) -> List[Path]:
        """解压 tar.gz 文件"""
        extracted = []
        TEMP_DIR.mkdir(parents=True, exist_ok=True)

        # 查找 tar.gz 文件
        for tar_file in INPUT_DIR.rglob("*.tar.gz"):
            print(f"解压: {tar_file.name}")
            try:
                with tarfile.open(tar_file, 'r:gz') as tar:
                    tar.extractall(TEMP_DIR)
                extracted.append(tar_file)
            except Exception as e:
                print(f"  解压失败: {e}")

        return extracted

    def process_scene_json(self, json_path: Path) -> Dict:
        """处理单个场景 JSON 文件"""
        try:
            with open(json_path, 'r', encoding='utf-8') as f:
                data = json.load(f)
        except Exception as e:
            return {}

        scene_info = {
            "uid": data.get("uid", json_path.stem),
            "furniture": [],
            "rooms": [],
            "relations": []
        }

        # 提取房间信息
        if "scene" in data:
            scene = data["scene"]

            # 处理房间
            for room in scene.get("room", []):
                room_id = room.get("id", "")
                room_type = room.get("type", "room").lower()

                # 映射到中文
                cn_type = FURNITURE_MAPPING.get(room_type, room_type)
                scene_info["rooms"].append({
                    "id": room_id,
                    "type": cn_type,
                    "origin": room_type
                })

                # 更新实体统计
                self.entity_types[cn_type]["count"] += 1
                self.entity_types[cn_type]["examples"].append(scene_info["uid"])

                # 处理房间内的家具
                for furniture in room.get("furniture", []):
                    furn_id = furniture.get("id", "")
                    furn_type = furniture.get("category", furniture.get("type", "object")).lower()

                    # 清理类型名称
                    furn_type = furn_type.replace("-", "_").replace(" ", "_")

                    # 映射到中文
                    cn_furn = FURNITURE_MAPPING.get(furn_type, furn_type)

                    # 提取位置信息
                    pos = furniture.get("pos", [0, 0, 0])
                    scale = furniture.get("scale", [1, 1, 1])
                    rot = furniture.get("rot", [0, 0, 0])

                    furn_info = {
                        "id": furn_id,
                        "type": cn_furn,
                        "origin": furn_type,
                        "room": room_id,
                        "position": pos,
                        "scale": scale,
                        "rotation": rot
                    }
                    scene_info["furniture"].append(furn_info)

                    # 更新实体统计
                    self.entity_types[cn_furn]["count"] += 1
                    if len(self.entity_types[cn_furn]["examples"]) < 10:
                        self.entity_types[cn_furn]["examples"].append(scene_info["uid"])

                    # 添加包含关系
                    relation = f"{cn_type}包含{cn_furn}"
                    self.relation_patterns[relation]["count"] += 1
                    scene_info["relations"].append({
                        "type": "contains",
                        "from": room_id,
                        "to": furn_id,
                        "description": relation
                    })

                    # 提取几何模板
                    geom_key = f"{cn_furn}_template"
                    self.geometry_templates[geom_key]["count"] += 1
                    if len(self.geometry_templates[geom_key]["examples"]) < 5:
                        self.geometry_templates[geom_key]["examples"].append({
                            "scale": scale,
                            "scene": scene_info["uid"]
                        })

        return scene_info

    def generate_training_sample(self, scene_info: Dict) -> Dict:
        """从场景信息生成训练样本"""
        if not scene_info.get("furniture") and not scene_info.get("rooms"):
            return {}

        # 构建自然语言描述
        descriptions = []

        # 房间描述
        for room in scene_info.get("rooms", []):
            room_type = room["type"]
            furniture_in_room = [f for f in scene_info.get("furniture", []) if f.get("room") == room["id"]]

            if furniture_in_room:
                furn_types = [f["type"] for f in furniture_in_room[:5]]  # 最多5个
                if len(furniture_in_room) > 5:
                    furn_types.append(f"等{len(furniture_in_room)}件家具")

                desc = f"{room_type}里有" + "和".join(furn_types)
                descriptions.append(desc)

        # 如果没有房间信息，直接描述家具
        if not descriptions and scene_info.get("furniture"):
            furn_types = [f["type"] for f in scene_info["furniture"][:5]]
            desc = "场景中有" + "和".join(furn_types)
            descriptions.append(desc)

        text = "。".join(descriptions) + "。"

        # 提取实体
        entities = []
        for room in scene_info.get("rooms", []):
            entities.append((room["type"], "building"))
        for furn in scene_info.get("furniture", []):
            entities.append((furn["type"], "furniture"))

        # 提取关系
        relations = []
        for rel in scene_info.get("relations", []):
            relations.append(rel["description"])

        # 构建几何信息
        geometry = {}
        for furn in scene_info.get("furniture", []):
            if furn["type"] not in geometry:
                geometry[furn["type"]] = {
                    "scale": furn.get("scale", [1, 1, 1]),
                    "count": 1
                }
            else:
                geometry[furn["type"]]["count"] += 1

        return {
            "id": scene_info["uid"],
            "text": text,
            "language": "zh",
            "source": "3d-front",
            "entities": entities,
            "relations": relations,
            "geometry": geometry,
            "raw": {
                "rooms": len(scene_info.get("rooms", [])),
                "furniture": len(scene_info.get("furniture", []))
            }
        }

    def process_all(self):
        """处理所有数据"""
        print("=" * 60)
        print("处理 3D-Front 数据集")
        print("=" * 60)

        # 1. 解压文件
        print("\n[1/4] 解压 tar.gz 文件...")
        extracted = self.extract_tar_files()
        print(f"  解压了 {len(extracted)} 个文件")

        # 2. 处理 JSON 场景文件
        print("\n[2/4] 处理场景文件...")

        # 查找所有 JSON 文件
        json_files = list(TEMP_DIR.rglob("*.json"))
        json_files.extend(list(INPUT_DIR.rglob("*.json")))

        # 去重
        json_files = list(set(json_files))

        print(f"  找到 {len(json_files)} 个 JSON 文件")

        for i, json_file in enumerate(json_files):
            if (i + 1) % 100 == 0:
                print(f"  处理进度: {i+1}/{len(json_files)}")

            scene_info = self.process_scene_json(json_file)
            if scene_info:
                sample = self.generate_training_sample(scene_info)
                if sample:
                    self.all_samples.append(sample)
                    self.processed_scenes += 1

        print(f"  处理了 {self.processed_scenes} 个有效场景")

        # 3. 生成统计
        print("\n[3/4] 生成统计...")
        print(f"  实体类型: {len(self.entity_types)}")
        print(f"  关系模式: {len(self.relation_patterns)}")
        print(f"  几何模板: {len(self.geometry_templates)}")
        print(f"  训练样本: {len(self.all_samples)}")

        # 4. 保存结果
        print("\n[4/4] 保存结果...")

        # 保存训练数据
        output_file = OUTPUT_DIR / "scenes_3dfront.json"
        output_data = {
            "version": "3d-front-1.0",
            "description": "3D-Front dataset converted to training format",
            "stats": {
                "scenes": self.processed_scenes,
                "entity_types": len(self.entity_types),
                "relation_patterns": len(self.relation_patterns),
                "geometry_templates": len(self.geometry_templates)
            },
            "data": self.all_samples
        }

        with open(output_file, 'w', encoding='utf-8') as f:
            json.dump(output_data, f, ensure_ascii=False, indent=2)

        size_mb = output_file.stat().st_size / (1024 * 1024)
        print(f"  保存到: {output_file} ({size_mb:.1f} MB)")

        # 保存学习模式
        patterns_file = OUTPUT_DIR / "patterns_3dfront.json"
        patterns_data = {
            "entity_types": dict(self.entity_types),
            "relation_patterns": dict(self.relation_patterns),
            "geometry_templates": dict(self.geometry_templates)
        }

        with open(patterns_file, 'w', encoding='utf-8') as f:
            json.dump(patterns_data, f, ensure_ascii=False, indent=2)

        print(f"  模式保存到: {patterns_file}")

        # 清理临时目录
        if TEMP_DIR.exists():
            shutil.rmtree(TEMP_DIR)
            print("  清理临时目录")

        return output_file

    def print_top_entities(self, n: int = 20):
        """打印最常见的实体类型"""
        print(f"\nTop {n} 实体类型:")
        sorted_entities = sorted(
            self.entity_types.items(),
            key=lambda x: x[1]["count"],
            reverse=True
        )[:n]

        for entity, info in sorted_entities:
            print(f"  {entity}: {info['count']}")


def main():
    processor = Front3DProcessor()
    output_file = processor.process_all()
    processor.print_top_entities()

    print("\n" + "=" * 60)
    print("处理完成!")
    print("=" * 60)
    print(f"\n输出文件: {output_file}")
    print("\n下一步: 运行 merge_with_3dfront.py 合并数据并重新训练")


if __name__ == "__main__":
    main()
