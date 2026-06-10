#!/usr/bin/env python3
"""
从 HuggingFace 下载 3D-Front 数据集

数据集信息：
- 名称: huanngzh/3D-Front
- 内容: 3D家具场景 (.glb) + 点云 (.npy) + 多视角渲染图
- 来源: 阿里巴巴 3D-FRONT 数据集

安装依赖:
    pip install huggingface_hub tqdm
"""

import os
import subprocess
from pathlib import Path

OUTPUT_DIR = Path("data/3d_front_hf")

def download_via_git_lfs():
    """使用 git lfs 下载（推荐用于大文件）"""
    print("=" * 60)
    print("下载 3D-Front 数据集 (HuggingFace)")
    print("=" * 60)

    # 创建目录
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    # 检查 git lfs 是否安装
    try:
        result = subprocess.run(['git', 'lfs', 'version'], capture_output=True, text=True)
        if result.returncode != 0:
            print("需要安装 git-lfs:")
            print("  Windows: winget install git-lfs")
            print("  Linux: sudo apt-get install git-lfs")
            return False
    except FileNotFoundError:
        print("git 未安装")
        return False

    # 初始化 git lfs
    subprocess.run(['git', 'lfs', 'install'], capture_output=True)

    # 克隆仓库
    repo_url = "https://huggingface.co/datasets/huanngzh/3D-Front"

    print(f"\n克隆仓库: {repo_url}")
    print(f"目标目录: {OUTPUT_DIR}")
    print("注意: 这是一个大型数据集，下载可能需要较长时间...\n")

    try:
        # 使用 git clone
        result = subprocess.run(
            ['git', 'clone', repo_url, str(OUTPUT_DIR)],
            capture_output=False,
            text=True
        )

        if result.returncode == 0:
            print("\n下载完成!")
            return True
        else:
            print(f"\n下载失败: {result.stderr}")
            return False

    except Exception as e:
        print(f"错误: {e}")
        return False

def download_via_huggingface_hub():
    """使用 huggingface_hub 下载"""
    try:
        from huggingface_hub import snapshot_download
        from tqdm import tqdm

        print("\n使用 huggingface_hub 下载...")

        snapshot_download(
            repo_id="huanngzh/3D-Front",
            repo_type="dataset",
            local_dir=OUTPUT_DIR,
            tqdm_class=tqdm
        )
        return True

    except ImportError:
        print("需要安装 huggingface_hub: pip install huggingface_hub")
        return False
    except Exception as e:
        print(f"下载失败: {e}")
        return False

def check_download():
    """检查下载结果"""
    if not OUTPUT_DIR.exists():
        return False

    # 检查文件数量
    files = list(OUTPUT_DIR.rglob("*"))
    file_count = len([f for f in files if f.is_file()])

    # 检查目录大小
    total_size = sum(f.stat().st_size for f in files if f.is_file())
    size_mb = total_size / (1024 * 1024)

    print(f"\n下载统计:")
    print(f"  文件数: {file_count}")
    print(f"  总大小: {size_mb:.1f} MB")

    # 检查是否有 .glb 文件
    glb_files = list(OUTPUT_DIR.rglob("*.glb"))
    print(f"  3D模型: {len(glb_files)} 个")

    return file_count > 0

def main():
    print("3D-Front 数据集下载器")
    print("=" * 60)

    # 方法1: git lfs
    success = download_via_git_lfs()

    # 方法2: huggingface_hub
    if not success:
        success = download_via_huggingface_hub()

    # 检查结果
    if check_download():
        print("\n[成功] 数据集已下载到:", OUTPUT_DIR)
    else:
        print("\n[失败] 数据集下载失败")

if __name__ == "__main__":
    main()
