import { Layout, Button, theme } from 'antd';
import {useState} from 'react'
import {MenuUnfoldOutlined ,MenuFoldOutlined} from '@ant-design/icons'
import NavbarLeft from '../../components/NavbarLeft/NavbarLeft';
import {Outlet} from 'react-router-dom'
import './HomePage.css'

export default function HomePage() {
    const { Header, Sider, Content } = Layout;
    const [collapsed, setCollapsed] = useState(false);
    const {
        token: { colorBgContainer },
    } = theme.useToken();
  
    return (
        <Layout>
            <Sider trigger={null} collapsible collapsed={collapsed}>
                <NavbarLeft/>
            </Sider>
            <Layout>
                <Header style={{ padding: 0, background: colorBgContainer }}>
                    <Button
                        type="text"
                        icon={collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />}
                        onClick={() => setCollapsed(!collapsed)}
                        style={{
                            fontSize: '16px',
                            width: 64,
                            height: 64,
                        }}
                    />
                </Header>
                <Content
                    style={{
                        margin: '24px 16px',
                        padding: 24,
                        height: '100vh',
                        background: colorBgContainer,
                    }}
                >
                    <Outlet/>
                </Content>
            </Layout>
        </Layout>
    )
}
