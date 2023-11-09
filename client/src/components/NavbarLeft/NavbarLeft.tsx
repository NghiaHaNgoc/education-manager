import { UserOutlined , HomeOutlined, UnorderedListOutlined, LogoutOutlined } from "@ant-design/icons";
import { Menu } from "antd";
import {useNavigate , useLocation} from 'react-router-dom'
import { MenuItem, getItem } from "../../utils/utils";

export default function NavbarLeft() {
    const navigate = useNavigate();
    const location = useLocation();
    const role = JSON.parse(localStorage.getItem('user') as string)?.role

    const handleConvertPage = ({key} : {key : string}) => {
        if(key === 'logout'){
            localStorage.removeItem('user');
            navigate('/login');
            return;
        }
        navigate(`/${key}`)
    }

    const items: MenuItem[] = [
        getItem('Main page', '', <HomeOutlined />),
        getItem('Your profile', 'profile', <UserOutlined />),      
        getItem('List', 'list', <UnorderedListOutlined />, [
          getItem('Students', 'list/students'),
          getItem('Lecturers', 'list/lecturers'),
          getItem('Classes', 'list/classes'),
        ]),
        getItem('Logout', 'logout', <LogoutOutlined />),      
    ]

    const customeItemByAuthorization = () => {
        console.log(items)
        switch (role) {
            case 'Admin':
                return items
            case 'Student':
            case 'Lecturer':
                return items.filter(item => item?.key !== 'list')
            default:
                break;
        }
    }

    return (
        <div>
            <div>Menu</div>
            <Menu
                theme="dark"
                mode="inline"
                defaultSelectedKeys={[location.pathname.substring(1)]}
                onClick={handleConvertPage}
                items={customeItemByAuthorization()}
            />
        </div>
    )
}
