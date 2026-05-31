import { useState, useEffect, useRef } from 'react'
import { Card, Table, Tag, Row, Col, Typography, Badge, Empty, Progress, Tooltip, Divider } from 'antd'
import {
  DollarOutlined,
  ArrowUpOutlined,
  ArrowDownOutlined,
  SwapOutlined,
  ClockCircleOutlined,
  WalletOutlined,
  TransactionOutlined,
  LineChartOutlined,
  PlusOutlined,
  MinusOutlined,
  UserOutlined,
} from '@ant-design/icons'
import type { ColumnsType } from 'antd/es/table'
import ReactECharts from 'echarts-for-react'

const { Title, Text } = Typography

// 交易记录接口 - 匹配后端 TransactionRecord
interface Transaction {
  id: string
  timestamp: number
  from_id: string
  from_name: string
  from_balance: number      // 交易后余额
  to_id: string
  to_name: string
  to_balance: number        // 交易后余额
  amount: number            // 正数=收入, 负数=支出
  kind: 'Deposit' | 'Withdraw' | 'Transfer'
  reason: string            // 简短原因
  detail: string            // 详细说明
}

// 货币统计接口
interface CurrencyStats {
  total_supply: number
  total_transactions: number
  velocity: number
  inflation_rate: number
}

// WebSocket 消息接口
interface CurrencyMessage {
  type: 'transaction' | 'stats' | 'history'
  data: Transaction | CurrencyStats | Transaction[]
}

// 最大保留记录数
const MAX_RECORDS = 1000

export default function CurrencyPage() {
  const [transactions, setTransactions] = useState<Transaction[]>([])
  const [stats, setStats] = useState<CurrencyStats | null>(null)
  const [connected, setConnected] = useState(false)
  const [supplyHistory, setSupplyHistory] = useState<number[]>([])
  const [velocityHistory, setVelocityHistory] = useState<number[]>([])
  const wsRef = useRef<WebSocket | null>(null)

  // WebSocket 连接
  useEffect(() => {
    const connect = () => {
      const wsUrl = 'ws://localhost:9000/ws/currency'
      console.log('[CurrencyPage] Connecting to:', wsUrl)

      const ws = new WebSocket(wsUrl)
      wsRef.current = ws

      ws.onopen = () => {
        console.log('[CurrencyPage] ✅ Connected to currency WebSocket')
        setConnected(true)
      }

      ws.onmessage = (event) => {
        try {
          const message: CurrencyMessage = JSON.parse(event.data)

          if (message.type === 'transaction') {
            const tx = message.data as Transaction
            setTransactions(prev => [tx, ...prev.slice(0, MAX_RECORDS - 1)])
          } else if (message.type === 'stats') {
            const newStats = message.data as CurrencyStats
            setStats(newStats)
            setSupplyHistory(prev => [...prev.slice(-59), newStats.total_supply])
            setVelocityHistory(prev => [...prev.slice(-59), newStats.velocity])
          } else if (message.type === 'history') {
            setTransactions((message.data as Transaction[]).slice(0, MAX_RECORDS))
          }
        } catch (e) {
          console.error('[CurrencyPage] Parse error:', e)
        }
      }

      ws.onclose = () => {
        console.log('[CurrencyPage] ❌ Disconnected')
        setConnected(false)
        setTimeout(connect, 5000)
      }

      ws.onerror = (error) => {
        console.error('[CurrencyPage] WebSocket error:', error)
      }
    }

    connect()

    return () => {
      wsRef.current?.close()
    }
  }, [])

  // 格式化时间
  const formatTime = (timestamp: number) => {
    const date = new Date(timestamp * 1000)
    return date.toLocaleTimeString('zh-CN', {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    })
  }

  // 格式化日期时间
  const formatDateTime = (timestamp: number) => {
    const date = new Date(timestamp * 1000)
    return date.toLocaleString('zh-CN', {
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    })
  }

  // 格式化金额
  const formatAmount = (amount: number) => {
    if (amount >= 1000000) {
      return `${(amount / 1000000).toFixed(2)}M`
    } else if (amount >= 1000) {
      return `${(amount / 1000).toFixed(2)}K`
    }
    return amount.toFixed(2)
  }

  // 格式化金额带符号
  const formatAmountWithSign = (amount: number, isIncome: boolean) => {
    const prefix = isIncome ? '+' : '-'
    const color = isIncome ? '#10b981' : '#ef4444'
    const formatted = formatAmount(Math.abs(amount))
    return (
      <Text style={{ color, fontWeight: 'bold', fontFamily: 'Orbitron, monospace' }}>
        {prefix}{formatted}
      </Text>
    )
  }

  // 表格列定义 - 简化显示
  const columns: ColumnsType<Transaction> = [
    {
      title: '时间',
      dataIndex: 'timestamp',
      key: 'timestamp',
      width: 90,
      render: (ts: number) => (
        <Tooltip title={formatDateTime(ts)}>
          <Text style={{ color: '#6b7280', fontSize: 12 }}>
            <ClockCircleOutlined style={{ marginRight: 4 }} />
            {formatTime(ts)}
          </Text>
        </Tooltip>
      ),
    },
    {
      title: '类型',
      dataIndex: 'kind',
      key: 'kind',
      width: 80,
      render: (kind: string) => {
        const config: Record<string, { color: string; icon: React.ReactNode; text: string }> = {
          Deposit: { color: 'green', icon: <PlusOutlined />, text: '获取' },
          Withdraw: { color: 'red', icon: <MinusOutlined />, text: '花费' },
          Transfer: { color: 'blue', icon: <SwapOutlined />, text: '转账' },
        }
        const { color, icon, text } = config[kind] || { color: 'default', icon: null, text: kind }
        return (
          <Tag color={color} style={{ borderRadius: 4, minWidth: 60, textAlign: 'center' }}>
            {icon} {text}
          </Tag>
        )
      },
    },
    {
      title: '来源',
      dataIndex: 'from_name',
      key: 'from',
      width: 100,
      render: (name: string, record) => (
        <Tooltip title={`ID: ${record.from_id || '系统'}`}>
          <div style={{ display: 'flex', alignItems: 'center', gap: 4 }}>
            <UserOutlined style={{ color: '#f59e0b', fontSize: 12 }} />
            <Text style={{ color: '#f59e0b', fontSize: 12 }}>{name || '系统'}</Text>
            {record.from_id !== 'system' && (
              <Text style={{ color: '#6b7280', fontSize: 10 }}>
                ¥{formatAmount(record.from_balance)}
              </Text>
            )}
          </div>
        </Tooltip>
      ),
    },
    {
      title: '目标',
      dataIndex: 'to_name',
      key: 'to',
      width: 100,
      render: (name: string, record) => (
        <Tooltip title={`ID: ${record.to_id || '系统'}`}>
          <div style={{ display: 'flex', alignItems: 'center', gap: 4 }}>
            <UserOutlined style={{ color: '#3b82f6', fontSize: 12 }} />
            <Text style={{ color: '#3b82f6', fontSize: 12 }}>{name || '系统'}</Text>
            {record.to_id !== 'system' && (
              <Text style={{ color: '#6b7280', fontSize: 10 }}>
                ¥{formatAmount(record.to_balance)}
              </Text>
            )}
          </div>
        </Tooltip>
      ),
    },
    {
      title: '流动',
      dataIndex: 'amount',
      key: 'amount',
      width: 100,
      render: (amount: number) => {
        const isIncome = amount > 0
        return (
          <div style={{ display: 'flex', alignItems: 'center', gap: 4 }}>
            {isIncome ? (
              <ArrowDownOutlined style={{ color: '#10b981' }} />
            ) : (
              <ArrowUpOutlined style={{ color: '#ef4444' }} />
            )}
            {formatAmountWithSign(Math.abs(amount), isIncome)}
          </div>
        )
      },
      sorter: (a, b) => a.amount - b.amount,
    },
    {
      title: '详细说明',
      dataIndex: 'detail',
      key: 'detail',
      render: (detail: string, record) => (
        <Tooltip title={`${record.reason} - ${detail}`}>
          <div>
            <Tag style={{ marginBottom: 4, fontSize: 11 }}>{record.reason}</Tag>
            <Text style={{ color: '#9ca3af', fontSize: 12, display: 'block' }}>
              {detail}
            </Text>
          </div>
        </Tooltip>
      ),
    },
  ]

  // 供应量图表配置
  const supplyChartOption = {
    backgroundColor: 'transparent',
    grid: { left: '10%', right: '5%', top: '15%', bottom: '15%' },
    xAxis: { type: 'category', show: false },
    yAxis: {
      type: 'value',
      axisLine: { lineStyle: { color: '#374151' } },
      splitLine: { lineStyle: { color: '#1f2937' } },
      axisLabel: { color: '#6b7280' },
    },
    series: [{
      type: 'line',
      data: supplyHistory,
      smooth: true,
      symbol: 'none',
      areaStyle: {
        color: {
          type: 'linear',
          x: 0, y: 0, x2: 0, y2: 1,
          colorStops: [
            { offset: 0, color: 'rgba(251, 191, 36, 0.4)' },
            { offset: 1, color: 'rgba(251, 191, 36, 0.05)' },
          ],
        },
      },
      lineStyle: { color: '#fbbf24', width: 2, shadowColor: 'rgba(251, 191, 36, 0.5)', shadowBlur: 10 },
    }],
  }

  // 流通速度图表配置
  const velocityChartOption = {
    backgroundColor: 'transparent',
    grid: { left: '10%', right: '5%', top: '15%', bottom: '15%' },
    xAxis: { type: 'category', show: false },
    yAxis: {
      type: 'value',
      axisLine: { lineStyle: { color: '#374151' } },
      splitLine: { lineStyle: { color: '#1f2937' } },
      axisLabel: { color: '#6b7280' },
    },
    series: [{
      type: 'bar',
      data: velocityHistory,
      itemStyle: {
        color: {
          type: 'linear',
          x: 0, y: 0, x2: 0, y2: 1,
          colorStops: [
            { offset: 0, color: '#8b5cf6' },
            { offset: 1, color: '#6366f1' },
          ],
        },
      },
    }],
  }

  return (
    <div>
      {/* 页面标题 */}
      <div style={{ marginBottom: 24 }}>
        <Title level={3} style={{ color: 'var(--text-primary)', margin: 0 }}>
          <DollarOutlined style={{ marginRight: 12, color: '#fbbf24' }} />
          金币流水
          <Badge
            status={connected ? 'success' : 'error'}
            text={connected ? '实时' : '离线'}
            style={{ marginLeft: 16 }}
          />
        </Title>
        <Text style={{ color: 'var(--text-muted)' }}>
          世界货币系统实时监控 · WebSocket 推送 · 最多保留 {MAX_RECORDS} 条记录
        </Text>
      </div>

      {/* 统计卡片 */}
      <Row gutter={[16, 16]}>
        <Col xs={24} sm={12} md={6}>
          <div className="game-card glow" style={{ borderColor: '#fbbf24' }}>
            <div className="data-label">MONEY SUPPLY</div>
            <div className="data-value" style={{ color: '#fbbf24' }}>
              <WalletOutlined style={{ marginRight: 8 }} />
              {stats ? formatAmount(stats.total_supply) : '--'}
            </div>
            <Progress
              percent={stats ? Math.min(stats.total_supply / 100000 * 100, 100) : 0}
              showInfo={false}
              strokeColor="#fbbf24"
              trailColor="#1f2937"
            />
          </div>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <div className="game-card" style={{ borderColor: '#6366f1' }}>
            <div className="data-label">TRANSACTIONS</div>
            <div className="data-value" style={{ color: '#6366f1' }}>
              <TransactionOutlined style={{ marginRight: 8 }} />
              {stats ? stats.total_transactions.toLocaleString() : '--'}
            </div>
            <Text style={{ color: 'var(--text-muted)', fontSize: 12 }}>
              Total Count
            </Text>
          </div>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <div className="game-card" style={{ borderColor: '#8b5cf6' }}>
            <div className="data-label">VELOCITY</div>
            <div className="data-value" style={{ color: '#8b5cf6' }}>
              <LineChartOutlined style={{ marginRight: 8 }} />
              {stats ? stats.velocity.toFixed(4) : '--'}
            </div>
            <Text style={{ color: 'var(--text-muted)', fontSize: 12 }}>
              V = T / M
            </Text>
          </div>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <div className="game-card" style={{ borderColor: '#10b981' }}>
            <div className="data-label">INFLATION</div>
            <div className="data-value" style={{ color: stats?.inflation_rate && stats.inflation_rate > 0 ? '#ef4444' : '#10b981' }}>
              {stats ? `${(stats.inflation_rate * 100).toFixed(2)}%` : '--'}
            </div>
            <Text style={{ color: 'var(--text-muted)', fontSize: 12 }}>
              π = k(MV - Y)
            </Text>
          </div>
        </Col>
      </Row>

      {/* 图表区域 */}
      <Row gutter={[16, 16]} style={{ marginTop: 16 }}>
        <Col xs={24} lg={12}>
          <Card style={{ background: 'var(--bg-card)', borderColor: 'var(--border-color)' }}>
            <div style={{ color: 'var(--text-secondary)', marginBottom: 8, fontSize: 12, textTransform: 'uppercase', letterSpacing: 1 }}>
              货币供应量趋势
            </div>
            <ReactECharts option={supplyChartOption} style={{ height: 200 }} />
          </Card>
        </Col>
        <Col xs={24} lg={12}>
          <Card style={{ background: 'var(--bg-card)', borderColor: 'var(--border-color)' }}>
            <div style={{ color: 'var(--text-secondary)', marginBottom: 8, fontSize: 12, textTransform: 'uppercase', letterSpacing: 1 }}>
              流通速度变化
            </div>
            <ReactECharts option={velocityChartOption} style={{ height: 200 }} />
          </Card>
        </Col>
      </Row>

      {/* 交易流水表 */}
      <Card
        style={{
          background: 'var(--bg-card)',
          borderColor: 'var(--border-color)',
          marginTop: 16,
        }}
        title={
          <span style={{ color: 'var(--text-primary)' }}>
            <DollarOutlined style={{ marginRight: 8, color: '#fbbf24' }} />
            实时交易流水
            <Tag color="gold" style={{ marginLeft: 8 }}>
              {transactions.length} / {MAX_RECORDS} 条
            </Tag>
          </span>
        }
        extra={
          <div style={{ display: 'flex', gap: 16, alignItems: 'center' }}>
            <Text style={{ color: '#10b981', fontSize: 12 }}>
              <PlusOutlined style={{ marginRight: 4 }} />
              获取 = +金额
            </Text>
            <Text style={{ color: '#ef4444', fontSize: 12 }}>
              <MinusOutlined style={{ marginRight: 4 }} />
              花费 = -金额
            </Text>
          </div>
        }
      >
        {transactions.length > 0 ? (
          <Table
            dataSource={transactions}
            columns={columns}
            rowKey="id"
            pagination={{ pageSize: 20, showSizeChanger: true, showTotal: (total) => `共 ${total} 条` }}
            style={{ background: 'transparent' }}
            scroll={{ x: 1000 }}
            size="small"
          />
        ) : (
          <Empty
            description={
              connected
                ? '等待交易数据...'
                : 'WebSocket 未连接，请检查后端服务'
            }
            style={{ padding: 40 }}
          />
        )}
      </Card>
    </div>
  )
}
