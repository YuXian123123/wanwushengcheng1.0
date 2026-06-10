#!/usr/bin/env python3
"""快速提取 - 解压部分文件"""

import os
import json
import tarfile
import shutil
from pathlib import Path
from collections import defaultdict

TAR_PATH = Path("E:/ai_006_data/3d_front_hf/3D-FRONT-RENDER.tar.gz")
TEMP_DIR = Path("E:/ai_006_data/temp_meta")
OUTPUT_PATH = Path("D:/ai_006/data/training/scenes_3dfront_real.json")

# 房间和家具映射
ROOM_MAP = {
    "masterbedroom": "主卧室", "secondbedroom": "次卧室", "bedroom": "卧室",
    "livingroom": "客厅", "living_room": "客厅", "kitchen": "厨房",
    "bathroom": "浴室", "diningroom": "餐厅", "study": "书房",
    "office": "办公室", "childrenroom": "儿童房", "guestroom": "客房",
}

FURNITURE_MAP = {
    "bed": "床", "nightstand": "床头柜", "wardrobe": "衣柜",
    "table": "桌子", "desk": "书桌", "coffee_table": "茶几",
    "chair": "椅子", "sofa": "沙发", "armchair": "扶手椅",
    "cabinet": "柜子", "bookshelf": "书架", "tv_stand": "电视柜",
    "lamp": "灯", "mirror": "镜子", "rug": "地毯",
    "tv": "电视", "refrigerator": "冰箱", "plant": "植物",
}

def get_room_type(name):
    n = name.lower().replace("-", "").replace("_", "")
    for eng, cn in ROOM_MAP.items():
        if eng.replace("_", "") in n:
            return cn
    return "房间"

def get_furniture(name):
    n = name.lower().replace("-", "_").replace(" ", "_")
    for eng, cn in FURNITURE_MAP.items():
        if eng in n:
            return cn
    return None

def main():
    print("=" * 60)
    print("快速提取家具信息")
    print("=" * 60)
    
    TEMP_DIR.mkdir(parents=True, exist_ok=True)
    
    samples = []
    furniture_stats = defaultdict(int)
    room_stats = defaultdict(int)
    
    print("\n扫描 tar 文件...")
    
    with tarfile.open(TAR_PATH, 'r:gz') as tar:
        # 获取所有 meta.json 文件
        meta_files = [m for m in tar.getmembers() if m.name.endswith('meta.json')]
        print(f"找到 {len(meta_files)} 个 meta.json 文件")
        
        for i, member in enumerate(meta_files):
            if (i + 1) % 200 == 0:
                print(f"  处理: {i+1}/{len(meta_files)}")
            
            try:
                # 解析路径
                parts = member.name.split('/')
                if len(parts) < 3:
                    continue
                
                scene_id = parts[1]
                room_folder = parts[2]
                room_name = room_folder.split('-')[0] if '-' in room_folder else room_folder
                room_type = get_room_type(room_name)
                room_stats[room_type] += 1
                
                # 读取 meta.json
                f = tar.extractfile(member)
                if f is None:
                    continue
                
                meta = json.load(f)
                
                # 提取家具
                furniture_list = []
                
                # 遍历可能的字段
                for key in ['furniture', 'models', 'objects', 'items']:
                    if key in meta and isinstance(meta[key], list):
                        for item in meta[key]:
                            if isinstance(item, dict):
                                for name_key in ['name', 'type', 'category', 'model_name']:
                                    if name_key in item:
                                        fname = str(item[name_key])
                                        cn_name = get_furniture(fname)
                                        if cn_name and cn_name not in furniture_list:
                                            furniture_list.append(cn_name)
                                            furniture_stats[cn_name] += 1
                                        break
                
                # 生成样本
                if furniture_list:
                    text = f"{room_type}里有{'和'.join(furniture_list[:3])}。"
                    
                    sample = {
                        "id": f"3dfront-{scene_id}-{room_folder}",
                        "text": text,
                        "room_type": room_type,
                        "furniture": furniture_list,
                    }
                    samples.append(sample)
                    
            except Exception as e:
                pass
    
    # 保存
    print(f"\n提取了 {len(samples)} 个样本")
    
    output = {
        "version": "3dfront-real",
        "samples": len(samples),
        "room_stats": dict(room_stats),
        "furniture_stats": dict(furniture_stats),
        "data": samples
    }
    
    with open(OUTPUT_PATH, 'w', encoding='utf-8') as f:
        json.dump(output, f, ensure_ascii=False, indent=2)
    
    print(f"保存到: {OUTPUT_PATH}")
    print(f"\n房间分布: {dict(list(room_stats.items())[:5])}")
    print(f"家具分布: {dict(list(furniture_stats.items())[:10])}")

if __name__ == "__main__":
    main()
