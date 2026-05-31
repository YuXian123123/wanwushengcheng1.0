// 简单的 WebSocket 客户端测试
const WebSocket = require('ws');

console.log('Connecting to ws://localhost:9000/ws...');

const ws = new WebSocket('ws://localhost:9000/ws');

ws.on('open', () => {
    console.log('✅ WebSocket connected!');
});

ws.on('message', (data) => {
    console.log('📨 Received:', data.toString().substring(0, 200));
});

ws.on('error', (error) => {
    console.log('❌ Error:', error.message);
});

ws.on('close', () => {
    console.log('🔌 Connection closed');
    process.exit(0);
});

// 5秒后关闭
setTimeout(() => {
    console.log('Closing after 5 seconds...');
    ws.close();
}, 5000);
