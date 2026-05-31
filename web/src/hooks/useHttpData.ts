import { useState, useEffect } from 'react'
import axios from 'axios'

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

interface WorldState {
  population: number
  health: number
  stability: number
  consciousness_emerged: boolean
}

interface Stats {
  world: {
    total_gus: number
    active_gus: number
    total_access_points: number
    avg_health: number
  }
  network: {
    sync_rate: number
    resonance_strength: number
    mean_frequency: number
  }
  safety: {
    safety_score: number
    trust_entropy: number
    degradation_phase: string
  }
}

interface HttpData {
  worldState: WorldState | null
  guList: GuInfo[]
  stats: Stats | null
  loading: boolean
  error: string | null
}

export function useHttpData(): HttpData {
  const [data, setData] = useState<HttpData>({
    worldState: null,
    guList: [],
    stats: null,
    loading: true,
    error: null,
  })

  const fetchData = async () => {
    console.log('[useHttpData] Fetching data from backend APIs...')

    try {
      const controller = new AbortController()
      const timeoutId = setTimeout(() => {
        controller.abort()
        console.error('[useHttpData] ❌ Request timeout after 5 seconds')
      }, 5000)

      try {
        const [worldRes, gusRes, statsRes] = await Promise.all([
          axios.get('/api/world', { signal: controller.signal }),
          axios.get('/api/gus', { signal: controller.signal }),
          axios.get('/api/stats', { signal: controller.signal }),
        ])

        clearTimeout(timeoutId)

        console.log('[useHttpData] ✅ API Response received:')
        console.log('  - /api/world:', worldRes.data)
        console.log('  - /api/gus:', gusRes.data)
        console.log('  - /api/stats:', statsRes.data)

        // 后端返回的是 { gus: [...], total: N } 格式
        const guList = gusRes.data.gus || gusRes.data || []

        setData({
          worldState: worldRes.data,
          guList: Array.isArray(guList) ? guList : [],
          stats: statsRes.data,
          loading: false,
          error: null,
        })
      } catch (apiError) {
        clearTimeout(timeoutId)

        // 构造详细的错误信息
        let errorMsg = 'Failed to fetch data from backend'
        if (axios.isCancel(apiError)) {
          errorMsg = 'Request timeout: Backend did not respond within 5 seconds'
        } else if (axios.isAxiosError(apiError)) {
          if (apiError.response) {
            errorMsg = `Backend error: ${apiError.response.status} ${apiError.response.statusText}`
          } else if (apiError.request) {
            errorMsg = 'Network error: Cannot connect to backend server'
          } else {
            errorMsg = `Request error: ${apiError.message}`
          }
        }

        console.error('[useHttpData] ❌ API Error:', errorMsg)
        console.error('[useHttpData] Full error:', apiError)

        setData({
          worldState: null,
          guList: [],
          stats: null,
          loading: false,
          error: errorMsg,
        })
      }
    } catch (e) {
      const errorMsg = `Unexpected error: ${e}`
      console.error('[useHttpData] ❌', errorMsg)
      console.error('[useHttpData] Full error:', e)

      setData(prev => ({
        ...prev,
        loading: false,
        error: errorMsg,
      }))
    }
  }

  useEffect(() => {
    fetchData()
    // 每30秒刷新一次低频数据
    const interval = setInterval(fetchData, 30000)
    return () => clearInterval(interval)
  }, [])

  return data
}
