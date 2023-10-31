import { Form, Input, Button} from 'antd';
import { UserOutlined, LockOutlined } from '@ant-design/icons';
import './login.scss'
import { loginService } from '../../services/userService';
import { Account } from '../../Model/userModel';

export default function LoginPage() {

  const handleLoginForm = async (values : Account) => {
    const res = await loginService(values);
  }

  return (
    <>
      <div id="background"></div> 
      <div id="LoginPage">
        <div className='form-login'>
          <div className="title">Đăng nhập</div>
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
              Or <a href="">register now!</a>
            </Form.Item>
          </Form>
        </div>        
      </div>  
    </>
  )
}
