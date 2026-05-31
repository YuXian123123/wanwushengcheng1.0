// Playwright 测试脚本 - 测试任务管理功能
const { chromium } = require('playwright');
const path = require('path');

async function main() {
  console.log('🚀 启动浏览器测试...');

  const browser = await chromium.launch({
    headless: false, // 显示浏览器窗口
    slowMo: 500, // 慢速操作便于观察
  });

  const context = await browser.newContext({
    viewport: { width: 1920, height: 1080 },
  });

  const page = await context.newPage();

  try {
    // 1. 访问首页
    console.log('📍 步骤1: 访问首页');
    await page.goto('http://localhost:9000', { waitUntil: 'networkidle' });
    await page.waitForTimeout(2000);
    await page.screenshot({ path: 'screenshots/01-homepage.png', fullPage: true });
    console.log('✅ 截图保存: 01-homepage.png');

    // 2. 点击任务管理菜单
    console.log('📍 步骤2: 点击任务管理菜单');
    await page.click('text=任务管理');
    await page.waitForTimeout(2000);
    await page.screenshot({ path: 'screenshots/02-task-page.png', fullPage: true });
    console.log('✅ 截图保存: 02-task-page.png');

    // 3. 点击发布任务按钮
    console.log('📍 步骤3: 点击发布任务按钮');
    await page.click('button:has-text("发布任务")');
    await page.waitForTimeout(1000);
    await page.screenshot({ path: 'screenshots/03-create-task-modal.png', fullPage: true });
    console.log('✅ 截图保存: 03-create-task-modal.png');

    // 4. 填写任务表单
    console.log('📍 步骤4: 填写任务表单');
    await page.fill('input[placeholder="例如：火焰喷射技能熟练度提升"]', '测试任务：收集灵石');
    await page.fill('textarea[placeholder="详细描述任务内容和目标"]', '前往灵石矿洞收集100块灵石，用于炼制新的蛊虫装备');
    await page.fill('input[placeholder="完成任务的奖励金币数量"]', '80');
    await page.waitForTimeout(500);
    await page.screenshot({ path: 'screenshots/04-filled-form.png', fullPage: true });
    console.log('✅ 截图保存: 04-filled-form.png');

    // 5. 提交任务
    console.log('📍 步骤5: 提交任务');
    // 使用表单提交，点击模态框内的提交按钮
    await page.click('.ant-modal button[type="submit"]');
    await page.waitForTimeout(2000);
    await page.screenshot({ path: 'screenshots/05-task-created.png', fullPage: true });
    console.log('✅ 截图保存: 05-task-created.png');

    // 6. 分配任务
    console.log('📍 步骤6: 分配任务给蛊虫');
    // 查找刚创建的任务的分配按钮
    const assignButton = await page.locator('button:has-text("分配")').first();
    if (await assignButton.isVisible()) {
      await assignButton.click();
      await page.waitForTimeout(1000);
      await page.screenshot({ path: 'screenshots/06-assign-modal.png', fullPage: true });
      console.log('✅ 截图保存: 06-assign-modal.png');

      // 选择蛊虫
      await page.click('.ant-select');
      await page.waitForTimeout(500);
      const option = await page.locator('.ant-select-dropdown .ant-select-item').first();
      if (await option.isVisible()) {
        await option.click();
        await page.waitForTimeout(500);
      }
      await page.click('button:has-text("确认分配")');
      await page.waitForTimeout(2000);
      await page.screenshot({ path: 'screenshots/07-task-assigned.png', fullPage: true });
      console.log('✅ 截图保存: 07-task-assigned.png');
    }

    // 7. 完成任务
    console.log('📍 步骤7: 完成任务');
    const completeButton = await page.locator('button:has-text("完成")').first();
    if (await completeButton.isVisible()) {
      await completeButton.click();
      await page.waitForTimeout(500);
      // 确认弹窗
      const confirmBtn = await page.locator('.ant-popconfirm .ant-btn-primary').first();
      if (await confirmBtn.isVisible()) {
        await confirmBtn.click();
        await page.waitForTimeout(2000);
      }
      await page.screenshot({ path: 'screenshots/08-task-completed.png', fullPage: true });
      console.log('✅ 截图保存: 08-task-completed.png');
    }

    // 8. 查看金币流水
    console.log('📍 步骤8: 查看金币流水');
    await page.click('text=金币流水');
    await page.waitForTimeout(2000);
    await page.screenshot({ path: 'screenshots/09-currency-page.png', fullPage: true });
    console.log('✅ 截图保存: 09-currency-page.png');

    console.log('\n🎉 测试完成！所有截图已保存到 screenshots 目录');

  } catch (error) {
    console.error('❌ 测试出错:', error.message);
    await page.screenshot({ path: 'screenshots/error.png', fullPage: true });
  } finally {
    await browser.close();
  }
}

main().catch(console.error);
