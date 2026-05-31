#!/bin/bash
# 启动 Herness 世界神经网络监控系统
#
# 使用方法:
#   ./start.sh          # 启动服务
#   ./start.sh --build  # 重新构建后启动

set -e

echo "🌐 Herness 世界神经网络监控系统"
echo "================================"

# 检查是否需要重新构建
if [ "$1" == "--build" ]; then
    echo "📦 构建后端..."
    cargo build --bin herness --release

    echo "📦 构建前端..."
    cd herness-web && npm run build && cd ..
fi

# 检查前端是否已构建
if [ ! -d "herness-web/build" ]; then
    echo "⚠️  前端未构建，正在构建..."
    cd herness-web && npm install && npm run build && cd ..
fi

echo ""
echo "🚀 启动服务..."
echo ""
echo "访问地址:"
echo "  • Web 界面:  http://localhost:9000"
echo "  • API 文档:  http://localhost:9000/api/world"
echo "  • WebSocket: ws://localhost:9000/ws"
echo ""
echo "按 Ctrl+C 停止服务"
echo ""

# 启动后端服务
cargo run --bin herness --release
