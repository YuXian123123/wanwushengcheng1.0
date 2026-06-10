// 调试学习流程
const WebSocket = require('ws');

console.log('=== 调试学习流程 ===\n');

const ws = new WebSocket('ws://localhost:9000/ws/world');
let startTime = null;

ws.on('open', () => {
    console.log('✅ 已连接到世界模型通道');

    setTimeout(() => {
        console.log('\n📚 发送学习命令...');
        startTime = Date.now();

        const command = {
            type: 'LearnDirectory',
            path: 'D:/训练数据/aireader/html',
            extensions: ['.html', '.htm', '.md', '.txt']
        };

        ws.send(JSON.stringify(command));
    }, 1000);
});

ws.on('message', (data) => {
    try {
        const msg = JSON.parse(data.toString());
        const elapsed = startTime ? Date.now() - startTime : 0;

        switch (msg.type) {
            case 'Manual':
                console.log(`[${elapsed}ms] 📖 收到使用说明书`);
                break;
            case 'Response':
                console.log(`[${elapsed}ms] 📥 响应:`, JSON.stringify(msg.data));
                break;
            case 'KnowledgeFile':
                console.log(`[${elapsed}ms] 📄 知识文件: ${msg.data?.filename}`);
                break;
            case 'ToolResult':
                console.log(`[${elapsed}ms] 🔧 工具结果: ${msg.success ? '✅' : '❌'} ${msg.message}`);
                break;
            case 'Error':
                console.log(`[${elapsed}ms] ❌ 错误: ${msg.message}`);
                break;
            default:
                console.log(`[${elapsed}ms] 📥 其他消息:`, msg.type);
        }
    } catch (e) {
        console.log('解析错误:', e.message);
    }
});

ws.on('error', (error) => {
    console.error('❌ WebSocket 错误:', error.message);
});

ws.on('close', (code, reason) => {
    console.log(`🔌 连接已关闭, code: ${code}, reason: ${reason}`);
});

// 保持运行更长时间
setTimeout(() => {
    console.log('\n⏰ 超时，检查最终状态...');
    fetch('http://localhost:9000/api/gus')
        .then(r => r.json())
        .then(data => {
            console.log(`蛊虫总数: ${data.total}`);
            data.gus.slice(0, 3).forEach(gu => {
                console.log(`- ${gu.name}: 能力=[${gu.abilities.join(', ')}]`);
            });
            process.exit(0);
        })
        .catch(e => {
            console.error('错误:', e);
            process.exit(1);
        });
}, 30000);
