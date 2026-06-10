// 完整学习测试 - 跟踪整个流程
const WebSocket = require('ws');

console.log('=== 开始学习测试 ===\n');

const ws = new WebSocket('ws://localhost:9000/ws/world');
let learningStartTime = null;
let fileCount = 0;

ws.on('open', () => {
    console.log('✅ 已连接到世界模型通道');

    // 等待接收说明书
    setTimeout(() => {
        console.log('\n📚 发送学习命令...');
        learningStartTime = Date.now();

        // 发送学习命令
        const command = {
            type: 'LearnDirectory',
            path: 'D:/训练数据/aireader/html',
            extensions: ['.html', '.htm', '.md', '.txt']
        };

        ws.send(JSON.stringify(command));
        console.log('📤 已发送学习命令');
    }, 1000);
});

ws.on('message', (data) => {
    try {
        const msg = JSON.parse(data.toString());

        if (msg.type === 'Manual') {
            console.log('📖 收到使用说明书');
        } else if (msg.type === 'Response') {
            console.log('\n📥 收到响应:', JSON.stringify(msg.data, null, 2));

            if (msg.data?.message?.includes('学习完成')) {
                const elapsed = Date.now() - learningStartTime;
                console.log(`\n✅ 学习完成! 耗时: ${elapsed}ms`);
                console.log('消息:', msg.data.message);
            }
        } else if (msg.type === 'KnowledgeFile') {
            fileCount++;
            if (fileCount % 5 === 0) {
                console.log(`📄 已处理 ${fileCount} 个文件...`);
            }
        } else if (msg.type === 'ToolResult') {
            console.log('🔧 工具结果:', msg.message);
        }
    } catch (e) {
        // 忽略解析错误
    }
});

ws.on('error', (error) => {
    console.error('❌ WebSocket 错误:', error.message);
});

ws.on('close', () => {
    console.log('🔌 连接已关闭');
});

// 60秒后评估结果
setTimeout(async () => {
    console.log('\n' + '='.repeat(50));
    console.log('📊 学习效果评估');
    console.log('='.repeat(50));

    try {
        // 获取蛊虫状态
        const response = await fetch('http://localhost:9000/api/gus');
        const data = await response.json();

        console.log(`\n蛊虫总数: ${data.total}`);

        // 统计技能
        const allAbilities = data.gus.flatMap(g => g.abilities);
        const uniqueAbilities = [...new Set(allAbilities)].filter(a => a !== '无');
        console.log(`获得技能种类: ${uniqueAbilities.length}`);
        if (uniqueAbilities.length > 0) {
            console.log('技能列表:', uniqueAbilities.join(', '));
        }

        // 统计资源变化
        const totalResources = data.gus.reduce((sum, g) => sum + g.resources, 0);
        const avgResources = totalResources / data.total;
        console.log(`平均资源: ${avgResources.toFixed(0)} 金币`);

        // 统计健康度
        const avgHealth = data.gus.reduce((sum, g) => sum + g.health, 0) / data.total;
        console.log(`平均健康度: ${(avgHealth * 100).toFixed(1)}%`);

        // 获取世界状态
        const worldRes = await fetch('http://localhost:9000/api/world');
        const world = await worldRes.json();

        console.log('\n--- 世界状态 ---');
        console.log(`健康度: ${(world.health * 100).toFixed(1)}%`);
        console.log(`同步率: ${(world.sync_rate * 100).toFixed(1)}%`);
        console.log(`意识涌现: ${world.consciousness_emerged ? '✅ 是' : '❌ 否'}`);
        console.log(`安全评分: ${(world.safety_score * 100).toFixed(1)}%`);
        console.log(`信任熵: ${world.trust_entropy.toFixed(2)}`);
        console.log(`降级阶段: ${world.degradation_phase}`);

        // 获取统计数据
        const statsRes = await fetch('http://localhost:9000/api/stats');
        const stats = await statsRes.json();

        console.log('\n--- 网络统计 ---');
        console.log(`活跃蛊虫: ${stats.world.active_gus}/${stats.world.total_gus}`);
        console.log(`平均健康度: ${(stats.world.avg_health * 100).toFixed(1)}%`);
        console.log(`共振强度: ${stats.network.resonance_strength.toFixed(4)}`);
        console.log(`平均频率: ${stats.network.mean_frequency.toFixed(4)}`);

        // 神经网络评分
        console.log('\n' + '='.repeat(50));
        console.log('🏆 神经网络综合评分');
        console.log('='.repeat(50));

        const scores = {
            health: world.health * 100,
            sync: world.sync_rate * 100,
            safety: world.safety_score * 100,
            consciousness: world.consciousness_emerged ? 100 : 0,
            learning: uniqueAbilities.length > 0 ? 80 : 20,
        };

        const weights = {
            health: 0.25,
            sync: 0.20,
            safety: 0.15,
            consciousness: 0.20,
            learning: 0.20,
        };

        const totalScore = Object.keys(scores).reduce((sum, key) => {
            const score = scores[key] * weights[key];
            console.log(`${key}: ${(scores[key]).toFixed(1)} × ${weights[key]} = ${score.toFixed(1)}`);
            return sum + score;
        }, 0);

        console.log(`\n总分: ${totalScore.toFixed(1)}/100`);

        // 评级
        let grade = 'F';
        if (totalScore >= 90) grade = 'A+';
        else if (totalScore >= 80) grade = 'A';
        else if (totalScore >= 70) grade = 'B';
        else if (totalScore >= 60) grade = 'C';
        else if (totalScore >= 50) grade = 'D';

        console.log(`评级: ${grade}`);

    } catch (e) {
        console.error('获取状态失败:', e.message);
    }

    process.exit(0);
}, 60000);
