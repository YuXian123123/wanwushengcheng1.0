// 正确格式的学习测试
const WebSocket = require('ws');

console.log('=== 学习测试（正确格式）===\n');

const ws = new WebSocket('ws://localhost:9000/ws/world');
let startTime = null;
let fileCount = 0;

ws.on('open', () => {
    console.log('✅ 已连接到世界模型通道');

    setTimeout(() => {
        console.log('\n📚 发送学习命令...');
        startTime = Date.now();

        // 正确的 Rust enum 序列化格式
        const command = {
            LearnDirectory: {
                path: 'D:/训练数据/aireader/html',
                extensions: ['.html', '.htm', '.md', '.txt']
            }
        };

        ws.send(JSON.stringify(command));
        console.log('📤 已发送:', JSON.stringify(command));
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
                console.log(`[${elapsed}ms] 📥 响应:`, JSON.stringify(msg.data, null, 2));
                break;
            case 'KnowledgeFile':
                fileCount++;
                console.log(`[${elapsed}ms] 📄 文件 #${fileCount}: ${msg.data?.filename}`);
                break;
            case 'ToolResult':
                console.log(`[${elapsed}ms] 🔧 结果: ${msg.success ? '✅' : '❌'} ${msg.message}`);
                break;
            case 'Error':
                console.log(`[${elapsed}ms] ❌ 错误: ${msg.message}`);
                break;
            default:
                console.log(`[${elapsed}ms] 📥`, JSON.stringify(msg).substring(0, 200));
        }
    } catch (e) {
        console.log('解析错误:', e.message);
    }
});

ws.on('error', (error) => {
    console.error('❌ WebSocket 错误:', error.message);
});

ws.on('close', (code, reason) => {
    console.log(`🔌 连接已关闭, code: ${code}`);
});

// 30秒后检查结果
setTimeout(async () => {
    console.log('\n' + '='.repeat(50));
    console.log('📊 学习结果');
    console.log('='.repeat(50));

    try {
        const r = await fetch('http://localhost:9000/api/gus');
        const data = await r.json();

        console.log(`\n蛊虫总数: ${data.total}`);
        console.log(`处理文件数: ${fileCount}`);

        // 检查技能
        const withSkills = data.gus.filter(g => g.abilities.length > 0 && !g.abilities.includes('无'));
        console.log(`有技能的蛊虫: ${withSkills.length}`);

        if (withSkills.length > 0) {
            console.log('\n=== 获得技能的蛊虫 ===');
            withSkills.forEach(gu => {
                console.log(`- ${gu.name}: [${gu.abilities.join(', ')}]`);
            });
        } else {
            console.log('\n前3只蛊虫状态:');
            data.gus.slice(0, 3).forEach(gu => {
                console.log(`- ${gu.name}: 能力=[${gu.abilities.join(', ')}], 资源=${gu.resources}`);
            });
        }

        // 世界状态
        const wr = await fetch('http://localhost:9000/api/world');
        const world = await wr.json();
        console.log('\n=== 世界状态 ===');
        console.log(`健康度: ${(world.health * 100).toFixed(1)}%`);
        console.log(`同步率: ${(world.sync_rate * 100).toFixed(1)}%`);
        console.log(`意识涌现: ${world.consciousness_emerged ? '✅' : '❌'}`);

    } catch (e) {
        console.error('错误:', e.message);
    }

    process.exit(0);
}, 30000);
