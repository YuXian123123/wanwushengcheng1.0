import { useState, useEffect, useRef } from 'react'
import { Card, Table, Tag, Row, Col, Typography, Progress, Button, Input, Space, Statistic, Empty, Badge, Divider, Tooltip, message } from 'antd'
import {
  RocketOutlined,
  PlayCircleOutlined,
  PauseCircleOutlined,
  StopOutlined,
  ThunderboltOutlined,
  BulbOutlined,
  DatabaseOutlined,
  LineChartOutlined,
  DollarOutlined,
  CheckCircleOutlined,
  CloseCircleOutlined,
  ClockCircleOutlined,
  EyeOutlined,
  SettingOutlined,
} from '@ant-design/icons'
import type { ColumnsType } from 'antd/es/table'
import ReactECharts from 'echarts-for-react'
import { useNavigate } from 'react-router-dom'

const { Title, Text } = Typography

// 训练状态接口
interface TrainingState {
  is_training: boolean
  is_paused: boolean
  epoch: number
  total_epochs: number
  samples_processed: number
  total_samples: number
  template_count: number
  rule_count: number
  loss: number
  throughput: number
  start_time: number | null
  eta: number | null
}

// 训练状态消息
interface TrainingStatusMessage {
  type: string
  state: TrainingState
  message: string | null
  timestamp: number
}

// 生成请求
interface GenerateRequest {
  request_id: string
  user_id: string
  text: string
}

// 金币变化
interface CoinChange {
  amount: number
  change_type: string
  new_balance: number
  reason: string
}

// 生成响应
interface GenerateResponse {
  request_id: string
  success: boolean
  scene: any
  error: string | null
  coin_change: CoinChange
  timestamp: number
}

// 生成历史记录
interface GenerationRecord {
  id: string
  text: string
  success: boolean
  reward: number
  timestamp: number
  scene?: any
  error?: string
  feedbackGiven?: 'reward' | 'punish' | null
}

// 货币统计
interface CurrencyStats {
  total_supply: number
  total_transactions: number
  velocity: number
  inflation_rate: number
}

// 货币 WebSocket 消息
interface CurrencyMessage {
  type: 'transaction' | 'stats' | 'history'
  data: any
}

// 3D 节点
interface SceneNode {
  id: string
  meridian_id: string
  position: [number, number, number]
  scale: [number, number, number]
  geometry: string
  color: [number, number, number, number]
  visible: boolean
}

// World3D 结构
interface World3D {
  id: string
  root: string
  nodes: Record<string, SceneNode>
  bounding_box: {
    min: [number, number, number]
    max: [number, number, number]
  }
}

// 用户ID
const USER_ID = 'user-' + Math.random().toString(36).substr(2, 9)

// 默认训练配置
const defaultTrainingConfig = {
  data_path: 'data/training/scenes_combined.json',
  epochs: 100,
  batch_size: 32,
  auto_save: true,
  save_interval: 60,
}

// 从 localStorage 获取训练配置
function getTrainingConfig() {
  try {
    const saved = localStorage.getItem('training_config')
    if (saved) {
      return { ...defaultTrainingConfig, ...JSON.parse(saved) }
    }
  } catch (e) {
    console.error('[Training] Failed to load config:', e)
  }
  return defaultTrainingConfig
}

export default function TrainingPage() {
  const navigate = useNavigate()
  // 训练状态
  const [trainingState, setTrainingState] = useState<TrainingState | null>(null)
  const [trainingConnected, setTrainingConnected] = useState(false)

  // 生成状态
  const [generateConnected, setGenerateConnected] = useState(false)
  const [inputText, setInputText] = useState('')
  const [history, setHistory] = useState<GenerationRecord[]>([])
  const [generating, setGenerating] = useState(false)

  // 货币状态
  const [currencyStats, setCurrencyStats] = useState<CurrencyStats | null>(null)
  const [currencyConnected, setCurrencyConnected] = useState(false)

  // 当前场景
  const [currentScene, setCurrentScene] = useState<SceneNode[] | null>(null)
  const [selectedRecord, setSelectedRecord] = useState<GenerationRecord | null>(null)

  // 图表数据
  const [lossHistory, setLossHistory] = useState<number[]>([])

  // WebSocket refs
  const trainingWsRef = useRef<WebSocket | null>(null)
  const generateWsRef = useRef<WebSocket | null>(null)
  const currencyWsRef = useRef<WebSocket | null>(null)
  const canvasRef = useRef<HTMLCanvasElement | null>(null)

  // 连接货币 WebSocket
  useEffect(() => {
    const connectCurrency = () => {
      const ws = new WebSocket('ws://localhost:9000/ws/currency')
      currencyWsRef.current = ws

      ws.onopen = () => {
        console.log('[Currency] ✅ Connected')
        setCurrencyConnected(true)
      }

      ws.onmessage = (event) => {
        try {
          const msg: CurrencyMessage = JSON.parse(event.data)
          if (msg.type === 'stats') {
            setCurrencyStats(msg.data as CurrencyStats)
          }
        } catch (e) {
          console.error('[Currency] Parse error:', e)
        }
      }

      ws.onclose = () => {
        console.log('[Currency] ❌ Disconnected')
        setCurrencyConnected(false)
        setTimeout(connectCurrency, 5000)
      }
    }

    connectCurrency()
    return () => { currencyWsRef.current?.close() }
  }, [])

  // 连接训练 WebSocket
  useEffect(() => {
    const connectTraining = () => {
      const ws = new WebSocket('ws://localhost:9000/ws/training')
      trainingWsRef.current = ws

      ws.onopen = () => {
        console.log('[Training] ✅ Connected')
        setTrainingConnected(true)
      }

      ws.onmessage = (event) => {
        try {
          const msg: TrainingStatusMessage = JSON.parse(event.data)
          if (msg.type === 'training_status') {
            setTrainingState(msg.state)
            setLossHistory(prev => [...prev.slice(-59), msg.state.loss])
          }
        } catch (e) {
          console.error('[Training] Parse error:', e)
        }
      }

      ws.onclose = () => {
        console.log('[Training] ❌ Disconnected')
        setTrainingConnected(false)
        setTimeout(connectTraining, 3000)
      }
    }

    connectTraining()
    return () => { trainingWsRef.current?.close() }
  }, [])

  // 当前输入文本（用于生成响应时记录）
  const inputTextRef = useRef<string>('')

  // 连接生成 WebSocket
  useEffect(() => {
    const connectGenerate = () => {
      const ws = new WebSocket('ws://localhost:9000/ws/generate')
      generateWsRef.current = ws

      ws.onopen = () => {
        console.log('[Generate] ✅ Connected')
        setGenerateConnected(true)
      }

      ws.onmessage = (event) => {
        try {
          const response: GenerateResponse = JSON.parse(event.data)
          setGenerating(false)

          // 添加到历史（使用 ref 保存的文本）
          const record: GenerationRecord = {
            id: response.request_id,
            text: inputTextRef.current,
            success: response.success,
            reward: response.coin_change.amount,
            timestamp: response.timestamp,
            scene: response.scene,
            error: response.error || undefined,
          }
          setHistory(prev => [record, ...prev.slice(0, 49)])

          // 设置当前场景
          if (response.success && response.scene?.nodes) {
            // 将 HashMap 转换为数组
            const world = response.scene as World3D
            const nodesArray: SceneNode[] = Object.values(world.nodes)
            setCurrentScene(nodesArray)
            setSelectedRecord(record)
          }
        } catch (e) {
          console.error('[Generate] Parse error:', e)
          setGenerating(false)
        }
      }

      ws.onclose = () => {
        console.log('[Generate] ❌ Disconnected')
        setGenerateConnected(false)
        setTimeout(connectGenerate, 3000)
      }
    }

    connectGenerate()
    return () => { generateWsRef.current?.close() }
  }, [])

  // 简单的 3D 渲染（Canvas 2D 模拟）
  useEffect(() => {
    if (!canvasRef.current || !currentScene) return

    const canvas = canvasRef.current
    const ctx = canvas.getContext('2d')
    if (!ctx) return

    const width = canvas.width
    const height = canvas.height

    // 清空画布
    ctx.fillStyle = '#0a0a0f'
    ctx.fillRect(0, 0, width, height)

    // 绘制网格
    ctx.strokeStyle = '#1f2937'
    ctx.lineWidth = 1
    for (let i = 0; i < width; i += 40) {
      ctx.beginPath()
      ctx.moveTo(i, 0)
      ctx.lineTo(i, height)
      ctx.stroke()
    }
    for (let i = 0; i < height; i += 40) {
      ctx.beginPath()
      ctx.moveTo(0, i)
      ctx.lineTo(width, i)
      ctx.stroke()
    }

    // 简单的等轴测投影
    const isoX = (x: number, y: number) => (x - y) * 0.866 + width / 2
    const isoY = (x: number, y: number, z: number) => (x + y) * 0.5 - z + height / 2

    // 绘制节点
    currentScene.forEach((node, index) => {
      if (!node.visible) return

      const [x, y, z] = node.position
      const [sx, sy, sz] = node.scale
      const [cr, cg, cb, ca] = node.color

      const screenX = isoX(x * 50, y * 50)
      const screenY = isoY(x * 50, y * 50, z * 50)

      // 绘制立方体
      const size = Math.min(sx, sy, sz) * 30

      // 转換顏色為 CSS 格式
      const color = `rgba(${Math.round(cr * 255)}, ${Math.round(cg * 255)}, ${Math.round(cb * 255)}, ${ca})`

      // 顶面
      ctx.fillStyle = color
      ctx.globalAlpha = 0.8
      ctx.beginPath()
      ctx.moveTo(screenX, screenY - size)
      ctx.lineTo(screenX + size * 0.866, screenY - size * 0.5)
      ctx.lineTo(screenX, screenY)
      ctx.lineTo(screenX - size * 0.866, screenY - size * 0.5)
      ctx.closePath()
      ctx.fill()

      // 左面
      ctx.globalAlpha = 0.6
      ctx.beginPath()
      ctx.moveTo(screenX - size * 0.866, screenY - size * 0.5)
      ctx.lineTo(screenX, screenY)
      ctx.lineTo(screenX, screenY + size)
      ctx.lineTo(screenX - size * 0.866, screenY + size * 0.5)
      ctx.closePath()
      ctx.fill()

      // 右面
      ctx.globalAlpha = 0.4
      ctx.beginPath()
      ctx.moveTo(screenX + size * 0.866, screenY - size * 0.5)
      ctx.lineTo(screenX, screenY)
      ctx.lineTo(screenX, screenY + size)
      ctx.lineTo(screenX + size * 0.866, screenY + size * 0.5)
      ctx.closePath()
      ctx.fill()

      ctx.globalAlpha = 1

      // 绘制名称
      ctx.fillStyle = '#e5e7eb'
      ctx.font = '11px sans-serif'
      ctx.textAlign = 'center'
      const geometryName = node.geometry?.split('::').pop() || 'Node'
      ctx.fillText(geometryName, screenX, screenY + size + 20)
    })

    // 绘制坐标轴
    ctx.strokeStyle = '#ef4444'
    ctx.lineWidth = 2
    ctx.beginPath()
    ctx.moveTo(50, height - 50)
    ctx.lineTo(100, height - 80)
    ctx.stroke()
    ctx.fillStyle = '#ef4444'
    ctx.fillText('X', 105, height - 75)

    ctx.strokeStyle = '#10b981'
    ctx.beginPath()
    ctx.moveTo(50, height - 50)
    ctx.lineTo(90, height - 30)
    ctx.stroke()
    ctx.fillStyle = '#10b981'
    ctx.fillText('Y', 95, height - 25)

    ctx.strokeStyle = '#3b82f6'
    ctx.beginPath()
    ctx.moveTo(50, height - 50)
    ctx.lineTo(50, height - 100)
    ctx.stroke()
    ctx.fillStyle = '#3b82f6'
    ctx.fillText('Z', 55, height - 105)

  }, [currentScene])

  // 开始训练
  const startTraining = () => {
    console.log('[Training] startTraining called')
    console.log('[Training] WebSocket ref:', trainingWsRef.current)
    console.log('[Training] readyState:', trainingWsRef.current?.readyState)
    console.log('[Training] WebSocket.OPEN:', WebSocket.OPEN)

    if (trainingWsRef.current?.readyState === WebSocket.OPEN) {
      // 从 localStorage 获取配置
      const config = getTrainingConfig()

      const command = {
        Start: {
          config: {
            data_path: config.data_path,
            epochs: config.epochs,
            batch_size: config.batch_size,
            auto_save: config.auto_save,
            save_interval: config.save_interval,
          }
        }
      }

      console.log('[Training] Sending start command:', JSON.stringify(command, null, 2))
      trainingWsRef.current.send(JSON.stringify(command))
      message.info('训练命令已发送')
    } else {
      console.error('[Training] WebSocket not connected, state:', trainingWsRef.current?.readyState)
      message.error('WebSocket 未连接，请刷新页面')
    }
  }

  // 暂停训练
  const pauseTraining = () => {
    if (trainingWsRef.current?.readyState === WebSocket.OPEN) {
      trainingWsRef.current.send(JSON.stringify({ Pause: null }))
    }
  }

  // 停止训练
  const stopTraining = () => {
    if (trainingWsRef.current?.readyState === WebSocket.OPEN) {
      trainingWsRef.current.send(JSON.stringify({ Stop: null }))
    }
  }

  // 生成场景
  const generateScene = () => {
    if (!inputText.trim()) return
    if (generateWsRef.current?.readyState === WebSocket.OPEN) {
      // 保存当前输入到 ref（用于响应时记录）
      inputTextRef.current = inputText.trim()

      setGenerating(true)
      const request: GenerateRequest = {
        request_id: 'req-' + Date.now(),
        user_id: USER_ID,
        text: inputText.trim(),
      }
      generateWsRef.current.send(JSON.stringify(request))
      setInputText('')
    }
  }

  // 查看历史场景
  const viewScene = (record: GenerationRecord) => {
    if (record.scene?.nodes) {
      const world = record.scene as World3D
      const nodesArray: SceneNode[] = Object.values(world.nodes)
      setCurrentScene(nodesArray)
      setSelectedRecord(record)
    }
  }

  // 发送反馈（奖励/惩罚）
  const sendFeedback = (record: GenerationRecord, isPositive: boolean) => {
    if (trainingWsRef.current?.readyState === WebSocket.OPEN) {
      const feedback = {
        Feedback: {
          request_id: record.id,
          text: record.text,
          is_positive: isPositive,
          correct_entities: null,
          correct_relations: null,
          timestamp: Math.floor(Date.now() / 1000),
        }
      }
      console.log('[Feedback] Sending:', feedback)
      trainingWsRef.current.send(JSON.stringify(feedback))

      // 更新历史记录
      setHistory(prev => prev.map(r =>
        r.id === record.id
          ? { ...r, feedbackGiven: isPositive ? 'reward' : 'punish' }
          : r
      ))

      // 更新选中记录
      if (selectedRecord?.id === record.id) {
        setSelectedRecord({ ...record, feedbackGiven: isPositive ? 'reward' : 'punish' })
      }

      message.success(isPositive ? '已奖励！神经网络会学习这个好结果' : '已惩罚！神经网络会避免这个错误')
    } else {
      message.error('WebSocket 未连接')
    }
  }

  // 连接状态
  const isConnected = trainingConnected && generateConnected && currencyConnected
  const isTraining = trainingState?.is_training && !trainingState?.is_paused
  const isPaused = trainingState?.is_training && trainingState?.is_paused

  // 金币总供应量
  const coinBalance = currencyStats?.total_supply ?? 0

  // 损失图表
  const lossChartOption = {
    backgroundColor: 'transparent',
    grid: { left: '10%', right: '5%', top: '15%', bottom: '15%' },
    xAxis: { type: 'category', show: false },
    yAxis: {
      type: 'value',
      axisLine: { lineStyle: { color: '#374151' } },
      splitLine: { lineStyle: { color: '#1f2937' } },
      axisLabel: { color: '#9ca3af' },
    },
    series: [{
      type: 'line',
      data: lossHistory,
      smooth: true,
      symbol: 'none',
      areaStyle: {
        color: {
          type: 'linear',
          x: 0, y: 0, x2: 0, y2: 1,
          colorStops: [
            { offset: 0, color: 'rgba(16, 185, 129, 0.4)' },
            { offset: 1, color: 'rgba(16, 185, 129, 0.05)' },
          ],
        },
      },
      lineStyle: { color: '#10b981', width: 2 },
    }],
  }

  // 历史表格列
  const historyColumns: ColumnsType<GenerationRecord> = [
    {
      title: <span style={{ color: '#9ca3af' }}>时间</span>,
      dataIndex: 'timestamp',
      width: 70,
      render: (ts: number) => {
        const date = new Date(ts * 1000)
        return <span style={{ color: '#9ca3af', fontSize: 12 }}>{date.toLocaleTimeString()}</span>
      },
    },
    {
      title: <span style={{ color: '#9ca3af' }}>描述</span>,
      dataIndex: 'text',
      ellipsis: true,
      render: (text: string) => <span style={{ color: '#f3f4f6' }}>{text}</span>,
    },
    {
      title: <span style={{ color: '#9ca3af' }}>状态</span>,
      dataIndex: 'success',
      width: 70,
      render: (success: boolean) => success
        ? <Tag icon={<CheckCircleOutlined />} color="success">成功</Tag>
        : <Tag icon={<CloseCircleOutlined />} color="error">失败</Tag>,
    },
    {
      title: <span style={{ color: '#9ca3af' }}>金币</span>,
      dataIndex: 'reward',
      width: 70,
      render: (reward: number) => (
        <span style={{ color: reward >= 0 ? '#10b981' : '#ef4444', fontWeight: 'bold' }}>
          {reward >= 0 ? '+' : ''}{reward.toFixed(0)}
        </span>
      ),
    },
    {
      title: <span style={{ color: '#9ca3af' }}>操作</span>,
      width: 60,
      render: (_: any, record: GenerationRecord) => (
        record.scene?.nodes ? (
          <Button
            type="link"
            size="small"
            icon={<EyeOutlined />}
            onClick={() => viewScene(record)}
            style={{ color: '#6366f1' }}
          >
            查看
          </Button>
        ) : null
      ),
    },
    {
      title: <span style={{ color: '#9ca3af' }}>反馈</span>,
      width: 120,
      render: (_: any, record: GenerationRecord) => {
        if (!record.scene?.nodes) return null
        if (record.feedbackGiven) {
          return (
            <Tag color={record.feedbackGiven === 'reward' ? 'success' : 'error'}>
              {record.feedbackGiven === 'reward' ? '已奖励' : '已惩罚'}
            </Tag>
          )
        }
        return (
          <Space size="small">
            <Button
              type="link"
              size="small"
              icon={<CheckCircleOutlined />}
              onClick={() => sendFeedback(record, true)}
              style={{ color: '#10b981', padding: '0 4px' }}
            />
            <Button
              type="link"
              size="small"
              icon={<CloseCircleOutlined />}
              onClick={() => sendFeedback(record, false)}
              style={{ color: '#ef4444', padding: '0 4px' }}
            />
          </Space>
        )
      },
    },
  ]

  return (
    <div style={{ padding: 0 }}>
      {/* 调试信息 - WebSocket 连接状态 */}
      <Card style={{
        background: 'rgba(0,0,0,0.3)',
        borderColor: '#374151',
        marginBottom: 16,
        borderRadius: 8,
      }}>
        <div style={{ display: 'flex', gap: 24, flexWrap: 'wrap' }}>
          <div>
            <span style={{ color: '#6b7280', fontSize: 12 }}>训练 WS: </span>
            <Tag color={trainingConnected ? 'success' : 'error'}>
              {trainingConnected ? '已连接' : '未连接'}
            </Tag>
          </div>
          <div>
            <span style={{ color: '#6b7280', fontSize: 12 }}>生成 WS: </span>
            <Tag color={generateConnected ? 'success' : 'error'}>
              {generateConnected ? '已连接' : '未连接'}
            </Tag>
          </div>
          <div>
            <span style={{ color: '#6b7280', fontSize: 12 }}>货币 WS: </span>
            <Tag color={currencyConnected ? 'success' : 'error'}>
              {currencyConnected ? '已连接' : '未连接'}
            </Tag>
          </div>
          <div>
            <span style={{ color: '#6b7280', fontSize: 12 }}>训练状态: </span>
            <Tag color={trainingState?.is_training ? 'processing' : 'default'}>
              {trainingState?.is_training ? `训练中 (轮次 ${trainingState.epoch})` : '空闲'}
            </Tag>
          </div>
        </div>
      </Card>

      {/* 连接状态提示 */}
      {!isConnected && (
        <Card style={{
          background: 'linear-gradient(135deg, rgba(239, 68, 68, 0.2), rgba(239, 68, 68, 0.1))',
          borderColor: '#ef4444',
          marginBottom: 16,
          borderRadius: 12,
        }}>
          <Space>
            <CloseCircleOutlined style={{ color: '#ef4444', fontSize: 18 }} />
            <span style={{ color: '#fca5a5', fontWeight: 500 }}>训练服务未连接，请检查后端服务</span>
          </Space>
        </Card>
      )}

      <Row gutter={[16, 16]}>
        {/* 左侧：训练状态面板 */}
        <Col xs={24} lg={6}>
          {/* 训练状态卡片 */}
          <Card
            title={
              <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                <RocketOutlined style={{ color: '#6366f1', fontSize: 18 }} />
                <span style={{ color: '#f3f4f6', fontWeight: 600 }}>训练状态</span>
                {isTraining && <Badge status="processing" style={{ marginLeft: 8 }} />}
              </div>
            }
            style={{
              background: 'linear-gradient(180deg, #1a1a2e 0%, #16162a 100%)',
              borderColor: '#2d2d4a',
              borderRadius: 12,
            }}
            headStyle={{ borderBottom: '1px solid #2d2d4a' }}
          >
            {/* 状态指标 */}
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
              <div>
                <div style={{ color: '#6b7280', fontSize: 12, marginBottom: 4 }}>状态</div>
                <div style={{
                  color: isTraining ? '#10b981' : isPaused ? '#f59e0b' : '#9ca3af',
                  fontSize: 16,
                  fontWeight: 600,
                }}>
                  {trainingState?.is_training ? (trainingState?.is_paused ? '暂停中' : '训练中') : '空闲'}
                </div>
              </div>
              <div>
                <div style={{ color: '#6b7280', fontSize: 12, marginBottom: 4 }}>轮次</div>
                <div style={{ color: '#f3f4f6', fontSize: 16, fontWeight: 600 }}>
                  {trainingState?.epoch || 0} / {trainingState?.total_epochs || 100}
                </div>
              </div>
              <div>
                <div style={{ color: '#6b7280', fontSize: 12, marginBottom: 4 }}>样本</div>
                <div style={{ color: '#9ca3af', fontSize: 14 }}>
                  {trainingState?.samples_processed || 0} / {trainingState?.total_samples || 0}
                </div>
              </div>
              <div>
                <div style={{ color: '#6b7280', fontSize: 12, marginBottom: 4 }}>损失</div>
                <div style={{ color: '#10b981', fontSize: 14, fontWeight: 600 }}>
                  {trainingState?.loss?.toFixed(4) || '--'}
                </div>
              </div>
              <div>
                <div style={{ color: '#6b7280', fontSize: 12, marginBottom: 4 }}><BulbOutlined /> 模板</div>
                <div style={{ color: '#a78bfa', fontSize: 14 }}>{trainingState?.template_count || 0}</div>
              </div>
              <div>
                <div style={{ color: '#6b7280', fontSize: 12, marginBottom: 4 }}><DatabaseOutlined /> 规则</div>
                <div style={{ color: '#60a5fa', fontSize: 14 }}>{trainingState?.rule_count || 0}</div>
              </div>
            </div>

            {/* 进度条 */}
            <div style={{ marginTop: 20 }}>
              <div style={{ color: '#6b7280', fontSize: 12, marginBottom: 8 }}>训练进度</div>
              <Progress
                percent={trainingState ? (trainingState.epoch / trainingState.total_epochs * 100) : 0}
                showInfo={false}
                strokeColor={{ '0%': '#6366f1', '100%': '#8b5cf6' }}
                trailColor="#1f2937"
                style={{ height: 8 }}
              />
            </div>

            {/* 控制按钮 */}
            <div style={{ marginTop: 20, display: 'flex', flexDirection: 'column', gap: 8 }}>
              <Button
                type="primary"
                icon={<PlayCircleOutlined />}
                onClick={startTraining}
                disabled={isTraining}
                block
                style={{
                  background: 'linear-gradient(135deg, #10b981, #059669)',
                  borderColor: '#10b981',
                  height: 40,
                  fontWeight: 600,
                }}
              >
                开始训练
              </Button>
              <div style={{ display: 'flex', gap: 8 }}>
                <Button
                  icon={<PauseCircleOutlined />}
                  onClick={pauseTraining}
                  disabled={!isTraining}
                  style={{ flex: 1, height: 36, color: isTraining ? '#f3f4f6' : '#6b7280', borderColor: '#374151' }}
                >
                  暂停
                </Button>
                <Button
                  icon={<StopOutlined />}
                  onClick={stopTraining}
                  disabled={!trainingState?.is_training}
                  danger
                  style={{ flex: 1, height: 36, color: trainingState?.is_training ? '#f3f4f6' : '#6b7280' }}
                >
                  停止
                </Button>
              </div>
              {/* 设置按钮 */}
              <Button
                icon={<SettingOutlined />}
                onClick={() => navigate('/settings')}
                style={{ marginTop: 8, borderColor: '#374151', color: '#9ca3af' }}
              >
                训练数据配置
              </Button>
            </div>
          </Card>

          {/* 金币统计卡片 */}
          <Card
            style={{
              background: 'linear-gradient(180deg, #1a1a2e 0%, #16162a 100%)',
              borderColor: '#2d2d4a',
              borderRadius: 12,
              marginTop: 16,
            }}
            bodyStyle={{ padding: 16 }}
          >
            <div style={{ display: 'flex', alignItems: 'center', gap: 16 }}>
              <div style={{
                background: 'linear-gradient(135deg, #fbbf24, #f59e0b)',
                borderRadius: '50%',
                width: 48,
                height: 48,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                boxShadow: '0 4px 12px rgba(251, 191, 36, 0.3)',
              }}>
                <DollarOutlined style={{ fontSize: 24, color: '#1f2937' }} />
              </div>
              <div style={{ flex: 1 }}>
                <div style={{ color: '#6b7280', fontSize: 12 }}>金币总供应量</div>
                <div style={{ color: '#fbbf24', fontSize: 28, fontWeight: 'bold' }}>
                  {coinBalance.toFixed(0)}
                </div>
              </div>
            </div>
            <div style={{ color: '#6b7280', fontSize: 11, marginTop: 12 }}>
              蛊虫出生500金币 · 维持生命消耗 · 成功+10 · 失败-5
            </div>
          </Card>

          {/* 损失曲线 */}
          <Card
            title={<span style={{ color: '#f3f4f6', fontWeight: 600 }}><LineChartOutlined /> 损失曲线</span>}
            style={{
              background: 'linear-gradient(180deg, #1a1a2e 0%, #16162a 100%)',
              borderColor: '#2d2d4a',
              borderRadius: 12,
              marginTop: 16,
            }}
            headStyle={{ borderBottom: '1px solid #2d2d4a' }}
          >
            {lossHistory.length > 0 ? (
              <ReactECharts option={lossChartOption} style={{ height: 120 }} />
            ) : (
              <Empty description={<span style={{ color: '#6b7280' }}>等待训练数据</span>} style={{ padding: 20 }} />
            )}
          </Card>
        </Col>

        {/* 右侧：生成面板 */}
        <Col xs={24} lg={18}>
          {/* 场景生成输入 */}
          <Card
            style={{
              background: 'linear-gradient(180deg, #1a1a2e 0%, #16162a 100%)',
              borderColor: '#2d2d4a',
              borderRadius: 12,
              marginBottom: 16,
            }}
            bodyStyle={{ padding: 16 }}
          >
            <div style={{ display: 'flex', gap: 12 }}>
              <Input
                placeholder="输入描述生成 3D 场景，例如：一个房子里有桌子和椅子"
                value={inputText}
                onChange={(e) => setInputText(e.target.value)}
                onPressEnter={generateScene}
                size="large"
                style={{
                  flex: 1,
                  background: 'rgba(0, 0, 0, 0.4)',
                  borderColor: '#374151',
                  color: '#f3f4f6',
                  borderRadius: 8,
                }}
              />
              <Button
                type="primary"
                icon={<RocketOutlined />}
                onClick={generateScene}
                loading={generating}
                size="large"
                style={{
                  background: 'linear-gradient(135deg, #10b981, #059669)',
                  borderColor: '#10b981',
                  borderRadius: 8,
                  fontWeight: 600,
                  minWidth: 100,
                }}
              >
                生成
              </Button>
            </div>
          </Card>

          {/* 3D 场景预览 */}
          <Card
            title={
              <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                  <EyeOutlined style={{ color: '#6366f1' }} />
                  <span style={{ color: '#f3f4f6', fontWeight: 600 }}>3D 场景预览</span>
                </div>
                {selectedRecord && (
                  <Tag color={selectedRecord.success ? 'success' : 'error'}>
                    {selectedRecord.success ? `成功 +${selectedRecord.reward}` : `失败 ${selectedRecord.reward}`}
                  </Tag>
                )}
              </div>
            }
            style={{
              background: 'linear-gradient(180deg, #1a1a2e 0%, #16162a 100%)',
              borderColor: '#2d2d4a',
              borderRadius: 12,
              marginBottom: 16,
            }}
            headStyle={{ borderBottom: '1px solid #2d2d4a' }}
          >
            {currentScene ? (
              <div>
                <canvas
                  ref={canvasRef}
                  width={800}
                  height={400}
                  style={{
                    width: '100%',
                    height: 300,
                    borderRadius: 8,
                    border: '1px solid #2d2d4a',
                  }}
                />
                <div style={{ marginTop: 12, display: 'flex', gap: 16, flexWrap: 'wrap' }}>
                  {currentScene.map((node, i) => (
                    <Tag key={i} style={{
                      background: 'rgba(99, 102, 241, 0.2)',
                      borderColor: '#6366f1',
                      color: '#c7d2fe',
                    }}>
                      {node.geometry?.split('::').pop() || 'Node'} ({node.position.map(p => p.toFixed(1)).join(', ')})
                    </Tag>
                  ))}
                </div>

                {/* 奖惩按钮 */}
                {selectedRecord && !selectedRecord.feedbackGiven && (
                  <div style={{
                    marginTop: 16,
                    padding: '12px 16px',
                    background: 'rgba(0,0,0,0.3)',
                    borderRadius: 8,
                    border: '1px solid #374151',
                  }}>
                    <div style={{ color: '#9ca3af', fontSize: 12, marginBottom: 8 }}>
                      对这个生成结果满意吗？让神经网络学习
                    </div>
                    <div style={{ display: 'flex', gap: 12 }}>
                      <Button
                        type="primary"
                        icon={<CheckCircleOutlined />}
                        onClick={() => sendFeedback(selectedRecord, true)}
                        style={{
                          background: 'linear-gradient(135deg, #10b981, #059669)',
                          borderColor: '#10b981',
                          flex: 1,
                        }}
                      >
                        奖励 (好)
                      </Button>
                      <Button
                        danger
                        icon={<CloseCircleOutlined />}
                        onClick={() => sendFeedback(selectedRecord, false)}
                        style={{ flex: 1 }}
                      >
                        惩罚 (差)
                      </Button>
                    </div>
                  </div>
                )}

                {selectedRecord?.feedbackGiven && (
                  <div style={{
                    marginTop: 16,
                    padding: '12px 16px',
                    background: selectedRecord.feedbackGiven === 'reward'
                      ? 'rgba(16, 185, 129, 0.2)'
                      : 'rgba(239, 68, 68, 0.2)',
                    borderRadius: 8,
                    border: `1px solid ${selectedRecord.feedbackGiven === 'reward' ? '#10b981' : '#ef4444'}`,
                  }}>
                    <Space>
                      {selectedRecord.feedbackGiven === 'reward' ? (
                        <>
                          <CheckCircleOutlined style={{ color: '#10b981' }} />
                          <span style={{ color: '#10b981' }}>已奖励！神经网络会学习这个好结果</span>
                        </>
                      ) : (
                        <>
                          <CloseCircleOutlined style={{ color: '#ef4444' }} />
                          <span style={{ color: '#ef4444' }}>已惩罚！神经网络会避免这个错误</span>
                        </>
                      )}
                    </Space>
                  </div>
                )}
              </div>
            ) : (
              <div style={{
                height: 300,
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                justifyContent: 'center',
                background: 'rgba(0, 0, 0, 0.2)',
                borderRadius: 8,
                border: '1px dashed #374151',
              }}>
                <EyeOutlined style={{ fontSize: 48, color: '#374151', marginBottom: 16 }} />
                <span style={{ color: '#6b7280' }}>生成场景后在此预览</span>
                <span style={{ color: '#4b5563', fontSize: 12, marginTop: 8 }}>
                  查看历史记录可重新加载场景
                </span>
              </div>
            )}
          </Card>

          {/* 生成历史 */}
          <Card
            title={<span style={{ color: '#f3f4f6', fontWeight: 600 }}><ClockCircleOutlined /> 生成历史</span>}
            style={{
              background: 'linear-gradient(180deg, #1a1a2e 0%, #16162a 100%)',
              borderColor: '#2d2d4a',
              borderRadius: 12,
            }}
            headStyle={{ borderBottom: '1px solid #2d2d4a' }}
          >
            {history.length > 0 ? (
              <Table
                dataSource={history}
                columns={historyColumns}
                rowKey="id"
                size="small"
                pagination={{ pageSize: 8, size: 'small' }}
                style={{ background: 'transparent' }}
                rowClassName={() => 'custom-table-row'}
              />
            ) : (
              <Empty
                description={<span style={{ color: '#6b7280' }}>暂无生成记录</span>}
                style={{ padding: 40 }}
              />
            )}
          </Card>
        </Col>
      </Row>

      {/* 内联样式 */}
      <style>{`
        .custom-table-row:hover td {
          background: rgba(99, 102, 241, 0.1) !important;
        }
        .ant-table {
          background: transparent !important;
        }
        .ant-table-thead > tr > th {
          background: rgba(0, 0, 0, 0.2) !important;
          border-bottom: 1px solid #2d2d4a !important;
        }
        .ant-table-tbody > tr > td {
          border-bottom: 1px solid #1f2937 !important;
        }
        .ant-empty-description {
          color: #6b7280 !important;
        }
        .ant-input::placeholder {
          color: #6b7280 !important;
        }
        .ant-input:focus {
          border-color: #6366f1 !important;
          box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.2) !important;
        }
      `}</style>
    </div>
  )
}
