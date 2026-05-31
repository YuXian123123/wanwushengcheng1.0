import { Table, Tag, Progress, Typography, Tooltip, Badge } from 'antd'
import { TeamOutlined, FireOutlined, ThunderboltOutlined, BulbOutlined, StarOutlined } from '@ant-design/icons'
import type { ColumnsType } from 'antd/es/table'

const { Text } = Typography

interface AccessPoints {
  perception: number
  action: number
  communication: number
  memory: number
  reasoning: number
}

interface GuInfo {
  id: string
  name: string
  health: number
  abilities: string[]
  resources: number
  access_points: AccessPoints
  active: boolean
  color: string
  generation: number
  is_primordial: boolean
}

interface GuListProps {
  gus: GuInfo[]
}

const accessPointLabels: Record<keyof AccessPoints, { label: string; icon: React.ReactNode; color: string }> = {
  perception: { label: '感知', icon: <BulbOutlined />, color: '#58a6ff' },
  action: { label: '行动', icon: <ThunderboltOutlined />, color: '#3fb950' },
  communication: { label: '通讯', icon: <TeamOutlined />, color: '#a371f7' },
  memory: { label: '记忆', icon: <FireOutlined />, color: '#d29922' },
  reasoning: { label: '推理', icon: <BulbOutlined />, color: '#f85149' },
}

// 获取世代颜色
function getGenerationColor(generation: number): string {
  const colors = [
    '#ff6b6b', // 1代 - 红
    '#feca57', // 2代 - 金
    '#48dbfb', // 3代 - 青
    '#ff9ff3', // 4代 - 粉
    '#54a0ff', // 5代 - 蓝
    '#5f27cd', // 6代 - 紫
    '#00d2d3', // 7代 - 绿松石
    '#ff9f43', // 8代 - 橙
  ]
  return colors[(generation - 1) % colors.length]
}

export default function GuList({ gus }: GuListProps) {
  const columns: ColumnsType<GuInfo> = [
    {
      title: '颜色',
      dataIndex: 'color',
      key: 'color',
      width: 50,
      render: (color: string, record: GuInfo) => (
        <Tooltip
          title={
            <div>
              <div>颜色: {color}</div>
              <div>世代: {record.generation}代</div>
              {record.is_primordial && <div style={{ color: '#fbbf24' }}>⭐ 原种</div>}
            </div>
          }
        >
          <div
            style={{
              width: 32,
              height: 32,
              borderRadius: '50%',
              background: color,
              border: record.is_primordial ? '2px solid #fbbf24' : '2px solid rgba(255,255,255,0.3)',
              boxShadow: `0 0 10px ${color}80`,
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              cursor: 'pointer',
            }}
          >
            {record.is_primordial && (
              <StarOutlined style={{ color: '#fff', fontSize: 12 }} />
            )}
          </div>
        </Tooltip>
      ),
    },
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
      render: (id: string) => <Text code style={{ color: '#8b949e' }}>{id.slice(0, 8)}</Text>,
    },
    {
      title: '名称',
      dataIndex: 'name',
      key: 'name',
      width: 140,
      render: (name: string, record: GuInfo) => (
        <span style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <Badge status={record.active ? 'success' : 'default'} />
          <Text style={{ color: record.color, fontWeight: 500 }}>{name}</Text>
          <Tag
            style={{
              background: `${getGenerationColor(record.generation)}20`,
              border: `1px solid ${getGenerationColor(record.generation)}`,
              color: getGenerationColor(record.generation),
              fontSize: 10,
            }}
          >
            {record.generation}代
          </Tag>
        </span>
      ),
    },
    {
      title: '生命值',
      dataIndex: 'health',
      key: 'health',
      width: 150,
      render: (health: number) => (
        <Progress
          percent={Math.round(health * 100)}
          size="small"
          strokeColor={health > 0.7 ? '#3fb950' : health > 0.3 ? '#d29922' : '#f85149'}
          format={(percent) => `${percent}%`}
        />
      ),
      sorter: (a, b) => a.health - b.health,
    },
    {
      title: '能力',
      dataIndex: 'abilities',
      key: 'abilities',
      width: 200,
      render: (abilities: string[]) => (
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: 4 }}>
          {abilities.slice(0, 3).map((ability, index) => (
            <Tag key={index} style={{ background: '#30363d', borderColor: '#484f58', color: '#c9d1d9' }}>
              {ability}
            </Tag>
          ))}
          {abilities.length > 3 && (
            <Tooltip title={abilities.slice(3).join(', ')}>
              <Tag style={{ background: '#30363d', borderColor: '#484f58', color: '#8b949e' }}>
                +{abilities.length - 3}
              </Tag>
            </Tooltip>
          )}
        </div>
      ),
    },
    {
      title: '资源',
      dataIndex: 'resources',
      key: 'resources',
      width: 100,
      render: (resources: number) => (
        <Text style={{ color: '#d29922' }}>💰 {resources}</Text>
      ),
      sorter: (a, b) => a.resources - b.resources,
    },
    {
      title: '接入点状态',
      dataIndex: 'access_points',
      key: 'access_points',
      render: (points: AccessPoints) => (
        <div style={{ display: 'flex', gap: 12 }}>
          {Object.entries(points).map(([key, value]) => {
            const { label, icon, color } = accessPointLabels[key as keyof AccessPoints]
            return (
              <Tooltip key={key} title={`${label}: ${Math.round(value * 100)}%`}>
                <div style={{ display: 'flex', alignItems: 'center', gap: 4 }}>
                  <span style={{ color }}>{icon}</span>
                  <Progress
                    type="circle"
                    percent={Math.round(value * 100)}
                    size={24}
                    strokeColor={color}
                    showInfo={false}
                  />
                </div>
              </Tooltip>
            )
          })}
        </div>
      ),
    },
  ]

  return (
    <div>
      {/* 统计信息 */}
      <div style={{ marginBottom: 16, display: 'flex', gap: 16 }}>
        <div>
          <Text style={{ color: '#9ca3af' }}>总数: </Text>
          <Text style={{ color: '#fff' }}>{gus.length}</Text>
        </div>
        <div>
          <Text style={{ color: '#9ca3af' }}>原种: </Text>
          <Text style={{ color: '#fbbf24' }}>{gus.filter(g => g.is_primordial).length}</Text>
        </div>
        <div>
          <Text style={{ color: '#9ca3af' }}>活跃: </Text>
          <Text style={{ color: '#3fb950' }}>{gus.filter(g => g.active).length}</Text>
        </div>
        <div>
          <Text style={{ color: '#9ca3af' }}>最高世代: </Text>
          <Text style={{ color: '#a371f7' }}>
            {gus.length > 0 ? Math.max(...gus.map(g => g.generation)) : 0}代
          </Text>
        </div>
      </div>

      <Table
        dataSource={gus}
        columns={columns}
        rowKey="id"
        pagination={{ pageSize: 10, showSizeChanger: true }}
        style={{ background: 'transparent' }}
        scroll={{ x: 1000 }}
        rowClassName={(record) => record.active ? '' : 'inactive-row'}
      />
    </div>
  )
}
