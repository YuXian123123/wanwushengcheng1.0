const { chromium } = require('playwright');
const path = require('path');
const fs = require('fs');

(async () => {
  console.log('========================================');
  console.log('  聊天页面截图测试');
  console.log('========================================\n');

  const browser = await chromium.launch({
    headless: true,
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

  // 等待服务器
  console.log('等待服务器...');
  let retries = 10;
  while (retries > 0) {
    try {
      const response = await page.goto('http://localhost:9000', {
        waitUntil: 'networkidle',
        timeout: 5000
      });
      if (response && response.ok()) {
        console.log('✅ 服务器已就绪');
        break;
      }
    } catch (e) {
      retries--;
      if (retries > 0) {
        console.log(`等待... (剩余 ${retries} 次)`);
        await page.waitForTimeout(2000);
      }
    }
  }

  // 截图 1: 聊天页面
  console.log('1. 截取聊天页面...');
  await page.goto('http://localhost:9000/chat', {
    waitUntil: 'networkidle',
    timeout: 30000
  });
  await page.waitForTimeout(2000);
  await page.screenshot({
    path: path.join(screenshotsDir, '10-chat-page.png'),
    fullPage: false
  });
  console.log('   ✓ screenshots/10-chat-page.png');

  // 截图 2: 点击世界频道
  console.log('2. 切换频道...');
  const worldChannelItem = await page.locator('text=世界意识').first();
  if (await worldChannelItem.isVisible()) {
    await worldChannelItem.click();
    await page.waitForTimeout(1000);
    await page.screenshot({
      path: path.join(screenshotsDir, '11-chat-world.png'),
      fullPage: false
    });
    console.log('   ✓ screenshots/11-chat-world.png');
  }

  // 截图 3: 输入消息
  console.log('3. 测试消息输入...');
  const textarea = await page.locator('textarea').first();
  if (await textarea.isVisible()) {
    await textarea.fill('你好，这是一个测试消息！');
    await page.waitForTimeout(500);
    await page.screenshot({
      path: path.join(screenshotsDir, '12-chat-input.png'),
      fullPage: false
    });
    console.log('   ✓ screenshots/12-chat-input.png');
  }

  // 截图 4: 导航栏
  console.log('4. 导航栏截图...');
  await page.goto('http://localhost:9000/', {
    waitUntil: 'networkidle',
    timeout: 30000
  });
  await page.waitForTimeout(2000);

  // 截取顶部导航
  const header = await page.locator('.ant-layout-header').first();
  if (await header.isVisible()) {
    await header.screenshot({
      path: path.join(screenshotsDir, '13-nav-with-chat.png')
    });
    console.log('   ✓ screenshots/13-nav-with-chat.png');
  }

  // 截图 5: 首页完整
  console.log('5. 首页完整截图...');
  await page.screenshot({
    path: path.join(screenshotsDir, '14-homepage-full.png'),
    fullPage: false
  });
  console.log('   ✓ screenshots/14-homepage-full.png');

  console.log('\n========================================');
  console.log('  测试完成！所有截图已保存。');
  console.log(`  截图目录: ${screenshotsDir}`);
  console.log('========================================');

  await browser.close();
})();
