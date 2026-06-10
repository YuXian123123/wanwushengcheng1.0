import { useState, useEffect } from 'react'
import {
  Typography, Card, Table, Button, Modal, Form, Input, InputNumber,
  Tag, Space, message, Popconfirm, Spin, Empty, Tooltip
} from 'antd'
import {
  PlusOutlined, CheckOutlined, CloseOutlined,
  CheckCircleOutlined, ClockCircleOutlined, StopOutlined,
  TrophyOutlined, RobotOutlined, QuestionCircleOutlined
} from '@ant-design/icons'
import type { ColumnsType } from 'antd/es/table'

const { Title, Text } = Typography
const { TextArea } = Input

// 任务状态枚举
enum TaskStatus {
  Pending = 'Pending',
  InProgress = 'InProgress',
  Completed = 'Completed',
  Cancelled = 'Cancelled',
}

// 任务接口
interface Task {
  id: string
  name: string
  description: string
  difficulty: number
  reward: number
  required_skills: string[]
  status: TaskStatus
  assigned_to: string | null
  assigned_to_name: string | null
  created_at: number
  completed_at: number | null
}

export default function TaskPage() {
  const [tasks, setTasks] = useState<Task[]>([])
  const [loading, setLoading] = useState(true)
  const [createModalOpen, setCreateModalOpen] = useState(false)
  const [form] = Form.useForm()

  // 获取任务列表
  const fetchTasks = async () => {
    try {
      const response = await fetch('/api/tasks')
      const data = await response.json()
      setTasks(data.tasks || [])
    } catch (error) {
      console.error('获取任务列表失败:', error)
      message.error('获取任务列表失败')
    }
  }

  useEffect(() => {
    const init = async () => {
      setLoading(true)
      await fetchTasks()
      setLoading(false)
    }
    init()

    // 定时刷新（世界模型会自动分配任务）
    const interval = setInterval(fetchTasks, 3000)
    return () => clearInterval(interval)
  }, [])

  // 创建任务
  const handleCreateTask = async (values: any) => {
    try {
      const response = await fetch('/api/tasks', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: values.name,
          description: values.description,
          reward: values.reward,
          skills: values.skills ? values.skills.split(',').map((s: string) => s.trim()).filter(Boolean) : [],
          difficulty: values.difficulty || 0.5,
        }),
      })

      if (response.ok) {
        message.success('任务创建成功，世界模型将自动分配执行者')
        setCreateModalOpen(false)
        form.resetFields()
        fetchTasks()
      } else {
        message.error('任务创建失败')
      }
    } catch (error) {
      console.error('创建任务失败:', error)
      message.error('创建任务失败')
    }
  }

  // 完成任务（用户确认）
  const handleCompleteTask = async (taskId: string) => {
    try {
      const response = await fetch(`/api/tasks/${taskId}/complete`, {
        method: 'POST',
      })

      const data = await response.json()
      if (response.ok) {
        message.success(`任务完成！奖励 ${data.reward} 金币已发放给 ${data.gu_name}`)
        fetchTasks()
      } else {
        message.error(data.message || '任务完成失败')
      }
    } catch (error) {
      console.error('完成任务失败:', error)
      message.error('完成任务失败')
    }
  }

  // 取消任务
  const handleCancelTask = async (taskId: string) => {
    try {
      const response = await fetch(`/api/tasks/${taskId}/cancel`, {
        method: 'POST',
      })

      if (response.ok) {
        message.success('任务已取消')
        fetchTasks()
      } else {
        message.error('取消任务失败')
      }
    } catch (error) {
      console.error('取消任务失败:', error)
      message.error('取消任务失败')
    }
  }

  // 状态标签渲染
  const renderStatusTag = (status: TaskStatus) => {
    switch (status) {
      case TaskStatus.Pending:
        return <Tag icon={<ClockCircleOutlined />} color="default">待领取</Tag>
      case TaskStatus.InProgress:
        return <Tag icon={<RobotOutlined />} color="processing">进行中</Tag>
      case TaskStatus.Completed:
        return <Tag icon={<CheckCircleOutlined />} color="success">已完成</Tag>
      case TaskStatus.Cancelled:
        return <Tag icon={<StopOutlined />} color="error">已取消</Tag>
      default:
        return <Tag>{status}</Tag>
    }
  }

  // 表格列定义
  const columns: ColumnsType<Task> = [
    {
      title: '任务名称',
      dataIndex: 'name',
      key: 'name',
      width: 200,
      render: (text: string) => <Text strong>{text}</Text>,
    },
    {
      title: '描述',
      dataIndex: 'description',
      key: 'description',
      width: 300,
      ellipsis: true,
    },
    {
      title: '奖励',
      dataIndex: 'reward',
      key: 'reward',
      width: 100,
      render: (reward: number) => (
        <Text style={{ color: '#f59e0b', fontWeight: 'bold' }}>
          <TrophyOutlined style={{ marginRight: 4 }} />
          {reward} 金币
        </Text>
      ),
    },
    {
      title: '难度',
      dataIndex: 'difficulty',
      key: 'difficulty',
      width: 80,
      render: (difficulty: number) => {
        const percent = Math.round(difficulty * 100)
        const color = difficulty < 0.3 ? '#10b981' : difficulty < 0.6 ? '#f59e0b' : '#ef4444'
        return <Tag style={{ borderColor: color, color }}>{percent}%</Tag>
      },
    },
    {
      title: '所需技能',
      dataIndex: 'required_skills',
      key: 'required_skills',
      width: 150,
      render: (skills: string[]) => (
        skills.length > 0
          ? skills.map((s, i) => <Tag key={i} color="blue">{s}</Tag>)
          : <Text type="secondary">无要求</Text>
      ),
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      width: 100,
      render: renderStatusTag,
    },
    {
      title: '执行者',
      dataIndex: 'assigned_to_name',
      key: 'assigned_to_name',
      width: 120,
      render: (name: string | null, record) => {
        if (name) {
          return (
            <Space>
              <span style={{
                display: 'inline-block',
                width: 8,
                height: 8,
                borderRadius: '50%',
                background: '#10b981',
              }} />
              {name}
            </Space>
          )
        }
        return (
          <Tooltip title="世界模型将自动分配">
            <Text type="secondary" style={{ fontStyle: 'italic' }}>
              <RobotOutlined style={{ marginRight: 4 }} />
              待分配
            </Text>
          </Tooltip>
        )
      },
    },
    {
      title: '操作',
      key: 'action',
      width: 180,
      render: (_, record) => (
        <Space size="small">
          {record.status === TaskStatus.InProgress && (
            <Popconfirm
              title="确认完成"
              description="确定任务已完成？奖励将发放给执行者。"
              onConfirm={() => handleCompleteTask(record.id)}
              okText="确认"
              cancelText="取消"
            >
              <Button type="primary" size="small" icon={<CheckOutlined />} style={{ background: '#10b981' }}>
                完成
              </Button>
            </Popconfirm>
          )}
          {(record.status === TaskStatus.Pending || record.status === TaskStatus.InProgress) && (
            <Popconfirm
              title="确认取消"
              description="确定要取消此任务吗？"
              onConfirm={() => handleCancelTask(record.id)}
              okText="确认"
              cancelText="返回"
            >
              <Button size="small" danger icon={<CloseOutlined />}>
                取消
              </Button>
            </Popconfirm>
          )}
          {record.status === TaskStatus.Completed && (
            <Text type="secondary">已完成</Text>
          )}
          {record.status === TaskStatus.Cancelled && (
            <Text type="secondary">已取消</Text>
          )}
        </Space>
      ),
    },
  ]

  return (
    <div style={{ padding: '0 24px' }}>
      <Card
        style={{ background: 'var(--bg-card)', borderColor: 'var(--border-color)' }}
        title={
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <Title level={4} style={{ color: 'var(--text-primary)', margin: 0 }}>
              <TrophyOutlined style={{ marginRight: 8, color: '#f59e0b' }} />
              任务管理
            </Title>
            <Space>
              <Tooltip title="世界模型会自动将任务分配给最合适的蛊虫">
                <QuestionCircleOutlined style={{ color: 'var(--text-muted)', cursor: 'help' }} />
              </Tooltip>
              <Button
                type="primary"
                icon={<PlusOutlined />}
                onClick={() => setCreateModalOpen(true)}
              >
                发布任务
              </Button>
            </Space>
          </div>
        }
      >
        {loading ? (
          <div style={{ textAlign: 'center', padding: 40 }}>
            <Spin size="large" />
          </div>
        ) : tasks.length === 0 ? (
          <Empty
            description="暂无任务，点击「发布任务」创建新任务"
            style={{ padding: 40 }}
          />
        ) : (
          <Table
            dataSource={tasks}
            columns={columns}
            rowKey="id"
            pagination={{ pageSize: 10 }}
            style={{ background: 'transparent' }}
            rowClassName={() => 'task-row'}
          />
        )}
      </Card>

      {/* 创建任务弹窗 */}
      <Modal
        title="发布新任务"
        open={createModalOpen}
        onCancel={() => setCreateModalOpen(false)}
        footer={null}
        width={600}
      >
        <div style={{
          background: 'rgba(99, 102, 241, 0.1)',
          padding: '12px 16px',
          borderRadius: 8,
          marginBottom: 16
        }}>
          <Text style={{ color: 'var(--text-muted)' }}>
            <RobotOutlined style={{ marginRight: 8 }} />
            任务创建后，世界模型会自动分配给最合适的蛊虫执行
          </Text>
        </div>

        <Form
          form={form}
          layout="vertical"
          onFinish={handleCreateTask}
          initialValues={{ difficulty: 0.5, reward: 50 }}
        >
          <Form.Item
            name="name"
            label="任务名称"
            rules={[{ required: true, message: '请输入任务名称' }]}
          >
            <Input placeholder="例如：火焰喷射技能熟练度提升" />
          </Form.Item>

          <Form.Item
            name="description"
            label="任务描述"
            rules={[{ required: true, message: '请输入任务描述' }]}
          >
            <TextArea rows={3} placeholder="详细描述任务内容和目标" />
          </Form.Item>

          <Form.Item
            name="reward"
            label="奖励金币"
            rules={[{ required: true, message: '请输入奖励金额' }]}
          >
            <InputNumber
              min={1}
              max={1000}
              style={{ width: '100%' }}
              placeholder="完成任务的奖励金币数量"
            />
          </Form.Item>

          <Form.Item
            name="difficulty"
            label="难度系数"
          >
            <InputNumber
              min={0}
              max={1}
              step={0.1}
              style={{ width: '100%' }}
              placeholder="0.0 - 1.0，默认 0.5"
            />
          </Form.Item>

          <Form.Item
            name="skills"
            label="所需技能（可选）"
          >
            <Input placeholder="用逗号分隔，例如：火焰喷射, 冰冻护盾" />
          </Form.Item>

          <Form.Item>
            <Space>
              <Button type="primary" htmlType="submit">
                发布任务
              </Button>
              <Button onClick={() => setCreateModalOpen(false)}>
                取消
              </Button>
            </Space>
          </Form.Item>
        </Form>
      </Modal>

      <style>{`
        .task-row:hover td {
          background: rgba(99, 102, 241, 0.1) !important;
        }
        .ant-table {
          background: transparent !important;
        }
        .ant-table-thead > tr > th {
          background: rgba(99, 102, 241, 0.1) !important;
          color: var(--text-primary) !important;
          border-bottom: 1px solid var(--border-color) !important;
        }
        .ant-table-tbody > tr > td {
          border-bottom: 1px solid var(--border-color) !important;
        }
        .ant-empty-description {
          color: var(--text-muted);
        }
      `}</style>
    </div>
  )
}
