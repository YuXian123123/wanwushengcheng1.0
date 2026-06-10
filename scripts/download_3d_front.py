#!/usr/bin/env python3
"""
下载 3D-FRONT 数据集

3D-FRONT 是阿里巴巴发布的3D家具和室内场景数据集，包含：
- 18,968 个房间布局
- 12,561 个家具模型
- 130+ 种家具类别

数据来源：
1. 官方：https://tianchi.aliyun.com/dataset/dataDetail?dataId=75242
2. HuggingFace镜像：https://huggingface.co/datasets/3d-front/3D-FRONT

安装依赖：
    pip install huggingface_hub requests tqdm
"""

import os
import json
import requests
from pathlib import Path
from tqdm import tqdm
import gzip
import shutil

# 数据集配置
DATASET_NAME = "3D-FRONT"
OUTPUT_DIR = Path("data/3d_front")
OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

# 已知的镜像源
MIRRORS = {
    "huggingface": {
        "repo": "3d-front/3D-FRONT",
        "files": [
            "3D-FRONT.tar.gz",
            "3D-FRONT-future-model-id.json",
            "3D-FRONT-future-model.json",
        ]
    },
    "tsinghua": {
        "base_url": "https://cloud.tsinghua.edu.cn/d/3d-front/",
        "files": ["3D-FRONT.tar.gz"]
    }
}

def check_disk_space(required_gb=30):
    """检查磁盘空间"""
    import shutil
    total, used, free = shutil.disk_usage(OUTPUT_DIR.parent)
    free_gb = free / (1024**3)

    print(f"磁盘空间: {free_gb:.1f} GB 可用")

    if free_gb < required_gb:
        print(f"⚠️  警告: 需要至少 {required_gb} GB，当前只有 {free_gb:.1f} GB")
        return False
    return True

def download_via_huggingface():
    """通过 HuggingFace 下载"""
    try:
        from huggingface_hub import snapshot_download, hf_hub_download
        from huggingface_hub.utils import RepositoryNotFoundError

        print("\n[方法1] 尝试从 HuggingFace 下载...")

        # 尝试下载整个仓库
        try:
            print(f"下载 3D-FRONT 数据集到 {OUTPUT_DIR}...")
            snapshot_download(
                repo_id="3d-front/3D-FRONT",
                repo_type="dataset",
                local_dir=OUTPUT_DIR,
                local_dir_use_symlinks=False,
                tqdm_class=tqdm
            )
            return True
        except RepositoryNotFoundError:
            print("HuggingFace 仓库未找到，尝试其他方法...")

        # 尝试其他可能的仓库
        alternative_repos = [
            "3d-front/3D-FRONT",
            "3dfront/3D-FRONT",
            "Tencent/3D-FRONT",
        ]

        for repo in alternative_repos:
            try:
                print(f"尝试仓库: {repo}")
                snapshot_download(
                    repo_id=repo,
                    repo_type="dataset",
                    local_dir=OUTPUT_DIR,
                    tqdm_class=tqdm
                )
                return True
            except Exception as e:
                print(f"  失败: {e}")
                continue

    except ImportError:
        print("需要安装 huggingface_hub: pip install huggingface_hub")
    except Exception as e:
        print(f"下载失败: {e}")

    return False

def download_via_direct_links():
    """通过直接链接下载"""
    print("\n[方法2] 尝试直接链接下载...")

    # 3D-FRONT 的已知下载链接
    direct_links = [
        # 清华镜像
        ("https://cloud.tsinghua.edu.cn/d/3d-front/?download=1", "3D-FRONT.tar.gz"),
        # 阿里云镜像
        ("https://tianchi.aliyun.com/dataset/dataDetail?dataId=75242", None),
    ]

    for url, filename in direct_links:
        if filename:
            try:
                print(f"下载: {url}")
                response = requests.get(url, stream=True, timeout=30)
                if response.status_code == 200:
                    output_path = OUTPUT_DIR / filename
                    total_size = int(response.headers.get('content-length', 0))

                    with open(output_path, 'wb') as f:
                        with tqdm(total=total_size, unit='B', unit_scale=True) as pbar:
                            for chunk in response.iter_content(chunk_size=8192):
                                f.write(chunk)
                                pbar.update(len(chunk))

                    print(f"✅ 下载完成: {output_path}")
                    return True
            except Exception as e:
                print(f"  失败: {e}")

    return False

def download_via_kaggle():
    """通过 Kaggle API 下载"""
    print("\n[方法3] 尝试从 Kaggle 下载...")

    try:
        import subprocess

        # 检查 kaggle 是否安装
        result = subprocess.run(['kaggle', '--version'], capture_output=True)
        if result.returncode != 0:
            print("需要安装 Kaggle: pip install kaggle")
            return False

        # 搜索相关数据集
        print("搜索 Kaggle 数据集...")
        result = subprocess.run(
            ['kaggle', 'datasets', 'list', '-s', '3d front furniture'],
            capture_output=True, text=True
        )
        print(result.stdout)

        # 如果找到，尝试下载
        # kaggle datasets download -d <dataset-name>

    except FileNotFoundError:
        print("Kaggle 未安装，跳过")
    except Exception as e:
        print(f"Kaggle 下载失败: {e}")

    return False

def create_synthetic_data():
    """
    如果无法下载真实数据集，创建合成数据
    基于 3D-FRONT 格式的训练样本
    """
    print("\n[备用方案] 创建合成训练数据...")

    furniture_types = {
        "furniture": ["chair", "table", "sofa", "bed", "desk", "couch", "bench", "shelf", "cabinet", "wardrobe", "drawer", "bookcase", "nightstand", "coffee_table", "dining_table"],
        "building": ["room", "bedroom", "living_room", "kitchen", "bathroom", "office", "dining_room", "study", "hallway", "balcony"],
        "object": ["lamp", "clock", "vase", "plant", "picture", "mirror", "rug", "curtain", "cushion", "book"],
        "appliance": ["tv", "refrigerator", "washing_machine", "air_conditioner", "heater", "fan"],
    }

    # 中文映射
    zh_mapping = {
        "chair": "椅子", "table": "桌子", "sofa": "沙发", "bed": "床",
        "desk": "书桌", "couch": "长沙发", "shelf": "架子", "cabinet": "柜子",
        "wardrobe": "衣柜", "drawer": "抽屉", "bookcase": "书架",
        "room": "房间", "bedroom": "卧室", "living_room": "客厅",
        "kitchen": "厨房", "bathroom": "浴室", "lamp": "灯", "tv": "电视",
        "refrigerator": "冰箱", "plant": "植物", "rug": "地毯",
    }

    # 生成训练样本
    samples = []
    templates = [
        "房间里有一个{zh}和一个{zh2}",
        "一个{zh}旁边有一个{zh}",
        "{zh}在{zh2}的旁边",
        "{zh}的上面有一个{zh2}",
        "这是一个有{zh}和{zh2}的房间",
    ]

    import random
    random.seed(42)

    for i in range(5000):  # 生成5000个样本
        # 随机选择家具类型
        cat1 = random.choice(list(furniture_types.keys()))
        item1 = random.choice(furniture_types[cat1])
        cat2 = random.choice(list(furniture_types.keys()))
        item2 = random.choice(furniture_types[cat2])

        zh1 = zh_mapping.get(item1, item1)
        zh2 = zh_mapping.get(item2, item2)

        template = random.choice(templates)
        text = template.format(zh=zh1, zh2=zh2)

        sample = {
            "id": f"syn_{i:05d}",
            "text": text,
            "language": "zh",
            "entities": [
                {"id": "e1", "name": zh1, "type": cat1},
                {"id": "e2", "name": zh2, "type": cat2},
            ],
            "relations": [
                {"from": "e1", "to": "e2", "type": random.choice(["adjacent", "on", "contains"])}
            ],
            "layout_3d": {
                "root": "e1",
                "nodes": [
                    {
                        "entity_id": "e1",
                        "position": [0, 0, 0],
                        "scale": [1.0, 1.0, 1.0],
                        "geometry": "box"
                    },
                    {
                        "entity_id": "e2",
                        "position": [1.5, 0, 0],
                        "scale": [0.8, 0.8, 0.8],
                        "geometry": "box"
                    }
                ]
            }
        }
        samples.append(sample)

    # 保存
    output_file = OUTPUT_DIR / "synthetic_chinese_scenes.json"
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump({
            "version": "1.0",
            "description": "Synthetic Chinese furniture scenes",
            "count": len(samples),
            "data": samples
        }, f, ensure_ascii=False, indent=2)

    print(f"[OK] 创建了 {len(samples)} 个合成样本: {output_file}")
    return True

def main():
    print("=" * 60)
    print("3D-FRONT 数据集下载器")
    print("=" * 60)

    # 检查空间
    if not check_disk_space(30):
        print("磁盘空间不足，将创建合成数据代替")
        create_synthetic_data()
        return

    # 尝试各种下载方法
    success = False

    # 方法1: HuggingFace
    if not success:
        success = download_via_huggingface()

    # 方法2: 直接链接
    if not success:
        success = download_via_direct_links()

    # 方法3: Kaggle
    if not success:
        success = download_via_kaggle()

    # 备用: 创建合成数据
    if not success:
        print("\n无法下载原始数据集，创建合成数据...")
        create_synthetic_data()

    print("\n" + "=" * 60)
    print("完成！数据位置:", OUTPUT_DIR)
    print("=" * 60)

if __name__ == "__main__":
    main()
