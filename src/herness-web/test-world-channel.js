// 测试世界模型通信通道
// 1. 连接 WebSocket
// 2. 接收使用说明书
// 3. 发送学习目录命令
// 4. 接收学习结果

const WebSocket = require('ws');

const WS_URL = 'ws://localhost:9000/ws/world';
const LEARN_PATH = 'D:\\训练数据\\aireader\\生物';

async function testWorldChannel() {
    console.log('🚀 连接世界模型通道:', WS_URL);

    const ws = new WebSocket(WS_URL);

    ws.on('open', () => {
        console.log('✅ WebSocket 连接成功');
    });

    ws.on('message', (data) => {
        try {
            const msg = JSON.parse(data.toString());
            console.log('\n📥 收到消息:', JSON.stringify(msg, null, 2).slice(0, 500));

            // 收到说明书后，发送学习命令
            if (msg.type === 'Manual') {
                console.log('\n📖 使用说明书已接收');
                console.log('📤 发送学习目录命令:', LEARN_PATH);

                const command = {
                    LearnDirectory: {
                        path: LEARN_PATH,
                        extensions: ['.md', '.txt']
                    }
                };

                ws.send(JSON.stringify(command));
            }

            // 收到学习结果
            if (msg.type === 'Response') {
                if (msg.data.ActionResult) {
                    console.log('\n✅ 学习结果:', msg.data.ActionResult.message);
                } else if (msg.data.Halt) {
                    console.log('\n🛑 熔断信号:', msg.data.Halt.reason);
                    ws.close();
                } else if (msg.data.Error) {
                    console.log('\n❌ 错误:', msg.data.Error);
                }
            }

            // 收到工具结果
            if (msg.type === 'ToolResult') {
                console.log(`\n📝 工具结果 [${msg.call_id}]: ${msg.success ? '成功' : '失败'} - ${msg.message}`);
            }
        } catch (e) {
            console.log('📥 原始消息:', data.toString().slice(0, 200));
        }
    });

    ws.on('error', (err) => {
        console.error('❌ WebSocket 错误:', err.message);
    });

    ws.on('close', () => {
        console.log('🔌 连接关闭');
        process.exit(0);
    });

    // 30秒超时
    setTimeout(() => {
        console.log('⏰ 测试超时，关闭连接');
        ws.close();
    }, 30000);
}

testWorldChannel();
