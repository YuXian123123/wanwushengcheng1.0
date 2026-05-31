# Herness Web API 文档

## 服务信息

| 项目 | 值 |
|------|-----|
| 后端端口 | 9000 |
| 前端端口 | 3000 |
| 后端地址 | http://localhost:9000 |
| 前端地址 | http://localhost:3000 |
| WebSocket | ws://localhost:9000/ws |

---

## HTTP API

### 1. GET /api/world - 获取世界状态

**请求**: 无参数

**响应**:
```json
{
  "health": 1.0,
  "population": 25,
  "access_point_count": 125,
  "health_status": "Healthy",
  "sync_rate": 0.0,
  "consciousness_emerged": false,
  "safety_score": 1.0,
  "trust_entropy": 0.0,
  "degradation_phase": "Normal"
}
```

### 2. GET /api/gus - 获取蛊虫列表

**请求**: 无参数

**响应**:
```json
{
  "gus": [
    {
      "id": "uuid-string",
      "name": "火灵虫",
      "health": 0.5,
      "trust_score": 0.5,
      "active": true,
      "abilities": ["火焰喷射", "冰冻护盾"],
      "resources": 136,
      "access_points": {
        "perception": 0.5,
        "action": 0.4,
        "communication": 0.3,
        "memory": 0.5,
        "reasoning": 0.4
      }
    }
  ],
  "total": 25
}
```

### 3. GET /api/gu/:id - 获取蛊虫详情

**请求**: 路径参数 `id` - 蛊虫UUID

**响应**:
```json
{
  "id": "uuid-string",
  "name": "火灵虫",
  "health": 0.85,
  "trust_score": 0.5,
  "expertise": {},
  "access_points": [
    {
      "id": "uuid-string",
      "name": "Perceive",
      "active": true,
      "load": 0.2,
      "capacity": 1.0,
      "connections": 3
    }
  ]
}
```

### 4. GET /api/stats - 获取统计数据

**请求**: 无参数

**响应**:
```json
{
  "world": {
    "total_gus": 25,
    "active_gus": 25,
    "total_access_points": 125,
    "avg_health": 1.0
  },
  "network": {
    "sync_rate": 0.0,
    "resonance_strength": 0.0,
    "mean_frequency": 40.0
  },
  "safety": {
    "safety_score": 1.0,
    "trust_entropy": 0.0,
    "degradation_phase": "Normal"
  }
}
```

### 5. GET /api/config - 获取配置

**请求**: 无参数

**响应**:
```json
{
  "emergence_threshold": 0.7,
  "base_frequency": 40.0,
  "survival_threshold": 0.5,
  "min_population": 10
}
```

### 6. POST /api/command - 执行命令

**请求**:
```json
{
  "command": "create_gu",
  "args": ["火灵虫"]
}
```

**响应**:
```json
{
  "success": true,
  "message": "命令 'create_gu' 已执行",
  "data": null
}
```

---

## WebSocket API

### 连接

连接地址: `ws://localhost:9000/ws`

### 消息格式

所有消息都是 JSON 格式：

```json
{
  "msg_type": "metrics",
  "timestamp": 1717012345678,
  "data": { ... }
}
```

### 消息类型

#### 1. metrics - 实时指标 (每500ms推送)

```json
{
  "msg_type": "metrics",
  "timestamp": 1717012345678,
  "data": {
    "health": 0.95,
    "sync_rate": 0.72,
    "resonance_strength": 0.68,
    "safety_score": 0.98,
    "consciousness_emerged": true,
    "emergence_factor": 0.84,
    "activity": 0.55,
    "perception": 0.72,
    "cognition": 0.65,
    "action": 0.78,
    "communication": 0.52,
    "survival": 0.88,
    "log": null,
    "event": null
  }
}
```

---

## 五接入点架构

每个蛊虫智能体有 5 个接入点连接到世界神经网络：

| 接入点 | 英文名 | 功能 | 默认权重 |
|--------|--------|------|----------|
| 感知接入点 | Perceive | 接收外部环境输入 | 1.0 |
| 认知接入点 | Cognitive | 高级推理与决策 | 2.0 |
| 行为接入点 | Behavior | 执行动作输出 | 1.5 |
| 通信接入点 | Comm | 蛊虫间信息交换 | 1.0 |
| 生存接入点 | Survival | 生命状态同步 | 0.5 |

---

## 意识涌现条件

根据黑塔的同步共振意识涌现机制：

```
consciousness_emerged = sync_rate > 0.7 AND emergence_factor > 0.5
emergence_factor = sqrt(sync_rate * health)
```

---

## 降级阶段

根据螺丝咕姆的安全机制：

| 阶段 | 健康度范围 | 描述 |
|------|------------|------|
| Normal | > 70% | 正常运行 |
| Warning | 50-70% | 警告状态 |
| Critical | 30-50% | 严重状态 |
| Emergency | 10-30% | 紧急状态 |
| Termination | < 10% | 终止状态 |

---

## 测试截图

测试截图保存在 `D:\ai_006\docs\screenshots\` 目录：

| 截图 | 说明 |
|------|------|
| 01_page_load.png | 页面加载测试 |
| 02_world_status.png | 世界状态卡片 |
| 03_gu_list.png | 蛊虫列表显示 |
| 04_access_points.png | 接入点面板 |
| 05_charts.png | 图表渲染 |
| 06_websocket.png | WebSocket 测试 |

---

## 启动命令

```bash
# 关闭旧进程
taskkill //f //im herness.exe
taskkill //f //im node.exe

# 启动后端
cd D:/ai_006/src && cargo run --bin herness

# 启动前端 (新终端)
cd D:/ai_006/src/herness-web && npm run dev
```

---

## E2E 测试

运行端到端测试：

```bash
cd D:/ai_006/tests && python e2e_test.py
```

测试结果：
- 页面加载: ✅ 通过
- 世界状态: ✅ 通过
- 蛊虫列表: ✅ 通过
- 接入点面板: ✅ 通过
- 图表渲染: ✅ 通过
- WebSocket: ✅ 通过
- 控制台检查: ✅ 通过
