import { UserOutlined , HomeOutlined } from "@ant-design/icons";
import { Menu } from "antd";
import {useNavigate , useLocation} from 'react-router-dom'

export default function NavbarLeft() {
    const navigate = useNavigate();
    const location = useLocation();

    const handleConvertPage = ({key} : {key : string}) => {
        navigate(`/${key}`)
    }

    return (
        <div>
            <div>Menu</div>
            <Menu
                theme="dark"
                mode="inline"
                defaultSelectedKeys={[location.pathname.substring(1)]}
                onClick={handleConvertPage}
                items={[
                    {
                        key: '',
                        icon: <HomeOutlined />,
                        label: 'Trang chủ',
                    },
                    {
                        key: 'profile',
                        icon: <UserOutlined />,
                        label: 'Hồ sơ của bạn',
                    },
                    {
                        key: 'students',
                        icon: <UserOutlined />,
                        label: 'Sinh viên',
                    },
                ]}
            />
        </div>
    )
}
