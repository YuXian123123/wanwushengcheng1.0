@echo off
REM 启动 Herness 世界神经网络监控系统 (Windows)
REM
REM 使用方法:
REM   start.bat          # 启动服务
REM   start.bat --build  # 重新构建后启动

echo 🌐 Herness 世界神经网络监控系统
echo ================================

REM 检查是否需要重新构建
if "%1"=="--build" (
    echo 📦 构建后端...
    cargo build --bin herness --release

    echo 📦 构建前端...
    cd herness-web
    call npm run build
    cd ..
)

REM 检查前端是否已构建
if not exist "herness-web\build" (
    echo ⚠️ 前端未构建，正在构建...
    cd herness-web
    call npm install
    call npm run build
    cd ..
)

echo.
echo 🚀 启动服务...
echo.
echo 访问地址:
echo   • Web 界面:  http://localhost:9000
echo   • API 文档:  http://localhost:9000/api/world
echo   • WebSocket: ws://localhost:9000/ws
echo.
echo 按 Ctrl+C 停止服务
echo.

REM 启动后端服务
cargo run --bin herness --release
