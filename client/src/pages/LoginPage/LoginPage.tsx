import { Form, Input, Button} from 'antd';
import { UserOutlined, LockOutlined } from '@ant-design/icons';
import './login.scss'
import { loginService } from '../../services/userService';
import { Account } from '../../Model/userModel';
import { toast } from 'react-toastify'
import { toastMSGObject } from '../../utils/utils';
import { useNavigate } from 'react-router-dom';

export default function LoginPage() {
  const navigate = useNavigate();

  const handleLoginForm = (values : Account) => {
    loginService(values)
      .then(res => {
        if(res['code_status'] === 200){
          const user = {
            userId : res['user_id'],
            fullName : res['full_name'],
            role : res['role'],
            token : res['token'],
          }
          localStorage.setItem('user', JSON.stringify(user))
          toast.success(res.message , toastMSGObject())
          navigate('/')
        }
      })
      .catch(error => toast.error('Username or Password is wrong' , toastMSGObject()))
  }

  return (
    <>
      <div id="background"></div> 
      <div id="LoginPage">
        <div className='form-login'>
          <div className="title">Login</div>
          <Form
            name="normal_login"
            style={{
              width : 300
            }}
            initialValues={{
              remember: true,
            }}
            onFinish={handleLoginForm}
          >
            <Form.Item
              name="username"
              rules={[
                {
                  required: true,
                  message: 'Vui lòng nhập tài khoản!',
                },
              ]}
            >
              <Input prefix={<UserOutlined className="site-form-item-icon" />} placeholder="Username" />
            </Form.Item>

            <Form.Item
              name="password"
              rules={[
                {
                  required: true,
                  message: 'Vui lòng nhập mật khẩu!',
                },
              ]}
            >
              <Input
                prefix={<LockOutlined className="site-form-item-icon" />}
                type="password"
                placeholder="Password"
              />
            </Form.Item>

            <Form.Item>
              <Button type="primary" htmlType="submit" className="login-form-button">
                Đăng nhập
              </Button>
            </Form.Item>
          </Form>
        </div>        
      </div>  
    </>
  )
}
