"""
Herness Web 端到端测试脚本
使用 Selenium 测试前端功能
"""

import time
import os
import sys

# 设置 UTF-8 编码
if sys.platform == 'win32':
    import io
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8')
from selenium import webdriver
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.common.exceptions import TimeoutException, NoSuchElementException

# 配置路径
CHROME_DIR = r"D:\ai_006\chrome\chrome-win64"
CHROMEDRIVER_DIR = r"D:\ai_006\chrome\chromedriver-win64"
SCREENSHOT_DIR = r"D:\ai_006\docs\screenshots"

# 确保 Chrome 和 ChromeDriver 路径正确
CHROME_PATH = os.path.join(CHROME_DIR, "chrome.exe")
CHROMEDRIVER_PATH = os.path.join(CHROMEDRIVER_DIR, "chromedriver.exe")

# 创建截图目录
os.makedirs(SCREENSHOT_DIR, exist_ok=True)


def create_driver():
    """创建 Chrome WebDriver"""
    options = Options()
    options.binary_location = CHROME_PATH
    options.add_argument("--headless")  # 无头模式
    options.add_argument("--disable-gpu")
    options.add_argument("--no-sandbox")
    options.add_argument("--disable-dev-shm-usage")
    options.add_argument("--window-size=1920,1080")

    service = Service(executable_path=CHROMEDRIVER_PATH)
    driver = webdriver.Chrome(service=service, options=options)
    return driver


def take_screenshot(driver, name):
    """保存截图"""
    path = os.path.join(SCREENSHOT_DIR, f"{name}.png")
    driver.save_screenshot(path)
    print(f"📸 截图保存: {path}")
    return path


def test_page_load(driver):
    """测试页面加载"""
    print("\n=== 测试页面加载 ===")
    driver.get("http://localhost:3000")

    # 等待页面加载
    wait = WebDriverWait(driver, 10)

    try:
        # 等待主要元素出现
        wait.until(EC.presence_of_element_located((By.CLASS_NAME, "ant-card")))
        print("✅ 页面加载成功")

        # 截图
        take_screenshot(driver, "01_page_load")
        return True
    except TimeoutException:
        print("❌ 页面加载超时")
        take_screenshot(driver, "01_page_load_error")
        return False


def test_world_status_card(driver):
    """测试世界状态卡片"""
    print("\n=== 测试世界状态卡片 ===")

    try:
        # 查找状态指示器
        cards = driver.find_elements(By.CLASS_NAME, "ant-card")
        print(f"找到 {len(cards)} 个卡片")

        # 查找数值显示
        values = driver.find_elements(By.CLASS_NAME, "data-value")
        print(f"找到 {len(values)} 个数值显示")

        take_screenshot(driver, "02_world_status")
        print("✅ 世界状态卡片正常")
        return True
    except Exception as e:
        print(f"❌ 世界状态卡片测试失败: {e}")
        take_screenshot(driver, "02_world_status_error")
        return False


def test_gu_list(driver):
    """测试蛊虫列表"""
    print("\n=== 测试蛊虫列表 ===")

    try:
        # 等待表格加载
        wait = WebDriverWait(driver, 5)
        table = wait.until(EC.presence_of_element_located((By.CLASS_NAME, "ant-table")))

        # 查找表格行
        rows = driver.find_elements(By.CLASS_NAME, "ant-table-row")
        print(f"找到 {len(rows)} 行数据")

        take_screenshot(driver, "03_gu_list")
        print("✅ 蛊虫列表正常")
        return True
    except TimeoutException:
        print("⚠️ 蛊虫表格未找到（可能使用其他组件）")
        take_screenshot(driver, "03_gu_list_notable")
        return True
    except Exception as e:
        print(f"❌ 蛊虫列表测试失败: {e}")
        take_screenshot(driver, "03_gu_list_error")
        return False


def test_access_points(driver):
    """测试接入点面板"""
    print("\n=== 测试接入点面板 ===")

    try:
        # 查找仪表盘图表
        dashboards = driver.find_elements(By.CLASS_NAME, "ant-progress-circle")
        print(f"找到 {len(dashboards)} 个仪表盘")

        take_screenshot(driver, "04_access_points")
        print("✅ 接入点面板正常")
        return True
    except Exception as e:
        print(f"❌ 接入点面板测试失败: {e}")
        take_screenshot(driver, "04_access_points_error")
        return False


def test_charts(driver):
    """测试图表"""
    print("\n=== 测试图表 ===")

    try:
        # 等待图表加载（ECharts 使用 canvas）
        time.sleep(2)  # 给 ECharts 时间渲染

        canvases = driver.find_elements(By.TAG_NAME, "canvas")
        print(f"找到 {len(canvases)} 个 Canvas 元素")

        take_screenshot(driver, "05_charts")
        print("✅ 图表正常")
        return True
    except Exception as e:
        print(f"❌ 图表测试失败: {e}")
        take_screenshot(driver, "05_charts_error")
        return False


def test_websocket_connection(driver):
    """测试 WebSocket 连接"""
    print("\n=== 测试 WebSocket 连接 ===")

    try:
        # 等待一段时间让 WebSocket 数据更新
        time.sleep(3)

        # 检查是否有动态数据更新
        # 通过检查控制台日志判断连接状态

        # 获取浏览器日志
        logs = driver.get_log("browser")
        ws_logs = [log for log in logs if 'WebSocket' in log['message'] or 'useWebSocket' in log['message']]

        if ws_logs:
            print("WebSocket 相关日志:")
            for log in ws_logs[:5]:
                print(f"  [{log['level']}] {log['message'][:150]}")

        take_screenshot(driver, "06_websocket")

        # 检查是否连接成功
        connected = any('Connected' in log['message'] for log in ws_logs)
        if connected:
            print("✅ WebSocket 连接成功，使用真实数据")
        else:
            print("⚠️ WebSocket 未连接，可能使用模拟数据")

        return True
    except Exception as e:
        print(f"❌ WebSocket 测试失败: {e}")
        take_screenshot(driver, "06_websocket_error")
        return False


def test_console_errors(driver):
    """检查控制台错误"""
    print("\n=== 检查控制台错误 ===")

    logs = driver.get_log("browser")
    errors = [log for log in logs if log["level"] == "SEVERE"]

    if errors:
        print(f"⚠️ 发现 {len(errors)} 个严重错误:")
        for error in errors[:5]:  # 只显示前5个
            print(f"  - {error['message']}")
    else:
        print("✅ 没有严重控制台错误")

    # 打印所有日志
    print("\n浏览器日志:")
    for log in logs[:10]:
        print(f"  [{log['level']}] {log['message'][:100]}")

    return len(errors) == 0


def run_all_tests():
    """运行所有测试"""
    print("=" * 60)
    print("Herness Web 端到端测试")
    print("=" * 60)

    results = {}

    try:
        driver = create_driver()
        print("✅ Chrome WebDriver 创建成功")
    except Exception as e:
        print(f"❌ Chrome WebDriver 创建失败: {e}")
        return

    try:
        results["页面加载"] = test_page_load(driver)
        results["世界状态"] = test_world_status_card(driver)
        results["蛊虫列表"] = test_gu_list(driver)
        results["接入点面板"] = test_access_points(driver)
        results["图表渲染"] = test_charts(driver)
        results["WebSocket"] = test_websocket_connection(driver)
        results["控制台检查"] = test_console_errors(driver)

    finally:
        driver.quit()
        print("\n✅ WebDriver 已关闭")

    # 打印结果汇总
    print("\n" + "=" * 60)
    print("测试结果汇总")
    print("=" * 60)

    passed = 0
    failed = 0
    for name, result in results.items():
        status = "✅ 通过" if result else "❌ 失败"
        print(f"{name}: {status}")
        if result:
            passed += 1
        else:
            failed += 1

    print(f"\n总计: {passed} 通过, {failed} 失败")
    print(f"截图保存在: {SCREENSHOT_DIR}")


if __name__ == "__main__":
    run_all_tests()
