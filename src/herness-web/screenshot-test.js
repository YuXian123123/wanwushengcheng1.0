const { chromium } = require('playwright');
const path = require('path');

(async () => {
  console.log('Launching Chrome browser...');
  const browser = await chromium.launch({
    headless: false,
    channel: 'chrome' // Use installed Chrome
  });

  const context = await browser.newContext({
    viewport: { width: 1920, height: 1080 }
  });

  const page = await context.newPage();

  // Create screenshots directory
  const screenshotsDir = path.join(__dirname, 'screenshots');
  const fs = require('fs');
  if (!fs.existsSync(screenshotsDir)) {
    fs.mkdirSync(screenshotsDir, { recursive: true });
  }

  console.log('Navigating to http://localhost:3000...');
  try {
    await page.goto('http://localhost:3000', { waitUntil: 'networkidle', timeout: 30000 });
    console.log('Page loaded successfully!');

    // Wait a bit for animations
    await page.waitForTimeout(2000);

    // Take full page screenshot
    console.log('Taking full page screenshot...');
    await page.screenshot({
      path: path.join(screenshotsDir, 'homepage-full.png'),
      fullPage: true
    });
    console.log('Saved: screenshots/homepage-full.png');

    // Take viewport screenshot
    await page.screenshot({
      path: path.join(screenshotsDir, 'homepage-viewport.png')
    });
    console.log('Saved: screenshots/homepage-viewport.png');

    // Check for error messages
    const errorElements = await page.locator('.ant-alert-error, .error-message').all();
    if (errorElements.length > 0) {
      console.log(`Found ${errorElements.length} error messages on page`);
      for (const el of errorElements) {
        const text = await el.textContent();
        console.log(`  Error: ${text}`);
      }
    }

    // Get page title
    const title = await page.title();
    console.log(`Page title: ${title}`);

    // Check if main components are visible
    const guListVisible = await page.locator('table, .ant-table').isVisible().catch(() => false);
    const topologyVisible = await page.locator('canvas').isVisible().catch(() => false);
    const statsVisible = await page.locator('.ant-statistic, .stat-card').first().isVisible().catch(() => false);

    console.log('\nComponent visibility:');
    console.log(`  - Gu List Table: ${guListVisible ? 'YES' : 'NO'}`);
    console.log(`  - World Topology Canvas: ${topologyVisible ? 'YES' : 'NO'}`);
    console.log(`  - Statistics Cards: ${statsVisible ? 'YES' : 'NO'}`);

    // Take screenshot of specific sections if they exist
    const canvas = await page.locator('canvas').first();
    if (await canvas.isVisible()) {
      await page.screenshot({
        path: path.join(screenshotsDir, 'world-topology.png'),
        clip: await canvas.boundingBox()
      });
      console.log('Saved: screenshots/world-topology.png');
    }

    // Check console for errors
    page.on('console', msg => {
      if (msg.type() === 'error') {
        console.log(`Console error: ${msg.text()}`);
      }
    });

    // Wait for user to see the page
    console.log('\nWaiting 5 seconds for visual inspection...');
    await page.waitForTimeout(5000);

  } catch (error) {
    console.error('Error loading page:', error.message);
    await page.screenshot({
      path: path.join(screenshotsDir, 'error-page.png')
    });
    console.log('Saved error screenshot: screenshots/error-page.png');
  }

  await browser.close();
  console.log('\nTest completed!');
})();
