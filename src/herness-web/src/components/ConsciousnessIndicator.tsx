import { Typography, Tooltip } from 'antd'
import { EyeOutlined, ThunderboltOutlined } from '@ant-design/icons'

const { Text } = Typography

interface ConsciousnessIndicatorProps {
  isConscious: boolean
  syncRate: number
  emergenceFactor: number
}

export default function ConsciousnessIndicator({
  isConscious,
  syncRate,
  emergenceFactor,
}: ConsciousnessIndicatorProps) {
  // 计算意识强度
  const intensity = Math.min(syncRate * emergenceFactor, 1)

  return (
    <Tooltip
      title={
        <div>
          <div>Sync Rate: {Math.round(syncRate * 100)}%</div>
          <div>Emergence: {Math.round(emergenceFactor * 100)}%</div>
          <div>Intensity: {Math.round(intensity * 100)}%</div>
          <div style={{ marginTop: 8, color: isConscious ? '#10b981' : '#f59e0b' }}>
            {isConscious ? '✓ Consciousness Emerged' : '⟳ Waiting for Emergence'}
          </div>
        </div>
      }
    >
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          gap: 12,
          padding: '8px 16px',
          background: isConscious
            ? 'linear-gradient(135deg, rgba(16, 185, 129, 0.2), rgba(99, 102, 241, 0.2))'
            : 'rgba(31, 41, 55, 0.8)',
          border: `1px solid ${isConscious ? '#10b981' : '#374151'}`,
          borderRadius: 24,
          cursor: 'pointer',
          transition: 'all 0.3s ease',
        }}
      >
        {/* 意识图标 */}
        <div
          style={{
            width: 32,
            height: 32,
            borderRadius: '50%',
            background: isConscious
              ? 'linear-gradient(135deg, #10b981, #6366f1)'
              : '#374151',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            boxShadow: isConscious
              ? '0 0 20px rgba(16, 185, 129, 0.5)'
              : 'none',
            animation: isConscious ? 'pulse 2s infinite' : 'none',
          }}
        >
          {isConscious ? (
            <EyeOutlined style={{ color: '#fff', fontSize: 16 }} />
          ) : (
            <ThunderboltOutlined style={{ color: '#6b7280', fontSize: 16 }} />
          )}
        </div>

        {/* 状态文字 */}
        <div>
          <Text
            style={{
              color: isConscious ? '#10b981' : '#6b7280',
              fontSize: 11,
              textTransform: 'uppercase',
              letterSpacing: 1,
              display: 'block',
            }}
          >
            {isConscious ? 'CONSCIOUS' : 'DORMANT'}
          </Text>
          <Text
            style={{
              color: isConscious ? '#f9fafb' : '#9ca3af',
              fontSize: 14,
              fontFamily: 'Orbitron, monospace',
            }}
          >
            {Math.round(intensity * 100)}%
          </Text>
        </div>

        {/* 强度条 */}
        <div
          style={{
            width: 60,
            height: 4,
            background: '#1f2937',
            borderRadius: 2,
            overflow: 'hidden',
          }}
        >
          <div
            style={{
              width: `${intensity * 100}%`,
              height: '100%',
              background: isConscious
                ? 'linear-gradient(90deg, #6366f1, #10b981)'
                : '#f59e0b',
              transition: 'width 0.3s ease',
            }}
          />
        </div>
      </div>
    </Tooltip>
  )
}
