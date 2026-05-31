import { useEffect, useRef, useState, useCallback } from 'react'
import { List, Tag, Typography, Input, Select, Space, Empty, Badge, Alert } from 'antd'
import { FilterOutlined, ClockCircleOutlined, WifiOutlined, DisconnectOutlined } from '@ant-design/icons'

const { Text } = Typography

interface LogEntry {
  id: string
  timestamp: number
  level: 'info' | 'warn' | 'error' | 'debug'
  message: string
  source?: string
}

interface WsLogMessage {
  msg_type: 'log' | 'metrics' | 'event'
  timestamp: number
  data: {
    log?: {
      level: string
      message: string
    }
    event?: {
      event_type: string
      description: string
    }
  }
}

const levelColors = {
  info: 'blue',
  warn: 'gold',
  error: 'red',
  debug: 'default',
}

const levelLabels = {
  info: 'INFO',
  warn: 'WARN',
  error: 'ERROR',
  debug: 'DEBUG',
}

// 生成唯一ID
function generateId(): string {
  return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`
}

export default function LogPanel() {
  const [logs, setLogs] = useState<LogEntry[]>([])
  const [filter, setFilter] = useState<string>('all')
  const [search, setSearch] = useState('')
  const [connected, setConnected] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [retryCount, setRetryCount] = useState(0)
  const wsRef = useRef<WebSocket | null>(null)
  const listRef = useRef<HTMLDivElement>(null)
  const reconnectTimerRef = useRef<NodeJS.Timeout>()

  // 连接 WebSocket 接收日志
  const connectWebSocket = useCallback(() => {
    const isDev = import.meta.env.DEV
    const wsUrl = isDev
      ? 'ws://localhost:9000/ws'
      : `${window.location.protocol === 'https:' ? 'wss:' : 'ws:'}//${window.location.host}/ws`

    console.log('[LogPanel] Connecting to WebSocket:', wsUrl)

    try {
      const ws = new WebSocket(wsUrl)

      ws.onopen = () => {
        setConnected(true)
        setError(null)
        setRetryCount(0)
        console.log('[LogPanel] ✅ Connected to WebSocket')

        // 添加连接成功日志
        setLogs(prev => [...prev.slice(-99), {
          id: generateId(),
          timestamp: Date.now(),
          level: 'info',
          message: 'WebSocket 连接成功',
          source: 'system',
        }])
      }

      ws.onclose = (event) => {
        setConnected(false)
        console.error('[LogPanel] ❌ Disconnected from WebSocket')
        console.error('[LogPanel] Close code:', event.code, 'Reason:', event.reason)

        // 添加断开连接日志
        setLogs(prev => [...prev.slice(-99), {
          id: generateId(),
          timestamp: Date.now(),
          level: 'warn',
          message: `WebSocket 断开连接 (code: ${event.code})`,
          source: 'system',
        }])

        // 3秒后重连
        reconnectTimerRef.current = setTimeout(() => {
          setRetryCount(c => c + 1)
          connectWebSocket()
        }, 3000)
      }

      ws.onerror = (err) => {
        const errorMsg = `WebSocket 连接错误 (重试 #${retryCount + 1})`
        console.error('[LogPanel] ❌', errorMsg)
        console.error('[LogPanel] Error:', err)
        setError(errorMsg)
        ws.close()
      }

      ws.onmessage = (event) => {
        try {
          const msg: WsLogMessage = JSON.parse(event.data)

          // 处理日志消息
          if (msg.msg_type === 'log' && msg.data.log) {
            const newLog: LogEntry = {
              id: generateId(),
              timestamp: msg.timestamp,
              level: msg.data.log.level as 'info' | 'warn' | 'error' | 'debug' || 'info',
              message: msg.data.log.message,
              source: 'system',
            }
            setLogs(prev => [...prev.slice(-99), newLog]) // 保留最近100条
          }

          // 处理事件消息（转为日志格式）
          if (msg.msg_type === 'event' && msg.data.event) {
            const newLog: LogEntry = {
              id: generateId(),
              timestamp: msg.timestamp,
              level: 'info',
              message: `[${msg.data.event.event_type}] ${msg.data.event.description}`,
              source: 'event',
            }
            setLogs(prev => [...prev.slice(-99), newLog])
          }
        } catch (e) {
          console.error('[LogPanel] Failed to parse message:', e)
        }
      }

      wsRef.current = ws
    } catch (e) {
      const errorMsg = `Failed to create WebSocket: ${e}`
      console.error('[LogPanel] ❌', errorMsg)
      setError(errorMsg)
    }
  }, [retryCount])

  useEffect(() => {
    connectWebSocket()

    return () => {
      wsRef.current?.close()
      if (reconnectTimerRef.current) {
        clearTimeout(reconnectTimerRef.current)
      }
    }
  }, [connectWebSocket])

  // 自动滚动到底部
  useEffect(() => {
    if (listRef.current) {
      listRef.current.scrollTop = listRef.current.scrollHeight
    }
  }, [logs])

  const filteredLogs = logs.filter(log => {
    const matchesLevel = filter === 'all' || log.level === filter
    const matchesSearch = log.message.toLowerCase().includes(search.toLowerCase())
    return matchesLevel && matchesSearch
  })

  const formatTime = (timestamp: number) => {
    const date = new Date(timestamp)
    return date.toLocaleTimeString('zh-CN', { hour12: false })
  }

  return (
    <div>
      {/* 连接错误提示 */}
      {!connected && error && (
        <Alert
          type="error"
          showIcon
          icon={<DisconnectOutlined />}
          message="WebSocket 未连接"
          description={`${error} - 请检查后端是否运行在 ws://localhost:9000/ws`}
          style={{ marginBottom: 16 }}
        />
      )}

      <Space style={{ marginBottom: 16, width: '100%', justifyContent: 'space-between' }}>
        <Space>
          <Input
            placeholder="搜索日志..."
            prefix={<FilterOutlined />}
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            style={{ width: 200 }}
            allowClear
          />
          <Select
            value={filter}
            onChange={setFilter}
            style={{ width: 120 }}
            options={[
              { label: '全部', value: 'all' },
              { label: 'INFO', value: 'info' },
              { label: 'WARN', value: 'warn' },
              { label: 'ERROR', value: 'error' },
              { label: 'DEBUG', value: 'debug' },
            ]}
          />
        </Space>
        <Badge
          status={connected ? 'success' : 'error'}
          text={
            <Text style={{ color: connected ? '#3fb950' : '#f85149', fontSize: 12 }}>
              {connected ? (
                <><WifiOutlined /> 已连接</>
              ) : (
                <><DisconnectOutlined /> 未连接 (重试 #{retryCount})</>
              )}
            </Text>
          }
        />
      </Space>

      <div
        ref={listRef}
        style={{
          height: 300,
          overflow: 'auto',
          background: '#0d1117',
          borderRadius: 8,
          padding: 8,
          border: '1px solid #30363d',
        }}
      >
        {filteredLogs.length === 0 ? (
          <Empty
            description={connected ? "暂无日志" : "等待连接..."}
            style={{ marginTop: 80 }}
          />
        ) : (
          <List
            dataSource={filteredLogs}
            renderItem={(log) => (
              <List.Item style={{ borderBottom: '1px solid #21262d', padding: '8px 0' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: 12, width: '100%' }}>
                  <Text style={{ color: '#484f58', fontSize: 12, minWidth: 70 }}>
                    <ClockCircleOutlined style={{ marginRight: 4 }} />
                    {formatTime(log.timestamp)}
                  </Text>
                  <Tag color={levelColors[log.level]} style={{ minWidth: 50, textAlign: 'center' }}>
                    {levelLabels[log.level]}
                  </Tag>
                  {log.source && (
                    <Tag style={{ background: '#30363d', borderColor: '#484f58', color: '#8b949e' }}>
                      {log.source}
                    </Tag>
                  )}
                  <Text style={{ color: '#c9d1d9', flex: 1 }}>{log.message}</Text>
                </div>
              </List.Item>
            )}
          />
        )}
      </div>
    </div>
  )
}
