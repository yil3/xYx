import { LockOutlined, UserOutlined, WechatOutlined } from "@ant-design/icons";
import { Card, Button, Checkbox, Form, Input, Space, message } from "antd";
import { login } from "@/api/modules/login";
import { useNavigate } from "react-router-dom";
import './index.less'


export default function Login(_props: any) {
  const navigate = useNavigate();
  const onFinish = (values: any) => {
    values.grant_type = 'password';
    values.client_id = '1';
    values.client_secret = '2';
    login(values).then(res => {
      if (res.success) {
        localStorage.setItem("token", res.data.access_token);
        localStorage.setItem("jwt_token", JSON.stringify(res.data));
        message.success("login success");
        navigate("/");
      } else {
        message.warn(res.msg);
      }
    });
  };

  const onFinishFailed = (errorInfo: any) => {
    console.log('Failed:', errorInfo);
  };

  return (
    <div style={{ display: "flex", flex: 1, alignItems: "center" }}>
      <div style={{ marginLeft: "auto", marginRight: "25%" }}>
        <Card>
          <div style={{ height: "80px" }}></div>
          <Form
            name="login"
            initialValues={{ remember: true }}
            className="login-form"
            onFinish={onFinish}
            onFinishFailed={onFinishFailed}
            autoComplete="off"
          >
            <Form.Item
              name="username"
              rules={[{ required: true, message: '请输入用户名' }]}
            >
              <Input prefix={<UserOutlined className="site-form-item-icon" />} placeholder="用户名" />
            </Form.Item>

            <Form.Item
              name="password"
              rules={[{ required: true, message: '请输入密码' }]}
            >
              <Input.Password
                prefix={<LockOutlined className="site-form-item-icon" />}
                type="password"
                placeholder="密码"
              />
            </Form.Item>

            <Form.Item name="remember" valuePropName="checked">
              <div>
                <Checkbox>下次自动登录</Checkbox>
                <a href="" style={{ float: "right" }}>忘记密码?</a>
              </div>
            </Form.Item>

            <Form.Item>
              <div style={{ display: "flex", flexDirection: "column" }}>
                <Button type="primary" block htmlType="submit">
                  <span>登录</span>
                </Button>
                <div style={{ marginLeft: "auto", marginTop: "5px" }}><span>没有帐号？</span>
                  <a onClick={() => navigate("/signup")}>立即注册</a>
                </div>
                <div style={{ display: "flex", flexFlow: "wrap" }}>
                  <Space>
                    <WechatOutlined style={{ fontSize: '20px' }}/>
                  </Space>
                </div>
              </div>
            </Form.Item>

          </Form>
        </Card>
      </div>
    </div>
  )
}

