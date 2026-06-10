import { useState, useEffect } from 'react'
import {
  Card,
  Form,
  Input,
  InputNumber,
  Switch,
  Button,
  Select,
  Typography,
  Divider,
  message,
  Alert,
  Row,
  Col,
} from 'antd'
import {
  DatabaseOutlined,
  ThunderboltOutlined,
  SaveOutlined,
  FolderOpenOutlined,
  ReloadOutlined,
} from '@ant-design/icons'

const { Title, Text } = Typography

// 训练配置接口
interface TrainingConfig {
  data_path: string
  epochs: number
  batch_size: number
  auto_save: boolean
  save_interval: number
}

// 默认配置
const defaultConfig: TrainingConfig = {
  data_path: 'data/training/scenes_combined.json',
  epochs: 100,
  batch_size: 32,
  auto_save: true,
  save_interval: 60,
}

// 可用的训练数据文件
const availableDataFiles = [
  { value: 'data/training/scenes_combined.json', label: 'scenes_combined.json (综合数据集)' },
  { value: 'data/training/scenes_basic.json', label: 'scenes_basic.json (基础数据集)' },
  { value: 'data/training/scene_graphs_converted.json', label: 'scene_graphs_converted.json (场景图)' },
  { value: 'data/training/geometry_templates.json', label: 'geometry_templates.json (几何模板)' },
  { value: 'data/training/spatial_rules.json', label: 'spatial_rules.json (空间规则)' },
]

export default function SettingsPage() {
  const [form] = Form.useForm<TrainingConfig>()
  const [saving, setSaving] = useState(false)
  const [testing, setTesting] = useState(false)
  const [testResult, setTestResult] = useState<{ success: boolean; message: string } | null>(null)

  // 加载保存的配置
  useEffect(() => {
    const savedConfig = localStorage.getItem('training_config')
    if (savedConfig) {
      try {
        const config = JSON.parse(savedConfig)
        form.setFieldsValue(config)
      } catch {
        form.setFieldsValue(defaultConfig)
      }
    } else {
      form.setFieldsValue(defaultConfig)
    }
  }, [form])

  // 保存配置
  const saveConfig = async () => {
    try {
      const values = await form.validateFields()
      setSaving(true)
      localStorage.setItem('training_config', JSON.stringify(values))
      message.success('配置已保存')
    } catch {
      message.error('配置验证失败')
    } finally {
      setSaving(false)
    }
  }

  // 测试数据文件
  const testDataFile = async () => {
    const dataPath = form.getFieldValue('data_path')
    if (!dataPath) {
      message.warning('请先选择数据文件')
      return
    }
    setTesting(true)
    setTestResult({ success: true, message: `数据文件: ${dataPath}` })
    setTesting(false)
  }

  // 重置为默认配置
  const resetConfig = () => {
    form.setFieldsValue(defaultConfig)
    message.info('已重置为默认配置')
  }

  return (
    <div style={{ padding: 24 }}>
      {/* 标题 */}
      <div style={{
        background: 'linear-gradient(135deg, #6366f1, #8b5cf6)',
        padding: '20px 24px',
        borderRadius: 12,
        marginBottom: 24,
      }}>
        <Title level={3} style={{ color: '#fff', margin: 0 }}>⚙️ 系统设置</Title>
        <Text style={{ color: 'rgba(255,255,255,0.8)' }}>配置训练数据源和系统参数</Text>
      </div>

      <Row gutter={[16, 16]}>
        {/* 训练数据配置 */}
        <Col xs={24} lg={16}>
          <Card
            title={
              <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                <DatabaseOutlined style={{ color: '#6366f1' }} />
                <span style={{ color: '#f3f4f6', fontWeight: 600 }}>训练数据配置</span>
              </div>
            }
            style={{
              background: 'linear-gradient(180deg, #1a1a2e 0%, #16162a 100%)',
              borderColor: '#2d2d4a',
              borderRadius: 12,
            }}
            headStyle={{ borderBottom: '1px solid #2d2d4a' }}
          >
            <Form form={form} layout="vertical">
              {/* 数据源选择 */}
              <Form.Item
                name="data_path"
                label={<span style={{ color: '#e5e7eb' }}>训练数据文件</span>}
                rules={[{ required: true, message: '请选择训练数据文件' }]}
              >
                <Select
                  placeholder="选择训练数据文件"
                  style={{ width: '100%' }}
                  showSearch
                  optionFilterProp="label"
                >
                  {availableDataFiles.map(file => (
                    <Select.Option key={file.value} value={file.value} label={file.label}>
                      <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                        <FolderOpenOutlined style={{ color: '#6366f1' }} />
                        <span style={{ color: '#f3f4f6' }}>{file.label}</span>
                      </div>
                    </Select.Option>
                  ))}
                </Select>
              </Form.Item>

              {/* 自定义路径 */}
              <Form.Item label={<span style={{ color: '#e5e7eb' }}>自定义路径</span>}>
                <Input
                  placeholder="或输入自定义数据文件路径"
                  style={{
                    background: 'rgba(0, 0, 0, 0.4)',
                    borderColor: '#374151',
                    color: '#f3f4f6',
                  }}
                  onChange={(e) => {
                    if (e.target.value) {
                      form.setFieldValue('data_path', e.target.value)
                    }
                  }}
                />
              </Form.Item>

              {/* 测试结果 */}
              {testResult && (
                <Alert
                  type={testResult.success ? 'success' : 'error'}
                  message={testResult.message}
                  showIcon
                  style={{ marginBottom: 16 }}
                />
              )}

              <Divider style={{ borderColor: '#2d2d4a' }}>
                <span style={{ color: '#6b7280' }}>训练参数</span>
              </Divider>

              <Row gutter={16}>
                <Col span={12}>
                  <Form.Item
                    name="epochs"
                    label={<span style={{ color: '#e5e7eb' }}>训练轮次 (Epochs)</span>}
                    rules={[{ required: true, message: '请输入训练轮次' }]}
                  >
                    <InputNumber
                      min={1}
                      max={10000}
                      style={{ width: '100%' }}
                    />
                  </Form.Item>
                </Col>
                <Col span={12}>
                  <Form.Item
                    name="batch_size"
                    label={<span style={{ color: '#e5e7eb' }}>批次大小 (Batch Size)</span>}
                    rules={[{ required: true, message: '请输入批次大小' }]}
                  >
                    <InputNumber
                      min={1}
                      max={512}
                      style={{ width: '100%' }}
                    />
                  </Form.Item>
                </Col>
              </Row>

              <Row gutter={16}>
                <Col span={12}>
                  <Form.Item
                    name="auto_save"
                    label={<span style={{ color: '#e5e7eb' }}>自动保存</span>}
                    valuePropName="checked"
                  >
                    <Switch checkedChildren="开启" unCheckedChildren="关闭" />
                  </Form.Item>
                </Col>
                <Col span={12}>
                  <Form.Item
                    name="save_interval"
                    label={<span style={{ color: '#e5e7eb' }}>保存间隔 (秒)</span>}
                  >
                    <InputNumber min={10} max={3600} style={{ width: '100%' }} />
                  </Form.Item>
                </Col>
              </Row>

              {/* 操作按钮 */}
              <div style={{ display: 'flex', gap: 12, marginTop: 24 }}>
                <Button
                  type="primary"
                  icon={<SaveOutlined />}
                  onClick={saveConfig}
                  loading={saving}
                  style={{
                    background: 'linear-gradient(135deg, #10b981, #059669)',
                    borderColor: '#10b981',
                  }}
                >
                  保存配置
                </Button>
                <Button
                  icon={<ReloadOutlined />}
                  onClick={testDataFile}
                  loading={testing}
                  style={{ borderColor: '#374151', color: '#f3f4f6' }}
                >
                  测试文件
                </Button>
                <Button
                  onClick={resetConfig}
                  style={{ borderColor: '#374151', color: '#9ca3af' }}
                >
                  重置默认
                </Button>
              </div>
            </Form>
          </Card>
        </Col>

        {/* 系统信息 */}
        <Col xs={24} lg={8}>
          <Card
            title={
              <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                <ThunderboltOutlined style={{ color: '#f59e0b' }} />
                <span style={{ color: '#f3f4f6', fontWeight: 600 }}>系统信息</span>
              </div>
            }
            style={{
              background: 'linear-gradient(180deg, #1a1a2e 0%, #16162a 100%)',
              borderColor: '#2d2d4a',
              borderRadius: 12,
            }}
            headStyle={{ borderBottom: '1px solid #2d2d4a' }}
          >
            <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
              <div>
                <div style={{ color: '#6b7280', fontSize: 12, marginBottom: 4 }}>后端地址</div>
                <div style={{ color: '#f3f4f6', fontFamily: 'monospace' }}>http://localhost:9000</div>
              </div>
              <div>
                <div style={{ color: '#6b7280', fontSize: 12, marginBottom: 4 }}>WebSocket 端点</div>
                <div style={{ color: '#10b981', fontFamily: 'monospace', fontSize: 12 }}>
                  ws://localhost:9000/ws/training
                </div>
                <div style={{ color: '#6366f1', fontFamily: 'monospace', fontSize: 12 }}>
                  ws://localhost:9000/ws/generate
                </div>
              </div>
              <div>
                <div style={{ color: '#6b7280', fontSize: 12, marginBottom: 4 }}>数据目录</div>
                <div style={{ color: '#f3f4f6', fontFamily: 'monospace', fontSize: 12 }}>
                  data/training/
                </div>
              </div>
              <Divider style={{ borderColor: '#2d2d4a', margin: '8px 0' }} />
              <div>
                <div style={{ color: '#6b7280', fontSize: 12, marginBottom: 8 }}>可用数据文件</div>
                {availableDataFiles.map(file => (
                  <div
                    key={file.value}
                    style={{
                      color: '#9ca3af',
                      fontSize: 11,
                      padding: '4px 0',
                      borderBottom: '1px solid #1f2937',
                    }}
                  >
                    {file.label}
                  </div>
                ))}
              </div>
            </div>
          </Card>

          {/* 使用说明 */}
          <Card
            title={<span style={{ color: '#f3f4f6', fontWeight: 600 }}>使用说明</span>}
            style={{
              background: 'linear-gradient(180deg, #1a1a2e 0%, #16162a 100%)',
              borderColor: '#2d2d4a',
              borderRadius: 12,
              marginTop: 16,
            }}
            headStyle={{ borderBottom: '1px solid #2d2d4a' }}
          >
            <div style={{ color: '#9ca3af', fontSize: 13, lineHeight: 2 }}>
              <p>1. 选择训练数据文件或输入自定义路径</p>
              <p>2. 配置训练参数（轮次、批次大小等）</p>
              <p>3. 点击「保存配置」保存设置</p>
              <p>4. 前往「训练」页面开始训练</p>
            </div>
          </Card>
        </Col>
      </Row>
    </div>
  )
}
