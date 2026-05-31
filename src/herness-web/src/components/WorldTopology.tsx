import { useEffect, useRef, useState } from 'react'
import { Typography, Tag, Tooltip, Badge } from 'antd'
import { TeamOutlined, BulbOutlined, ThunderboltOutlined } from '@ant-design/icons'

const { Text, Title } = Typography

interface GuInfo {
  id: string
  name: string
  health: number
  abilities: string[]
  resources: number
  access_points: {
    perception: number
    action: number
    communication: number
    memory: number
    reasoning: number
  }
  active: boolean
  color: string
  generation: number
  is_primordial: boolean
}

interface WorldTopologyProps {
  gus: GuInfo[]
  syncRate: number
}

export default function WorldTopology({ gus, syncRate }: WorldTopologyProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const [nodes, setNodes] = useState<Array<{
    x: number
    y: number
    vx: number
    vy: number
    gu: GuInfo
  }>>([])
  const animationRef = useRef<number>()

  // 初始化节点位置
  useEffect(() => {
    if (!gus.length) return

    const width = canvasRef.current?.width || 800
    const height = canvasRef.current?.height || 400
    const centerX = width / 2
    const centerY = height / 2

    const newNodes = gus.map((gu, index) => {
      const angle = (index / gus.length) * Math.PI * 2
      const radius = 100 + Math.random() * 50
      return {
        x: centerX + Math.cos(angle) * radius,
        y: centerY + Math.sin(angle) * radius,
        vx: (Math.random() - 0.5) * 0.5,
        vy: (Math.random() - 0.5) * 0.5,
        gu,
      }
    })
    setNodes(newNodes)
  }, [gus])

  // 动画循环
  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) return

    const ctx = canvas.getContext('2d')
    if (!ctx) return

    const width = canvas.width
    const height = canvas.height
    const centerX = width / 2
    const centerY = height / 2

    const animate = () => {
      // 清空画布
      ctx.fillStyle = 'rgba(10, 10, 15, 0.1)'
      ctx.fillRect(0, 0, width, height)

      // 绘制中心（世界意识核心）
      const coreGlow = ctx.createRadialGradient(centerX, centerY, 0, centerX, centerY, 50)
      coreGlow.addColorStop(0, 'rgba(99, 102, 241, 0.8)')
      coreGlow.addColorStop(0.5, 'rgba(99, 102, 241, 0.2)')
      coreGlow.addColorStop(1, 'rgba(99, 102, 241, 0)')
      ctx.beginPath()
      ctx.arc(centerX, centerY, 50, 0, Math.PI * 2)
      ctx.fillStyle = coreGlow
      ctx.fill()

      // 绘制核心圆
      ctx.beginPath()
      ctx.arc(centerX, centerY, 15, 0, Math.PI * 2)
      ctx.fillStyle = '#6366f1'
      ctx.fill()
      ctx.strokeStyle = '#818cf8'
      ctx.lineWidth = 2
      ctx.stroke()

      // 绘制连接线
      nodes.forEach((node, i) => {
        // 到中心的连接
        const gradient = ctx.createLinearGradient(node.x, node.y, centerX, centerY)
        gradient.addColorStop(0, `rgba(99, 102, 241, ${node.gu.active ? 0.6 : 0.2})`)
        gradient.addColorStop(1, 'rgba(99, 102, 241, 0.1)')

        ctx.beginPath()
        ctx.moveTo(node.x, node.y)
        ctx.lineTo(centerX, centerY)
        ctx.strokeStyle = gradient
        ctx.lineWidth = node.gu.active ? 1.5 : 0.5
        ctx.stroke()

        // 节点间的连接（相邻节点）
        if (syncRate > 0.5) {
          nodes.forEach((other, j) => {
            if (i !== j && Math.random() < syncRate * 0.1) {
              const dist = Math.hypot(node.x - other.x, node.y - other.y)
              if (dist < 150) {
                ctx.beginPath()
                ctx.moveTo(node.x, node.y)
                ctx.lineTo(other.x, other.y)
                ctx.strokeStyle = `rgba(16, 185, 129, ${0.3 * syncRate})`
                ctx.lineWidth = 0.5
                ctx.stroke()
              }
            }
          })
        }

        // 更新节点位置（轻微漂移）
        node.x += node.vx
        node.y += node.vy

        // 边界反弹
        if (node.x < 50 || node.x > width - 50) node.vx *= -1
        if (node.y < 50 || node.y > height - 50) node.vy *= -1

        // 向中心的引力
        const dx = centerX - node.x
        const dy = centerY - node.y
        const dist = Math.hypot(dx, dy)
        if (dist > 150) {
          node.vx += dx * 0.0001
          node.vy += dy * 0.0001
        }
      })

      // 绘制节点
      nodes.forEach((node) => {
        const radius = 8 + node.gu.health * 4

        // 使用蛊虫的实际颜色
        const color = node.gu.active ? node.gu.color : '#6b7280'

        // 光晕
        const glow = ctx.createRadialGradient(node.x, node.y, 0, node.x, node.y, radius * 2)
        glow.addColorStop(0, color)
        glow.addColorStop(1, 'transparent')
        ctx.beginPath()
        ctx.arc(node.x, node.y, radius * 2, 0, Math.PI * 2)
        ctx.fillStyle = glow
        ctx.globalAlpha = node.gu.active ? 0.5 : 0.2
        ctx.fill()
        ctx.globalAlpha = 1

        // 节点本体
        ctx.beginPath()
        ctx.arc(node.x, node.y, radius, 0, Math.PI * 2)
        ctx.fillStyle = color
        ctx.fill()
        ctx.strokeStyle = node.gu.active ? '#fff' : '#374151'
        ctx.lineWidth = node.gu.is_primordial ? 2 : 1
        ctx.stroke()

        // 原种标记
        if (node.gu.is_primordial) {
          ctx.beginPath()
          ctx.arc(node.x, node.y, radius + 4, 0, Math.PI * 2)
          ctx.strokeStyle = '#fbbf24'
          ctx.lineWidth = 1
          ctx.stroke()
        }

        // 节点标签
        ctx.fillStyle = node.gu.active ? '#fff' : '#6b7280'
        ctx.font = '10px Orbitron, monospace'
        ctx.textAlign = 'center'
        ctx.fillText(node.gu.name.slice(0, 6), node.x, node.y + radius + 12)
      })

      animationRef.current = requestAnimationFrame(animate)
    }

    animate()

    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current)
      }
    }
  }, [nodes, syncRate])

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 16 }}>
        <div>
          <Title level={5} style={{ color: 'var(--text-primary)', margin: 0 }}>
            <TeamOutlined style={{ marginRight: 8 }} />
            World Neural Topology
          </Title>
          <Text style={{ color: 'var(--text-muted)', fontSize: 12 }}>
            Real-time visualization of neural node connections
          </Text>
        </div>
        <div style={{ display: 'flex', gap: 16 }}>
          <Tag style={{ background: 'rgba(16, 185, 129, 0.2)', border: '1px solid #10b981', color: '#10b981' }}>
            <BulbOutlined /> Active: {nodes.filter(n => n.gu.active).length}
          </Tag>
          <Tag style={{ background: 'rgba(107, 114, 128, 0.2)', border: '1px solid #6b7280', color: '#6b7280' }}>
            <ThunderboltOutlined /> Dormant: {nodes.filter(n => !n.gu.active).length}
          </Tag>
        </div>
      </div>

      <div
        style={{
          background: 'var(--bg-deep)',
          borderRadius: 8,
          border: '1px solid var(--border-color)',
          overflow: 'hidden',
        }}
      >
        <canvas
          ref={canvasRef}
          width={800}
          height={400}
          style={{ width: '100%', height: 400 }}
        />
      </div>

      <div style={{ marginTop: 12, display: 'flex', gap: 24 }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <div style={{ width: 12, height: 12, borderRadius: '50%', background: '#10b981' }} />
          <Text style={{ color: 'var(--text-muted)', fontSize: 12 }}>High Health</Text>
        </div>
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <div style={{ width: 12, height: 12, borderRadius: '50%', background: '#f59e0b' }} />
          <Text style={{ color: 'var(--text-muted)', fontSize: 12 }}>Medium Health</Text>
        </div>
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <div style={{ width: 12, height: 12, borderRadius: '50%', background: '#ef4444' }} />
          <Text style={{ color: 'var(--text-muted)', fontSize: 12 }}>Low Health</Text>
        </div>
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <div style={{ width: 12, height: 12, borderRadius: '50%', background: '#6b7280' }} />
          <Text style={{ color: 'var(--text-muted)', fontSize: 12 }}>Dormant</Text>
        </div>
      </div>
    </div>
  )
}
