#!/usr/bin/env python3
"""
在 E: 盘下载 3D-Front 数据集
"""

import os
from pathlib import Path

def download_to_e_drive():
    """下载到 E: 盘"""
    print("=" * 60)
    print("下载 3D-Front 数据集到 E: 盘")
    print("=" * 60)

    output_dir = Path("E:/ai_006_data/3d_front_hf")
    output_dir.mkdir(parents=True, exist_ok=True)

    try:
        from huggingface_hub import snapshot_download
        from tqdm import tqdm

        print(f"\n目标目录: {output_dir}")
        print("开始下载...\n")

        snapshot_download(
            repo_id="huanngzh/3D-Front",
            repo_type="dataset",
            local_dir=output_dir,
            tqdm_class=tqdm
        )

        print("\n下载完成!")
        return True

    except Exception as e:
        print(f"下载失败: {e}")
        return False


def check_download():
    """检查下载结果"""
    output_dir = Path("E:/ai_006_data/3d_front_hf")

    if not output_dir.exists():
        return False

    # 统计文件
    all_files = list(output_dir.rglob("*"))
    files = [f for f in all_files if f.is_file() and not f.name.endswith(('.incomplete', '.lock', '.metadata'))]

    total_size = sum(f.stat().st_size for f in files)
    size_gb = total_size / (1024**3)

    print(f"\n下载统计:")
    print(f"  文件数: {len(files)}")
    print(f"  总大小: {size_gb:.1f} GB")

    # 列出主要文件
    print("\n主要文件:")
    for f in sorted(files, key=lambda x: x.stat().st_size, reverse=True)[:10]:
        size_mb = f.stat().st_size / (1024**2)
        print(f"  {f.name}: {size_mb:.1f} MB")

    return len(files) > 0


if __name__ == "__main__":
    success = download_to_e_drive()
    if success:
        check_download()
