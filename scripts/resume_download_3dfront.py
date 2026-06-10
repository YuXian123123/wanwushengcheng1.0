#!/usr/bin/env python3
"""
重新下载 3D-Front 数据集（使用断点续传）

使用 huggingface_hub 的 resume_download 功能
"""

import os
from pathlib import Path

def download_with_resume():
    """使用断点续传下载"""
    print("=" * 60)
    print("继续下载 3D-Front 数据集")
    print("=" * 60)

    try:
        from huggingface_hub import snapshot_download
        from tqdm import tqdm

        output_dir = Path("data/3d_front_hf")

        print("\n使用断点续传下载...")

        snapshot_download(
            repo_id="huanngzh/3D-Front",
            repo_type="dataset",
            local_dir=output_dir,
            resume_download=True,  # 启用断点续传
            tqdm_class=tqdm
        )

        print("\n下载完成!")
        return True

    except Exception as e:
        print(f"下载失败: {e}")
        return False


def check_download():
    """检查下载结果"""
    output_dir = Path("data/3d_front_hf")

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
    success = download_with_resume()
    if success:
        check_download()
