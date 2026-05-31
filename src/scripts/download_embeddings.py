#!/usr/bin/env python3
"""
下载词向量模型脚本

支持的模型：
1. fastText 中文词向量 (cc.zh.300.vec.gz) - 约 1.2GB 压缩，解压后约 4GB
2. fastText 多语言词向量 (cc.en.300.vec.gz) - 约 1.5GB 压缩
3. 简化测试数据 - 小型测试集

用法：
    python download_embeddings.py --model fasttext-zh
    python download_embeddings.py --model fasttext-en
    python download_embeddings.py --model test
    python download_embeddings.py --model all
"""

import argparse
import gzip
import os
import urllib.request
import shutil
import json
from pathlib import Path


# 模型配置
MODELS = {
    "fasttext-zh": {
        "url": "https://dl.fbaipublicfiles.com/fasttext/vectors-crawl/cc.zh.300.vec.gz",
        "filename": "cc.zh.300.vec.gz",
        "output": "cc.zh.300.vec",
        "description": "fastText 中文词向量 (300维，约200万词)",
        "dimension": 300,
    },
    "fasttext-en": {
        "url": "https://dl.fbaipublicfiles.com/fasttext/vectors-crawl/cc.en.300.vec.gz",
        "filename": "cc.en.300.vec.gz",
        "output": "cc.en.300.vec",
        "description": "fastText 英文词向量 (300维，约200万词)",
        "dimension": 300,
    },
    "fasttext-zh-small": {
        "url": "https://dl.fbaipublicfiles.com/fasttext/vectors-wiki-news-300d-1M.vec.zip",
        "filename": "wiki-news-300d-1M.vec.zip",
        "output": "wiki-news-300d-1M.vec",
        "description": "fastText 多语言词向量 (300维，约100万词)",
        "dimension": 300,
    },
}


def download_file(url: str, dest: Path, desc: str = None):
    """下载文件并显示进度"""
    print(f"下载: {url}")
    print(f"目标: {dest}")
    if desc:
        print(f"说明: {desc}")

    def report_hook(count, block_size, total_size):
        percent = int(count * block_size * 100 / total_size)
        mb_downloaded = count * block_size / (1024 * 1024)
        mb_total = total_size / (1024 * 1024)
        print(f"\r进度: {percent}% ({mb_downloaded:.1f}MB / {mb_total:.1f}MB)", end="", flush=True)

    urllib.request.urlretrieve(url, dest, reporthook=report_hook)
    print()  # 换行


def extract_gz(gz_path: Path, output_path: Path):
    """解压 .gz 文件"""
    print(f"解压: {gz_path} -> {output_path}")
    with gzip.open(gz_path, 'rb') as f_in:
        with open(output_path, 'wb') as f_out:
            shutil.copyfileobj(f_in, f_out)
    print("解压完成")


def create_test_embedding(output_path: Path):
    """创建测试用的小型词向量文件"""
    print("创建测试词向量...")

    # 一些简单的测试词向量 (10维)
    test_words = {
        # 数词
        "一": [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "二": [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "三": [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],

        # 动物
        "猫": [0.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "狗": [0.85, 0.15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "鸟": [0.0, 0.0, 0.8, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "鱼": [0.0, 0.0, 0.75, 0.25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],

        # 颜色
        "红": [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "蓝": [0.0, 0.0, 0.0, 0.0, 0.9, 0.1, 0.0, 0.0, 0.0, 0.0],
        "绿": [0.0, 0.0, 0.0, 0.0, 0.85, 0.15, 0.0, 0.0, 0.0, 0.0],

        # 动作
        "跑": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        "跳": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 0.1, 0.0, 0.0],
        "飞": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.85, 0.15, 0.0, 0.0],

        # 情感
        "爱": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        "恨": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0],
        "喜": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 0.1],
        "怒": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.8, 0.2],

        # 人物
        "人": [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
        "男": [0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 0.0, 0.0, 0.0, 0.1],
        "女": [0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 0.0, 0.0, 0.0, -0.1],

        # 自然
        "天": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        "地": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 0.0, 0.0],
        "水": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        "火": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9],

        # 蛊虫相关
        "蛊": [0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5],
        "虫": [0.3, 0.3, 0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "毒": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 0.5],
        "灵": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.8, 0.8],

        # 世界相关
        "世界": [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
        "智能": [0.6, 0.6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.6, 0.6],
        "意识": [0.0, 0.0, 0.7, 0.7, 0.0, 0.0, 0.0, 0.0, 0.7, 0.7],
        "神经网络": [0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4],
    }

    # 写入 fastText .vec 格式
    with open(output_path, 'w', encoding='utf-8') as f:
        dim = 10
        f.write(f"{len(test_words)} {dim}\n")
        for word, vec in test_words.items():
            vec_str = ' '.join(f"{v:.6f}" for v in vec)
            f.write(f"{word} {vec_str}\n")

    print(f"创建完成: {output_path} ({len(test_words)} 词, {dim} 维)")


def create_code_test_embedding(output_path: Path):
    """创建代码测试词向量"""
    print("创建代码测试词向量...")

    # 编程相关词汇 (16维)
    code_words = {
        # Rust 关键字
        "fn": [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "let": [0.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "mut": [0.85, 0.15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "impl": [0.8, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "struct": [0.75, 0.25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "enum": [0.7, 0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "trait": [0.65, 0.35, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],

        # 常用类型
        "String": [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "Vec": [0.0, 0.0, 0.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "Option": [0.0, 0.0, 0.8, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "Result": [0.0, 0.0, 0.7, 0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "HashMap": [0.0, 0.0, 0.6, 0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],

        # 控制流
        "if": [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "else": [0.0, 0.0, 0.0, 0.0, 0.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "match": [0.0, 0.0, 0.0, 0.0, 0.8, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "for": [0.0, 0.0, 0.0, 0.0, 0.7, 0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "loop": [0.0, 0.0, 0.0, 0.0, 0.6, 0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],

        # 异步
        "async": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "await": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],

        # 安全
        "unsafe": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "pub": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],

        # 神经网络相关
        "neuron": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "synapse": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 0.1, 0.0, 0.0, 0.0, 0.0],
        "weight": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.8, 0.2, 0.0, 0.0, 0.0, 0.0],
        "embedding": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
    }

    # 写入 fastText .vec 格式
    with open(output_path, 'w', encoding='utf-8') as f:
        dim = 16
        f.write(f"{len(code_words)} {dim}\n")
        for word, vec in code_words.items():
            vec_str = ' '.join(f"{v:.6f}" for v in vec)
            f.write(f"{word} {vec_str}\n")

    print(f"创建完成: {output_path} ({len(code_words)} 词, {dim} 维)")


def create_config_file(output_dir: Path):
    """创建配置文件"""
    config = {
        "text": {
            "dimension": 300,
            "path": "cc.zh.300.vec",
            "source_type": "FastText"
        },
        "code": {
            "dimension": 256,
            "path": "code.vec",
            "source_type": "Multimodal"
        },
        "image": {
            "dimension": 512,
            "path": "image.vec",
            "source_type": "Multimodal"
        },
        "audio": {
            "dimension": 256,
            "path": "audio.vec",
            "source_type": "Multimodal"
        },
        "video": {
            "dimension": 512,
            "path": "video.vec",
            "source_type": "Multimodal"
        },
        "unified_dimension": 512,
        "cross_modal_alignment": True,
        "alignment_temperature": 0.07
    }

    config_path = output_dir / "embedding_config.json"
    with open(config_path, 'w', encoding='utf-8') as f:
        json.dump(config, f, indent=2, ensure_ascii=False)

    print(f"配置文件: {config_path}")


def main():
    parser = argparse.ArgumentParser(description="下载词向量模型")
    parser.add_argument(
        "--model",
        choices=list(MODELS.keys()) + ["test", "all"],
        default="test",
        help="要下载的模型",
    )
    parser.add_argument(
        "--output-dir",
        default="data/embeddings",
        help="输出目录",
    )
    parser.add_argument(
        "--keep-compressed",
        action="store_true",
        help="保留压缩文件",
    )

    args = parser.parse_args()

    # 创建输出目录
    output_dir = Path(args.output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    if args.model == "test":
        # 创建测试数据
        create_test_embedding(output_dir / "test.vec")
        create_code_test_embedding(output_dir / "code.vec")
        create_config_file(output_dir)
        return

    if args.model == "all":
        # 下载所有模型
        for model_name in MODELS.keys():
            download_model(model_name, output_dir, args.keep_compressed)
    else:
        download_model(args.model, output_dir, args.keep_compressed)

    # 创建配置文件
    create_config_file(output_dir)

    print(f"\n完成! 词向量文件目录: {output_dir}")


def download_model(model_name: str, output_dir: Path, keep_compressed: bool):
    """下载指定模型"""
    model = MODELS[model_name]
    gz_path = output_dir / model["filename"]
    vec_path = output_dir / model["output"]

    # 检查是否已存在
    if vec_path.exists():
        print(f"文件已存在: {vec_path}")
        return

    # 下载
    download_file(model["url"], gz_path, model.get("description"))

    # 解压
    if gz_path.suffix == ".gz":
        extract_gz(gz_path, vec_path)

    # 删除压缩文件
    if not keep_compressed and gz_path.exists():
        os.remove(gz_path)
        print(f"已删除: {gz_path}")


if __name__ == "__main__":
    main()
