import { Card, Row, Col, Progress, Typography, Tag, Tooltip, Alert, Badge } from 'antd'
import {
  EyeOutlined,
  ThunderboltOutlined,
  TeamOutlined,
  BulbOutlined,
  HeartOutlined,
  WifiOutlined,
  DisconnectOutlined,
} from '@ant-design/icons'
import { useWebSocket } from '../hooks/useWebSocket'

const { Text, Title } = Typography

/**
 * 五个接入点类型 - 与 Rust 后端一致
 * 定义在: src/world/access_point.rs
 *
 * 每个蛊虫智能体有 5 个接入点连接到世界神经网络：
 * - Perceive (感知): 接收外部输入
 * - Cognitive (认知): 推理与决策
 * - Behavior (行为): 输出行为
 * - Comm (通信): 与其他蛊虫通信
 * - Survival (生存): 生命状态同步
 */

interface AccessPointData {
  name: string
  nameEn: string
  value: number | null
  icon: React.ReactNode
  color: string
  description: string
  weight: number // 默认权重（来自后端）
}

// 接入点静态配置（名称、图标、颜色、描述、权重）
const accessPointConfig: Omit<AccessPointData, 'value'>[] = [
  {
    name: '感知接入点',
    nameEn: 'Perceive',
    icon: <BulbOutlined style={{ fontSize: 28 }} />,
    color: '#58a6ff',
    description: '接收外部环境输入，包括视觉、听觉等感知信息',
    weight: 1.0,
  },
  {
    name: '认知接入点',
    nameEn: 'Cognitive',
    icon: <EyeOutlined style={{ fontSize: 28 }} />,
    color: '#a371f7',
    description: '高级推理与决策通道，支持逻辑思维和复杂决策',
    weight: 2.0,
  },
  {
    name: '行为接入点',
    nameEn: 'Behavior',
    icon: <ThunderboltOutlined style={{ fontSize: 28 }} />,
    color: '#3fb950',
    description: '执行动作输出通道，控制蛊虫的行为决策',
    weight: 1.5,
  },
  {
    name: '通信接入点',
    nameEn: 'Comm',
    icon: <TeamOutlined style={{ fontSize: 28 }} />,
    color: '#f59e0b',
    description: '蛊虫间信息交换通道，支持协作与共识机制',
    weight: 1.0,
  },
  {
    name: '生存接入点',
    nameEn: 'Survival',
    icon: <HeartOutlined style={{ fontSize: 28 }} />,
    color: '#ef4444',
    description: '生命状态同步通道，维持蛊虫与世界生存绑定',
    weight: 0.5,
  },
]

// 信号类型说明
const signalTypes = [
  { name: 'Sensory', desc: '感知信号', color: '#58a6ff' },
  { name: 'Cognitive', desc: '认知信号', color: '#a371f7' },
  { name: 'Behavioral', desc: '行为信号', color: '#3fb950' },
  { name: 'Communication', desc: '通信信号', color: '#f59e0b' },
  { name: 'Survival', desc: '生存信号', color: '#ef4444' },
]

export default function AccessPointPanel() {
  // 从 WebSocket 获取实时指标数据
  const { metrics, connected, error } = useWebSocket()

  // 根据后端数据构建接入点状态
  const accessPoints: AccessPointData[] = accessPointConfig.map((config, index) => {
    // 从 WebSocket metrics 获取实时值，如果没有数据则为 null
    let value: number | null = null
    if (metrics) {
      switch (index) {
        case 0: value = metrics.perception ?? null; break
        case 1: value = metrics.cognition ?? null; break
        case 2: value = metrics.action ?? null; break
        case 3: value = metrics.communication ?? null; break
        case 4: value = metrics.survival ?? null; break
      }
    }

    return {
      ...config,
      value,
    }
  })

  return (
    <div>
      {/* 连接状态提示 */}
      {!connected && (
        <Alert
          type="error"
          showIcon
          icon={<DisconnectOutlined />}
          message="WebSocket 未连接"
          description={error || '无法连接到后端 WebSocket 服务器，请检查后端是否运行在 ws://localhost:9000/ws'}
          style={{ marginBottom: 16 }}
        />
      )}

      {/* 核心公式说明 */}
      <Card
        style={{
          background: 'linear-gradient(135deg, rgba(99, 102, 241, 0.1), rgba(139, 92, 246, 0.1))',
          borderColor: 'rgba(99, 102, 241, 0.3)',
          marginBottom: 16,
        }}
      >
        <div style={{ display: 'flex', alignItems: 'center', gap: 16 }}>
          <div
            style={{
              width: 48,
              height: 48,
              borderRadius: '50%',
              background: 'linear-gradient(135deg, #6366f1, #8b5cf6)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              fontSize: 20,
            }}
          >
            🧬
          </div>
          <div style={{ flex: 1 }}>
            <Title level={5} style={{ color: 'var(--text-primary)', margin: 0 }}>
              五接入点架构
            </Title>
            <Text style={{ color: 'var(--text-secondary)', fontSize: 13 }}>
              每个蛊虫智能体通过 5 个接入点连接到世界神经网络，形成统一意识
            </Text>
          </div>
          <Badge
            status={connected ? 'success' : 'error'}
            text={
              <Text style={{ color: connected ? '#3fb950' : '#f85149', fontSize: 12 }}>
                {connected ? (
                  <><WifiOutlined /> 已连接</>
                ) : (
                  <><DisconnectOutlined /> 未连接</>
                )}
              </Text>
            }
          />
        </div>

        <div
          style={{
            marginTop: 16,
            padding: 12,
            background: 'rgba(0, 0, 0, 0.2)',
            borderRadius: 8,
            fontFamily: 'Fira Code, monospace',
            fontSize: 13,
          }}
        >
          <div style={{ color: '#10b981', marginBottom: 4 }}>
            // 容量计算公式
          </div>
          <div style={{ color: 'var(--text-primary)' }}>
            Capacity = Base_capacity × (1 + Skill_bonus)
          </div>
          <div style={{ color: '#f59e0b', marginTop: 8, marginBottom: 4 }}>
            // 信号强度计算公式
          </div>
          <div style={{ color: 'var(--text-primary)' }}>
            S_received = S_sent × e^(-α×distance) × W_connection
          </div>
        </div>
      </Card>

      {/* 接入点状态 */}
      <Row gutter={[16, 16]}>
        {accessPoints.map((point) => (
          <Col xs={24} sm={12} lg={24 / 5 * 5} key={point.nameEn} style={{ maxWidth: '20%', flex: '0 0 20%' }}>
            <Tooltip
              title={
                <div>
                  <div style={{ fontWeight: 'bold', marginBottom: 4 }}>{point.name}</div>
                  <div style={{ fontSize: 12, opacity: 0.8 }}>{point.description}</div>
                  <div style={{ marginTop: 8, fontSize: 11 }}>
                    默认权重: {point.weight}
                  </div>
                  {point.value === null && (
                    <div style={{ marginTop: 8, color: '#f85149', fontSize: 11 }}>
                      ⚠️ 无数据
                    </div>
                  )}
                </div>
              }
            >
              <Card
                style={{
                  background: 'var(--bg-card)',
                  borderColor: point.value === null ? '#f85149' : 'var(--border-color)',
                  height: '100%',
                  cursor: 'pointer',
                  transition: 'all 0.3s ease',
                  opacity: point.value === null ? 0.6 : 1,
                }}
                hoverable
                className="hover-lift"
              >
                <div style={{ textAlign: 'center', marginBottom: 12 }}>
                  <div
                    style={{
                      width: 48,
                      height: 48,
                      margin: '0 auto',
                      borderRadius: '50%',
                      background: `linear-gradient(135deg, ${point.color}33, ${point.color}11)`,
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'center',
                      border: `2px solid ${point.value === null ? '#f85149' : point.color}`,
                    }}
                  >
                    <span style={{ color: point.color }}>{point.icon}</span>
                  </div>
                </div>

                <div style={{ textAlign: 'center', marginBottom: 8 }}>
                  <Text
                    style={{
                      color: point.color,
                      fontSize: 12,
                      fontWeight: 600,
                      textTransform: 'uppercase',
                      letterSpacing: 1,
                    }}
                  >
                    {point.nameEn}
                  </Text>
                </div>

                {point.value !== null ? (
                  <Progress
                    type="dashboard"
                    percent={Math.round(point.value * 100)}
                    strokeColor={point.color}
                    trailColor="#1f2937"
                    size={80}
                    format={(percent) => (
                      <span style={{ color: '#fff', fontSize: 16, fontFamily: 'Orbitron, monospace' }}>
                        {percent}
                      </span>
                    )}
                  />
                ) : (
                  <div style={{ textAlign: 'center', height: 80, display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
                    <Text style={{ color: '#f85149', fontSize: 12 }}>无数据</Text>
                  </div>
                )}
              </Card>
            </Tooltip>
          </Col>
        ))}
      </Row>

      {/* 信号类型说明 */}
      <Card
        style={{
          background: 'var(--bg-card)',
          borderColor: 'var(--border-color)',
          marginTop: 16,
        }}
      >
        <Title level={5} style={{ color: 'var(--text-primary)', marginBottom: 12 }}>
          信号类型
        </Title>
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: 8 }}>
          {signalTypes.map((signal) => (
            <Tag
              key={signal.name}
              style={{
                background: `${signal.color}22`,
                border: `1px solid ${signal.color}`,
                color: signal.color,
                padding: '4px 12px',
                borderRadius: 4,
              }}
            >
              {signal.name} ({signal.desc})
            </Tag>
          ))}
        </div>

        <div style={{ marginTop: 16, padding: 12, background: 'rgba(0, 0, 0, 0.2)', borderRadius: 8 }}>
          <Text style={{ color: 'var(--text-secondary)', fontSize: 13 }}>
            <strong style={{ color: 'var(--text-primary)' }}>设计理念：</strong>
            接入点作为蛊虫智能体与世界神经网络的接口，将个体的认知能力融入集体意识。
            每个接入点有独立的信号队列和处理容量，通过 Hebbian 学习规则不断优化连接权重。
          </Text>
        </div>
      </Card>
    </div>
  )
}
