import { Typography, Menu, Tag } from 'antd'
import {
  DashboardOutlined,
  DollarOutlined,
  TeamOutlined,
  SettingOutlined,
  TrophyOutlined,
} from '@ant-design/icons'
import { useLocation, useNavigate } from 'react-router-dom'

const { Text } = Typography

export default function NavMenu() {
  const location = useLocation()
  const navigate = useNavigate()

  const menuItems = [
    {
      key: '/',
      icon: <DashboardOutlined />,
      label: '监控面板',
    },
    {
      key: '/currency',
      icon: <DollarOutlined />,
      label: (
        <span>
          金币流水
          <Tag
            color="gold"
            style={{ marginLeft: 8, fontSize: 10 }}
          >
            NEW
          </Tag>
        </span>
      ),
    },
    {
      key: '/tasks',
      icon: <TrophyOutlined />,
      label: (
        <span>
          任务管理
          <Tag
            color="cyan"
            style={{ marginLeft: 8, fontSize: 10 }}
          >
            NEW
          </Tag>
        </span>
      ),
    },
    {
      key: '/gus',
      icon: <TeamOutlined />,
      label: '蛊虫管理',
    },
    {
      key: '/settings',
      icon: <SettingOutlined />,
      label: '系统设置',
    },
  ]

  return (
    <Menu
      theme="dark"
      mode="horizontal"
      selectedKeys={[location.pathname]}
      items={menuItems}
      onClick={({ key }) => navigate(key)}
      style={{
        flex: 1,
        minWidth: 0,
        background: 'transparent',
        borderBottom: 'none',
      }}
    />
  )
}
