import { Layout,theme} from 'antd';
import NavbarLeft from '../../components/NavbarLeft/NavbarLeft';
import {Outlet} from 'react-router-dom'
import {useState} from 'react'
import HeaderComponent from '../../components/HeaderComponent/HeaderComponent';


export default function HomePage() {
    const { Header, Sider, Content } = Layout;
    const {
        token: { colorBgContainer },
    } = theme.useToken();
    const [collapsed, setCollapsed] = useState(false);
  
    return (
        <Layout style={{minHeight: '100vh',}}>
            <Sider trigger={null} collapsible collapsed={collapsed}>
                <NavbarLeft/>
            </Sider>
            <Layout>
                <Header style={{ padding: '0 15px', background: colorBgContainer }}>
                    <HeaderComponent collapsed={collapsed} setCollapsed={setCollapsed}/>
                </Header>
                <Content
                    style={{
                        margin: '24px 16px',
                        padding: 24,
                        background: colorBgContainer,
                    }}
                >
                    <Outlet/>
                </Content>
            </Layout>
        </Layout>
    )
}
