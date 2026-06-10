import { Typography, Card, Tag } from 'antd'
import {
  DashboardOutlined,
  ApiOutlined,
  TeamOutlined,
} from '@ant-design/icons'

const { Title, Text } = Typography

export default function DashboardPage() {
  return (
    <div style={{ textAlign: 'center', padding: 60 }}>
      <Title level={2} style={{ color: 'var(--text-primary)' }}>
        <DashboardOutlined style={{ marginRight: 12, color: '#6366f1' }} />
        世界神经网络监控面板
      </Title>
      <Text style={{ color: 'var(--text-muted)', fontSize: 16 }}>
        Herness World Neural Network Monitor
      </Text>
      <div style={{ marginTop: 40 }}>
        <Card
          style={{
            background: 'var(--bg-card)',
            borderColor: 'var(--border-color)',
            maxWidth: 600,
            margin: '0 auto',
          }}
        >
          <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 16 }}>
              <DashboardOutlined style={{ fontSize: 24, color: '#6366f1' }} />
              <div style={{ textAlign: 'left' }}>
                <Title level={5} style={{ color: 'var(--text-primary)', margin: 0 }}>
                  监控面板
                </Title>
                <Text style={{ color: 'var(--text-muted)' }}>
                  世界状态、蛊虫列表、神经网络拓扑
                </Text>
              </div>
            </div>
            <div style={{ display: 'flex', alignItems: 'center', gap: 16 }}>
              <TeamOutlined style={{ fontSize: 24, color: '#10b981' }} />
              <div style={{ textAlign: 'left' }}>
                <Title level={5} style={{ color: 'var(--text-primary)', margin: 0 }}>
                  蛊虫管理
                </Title>
                <Text style={{ color: 'var(--text-muted)' }}>
                  蛊虫创建、繁殖、状态监控
                </Text>
              </div>
            </div>
            <div style={{ display: 'flex', alignItems: 'center', gap: 16 }}>
              <ApiOutlined style={{ fontSize: 24, color: '#f59e0b' }} />
              <div style={{ textAlign: 'left' }}>
                <Title level={5} style={{ color: 'var(--text-primary)', margin: 0 }}>
                  接入点管理
                </Title>
                <Text style={{ color: 'var(--text-muted)' }}>
                  五接入点架构：感知、认知、行为、通信、生存
                </Text>
              </div>
            </div>
          </div>
        </Card>
      </div>
    </div>
  )
}
