import { Button,Row, Col, Avatar } from 'antd';
import {MenuUnfoldOutlined ,MenuFoldOutlined, UserOutlined} from '@ant-design/icons'
import {useParams} from 'react-router-dom'
import './Header.scss'

interface propsHeader {
    collapsed : boolean,
    setCollapsed : (collapsed : boolean) => void
}

export default function HeaderComponent(props : propsHeader) {
    const {collapsed , setCollapsed} = props
    const {type} = useParams();

    const nameUser = JSON.parse(localStorage.getItem('user') as string)?.fullName

    return (
        <Row>
            <Col span={2} className='header-btn'>
                <Button
                    type="text"
                    icon={collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />}
                    onClick={() => setCollapsed && setCollapsed(!collapsed)}
                    style={{
                        fontSize: '16px',
                        width: 64,
                        height: 64,
                    }}
                />
            </Col>
            <Col span={22} className='header-layout'>
                <div className='header-layout__title'>{`List ${type}`}</div>
                <div className="header-layout__avatar">
                    <Avatar style={{ backgroundColor: '#87d068' }} icon={<UserOutlined />} />
                    <div className="header-layout__avatar--name">{nameUser}</div>
                </div>              
            </Col>
        </Row> 
    )
}
