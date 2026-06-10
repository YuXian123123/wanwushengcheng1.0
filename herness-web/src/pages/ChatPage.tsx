import { useState, useEffect, useRef } from 'react'
import { Card, List, Avatar, Input, Button, Tag, Typography, Row, Col, Badge, Spin, Empty, Tooltip, Space, message } from 'antd'
import {
  SendOutlined,
  TeamOutlined,
  GlobalOutlined,
  SettingOutlined,
  CommentOutlined,
  UserOutlined,
  RobotOutlined,
  CodeOutlined,
  FunctionOutlined,
  CheckCircleOutlined,
  CloseCircleOutlined,
  BulbOutlined,
  WarningOutlined,
  MessageOutlined,
} from '@ant-design/icons'

const { Text, Title, Paragraph } = Typography
const { TextArea } = Input

// API 基础 URL
const API_BASE = 'http://localhost:9000/api'

// 类型定义
interface ChatMessage {
  id: string
  sender_id: string
  sender_name: string
  sender_role: string
  content: any
  sent_at: number
}

interface ChatChannel {
  id: string
  name: string
  channel_type: string
  online_count: number
  message_count: number
}

// 角色图标映射
const roleIcons: Record<string, React.ReactNode> = {
  BlackTower: '🗼',
  Screwllum: '🔧',
  Latio: '📐',
  Gu: '🐛',
  System: '⚙️',
}

// 角色颜色映射
const roleColors: Record<string, string> = {
  BlackTower: '#9370db',
  Screwllum: '#27ae60',
  Latio: '#3498db',
  Gu: '#6366f1',
  System: '#feca57',
}

// 消息内容渲染组件
function MessageContent({ content }: { content: any }) {
  if (!content) return null

  switch (content.type || content.Text ? 'Text' : content.type) {
    case 'Text':
      return (
        <div className="message-text" style={{ lineHeight: 1.6 }}>
          {(content.Text || content.text || '').split('\n').map((line: string, i: number) => (
            <div key={i}>{line || <br />}</div>
          ))}
        </div>
      )

    case 'Formula':
      return (
        <div className="message-formula" style={{
          background: 'rgba(52,152,219,0.1)',
          borderLeft: '3px solid #3498db',
          padding: '10px 15px',
          margin: '8px 0',
          borderRadius: '0 8px 8px 0',
        }}>
          <FunctionOutlined style={{ marginRight: 8, color: '#3498db' }} />
          <code style={{ fontFamily: 'Cambria Math, serif', fontSize: 14 }}>
            {content.formula || content.Formula?.formula}
          </code>
          {content.description && (
            <Text type="secondary" style={{ display: 'block', marginTop: 8, fontSize: 12 }}>
              {content.description}
            </Text>
          )}
        </div>
      )

    case 'Code':
      return (
        <div className="message-code" style={{
          background: 'rgba(0,0,0,0.3)',
          borderRadius: 8,
          overflow: 'hidden',
          margin: '8px 0',
        }}>
          <div style={{
            background: 'rgba(0,0,0,0.2)',
            padding: '4px 12px',
            fontSize: 12,
            color: '#6b7280',
            borderBottom: '1px solid rgba(255,255,255,0.1)',
          }}>
            <CodeOutlined style={{ marginRight: 8 }} />
            {content.language || 'code'}
          </div>
          <pre style={{ margin: 0, padding: 12, fontSize: 13, overflow: 'auto', maxHeight: 300 }}>
            <code>{content.code || content.Code?.code}</code>
          </pre>
        </div>
      )

    case 'Proposal':
      return (
        <div className="message-proposal" style={{
          background: 'rgba(147,112,219,0.1)',
          border: '1px solid rgba(147,112,219,0.3)',
          borderRadius: 12,
          padding: 16,
          margin: '8px 0',
        }}>
          <div style={{ display: 'flex', alignItems: 'center', marginBottom: 12 }}>
            <BulbOutlined style={{ color: '#9370db', marginRight: 8 }} />
            <Text strong style={{ color: '#9370db' }}>
              📋 提案: {content.topic}
            </Text>
            <Tag color="purple" style={{ marginLeft: 8 }}>{content.part}</Tag>
          </div>
          <Paragraph style={{ margin: 0, color: 'var(--text-primary)' }}>
            {content.content}
          </Paragraph>
          <div style={{ marginTop: 12, display: 'flex', gap: 8 }}>
            <Button
              type="primary"
              size="small"
              icon={<CheckCircleOutlined />}
              style={{ background: '#27ae60', borderColor: '#27ae60' }}
            >
              支持
            </Button>
            <Button
              danger
              size="small"
              icon={<CloseCircleOutlined />}
            >
              反对
            </Button>
          </div>
        </div>
      )

    case 'Vote':
      return (
        <div className="message-vote" style={{
          display: 'flex',
          alignItems: 'center',
          gap: 8,
          padding: '8px 12px',
          background: content.support ? 'rgba(39,174,96,0.1)' : 'rgba(231,76,60,0.1)',
          borderRadius: 8,
          margin: '4px 0',
        }}>
          {content.support ? (
            <CheckCircleOutlined style={{ color: '#27ae60' }} />
          ) : (
            <CloseCircleOutlined style={{ color: '#e74c3c' }} />
          )}
          <Text style={{ color: content.support ? '#27ae60' : '#e74c3c' }}>
            {content.support ? '支持' : '反对'}
          </Text>
          <Text type="secondary" style={{ fontSize: 12 }}>
            {content.reason}
          </Text>
        </div>
      )

    case 'Opposition':
      return (
        <div className="message-opposition" style={{
          background: 'rgba(231,76,60,0.1)',
          borderLeft: '3px solid #e74c3c',
          padding: 12,
          margin: '8px 0',
          borderRadius: '0 8px 8px 0',
        }}>
          <div style={{ display: 'flex', alignItems: 'center', marginBottom: 8 }}>
            <WarningOutlined style={{ color: '#e74c3c', marginRight: 8 }} />
            <Text strong style={{ color: '#e74c3c' }}>⚠️ 反对意见</Text>
          </div>
          <Text>{content.reason}</Text>
          {content.suggestion && (
            <Text type="secondary" style={{ display: 'block', marginTop: 8 }}>
              💡 建议: {content.suggestion}
            </Text>
          )}
        </div>
      )

    case 'ConflictResolution':
      return (
        <div className="message-resolution" style={{
          background: 'rgba(39,174,96,0.1)',
          borderRadius: 12,
          padding: 16,
          margin: '8px 0',
        }}>
          <div style={{ display: 'flex', alignItems: 'center', marginBottom: 8 }}>
            <CheckCircleOutlined style={{ color: '#27ae60', marginRight: 8 }} />
            <Text strong style={{ color: '#27ae60' }}>✅ 冲突解决</Text>
            <Tag color="green" style={{ marginLeft: 8 }}>{content.method}</Tag>
          </div>
          <Text>{content.final_content}</Text>
        </div>
      )

    case 'ConsensusReached':
      return (
        <div className="message-consensus" style={{
          background: 'linear-gradient(135deg, rgba(254,202,87,0.2), rgba(255,107,107,0.2))',
          borderRadius: 12,
          padding: 16,
          margin: '8px 0',
          textAlign: 'center',
        }}>
          <Text style={{ fontSize: 24 }}>🎉</Text>
          <Title level={5} style={{ color: '#feca57', margin: '8px 0' }}>共识达成!</Title>
          <Text>主题: {content.topic}</Text>
          <br />
          <Text type="secondary">参与者: {content.participants} 只蛊虫</Text>
        </div>
      )

    case 'SystemNotification':
      return (
        <div className="system-notification" style={{
          textAlign: 'center',
          padding: '8px 16px',
          margin: '8px 0',
        }}>
          <Tag
            color={
              content.notification_type === 'KnowledgeStored' ? 'green' :
              content.notification_type === 'DiscussionStarted' ? 'blue' :
              content.notification_type === 'ConflictResolved' ? 'orange' :
              'default'
            }
            style={{ borderRadius: 12 }}
          >
            {content.content}
          </Tag>
        </div>
      )

    case 'ExperienceSummary':
      return (
        <div className="message-experience" style={{
          background: 'rgba(156,39,176,0.1)',
          borderLeft: '3px solid #9c27b0',
          padding: 12,
          margin: '8px 0',
          borderRadius: '0 8px 8px 0',
        }}>
          <Text strong style={{ color: '#9c27b0' }}>📚 经验总结: {content.topic}</Text>
          <ul style={{ margin: '8px 0 0 20px', padding: 0 }}>
            {content.lessons?.map((lesson: string, i: number) => (
              <li key={i} style={{ color: 'var(--text-secondary)', marginBottom: 4 }}>
                {lesson}
              </li>
            ))}
          </ul>
        </div>
      )

    default:
      // 尝试解析可能是 JSON 字符串的内容
      if (typeof content === 'string') {
        try {
          const parsed = JSON.parse(content)
          return <MessageContent content={parsed} />
        } catch {
          return <Text>{content}</Text>
        }
      }
      return <Text>{JSON.stringify(content)}</Text>
  }
}

// 消息项组件
function MessageItem({ message }: { message: ChatMessage }) {
  const role = message.sender_role || 'Gu'
  const icon = roleIcons[role] || '🐛'
  const color = roleColors[role] || '#6366f1'

  return (
    <div className="message-item" style={{
      display: 'flex',
      gap: 12,
      padding: '12px 16px',
      borderBottom: '1px solid rgba(255,255,255,0.05)',
    }}>
      <Avatar
        size={40}
        style={{
          background: `${color}33`,
          color: color,
          flexShrink: 0,
        }}
      >
        {icon}
      </Avatar>
      <div style={{ flex: 1, minWidth: 0 }}>
        <div style={{ display: 'flex', alignItems: 'baseline', gap: 8, marginBottom: 4 }}>
          <Text strong style={{ color }}>
            {message.sender_name}
          </Text>
          <Text type="secondary" style={{ fontSize: 12 }}>
            {new Date(message.sent_at * 1000).toLocaleTimeString()}
          </Text>
        </div>
        <MessageContent content={message.content} />
      </div>
    </div>
  )
}

// 频道图标
function getChannelIcon(type: string) {
  switch (type) {
    case 'World':
      return <GlobalOutlined />
    case 'Collaboration':
      return <TeamOutlined />
    case 'Private':
      return <UserOutlined />
    case 'System':
      return <SettingOutlined />
    default:
      return <CommentOutlined />
  }
}

export default function ChatPage() {
  const [channels, setChannels] = useState<ChatChannel[]>([])
  const [selectedChannel, setSelectedChannel] = useState<string>('knowledge')
  const [messages, setMessages] = useState<ChatMessage[]>([])
  const [inputValue, setInputValue] = useState('')
  const [loading, setLoading] = useState(false)
  const [sending, setSending] = useState(false)
  const messagesEndRef = useRef<HTMLDivElement>(null)

  // 加载频道列表
  useEffect(() => {
    async function loadChannels() {
      try {
        const response = await fetch(`${API_BASE}/chat/channels`)
        if (response.ok) {
          const data = await response.json()
          setChannels(data)
        }
      } catch (e) {
        // 使用默认频道
        setChannels([
          { id: 'world', name: '世界意识', channel_type: 'World', online_count: 25, message_count: 0 },
          { id: 'knowledge', name: '知识讨论', channel_type: 'Collaboration', online_count: 12, message_count: 0 },
          { id: 'genius-council', name: '天才议会', channel_type: 'Collaboration', online_count: 3, message_count: 0 },
          { id: 'system', name: '系统日志', channel_type: 'System', online_count: 1, message_count: 0 },
        ])
      }
    }
    loadChannels()
  }, [])

  // 加载消息
  useEffect(() => {
    async function loadMessages() {
      setLoading(true)
      try {
        const response = await fetch(`${API_BASE}/chat/channels/${selectedChannel}/messages`)
        if (response.ok) {
          const data = await response.json()
          setMessages(data)
        }
      } catch (e) {
        // 使用模拟消息
        setMessages(getMockMessages())
      }
      setLoading(false)
    }
    loadMessages()
  }, [selectedChannel])

  // 滚动到底部
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [messages])

  // 发送消息
  const handleSend = async () => {
    if (!inputValue.trim()) return

    setSending(true)
    try {
      const response = await fetch(`${API_BASE}/chat/channels/${selectedChannel}/messages`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          sender_id: 'user-001',
          sender_name: '用户',
          content: inputValue,
        }),
      })

      if (response.ok) {
        // 重新加载消息
        const messagesResponse = await fetch(`${API_BASE}/chat/channels/${selectedChannel}/messages`)
        if (messagesResponse.ok) {
          const data = await messagesResponse.json()
          setMessages(data)
        }
        setInputValue('')
        message.success('消息已发送')
      } else {
        // 本地添加消息
        setMessages(prev => [...prev, {
          id: `local-${Date.now()}`,
          sender_id: 'user-001',
          sender_name: '用户',
          sender_role: 'Gu',
          content: { type: 'Text', text: inputValue },
          sent_at: Math.floor(Date.now() / 1000),
        }])
        setInputValue('')
      }
    } catch (e) {
      // 离线模式，本地添加
      setMessages(prev => [...prev, {
        id: `local-${Date.now()}`,
        sender_id: 'user-001',
        sender_name: '用户',
        sender_role: 'Gu',
        content: { type: 'Text', text: inputValue },
        sent_at: Math.floor(Date.now() / 1000),
      }])
      setInputValue('')
    }
    setSending(false)
  }

  return (
    <div style={{ height: 'calc(100vh - 112px)', display: 'flex', gap: 16 }}>
      {/* 频道列表 */}
      <Card
        style={{
          width: 260,
          background: 'var(--bg-card)',
          borderColor: 'var(--border-color)',
          display: 'flex',
          flexDirection: 'column',
        }}
        bodyStyle={{ flex: 1, overflow: 'auto', padding: 0 }}
        title={
          <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <CommentOutlined />
            <Text style={{ color: 'var(--text-primary)' }}>通信频道</Text>
          </div>
        }
      >
        <List
          dataSource={channels}
          renderItem={(channel) => (
            <List.Item
              onClick={() => setSelectedChannel(channel.id)}
              style={{
                cursor: 'pointer',
                padding: '12px 16px',
                background: selectedChannel === channel.id
                  ? 'rgba(99,102,241,0.1)'
                  : 'transparent',
                borderLeft: selectedChannel === channel.id
                  ? '3px solid #6366f1'
                  : '3px solid transparent',
              }}
            >
              <div style={{ display: 'flex', alignItems: 'center', gap: 12, width: '100%' }}>
                <Avatar
                  size={36}
                  icon={getChannelIcon(channel.channel_type)}
                  style={{
                    background: selectedChannel === channel.id
                      ? '#6366f1'
                      : 'rgba(255,255,255,0.1)',
                  }}
                />
                <div style={{ flex: 1 }}>
                  <Text style={{ color: selectedChannel === channel.id ? '#6366f1' : 'var(--text-primary)' }}>
                    {channel.name}
                  </Text>
                  <div>
                    <Text type="secondary" style={{ fontSize: 12 }}>
                      {channel.online_count} 在线
                    </Text>
                  </div>
                </div>
                {channel.message_count > 0 && (
                  <Badge count={channel.message_count} />
                )}
              </div>
            </List.Item>
          )}
        />
      </Card>

      {/* 消息区域 */}
      <Card
        style={{
          flex: 1,
          background: 'var(--bg-card)',
          borderColor: 'var(--border-color)',
          display: 'flex',
          flexDirection: 'column',
          overflow: 'hidden',
        }}
        bodyStyle={{ flex: 1, display: 'flex', flexDirection: 'column', padding: 0, overflow: 'hidden' }}
        title={
          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
            <Space>
              {getChannelIcon(channels.find(c => c.id === selectedChannel)?.channel_type || 'Collaboration')}
              <Text style={{ color: 'var(--text-primary)' }}>
                {channels.find(c => c.id === selectedChannel)?.name || selectedChannel}
              </Text>
            </Space>
            <Badge
              status="success"
              text={
                <Text type="secondary" style={{ fontSize: 12 }}>
                  {channels.find(c => c.id === selectedChannel)?.online_count || 0} 在线
                </Text>
              }
            />
          </div>
        }
      >
        {/* 消息列表 - 固定高度可滚动 */}
        <div style={{
          flex: 1,
          overflowY: 'auto',
          overflowX: 'hidden',
          display: 'flex',
          flexDirection: 'column',
          minHeight: 0,
        }}>
          {loading ? (
            <div style={{ textAlign: 'center', padding: 40 }}>
              <Spin />
            </div>
          ) : messages.length === 0 ? (
            <Empty
              description={<Text type="secondary">暂无消息</Text>}
              style={{ margin: 'auto' }}
            />
          ) : (
            messages.map((msg) => (
              <MessageItem key={msg.id} message={msg} />
            ))
          )}
          <div ref={messagesEndRef} />
        </div>

        {/* 输入区域 */}
        <div style={{
          padding: 16,
          borderTop: '1px solid var(--border-color)',
          background: 'rgba(0,0,0,0.2)',
        }}>
          <div style={{ display: 'flex', gap: 8 }}>
            <TextArea
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              placeholder="输入消息... (支持 Markdown)"
              autoSize={{ minRows: 1, maxRows: 4 }}
              style={{
                background: 'rgba(255,255,255,0.05)',
                borderColor: 'var(--border-color)',
                color: 'var(--text-primary)',
              }}
              onPressEnter={(e) => {
                if (!e.shiftKey) {
                  e.preventDefault()
                  handleSend()
                }
              }}
            />
            <Button
              type="primary"
              icon={<SendOutlined />}
              onClick={handleSend}
              loading={sending}
              style={{
                background: 'linear-gradient(135deg, #6366f1, #8b5cf6)',
                borderColor: 'transparent',
                alignSelf: 'flex-end',
              }}
            >
              发送
            </Button>
          </div>
          <div style={{ marginTop: 8, display: 'flex', gap: 8 }}>
            <Button size="small" type="text" style={{ color: 'var(--text-muted)' }}>
              📋 提案
            </Button>
            <Button size="small" type="text" style={{ color: 'var(--text-muted)' }}>
              📐 公式
            </Button>
            <Button size="small" type="text" style={{ color: 'var(--text-muted)' }}>
              💻 代码
            </Button>
          </div>
        </div>
      </Card>
    </div>
  )
}

// 模拟消息
function getMockMessages(): ChatMessage[] {
  return [
    {
      id: '1',
      sender_id: 'system',
      sender_name: '系统',
      sender_role: 'System',
      content: { type: 'SystemNotification', notification_type: 'DiscussionStarted', content: '📌 新知识讨论开始: HTML 文档结构' },
      sent_at: Math.floor(Date.now() / 1000) - 3600,
    },
    {
      id: '2',
      sender_id: 'gu-001',
      sender_name: '黑塔',
      sender_role: 'BlackTower',
      content: { type: 'Text', text: '我提议 HTML 文档结构知识的定义应该是：\n\nHTML 文档由 DOCTYPE 声明、html 根元素、head 头部和 body 主体四部分组成。\n\n这是基础架构，需要共识。' },
      sent_at: Math.floor(Date.now() / 1000) - 3500,
    },
    {
      id: '3',
      sender_id: 'gu-002',
      sender_name: '螺丝咕姆',
      sender_role: 'Screwllum',
      content: { type: 'Text', text: '需要补充安全性说明：\n\nDOCTYPE 声明确保浏览器使用标准模式渲染，防止怪异模式攻击。' },
      sent_at: Math.floor(Date.now() / 1000) - 3400,
    },
    {
      id: '4',
      sender_id: 'gu-003',
      sender_name: '拉蒂奥',
      sender_role: 'Latio',
      content: { type: 'Formula', formula: '完整度 = (DOCTYPE?1:0 + html?1:0 + head?1:0 + body?1:0) / 4', description: '结构完整度公式' },
      sent_at: Math.floor(Date.now() / 1000) - 3300,
    },
    {
      id: '5',
      sender_id: 'gu-001',
      sender_name: '黑塔',
      sender_role: 'BlackTower',
      content: { type: 'Proposal', topic: 'HTML 文档结构', part: 'Definition', content: 'HTML 文档由 DOCTYPE 声明、html 根元素、head 头部和 body 主体四部分组成，DOCTYPE 确保标准模式渲染。', proposal_id: 'prop-001' },
      sent_at: Math.floor(Date.now() / 1000) - 3200,
    },
    {
      id: '6',
      sender_id: 'gu-002',
      sender_name: '螺丝咕姆',
      sender_role: 'Screwllum',
      content: { type: 'Vote', proposal_id: 'prop-001', support: true, reason: '同意该提议，安全性已补充' },
      sent_at: Math.floor(Date.now() / 1000) - 3100,
    },
    {
      id: '7',
      sender_id: 'gu-003',
      sender_name: '拉蒂奥',
      sender_role: 'Latio',
      content: { type: 'Vote', proposal_id: 'prop-001', support: true, reason: '公式化验证优雅' },
      sent_at: Math.floor(Date.now() / 1000) - 3000,
    },
    {
      id: '8',
      sender_id: 'system',
      sender_name: '系统',
      sender_role: 'System',
      content: { type: 'ConsensusReached', topic: 'HTML 文档结构', participants: 3 },
      sent_at: Math.floor(Date.now() / 1000) - 2900,
    },
    {
      id: '9',
      sender_id: 'system',
      sender_name: '系统',
      sender_role: 'System',
      content: { type: 'SystemNotification', notification_type: 'KnowledgeStored', content: '✅ 知识「HTML 文档结构」已入库' },
      sent_at: Math.floor(Date.now() / 1000) - 2800,
    },
  ]
}
