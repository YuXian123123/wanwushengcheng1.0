#!/usr/bin/env python3
"""
学习 UltraChat 数据集

将 HuggingFace UltraChat 数据集转换为蛊虫可学习的格式，
通过 HTTP API 发送给世界模型学习。

用法:
    python learn_ultrachat.py [--limit N] [--file FILE]

示例:
    # 学习前100条对话
    python learn_ultrachat.py --limit 100

    # 学习特定文件
    python learn_ultrachat.py --file test_sft-00000-of-00001-f7dfac4afe5b93f4.parquet
"""

import os
import sys
import json
import argparse
import requests
from pathlib import Path

try:
    import pyarrow.parquet as pq
    import pandas as pd
except ImportError:
    print("请安装依赖: pip install pyarrow pandas requests")
    sys.exit(1)

# 数据集目录
DATA_DIR = Path(r"D:\数据集\datasets--HuggingFaceH4--ultrachat_200k\snapshots\8049631c405ae6576f93f445c6b8166f76f5505a\data")

# API 端点
API_BASE = "http://localhost:9000"


def convert_to_learnable_text(row) -> str:
    """将对话数据转换为可学习的文本格式"""
    prompt = row.get("prompt", "")
    messages = row.get("messages", [])

    # 构建学习内容
    parts = []

    # 添加主题/问题
    if prompt:
        parts.append(f"# 主题\n{prompt}\n")

    # 添加对话内容
    parts.append("# 对话\n")
    for msg in messages:
        role = msg.get("role", "unknown")
        content = msg.get("content", "")

        # 处理列表形式的 content
        if isinstance(content, list):
            text_parts = []
            for c in content:
                if isinstance(c, dict):
                    text_parts.append(c.get("text", str(c)))
                else:
                    text_parts.append(str(c))
            content = " ".join(text_parts)

        role_name = {
            "user": "用户",
            "assistant": "助手",
            "system": "系统"
        }.get(role, role)

        parts.append(f"**{role_name}**: {content}\n")

    return "\n".join(parts)


def learn_via_api(content: str, filename: str) -> bool:
    """通过 API 学习内容"""
    # 方法1: 直接发送内容（需要创建临时文件）
    # 方法2: 通过 WebSocket 发送

    # 这里我们使用一个变通方法：创建临时文件然后学习
    temp_dir = Path("D:/ai_006/temp_learn")
    temp_dir.mkdir(exist_ok=True)

    temp_file = temp_dir / filename
    temp_file.write_text(content, encoding="utf-8")

    try:
        response = requests.post(
            f"{API_BASE}/api/learn/file",
            json={"path": str(temp_file)},
            timeout=30
        )
        return response.status_code == 200
    except Exception as e:
        print(f"  API 错误: {e}")
        return False
    finally:
        # 清理临时文件
        if temp_file.exists():
            temp_file.unlink()


def process_parquet_file(file_path: Path, limit: int = None) -> dict:
    """处理单个 parquet 文件"""
    print(f"\n处理文件: {file_path.name}")

    df = pd.read_parquet(file_path)
    total = len(df)

    if limit:
        df = df.head(limit)

    print(f"  总行数: {total}, 处理: {len(df)}")

    success_count = 0
    for idx, row in df.iterrows():
        # 转换为可学习格式
        content = convert_to_learnable_text(row)

        # 生成文件名
        prompt_id = row.get("prompt_id", f"row_{idx}")
        filename = f"ultrachat_{prompt_id[:16]}.md"

        # 学习
        if learn_via_api(content, filename):
            success_count += 1
            if success_count % 10 == 0:
                print(f"  已学习: {success_count}/{len(df)}")
        else:
            print(f"  学习失败: {filename}")

    return {
        "file": file_path.name,
        "total": len(df),
        "success": success_count
    }


def main():
    parser = argparse.ArgumentParser(description="学习 UltraChat 数据集")
    parser.add_argument("--limit", type=int, default=None, help="每个文件限制学习数量")
    parser.add_argument("--file", type=str, default=None, help="指定学习的文件")
    parser.add_argument("--list", action="store_true", help="列出可用文件")
    args = parser.parse_args()

    # 列出文件
    if args.list:
        print("可用文件:")
        for f in DATA_DIR.glob("*.parquet"):
            size_mb = f.stat().st_size / (1024 * 1024)
            print(f"  {f.name} ({size_mb:.1f} MB)")
        return

    # 检查 API 是否可用
    try:
        response = requests.get(f"{API_BASE}/api/world", timeout=5)
        print(f"[OK] API 已连接 (状态: {response.status_code})")
    except Exception as e:
        print(f"[ERROR] API 未连接: {e}")
        print("请先启动 Herness 服务: cargo run --release --bin herness")
        return

    # 处理文件
    results = []

    if args.file:
        # 处理指定文件
        file_path = DATA_DIR / args.file
        if file_path.exists():
            results.append(process_parquet_file(file_path, args.limit))
        else:
            print(f"文件不存在: {file_path}")
    else:
        # 处理所有文件
        parquet_files = sorted(DATA_DIR.glob("*.parquet"))
        print(f"\n找到 {len(parquet_files)} 个文件")

        for file_path in parquet_files:
            results.append(process_parquet_file(file_path, args.limit))

    # 汇总
    print("\n" + "=" * 50)
    print("学习汇总:")
    total_learned = 0
    for r in results:
        print(f"  {r['file']}: {r['success']}/{r['total']}")
        total_learned += r['success']
    print(f"\n总计学习: {total_learned} 条对话")


if __name__ == "__main__":
    main()
