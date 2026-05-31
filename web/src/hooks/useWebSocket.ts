import { useEffect, useRef, useState, useCallback } from 'react'

interface Metrics {
  health: number
  sync_rate: number
  resonance_strength: number
  safety_score: number
  consciousness_emerged: boolean
  emergence_factor: number
  activity: number
  perception: number
  cognition: number
  action: number
  communication: number
  survival: number
}

interface WsMessage {
  msg_type: 'metrics' | 'log' | 'event'
  timestamp: number
  data: Metrics & {
    log?: { level: string; message: string }
    event?: { event_type: string; description: string }
  }
}

export function useWebSocket() {
  const [metrics, setMetrics] = useState<Metrics | null>(null)
  const [connected, setConnected] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const wsRef = useRef<WebSocket | null>(null)
  const reconnectTimerRef = useRef<NodeJS.Timeout>()
  const retryCountRef = useRef<number>(0)

  const connect = useCallback(() => {
    // 在开发环境直接连接后端 WebSocket
    // 生产环境使用相对路径
    const isDev = import.meta.env.DEV
    const wsUrl = isDev
      ? 'ws://localhost:9000/ws'
      : `${window.location.protocol === 'https:' ? 'wss:' : 'ws:'}//${window.location.host}/ws`

    console.log('[useWebSocket] Connecting to:', wsUrl)

    try {
      const ws = new WebSocket(wsUrl)

      ws.onopen = () => {
        setConnected(true)
        setError(null)
        retryCountRef.current = 0
        console.log('[useWebSocket] ✅ Connected to backend WebSocket')
      }

      ws.onclose = (event) => {
        setConnected(false)
        console.error('[useWebSocket] ❌ Disconnected from backend WebSocket')
        console.error('[useWebSocket] Close code:', event.code, 'Reason:', event.reason)

        // 3秒后重连
        reconnectTimerRef.current = setTimeout(connect, 3000)
      }

      ws.onerror = (error) => {
        retryCountRef.current++
        const errorMsg = `WebSocket connection failed (retry #${retryCountRef.current})`
        console.error('[useWebSocket] ❌ Connection error:', error)
        console.error('[useWebSocket] ', errorMsg)
        setError(errorMsg)
        ws.close()
      }

      ws.onmessage = (event) => {
        try {
          const msg = JSON.parse(event.data)
          // 后端使用 #[serde(rename = "type")]，所以字段名是 "type" 而不是 "msg_type"
          if (msg.type === 'metrics' || msg.msg_type === 'metrics') {
            setMetrics({
              health: msg.data.health,
              sync_rate: msg.data.sync_rate,
              resonance_strength: msg.data.resonance_strength,
              safety_score: msg.data.safety_score,
              consciousness_emerged: msg.data.consciousness_emerged,
              emergence_factor: msg.data.emergence_factor || 0.5,
              activity: msg.data.activity || 0.5,
              perception: msg.data.perception || 0.7,
              cognition: msg.data.cognition || 0.6,
              action: msg.data.action || 0.8,
              communication: msg.data.communication || 0.5,
              survival: msg.data.survival || 0.9,
            })
          }
        } catch (e) {
          console.error('[useWebSocket] Failed to parse message:', e)
        }
      }

      wsRef.current = ws
    } catch (e) {
      const errorMsg = `Failed to create WebSocket: ${e}`
      console.error('[useWebSocket] ❌', errorMsg)
      setError(errorMsg)
    }
  }, [])

  useEffect(() => {
    connect()

    return () => {
      wsRef.current?.close()
      if (reconnectTimerRef.current) {
        clearTimeout(reconnectTimerRef.current)
      }
    }
  }, [connect])

  const sendCommand = useCallback((command: string) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify({ type: 'command', command }))
    } else {
      console.error('[useWebSocket] ❌ Cannot send command: WebSocket not connected')
    }
  }, [])

  return { metrics, connected, error, sendCommand }
}
