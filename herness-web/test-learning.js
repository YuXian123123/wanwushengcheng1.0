// 测试学习流程
// 连接世界模型通道并触发学习

const WebSocket = require('ws');

const ws = new WebSocket('ws://localhost:9000/ws/world');

ws.on('open', () => {
    console.log('✅ 已连接到世界模型通道');

    // 等待接收说明书
    setTimeout(() => {
        console.log('\n📚 发送学习命令...');

        // 发送学习命令
        const command = {
            type: 'LearnDirectory',
            path: 'D:/训练数据/aireader/html',
            extensions: ['.html', '.htm', '.md', '.txt']
        };

        ws.send(JSON.stringify(command));
        console.log('📤 已发送学习命令:', command);
    }, 2000);
});

ws.on('message', (data) => {
    try {
        const msg = JSON.parse(data.toString());
        console.log('\n📥 收到消息:', JSON.stringify(msg, null, 2).substring(0, 500));

        if (msg.type === 'Response' && msg.data?.message?.includes('学习完成')) {
            console.log('\n✅ 学习完成!');
            console.log('消息:', msg.data.message);
        }
    } catch (e) {
        console.log('📥 收到原始消息:', data.toString().substring(0, 200));
    }
});

ws.on('error', (error) => {
    console.error('❌ WebSocket 错误:', error.message);
});

ws.on('close', () => {
    console.log('🔌 连接已关闭');
});

// 30秒后检查蛊虫状态
setTimeout(async () => {
    console.log('\n📊 检查蛊虫状态...');

    try {
        const response = await fetch('http://localhost:9000/api/gus');
        const data = await response.json();

        console.log(`\n总数: ${data.total} 只蛊虫`);

        // 检查有技能的蛊虫
        const withSkills = data.gus.filter(g => g.abilities.length > 0 && g.abilities[0] !== '无');
        console.log(`有技能的蛊虫: ${withSkills.length}`);

        if (withSkills.length > 0) {
            console.log('\n=== 获得技能的蛊虫 ===');
            withSkills.slice(0, 5).forEach(gu => {
                console.log(`- ${gu.name}: ${gu.abilities.join(', ')}`);
            });
        }

        // 检查世界状态
        const worldRes = await fetch('http://localhost:9000/api/world');
        const world = await worldRes.json();
        console.log('\n=== 世界状态 ===');
        console.log(`健康度: ${(world.health * 100).toFixed(1)}%`);
        console.log(`同步率: ${(world.sync_rate * 100).toFixed(1)}%`);
        console.log(`意识涌现: ${world.consciousness_emerged ? '是' : '否'}`);
        console.log(`安全评分: ${(world.safety_score * 100).toFixed(1)}%`);

    } catch (e) {
        console.error('获取状态失败:', e.message);
    }

    process.exit(0);
}, 35000);
