# Herness Web 数据流文档

## 概述

Herness Web 监控界面**完全从后端获取数据**，没有任何模拟数据或硬编码数值。

## 错误处理策略

当后端不可用时：
- **不使用模拟数据** - 直接暴露错误
- **显示错误信息** - 在页面上清晰展示连接失败原因
- **打印详细日志** - 控制台输出完整的错误信息

## 数据来源

### WebSocket 实时数据 (`ws://localhost:9000/ws`)

推送频率：2Hz (每500ms)

| 字段 | 类型 | 说明 |
|------|------|------|
| `health` | f64 | 世界健康度 |
| `sync_rate` | f64 | 同步率 |
| `resonance_strength` | f64 | 共振强度 |
| `safety_score` | f64 | 安全评分 |
| `consciousness_emerged` | bool | 意识是否涌现 |
| `emergence_factor` | f64 | 涌现因子 |
| `activity` | f64 | 活跃度 |
| `perception` | f64 | 感知接入点活动 |
| `cognition` | f64 | 认知接入点活动 |
| `action` | f64 | 行为接入点活动 |
| `communication` | f64 | 通信接入点活动 |
| `survival` | f64 | 生存接入点活动 |

**连接失败时：**
- 控制台打印错误: `[useWebSocket] ❌ Connection error`
- 页面显示红色 Alert 提示
- 数据显示为 `--` 或 "无数据"

### HTTP API 低频数据

| API | 端点 | 数据内容 |
|-----|------|----------|
| 世界状态 | `GET /api/world` | 健康度、种群数量、同步率、意识状态 |
| 蛊虫列表 | `GET /api/gus` | 所有蛊虫的详细信息 |
| 蛊虫详情 | `GET /api/gu/:id` | 单个蛊虫的详细信息 |
| 统计数据 | `GET /api/stats` | 世界统计、网络统计、安全统计 |
| 配置信息 | `GET /api/config` | 系统配置参数 |
| 命令执行 | `POST /api/command` | 执行命令并返回结果 |

**连接失败时：**
- 控制台打印错误: `[useHttpData] ❌ API Error: <详细错误>`
- 页面显示错误信息
- 数据为 `null`，组件显示 "无数据"

## 组件数据流

```
App.tsx
├── useWebSocket() ──────────────────────────────────────────────┐
│   └── metrics: { health, sync_rate, perception, ... } | null  │
│   └── connected: boolean                                       │
│   └── error: string | null                                     │
├── useHttpData() ───────────────────────────────────────────────┤
│   └── worldState: WorldState | null                            │
│   └── guList: GuInfo[]                                         │
│   └── stats: Stats | null                                      │
│   └── loading: boolean                                         │
│   └── error: string | null                                     │
└────────────────────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────────────┐
│  子组件                                                          │
├─────────────────────────────────────────────────────────────────┤
│  WorldTopology ─────── guList, syncRate (来自 props)           │
│  GuList ─────────────── guList (来自 props)                     │
│  ConsciousnessIndicator ─ isConscious, syncRate, emergenceFactor│
│  AccessPointPanel ───── useWebSocket() (直接连接)               │
│  LogPanel ───────────── WebSocket (独立连接，接收 log 消息)     │
│  CommandPanel ───────── /api/command (直接调用)                 │
└─────────────────────────────────────────────────────────────────┘
```

## 控制台日志格式

### 正常连接
```
[useWebSocket] Connecting to: ws://localhost:9000/ws
[useWebSocket] ✅ Connected to backend WebSocket
[useHttpData] Fetching data from backend APIs...
[useHttpData] ✅ API Response received:
  - /api/world: {health: 0.85, population: 25, ...}
  - /api/gus: {gus: [...], total: 25}
  - /api/stats: {world: {...}, network: {...}, safety: {...}}
```

### 连接失败
```
[useWebSocket] Connecting to: ws://localhost:9000/ws
[useWebSocket] ❌ Connection error: [object Event]
[useWebSocket] WebSocket connection failed (retry #1)
[useHttpData] Fetching data from backend APIs...
[useHttpData] ❌ API Error: Network error: Cannot connect to backend server
[useHttpData] Full error: AxiosError {message: "Network Error", ...}
```

## 验证方法

1. 启动后端：`cargo run --bin herness_web`
2. 启动前端：`cd herness-web && npm run dev`
3. 打开浏览器控制台 (F12)
4. 检查日志：
   - 看到 `✅ Connected` 表示连接成功
   - 看到 `❌` 表示连接失败，需要检查后端

## 更新历史

- 2026-05-31: 移除所有模拟数据和 fallback
- 2026-05-31: 添加详细的错误日志和页面错误提示
- 2026-05-31: 当数据不可用时显示 "--" 或 "无数据"
