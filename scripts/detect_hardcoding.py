#!/usr/bin/env python3
"""
硬编码检测工具

检测Rust代码中的潜在硬编码值，帮助维护代码质量。
"""

import re
import sys
from pathlib import Path
from typing import List, Tuple

# 允许的模式（不视为硬编码）
ALLOWED_PATTERNS = [
    r'//.*',                    # 注释
    r'/\*.*?\*/',               # 块注释
    r'test',                    # 测试代码
    r'config',                  # 配置模块
    r'assert!',                 # 断言
    r'assert_eq!',              # 断言相等
    r'assert_ne!',              # 断言不相等
    r'vec\!\[',                 # vec宏
    r'\[.*\d+.*\]',             # 数组初始化
    r'0x[0-9A-Fa-f]+',          # 十六进制（Unicode码点等）
    r'\d+\.\d+e[-+]?\d+',       # 科学计数法（用于算法）
    r'"[^"]*"',                 # 字符串字面量
]

# 可疑的硬编码模式
SUSPICIOUS_PATTERNS = [
    # 浮点数字面量
    (r'(?<![a-zA-Z_])\d+\.\d+(?![a-zA-Z_0-9])', '浮点数硬编码'),
    # 整数比较阈值
    (r'(if|while)\s*\([^)]*[<>]=?\s*\d+\s*\)', '整数阈值硬编码'),
    # const定义（排除const fn）
    (r'(?<!fn\s)const\s+\w+\s*:\s*\w+\s*=\s*\d+\s*;', '常量硬编码'),
    # 范围硬编码
    (r'\.\.\s*=?\s*\d+', '范围结束硬编码'),
    (r'\d+\s*\.\.', '范围开始硬编码'),
]


def is_in_allowed_context(line: str, file_path: str) -> bool:
    """检查是否在允许的上下文中"""
    # 检查文件路径
    if 'config' in file_path.replace('\\', '/'):
        return True
    if 'test' in file_path.replace('\\', '/'):
        return True

    # 检查行内容
    for pattern in ALLOWED_PATTERNS:
        if re.search(pattern, line, re.IGNORECASE):
            return True

    return False


def detect_hardcoding(file_path: str) -> List[Tuple[int, str, str]]:
    """检测文件中的硬编码"""
    violations = []

    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"无法读取文件 {file_path}: {e}", file=sys.stderr)
        return violations

    for line_num, line in enumerate(lines, 1):
        if is_in_allowed_context(line, file_path):
            continue

        for pattern, description in SUSPICIOUS_PATTERNS:
            matches = re.finditer(pattern, line)
            for match in matches:
                matched_text = match.group()
                # 过滤一些明显不是硬编码的情况
                if matched_text in ['0.0', '1.0', '0', '1']:
                    continue  # 常见的边界值
                violations.append((line_num, line.strip(), description))

    return violations


def scan_directory(directory: str) -> None:
    """扫描目录中的所有Rust文件"""
    rust_files = list(Path(directory).rglob('*.rs'))

    total_violations = 0

    for file_path in rust_files:
        violations = detect_hardcoding(str(file_path))
        if violations:
            print(f"\n[FILE] {file_path}")
            print("-" * 60)
            for line_num, line, desc in violations:
                print(f"  [!] Line {line_num}: [{desc}]")
                print(f"     {line}")
                total_violations += 1

    print("\n" + "=" * 60)
    if total_violations > 0:
        print(f"[ERROR] Found {total_violations} potential hardcoding issues")
        print("\nSuggestion: Move these values to config system")
        sys.exit(1)
    else:
        print("[OK] No hardcoding issues found")
        sys.exit(0)


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("用法: python detect_hardcoding.py <src目录>")
        sys.exit(1)

    directory = sys.argv[1]
    scan_directory(directory)
