import { useState } from 'react'
import { Input, Button, Space, Typography, List, Tag, message, Tooltip, Spin } from 'antd'
import {
  SendOutlined,
  HistoryOutlined,
  QuestionCircleOutlined,
  ThunderboltOutlined,
  CheckCircleOutlined,
  CloseCircleOutlined,
} from '@ant-design/icons'

const { Text, Title } = Typography

interface CommandHistory {
  command: string
  result: string
  timestamp: number
  success: boolean
}

const availableCommands = [
  { cmd: 'create_gu <name>', desc: '创建新的蛊虫' },
  { cmd: 'list_gus', desc: '列出所有蛊虫' },
  { cmd: 'inspect <id>', desc: '查看蛊虫详情' },
  { cmd: 'task <type>', desc: '创建任务' },
  { cmd: 'status', desc: '显示世界状态' },
  { cmd: 'sync', desc: '强制同步' },
  { cmd: 'help', desc: '显示帮助' },
]

export default function CommandPanel() {
  const [input, setInput] = useState('')
  const [history, setHistory] = useState<CommandHistory[]>([])
  const [loading, setLoading] = useState(false)

  const executeCommand = async () => {
    if (!input.trim()) return

    setLoading(true)
    const startTime = Date.now()

    try {
      // 调用真实后端 API
      const response = await fetch('/api/command', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          command: input,
          args: []
        }),
      })

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }

      const data = await response.json()

      setHistory(prev => [...prev, {
        command: input,
        result: data.message || JSON.stringify(data.data) || '执行成功',
        timestamp: startTime,
        success: data.success !== false,
      }])

      if (data.success !== false) {
        message.success(data.message || '命令执行成功')
      } else {
        message.error(data.message || '命令执行失败')
      }

      setInput('')
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : '未知错误'

      setHistory(prev => [...prev, {
        command: input,
        result: `执行失败: ${errorMessage}`,
        timestamp: startTime,
        success: false,
      }])
      message.error(`命令执行失败: ${errorMessage}`)
    } finally {
      setLoading(false)
    }
  }

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      executeCommand()
    }
  }

  return (
    <div>
      <Title level={5} style={{ color: '#c9d1d9', marginBottom: 16 }}>
        <ThunderboltOutlined style={{ marginRight: 8 }} />
        命令终端
      </Title>

      <Space.Compact style={{ width: '100%', marginBottom: 16 }}>
        <Input
          placeholder="输入命令..."
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyPress={handleKeyPress}
          style={{ background: '#0d1117', borderColor: '#30363d', color: '#c9d1d9' }}
          prefix={<Text style={{ color: '#3fb950' }}>›</Text>}
          disabled={loading}
        />
        <Button
          type="primary"
          icon={loading ? <Spin size="small" /> : <SendOutlined />}
          onClick={executeCommand}
          loading={loading}
        >
          执行
        </Button>
      </Space.Compact>

      <div style={{ marginBottom: 16 }}>
        <Text style={{ color: '#8b949e', marginRight: 8 }}>快速命令:</Text>
        <Space wrap>
          {availableCommands.slice(0, 4).map((cmd) => (
            <Tooltip key={cmd.cmd} title={cmd.desc}>
              <Tag
                style={{ cursor: 'pointer', background: '#21262d', borderColor: '#30363d' }}
                onClick={() => setInput(cmd.cmd.split(' ')[0])}
              >
                {cmd.cmd.split(' ')[0]}
              </Tag>
            </Tooltip>
          ))}
          <Tooltip title="查看所有命令">
            <Tag
              style={{ cursor: 'pointer', background: '#21262d', borderColor: '#30363d' }}
              onClick={() => setInput('help')}
            >
              <QuestionCircleOutlined /> help
            </Tag>
          </Tooltip>
        </Space>
      </div>

      <div>
        <Text style={{ color: '#8b949e' }}>
          <HistoryOutlined style={{ marginRight: 8 }} />
          命令历史
        </Text>
        <div
          style={{
            marginTop: 8,
            height: 200,
            overflow: 'auto',
            background: '#0d1117',
            borderRadius: 8,
            padding: 8,
            border: '1px solid #30363d',
          }}
        >
          {history.length === 0 ? (
            <Text style={{ color: '#484f58' }}>暂无命令历史</Text>
          ) : (
            <List
              dataSource={history.slice().reverse()}
              renderItem={(item) => (
                <List.Item style={{ borderBottom: '1px solid #21262d', padding: '4px 0' }}>
                  <div style={{ width: '100%' }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                      <Text code style={{ color: '#58a6ff' }}>› {item.command}</Text>
                      {item.success ? (
                        <CheckCircleOutlined style={{ color: '#3fb950', marginLeft: 'auto' }} />
                      ) : (
                        <CloseCircleOutlined style={{ color: '#f85149', marginLeft: 'auto' }} />
                      )}
                    </div>
                    <Text style={{ color: '#8b949e', fontSize: 12 }}>{item.result}</Text>
                  </div>
                </List.Item>
              )}
            />
          )}
        </div>
      </div>
    </div>
  )
}
