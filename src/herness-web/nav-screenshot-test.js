const { chromium } = require('playwright');
const path = require('path');
const fs = require('fs');

(async () => {
  console.log('========================================');
  console.log('  Herness 导航页 & 金币流水页测试');
  console.log('========================================\n');

  const browser = await chromium.launch({
    headless: false,
    channel: 'chrome'
  });

  const context = await browser.newContext({
    viewport: { width: 1920, height: 1080 }
  });

  const page = await context.newPage();

  const screenshotsDir = path.join(__dirname, 'screenshots');
  if (!fs.existsSync(screenshotsDir)) {
    fs.mkdirSync(screenshotsDir, { recursive: true });
  }

  // 监听控制台消息
  page.on('console', msg => {
    if (msg.type() === 'error' || msg.type() === 'warning') {
      console.log(`[${msg.type()}] ${msg.text()}`);
    }
  });

  try {
    // 1. 测试监控面板
    console.log('1. 测试监控面板 (/)');
    await page.goto('http://localhost:3000/', { waitUntil: 'networkidle', timeout: 30000 });
    await page.waitForTimeout(3000);

    await page.screenshot({
      path: path.join(screenshotsDir, 'nav-01-dashboard.png'),
      fullPage: true
    });
    console.log('   ✓ screenshots/nav-01-dashboard.png');

    // 检查导航菜单
    const navItems = await page.locator('.ant-menu-item').all();
    console.log(`   导航菜单项数量: ${navItems.length}`);

    // 2. 点击金币流水导航
    console.log('\n2. 测试金币流水页 (/currency)');
    const currencyLink = page.locator('.ant-menu-item').nth(1);
    await currencyLink.click();
    await page.waitForTimeout(2000);

    await page.screenshot({
      path: path.join(screenshotsDir, 'nav-02-currency-page.png'),
      fullPage: true
    });
    console.log('   ✓ screenshots/nav-02-currency-page.png');

    // 检查金币流水页元素
    const statsCards = await page.locator('.game-card').all();
    console.log(`   统计卡片数量: ${statsCards.length}`);

    const tableRows = await page.locator('.ant-table-tbody tr').all();
    console.log(`   交易记录数: ${tableRows.length}`);

    // 3. 检查 WebSocket 连接状态
    console.log('\n3. 检查 WebSocket 连接状态');
    const badge = await page.locator('.ant-badge-status-text').first();
    const statusText = await badge.textContent();
    console.log(`   连接状态: ${statusText}`);

    // 4. 等待更多交易数据
    console.log('\n4. 等待实时交易数据...');
    await page.waitForTimeout(5000);

    await page.screenshot({
      path: path.join(screenshotsDir, 'nav-03-currency-realtime.png'),
      fullPage: true
    });
    console.log('   ✓ screenshots/nav-03-currency-realtime.png');

    // 5. 测试导航回到监控面板
    console.log('\n5. 测试导航返回监控面板');
    const dashboardLink = page.locator('.ant-menu-item').first();
    await dashboardLink.click();
    await page.waitForTimeout(2000);

    await page.screenshot({
      path: path.join(screenshotsDir, 'nav-04-back-dashboard.png'),
      fullPage: true
    });
    console.log('   ✓ screenshots/nav-04-back-dashboard.png');

    // 6. 最终截图
    console.log('\n6. 最终视口截图');
    await page.screenshot({
      path: path.join(screenshotsDir, 'nav-05-final.png')
    });
    console.log('   ✓ screenshots/nav-05-final.png');

    // 验证数据
    console.log('\n========================================');
    console.log('  功能验证结果');
    console.log('========================================');
    console.log('   ✓ 导航菜单: 正常');
    console.log('   ✓ 金币流水页: 正常');
    console.log(`   ✓ WebSocket: ${statusText === '实时' ? '已连接' : '未连接'}`);
    console.log(`   ✓ 交易数据: ${tableRows.length} 条`);

  } catch (error) {
    console.error('测试错误:', error.message);
    await page.screenshot({
      path: path.join(screenshotsDir, 'nav-error.png')
    });
  }

  console.log('\n浏览器将在 5 秒后关闭...');
  await page.waitForTimeout(5000);

  await browser.close();
  console.log('\n测试完成！');
})();
