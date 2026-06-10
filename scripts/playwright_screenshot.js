const { chromium } = require('playwright');
const path = require('path');
const fs = require('fs');

async function main() {
    console.log('启动 Chromium 浏览器...');

    const browser = await chromium.launch({
        headless: false, // 显示浏览器窗口以便观察
        slowMo: 300      // 减慢操作便于观察
    });

    const context = await browser.newContext({
        viewport: { width: 1920, height: 1080 }
    });

    const page = await context.newPage();

    try {
        // 1. 访问页面
        console.log('访问 http://localhost:9000 ...');
        await page.goto('http://localhost:9000', { waitUntil: 'domcontentloaded', timeout: 30000 });
        console.log('页面加载完成');

        // 等待页面稳定
        await page.waitForTimeout(3000);

        // 打印页面标题
        const title = await page.title();
        console.log('页面标题:', title);

        // 打印页面HTML结构（用于调试）
        const bodyContent = await page.$eval('body', el => el.innerHTML.substring(0, 5000));
        console.log('页面内容预览:', bodyContent);

        // 查找所有可能的导航元素
        const allClickable = await page.$$eval('a, button, [role="button"], nav, [role="navigation"]', els =>
            els.map(e => ({
                tag: e.tagName,
                text: e.textContent?.trim().substring(0, 50),
                href: e.href || e.getAttribute('href') || '',
                role: e.getAttribute('role') || ''
            }))
        );
        console.log('可点击元素:', JSON.stringify(allClickable, null, 2));

        // 2. 查找并点击 "聊天频道" 菜单
        console.log('查找聊天频道菜单...');

        // 尝试多种选择器查找聊天频道
        const chatSelectors = [
            'a:has-text("聊天频道")',
            'nav a:has-text("聊天")',
            '[data-testid="chat-channel"]',
            'text=聊天频道',
            'a[href*="chat"]',
            'button:has-text("聊天频道")'
        ];

        let chatLink = null;
        for (const selector of chatSelectors) {
            try {
                chatLink = await page.$(selector);
                if (chatLink) {
                    console.log(`找到聊天频道元素: ${selector}`);
                    break;
                }
            } catch (e) {
                // 继续尝试下一个选择器
            }
        }

        if (!chatLink) {
            // 打印页面内容帮助调试
            console.log('页面内容:');
            const navContent = await page.$eval('nav', el => el.innerHTML).catch(() => '无法获取导航栏');
            console.log(navContent);

            // 尝试获取所有链接
            const allLinks = await page.$$eval('nav a, nav button', els => els.map(e => e.textContent));
            console.log('导航栏中的链接:', allLinks);
        }

        if (chatLink) {
            await chatLink.click();
            console.log('已点击聊天频道');
        } else {
            // 如果找不到，直接尝试聊天频道URL
            console.log('尝试直接访问聊天频道URL...');
            await page.goto('http://localhost:9000/chat', { waitUntil: 'networkidle' });
        }

        // 3. 等待页面加载
        await page.waitForLoadState('networkidle');
        await page.waitForTimeout(2000);
        console.log('聊天频道页面加载完成');

        // 4. 查找并点击 "知识讨论" 页签
        console.log('查找知识讨论页签...');

        const tabSelectors = [
            'button:has-text("知识讨论")',
            '[role="tab"]:has-text("知识讨论")',
            'text=知识讨论',
            '.tab:has-text("知识讨论")',
            '[data-tab="knowledge"]',
            'a:has-text("知识讨论")'
        ];

        let tabFound = false;
        for (const selector of tabSelectors) {
            try {
                const tab = await page.$(selector);
                if (tab) {
                    console.log(`找到知识讨论页签: ${selector}`);
                    await tab.click();
                    tabFound = true;
                    break;
                }
            } catch (e) {
                // 继续尝试
            }
        }

        if (!tabFound) {
            // 打印页面上的所有标签页
            console.log('查找页面上的标签页...');
            const tabs = await page.$$eval('[role="tab"], button, .tab', els =>
                els.map(e => e.textContent?.trim()).filter(Boolean)
            );
            console.log('找到的标签/按钮:', tabs);
        }

        // 等待内容加载
        await page.waitForTimeout(2000);
        await page.waitForLoadState('networkidle');

        // 5. 截图
        const screenshotPath = 'D:/ai_006/screenshots/knowledge_discussion.png';
        console.log(`保存截图到: ${screenshotPath}`);

        // 全页面截图
        await page.screenshot({
            path: screenshotPath,
            fullPage: true
        });

        console.log('截图保存成功!');

        // 等待几秒便于观察
        await page.waitForTimeout(3000);

    } catch (error) {
        console.error('发生错误:', error.message);

        // 出错时也截图
        const errorScreenshotPath = 'D:/ai_006/screenshots/error_state.png';
        await page.screenshot({ path: errorScreenshotPath, fullPage: true });
        console.log('错误状态截图已保存到:', errorScreenshotPath);
    } finally {
        await browser.close();
        console.log('浏览器已关闭');
    }
}

main().catch(console.error);