const { chromium } = require('playwright');
const path = require('path');
const fs = require('fs');

(async () => {
  console.log('Launching Chrome browser...');
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

  // Collect console messages
  const consoleMessages = [];
  page.on('console', msg => {
    consoleMessages.push({ type: msg.type(), text: msg.text() });
    console.log(`[Console ${msg.type()}] ${msg.text()}`);
  });

  // Collect network errors
  const networkErrors = [];
  page.on('requestfailed', request => {
    networkErrors.push({
      url: request.url(),
      failure: request.failure()
    });
    console.log(`[Network Error] ${request.url()} - ${request.failure()?.errorText}`);
  });

  console.log('Navigating to http://localhost:3000...');
  try {
    await page.goto('http://localhost:3000', { waitUntil: 'networkidle', timeout: 30000 });
    console.log('Page loaded successfully!');

    // Wait for React to render
    await page.waitForTimeout(5000);

    // Check for loading spinners
    const spinners = await page.locator('.ant-spin, .loading').all();
    console.log(`Found ${spinners.length} loading spinners`);

    // Check for error alerts
    const alerts = await page.locator('.ant-alert').all();
    console.log(`Found ${alerts.length} alerts`);
    for (const alert of alerts) {
      const text = await alert.textContent();
      const type = await alert.getAttribute('class');
      console.log(`  Alert (${type}): ${text}`);
    }

    // Check for error messages in the DOM
    const errorTexts = await page.locator('text=/error|failed|cannot connect/i').all();
    if (errorTexts.length > 0) {
      console.log('\nError messages found on page:');
      for (const el of errorTexts) {
        const text = await el.textContent();
        console.log(`  - ${text}`);
      }
    }

    // Get page HTML for debugging
    const bodyHTML = await page.innerHTML('body');
    console.log('\nPage body length:', bodyHTML.length);

    // Check if root element has content
    const rootContent = await page.innerHTML('#root');
    console.log('Root content length:', rootContent.length);

    // Take screenshots
    await page.screenshot({
      path: path.join(screenshotsDir, 'debug-full.png'),
      fullPage: true
    });

    // Try to find any visible text
    const visibleText = await page.textContent('body');
    console.log('\nVisible text on page (first 500 chars):');
    console.log(visibleText.substring(0, 500));

    // Check API calls
    console.log('\nNetwork requests to backend:');
    const requests = await page.evaluate(() => {
      return window.performance.getEntriesByType('resource')
        .filter(r => r.name.includes('/api/') || r.name.includes('localhost:9000'))
        .map(r => ({ name: r.name, duration: r.duration }));
    });
    console.log(JSON.stringify(requests, null, 2));

  } catch (error) {
    console.error('Error:', error.message);
  }

  console.log('\nWaiting 10 seconds for inspection...');
  await page.waitForTimeout(10000);

  await browser.close();
  console.log('\nTest completed!');
})();
