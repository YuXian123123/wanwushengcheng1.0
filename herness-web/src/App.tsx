import { useState, useEffect } from 'react'
import { BrowserRouter, Routes, Route } from 'react-router-dom'
import { Layout, Tabs, Card, Row, Col, Tag, Progress, Typography, Alert, Spin } from 'antd'
import {
  TeamOutlined,
  ApiOutlined,
  SyncOutlined,
  SafetyOutlined,
  DisconnectOutlined,
  DatabaseOutlined,
  ThunderboltOutlined,
} from '@ant-design/icons'
import ReactECharts from 'echarts-for-react'

import { useWebSocket } from './hooks/useWebSocket'
import { useHttpData } from './hooks/useHttpData'
import NavMenu from './components/NavMenu'
import GuList from './components/GuList'
import AccessPointPanel from './components/AccessPointPanel'
import LogPanel from './components/LogPanel'
import CommandPanel from './components/CommandPanel'
import WorldTopology from './components/WorldTopology'
import ConsciousnessIndicator from './components/ConsciousnessIndicator'
import CurrencyPage from './pages/CurrencyPage'
import TaskPage from './pages/TaskPage'
import ChatPage from './pages/ChatPage'
import TrainingPage from './pages/TrainingPage'
import SettingsPage from './pages/SettingsPage'
import './theme.css'

const { Header, Content } = Layout
const { Title, Text } = Typography

// 监控面板组件
function MonitorPage() {
  const { metrics, connected, error: wsError } = useWebSocket()
  const { worldState, guList, stats, loading, error: httpError } = useHttpData()

  const [syncHistory, setSyncHistory] = useState<number[]>([])
  const [healthHistory, setHealthHistory] = useState<number[]>([])
  const [activityHistory, setActivityHistory] = useState<number[]>([])

  useEffect(() => {
    if (metrics) {
      setSyncHistory(prev => [...prev.slice(-59), metrics.sync_rate])
      setHealthHistory(prev => [...prev.slice(-59), metrics.health])
      setActivityHistory(prev => [...prev.slice(-59), metrics.activity || 0.5])
    }
  }, [metrics])

  const hasError = !connected || httpError
  const errorMessage = httpError || wsError

  const syncChartOption = {
    backgroundColor: 'transparent',
    grid: { left: '10%', right: '5%', top: '15%', bottom: '15%' },
    xAxis: { type: 'category', show: false },
    yAxis: {
      type: 'value',
      min: 0,
      max: 1,
      axisLine: { lineStyle: { color: '#374151' } },
      splitLine: { lineStyle: { color: '#1f2937' } },
      axisLabel: { color: '#6b7280' },
    },
    series: [{
      type: 'line',
      data: syncHistory,
      smooth: true,
      symbol: 'none',
      areaStyle: {
        color: {
          type: 'linear',
          x: 0, y: 0, x2: 0, y2: 1,
          colorStops: [
            { offset: 0, color: 'rgba(99, 102, 241, 0.4)' },
            { offset: 1, color: 'rgba(99, 102, 241, 0.05)' },
          ],
        },
      },
      lineStyle: { color: '#6366f1', width: 2, shadowColor: 'rgba(99, 102, 241, 0.5)', shadowBlur: 10 },
    }],
  }

  const healthGaugeOption = {
    backgroundColor: 'transparent',
    series: [{
      type: 'gauge',
      startAngle: 200,
      endAngle: -20,
      min: 0,
      max: 100,
      splitNumber: 10,
      itemStyle: {
        color: {
          type: 'linear',
          x: 0, y: 0, x2: 1, y2: 0,
          colorStops: [
            { offset: 0, color: '#ef4444' },
            { offset: 0.5, color: '#f59e0b' },
            { offset: 1, color: '#10b981' },
          ],
        },
      },
      progress: { show: true, width: 20 },
      pointer: { show: true, length: '60%', width: 4, itemStyle: { color: '#6366f1' } },
      axisLine: { lineStyle: { width: 20, color: [[1, '#1f2937']] } },
      axisTick: { show: false },
      splitLine: { length: 12, lineStyle: { width: 2, color: '#374151' } },
      axisLabel: { distance: 20, color: '#6b7280', fontSize: 10 },
      title: { show: false },
      detail: {
        valueAnimation: true,
        fontSize: 28,
        fontFamily: 'Orbitron, monospace',
        offsetCenter: [0, '70%'],
        formatter: '{value}%',
        color: '#f9fafb',
      },
      data: [{ value: Math.round((metrics?.health || 0) * 100), name: '健康度' }],
    }],
  }

  const radarOption = {
    backgroundColor: 'transparent',
    radar: {
      indicator: [
        { name: '感知', max: 100 },
        { name: '认知', max: 100 },
        { name: '行动', max: 100 },
        { name: '通讯', max: 100 },
        { name: '生存', max: 100 },
      ],
      axisName: { color: '#9ca3af' },
      splitLine: { lineStyle: { color: '#374151' } },
      splitArea: { areaStyle: { color: ['rgba(99, 102, 241, 0.05)', 'rgba(99, 102, 241, 0.1)'] } },
    },
    series: [{
      type: 'radar',
      data: [{
        value: [
          (metrics?.perception || 0) * 100,
          (metrics?.cognition || 0) * 100,
          (metrics?.action || 0) * 100,
          (metrics?.communication || 0) * 100,
          (metrics?.survival || 0) * 100,
        ],
        name: '世界能力',
        areaStyle: { color: 'rgba(99, 102, 241, 0.3)' },
        lineStyle: { color: '#6366f1', width: 2 },
        itemStyle: { color: '#6366f1' },
      }],
    }],
  }

  const isConscious = metrics?.consciousness_emerged || false
  const syncRate = metrics?.sync_rate || 0
  const emergenceFactor = metrics?.emergence_factor || 0

  return (
    <>
      {hasError && (
        <Alert
          type="error"
          showIcon
          icon={<DisconnectOutlined />}
          message="后端连接失败"
          description={
            <div>
              <p><strong>错误信息:</strong> {errorMessage}</p>
              <p>请检查后端服务是否运行在 <code>http://localhost:9000</code></p>
            </div>
          }
          style={{ marginBottom: 16 }}
        />
      )}

      {loading && (
        <div style={{ textAlign: 'center', padding: 40 }}>
          <Spin size="large" />
          <Text style={{ display: 'block', marginTop: 16, color: 'var(--text-muted)' }}>
            正在连接后端...
          </Text>
        </div>
      )}

      {/* 核心指标卡片 */}
      <Row gutter={[16, 16]}>
        <Col xs={24} sm={12} md={6}>
          <div className={`game-card ${connected && metrics ? 'glow' : ''}`} style={{ opacity: metrics ? 1 : 0.5 }}>
            <div className="data-label">WORLD HEALTH</div>
            <div className="data-value highlight">
              {metrics ? (
                <>{Math.round(metrics.health * 100)}<span style={{ fontSize: 14, marginLeft: 4 }}>%</span></>
              ) : (
                <span style={{ color: '#f85149' }}>--</span>
              )}
            </div>
            <Progress percent={(metrics?.health || 0) * 100} showInfo={false} strokeColor={{ '0%': '#ef4444', '50%': '#f59e0b', '100%': '#10b981' }} trailColor="#1f2937" />
          </div>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <div className="game-card" style={{ opacity: metrics ? 1 : 0.5 }}>
            <div className="data-label">SYNC RATE</div>
            <div className="data-value" style={{ color: syncRate > 0.7 ? '#10b981' : '#f59e0b' }}>
              {metrics ? (
                <>{Math.round(syncRate * 100)}<span style={{ fontSize: 14, marginLeft: 4 }}>%</span></>
              ) : (
                <span style={{ color: '#f85149' }}>--</span>
              )}
            </div>
            <Progress percent={syncRate * 100} showInfo={false} strokeColor="#6366f1" trailColor="#1f2937" />
          </div>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <div className="game-card" style={{ opacity: worldState ? 1 : 0.5 }}>
            <div className="data-label">POPULATION</div>
            <div className="data-value">
              <TeamOutlined style={{ marginRight: 8, color: '#6366f1' }} />
              {worldState?.population ?? <span style={{ color: '#f85149' }}>--</span>}
            </div>
            <Text style={{ color: 'var(--text-muted)', fontSize: 12 }}>Active Neural Nodes</Text>
          </div>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <div className="game-card" style={{ opacity: metrics ? 1 : 0.5 }}>
            <div className="data-label">SAFETY SCORE</div>
            <div className="data-value" style={{ color: '#a371f7' }}>
              <SafetyOutlined style={{ marginRight: 8 }} />
              {metrics ? `${Math.round(metrics.safety_score * 100)}%` : <span style={{ color: '#f85149' }}>--</span>}
            </div>
            <Progress percent={(metrics?.safety_score || 0) * 100} showInfo={false} strokeColor="#a371f7" trailColor="#1f2937" />
          </div>
        </Col>
      </Row>

      {/* 图表区域 */}
      <Row gutter={[16, 16]} style={{ marginTop: 16 }}>
        <Col xs={24} lg={8}>
          <Card style={{ background: 'var(--bg-card)', borderColor: 'var(--border-color)' }}>
            <div style={{ color: 'var(--text-secondary)', marginBottom: 8, fontSize: 12, textTransform: 'uppercase', letterSpacing: 1 }}>
              Sync Trend {syncHistory.length === 0 && <Tag color="red">无数据</Tag>}
            </div>
            <ReactECharts option={syncChartOption} style={{ height: 180 }} />
          </Card>
        </Col>
        <Col xs={24} lg={8}>
          <Card style={{ background: 'var(--bg-card)', borderColor: 'var(--border-color)' }}>
            <div style={{ color: 'var(--text-secondary)', marginBottom: 8, fontSize: 12, textTransform: 'uppercase', letterSpacing: 1 }}>
              Health Gauge {!metrics && <Tag color="red">无数据</Tag>}
            </div>
            <ReactECharts option={healthGaugeOption} style={{ height: 180 }} />
          </Card>
        </Col>
        <Col xs={24} lg={8}>
          <Card style={{ background: 'var(--bg-card)', borderColor: 'var(--border-color)' }}>
            <div style={{ color: 'var(--text-secondary)', marginBottom: 8, fontSize: 12, textTransform: 'uppercase', letterSpacing: 1 }}>
              Capability Matrix {!metrics && <Tag color="red">无数据</Tag>}
            </div>
            <ReactECharts option={radarOption} style={{ height: 180 }} />
          </Card>
        </Col>
      </Row>

      {/* 世界拓扑可视化 */}
      <Row gutter={[16, 16]} style={{ marginTop: 16 }}>
        <Col xs={24}>
          <Card style={{ background: 'var(--bg-card)', borderColor: 'var(--border-color)' }}>
            {guList.length > 0 ? (
              <WorldTopology gus={guList} syncRate={syncRate} />
            ) : (
              <div style={{ textAlign: 'center', padding: 40 }}>
                <Text style={{ color: 'var(--text-muted)' }}>{httpError || '等待蛊虫数据...'}</Text>
              </div>
            )}
          </Card>
        </Col>
      </Row>

      {/* 标签页区域 */}
      <Card style={{ background: 'var(--bg-card)', borderColor: 'var(--border-color)', marginTop: 16 }}>
        <Tabs
          defaultActiveKey="gus"
          style={{ color: 'var(--text-primary)' }}
          items={[
            {
              key: 'gus',
              label: <span style={{ color: 'var(--text-secondary)' }}><TeamOutlined style={{ marginRight: 4 }} /> Neural Nodes {guList.length === 0 && <Tag color="red" style={{ marginLeft: 8 }}>无数据</Tag>}</span>,
              children: guList.length > 0 ? (
                <GuList gus={guList} />
              ) : (
                <div style={{ textAlign: 'center', padding: 40 }}>
                  <DisconnectOutlined style={{ fontSize: 48, color: '#f85149' }} />
                  <Text style={{ display: 'block', marginTop: 16, color: 'var(--text-muted)' }}>{httpError || '无法获取蛊虫列表'}</Text>
                </div>
              ),
            },
            {
              key: 'access',
              label: <span style={{ color: 'var(--text-secondary)' }}><ApiOutlined style={{ marginRight: 4 }} /> Access Points {!connected && <Tag color="red" style={{ marginLeft: 8 }}>离线</Tag>}</span>,
              children: <AccessPointPanel />,
            },
            {
              key: 'logs',
              label: <span style={{ color: 'var(--text-secondary)' }}><DatabaseOutlined style={{ marginRight: 4 }} /> System Logs</span>,
              children: <LogPanel />,
            },
            {
              key: 'command',
              label: <span style={{ color: 'var(--text-secondary)' }}><ThunderboltOutlined style={{ marginRight: 4 }} /> Command</span>,
              children: <CommandPanel />,
            },
          ]}
        />
      </Card>
    </>
  )
}

function App() {
  const { metrics, connected, error: wsError } = useWebSocket()
  const isConscious = metrics?.consciousness_emerged || false
  const syncRate = metrics?.sync_rate || 0
  const emergenceFactor = metrics?.emergence_factor || 0

  return (
    <BrowserRouter>
      <Layout style={{ minHeight: '100vh', background: 'var(--bg-deep)' }}>
        {/* 顶部导航 */}
        <Header style={{
          background: 'linear-gradient(180deg, #111827 0%, #0a0a0f 100%)',
          padding: '0 24px',
          borderBottom: '1px solid var(--border-color)',
          height: 64,
          display: 'flex',
          alignItems: 'center',
        }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: 16 }}>
            <div style={{
              width: 40,
              height: 40,
              borderRadius: '50%',
              background: 'linear-gradient(135deg, #6366f1, #8b5cf6)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              boxShadow: '0 0 20px rgba(99, 102, 241, 0.5)',
            }}>
              <span style={{ fontSize: 20 }}>🌐</span>
            </div>
            <div>
              <Title level={4} style={{
                color: 'var(--text-primary)',
                margin: 0,
                fontFamily: 'Orbitron, monospace',
                letterSpacing: 2,
              }}>
                HERNESS
              </Title>
              <Text style={{ color: 'var(--text-muted)', fontSize: 10 }}>
                World Neural Network Monitor
              </Text>
            </div>
          </div>

          {/* 导航菜单 */}
          <NavMenu />

          {/* 右侧状态 */}
          <div style={{ display: 'flex', alignItems: 'center', gap: 16 }}>
            {metrics && (
              <ConsciousnessIndicator
                isConscious={isConscious}
                syncRate={syncRate}
                emergenceFactor={emergenceFactor}
              />
            )}
            <div className="status-indicator" style={{
              background: connected ? 'rgba(16, 185, 129, 0.2)' : 'rgba(239, 68, 68, 0.2)',
              color: connected ? '#10b981' : '#ef4444',
              border: `1px solid ${connected ? '#10b981' : '#ef4444'}`,
            }}>
              <span className="status-dot" style={{ background: connected ? '#10b981' : '#ef4444' }} />
              {connected ? 'LIVE' : 'OFFLINE'}
            </div>
          </div>
        </Header>

        <Layout>
          <Content style={{ padding: '24px' }}>
            <Routes>
              <Route path="/" element={<MonitorPage />} />
              <Route path="/currency" element={<CurrencyPage />} />
              <Route path="/tasks" element={<TaskPage />} />
              <Route path="/training" element={<TrainingPage />} />
              <Route path="/chat" element={<ChatPage />} />
              <Route path="/gus" element={<MonitorPage />} />
              <Route path="/settings" element={<SettingsPage />} />
            </Routes>
          </Content>
        </Layout>
      </Layout>
    </BrowserRouter>
  )
}

export default App
