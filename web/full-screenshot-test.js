const { chromium } = require('playwright');
const path = require('path');
const fs = require('fs');

(async () => {
  console.log('========================================');
  console.log('  Herness 世界神经网络监控系统 - 截图测试');
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

  console.log('1. 导航到 http://localhost:3000...');
  await page.goto('http://localhost:3000', { waitUntil: 'networkidle', timeout: 30000 });

  // 等待数据加载和渲染
  console.log('2. 等待数据加载...');
  await page.waitForTimeout(3000);

  // 截取完整首页
  console.log('3. 截取完整首页...');
  await page.screenshot({
    path: path.join(screenshotsDir, '01-homepage-full.png'),
    fullPage: true
  });
  console.log('   ✓ screenshots/01-homepage-full.png');

  // 截取顶部状态栏
  console.log('4. 截取顶部状态区域...');
  const header = await page.locator('.ant-layout-header, header, .status-bar').first();
  if (await header.isVisible()) {
    await header.screenshot({ path: path.join(screenshotsDir, '02-header.png') });
    console.log('   ✓ screenshots/02-header.png');
  }

  // 截取统计卡片区域
  console.log('5. 截取统计卡片...');
  const statsCards = await page.locator('.ant-row').first();
  if (await statsCards.isVisible()) {
    await statsCards.screenshot({ path: path.join(screenshotsDir, '03-stats-cards.png') });
    console.log('   ✓ screenshots/03-stats-cards.png');
  }

  // 截取世界拓扑图（Canvas）
  console.log('6. 截取世界神经网络拓扑图...');
  const canvas = await page.locator('canvas').first();
  if (await canvas.isVisible()) {
    const box = await canvas.boundingBox();
    if (box) {
      await page.screenshot({
        path: path.join(screenshotsDir, '04-world-topology.png'),
        clip: { x: box.x - 10, y: box.y - 50, width: box.width + 20, height: box.height + 70 }
      });
      console.log('   ✓ screenshots/04-world-topology.png');
    }
  }

  // 截取蛊虫列表表格
  console.log('7. 截取蛊虫列表...');
  const table = await page.locator('.ant-table-wrapper');
  if (await table.isVisible()) {
    await table.screenshot({ path: path.join(screenshotsDir, '05-gu-list.png') });
    console.log('   ✓ screenshots/05-gu-list.png');
  }

  // 截取颜色列（新增功能）
  console.log('8. 截取蛊虫颜色列...');
  const colorColumn = await page.locator('.ant-table-tbody tr').first();
  if (await colorColumn.isVisible()) {
    await colorColumn.screenshot({ path: path.join(screenshotsDir, '06-color-column.png') });
    console.log('   ✓ screenshots/06-color-column.png');
  }

  // 滚动页面查看更多内容
  console.log('9. 滚动页面查看更多内容...');
  await page.evaluate(() => window.scrollBy(0, 500));
  await page.waitForTimeout(500);

  await page.screenshot({
    path: path.join(screenshotsDir, '07-scrolled-content.png'),
    fullPage: true
  });
  console.log('   ✓ screenshots/07-scrolled-content.png');

  // 获取页面数据验证
  console.log('\n========================================');
  console.log('  页面数据验证');
  console.log('========================================');

  const worldData = await page.evaluate(() => {
    // 从页面提取显示的数据
    const healthText = document.querySelector('.ant-statistic-content-value')?.textContent || 'N/A';
    const populationText = document.querySelector('[style*="population"], .population')?.textContent || 'N/A';

    // 检查蛊虫表格行数
    const rows = document.querySelectorAll('.ant-table-tbody tr');
    const guNames = [];
    rows.forEach(row => {
      const name = row.querySelector('td:nth-child(3)')?.textContent?.trim();
      if (name) guNames.push(name);
    });

    return {
      health: healthText,
      population: populationText,
      guCount: rows.length,
      guNames: guNames.slice(0, 10)
    };
  });

  console.log(`   健康度: ${worldData.health}`);
  console.log(`   蛊虫数量: ${worldData.guCount}`);
  console.log(`   蛊虫列表: ${worldData.guNames.join(', ')}...`);

  // 最终截图
  console.log('\n10. 最终视口截图...');
  await page.screenshot({
    path: path.join(screenshotsDir, '08-final-viewport.png')
  });
  console.log('   ✓ screenshots/08-final-viewport.png');

  console.log('\n========================================');
  console.log('  测试完成！所有截图已保存。');
  console.log('========================================');

  // 等待用户查看
  console.log('\n浏览器将在 5 秒后关闭...');
  await page.waitForTimeout(5000);

  await browser.close();
})();
