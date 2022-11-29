import { LockOutlined, MailOutlined, UserAddOutlined, UserOutlined } from "@ant-design/icons";
import { Card, Button, Form, Input, Space, message } from "antd";
import { useNavigate } from "react-router-dom";
import './index.less'
import { register } from "@/api/modules/login";

export default function SignUp(_props: any) {
  const navigate = useNavigate();
  const onFinish = async (values: any) => {
    values.account = values.username;
    let res = await register(values);
    if (res.success) {
      message.success("register success");
      navigate("/login");
    } else {
      message.warning(res.message);
    }
  };

  const onFinishFailed = (errorInfo: any) => {
    console.log('Failed:', errorInfo);
  };
  return (
    <div style={{ display: "flex", flex: 1, alignItems: "center" }}>
      <div style={{ marginLeft: "auto", marginRight: "25%" }}>
        <Card>
          <div style={{}}></div>
          <Form
            name="register"
            initialValues={{ remember: true }}
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
            <Form.Item
              name="confirm"
              dependencies={['password']}
              hasFeedback
              rules={[
                {
                  required: true,
                  message: '请确认密码',
                },
                ({ getFieldValue }) => ({
                  validator(_, value) {
                    if (!value || getFieldValue('password') === value) {
                      return Promise.resolve();
                    }
                    return Promise.reject(new Error('输入的两个密码不匹配!'));
                  },
                }),
              ]}
            >
              <Input.Password
                prefix={<LockOutlined className="site-form-item-icon" />}
                type="password"
                placeholder="再次输入密码"

              />
            </Form.Item>

            <Form.Item name="nickname" rules={[{ required: true, message: '请输入昵称' }]}>
              <Input prefix={<UserAddOutlined />} placeholder="昵称" />
            </Form.Item>

            <Form.Item name="email" rules={[{ type: "email", required: true, message: '请输入正确的邮箱地址' }]}>
              <Input prefix={<MailOutlined />} placeholder="邮箱" />
            </Form.Item>

            <Form.Item>
              <div style={{ display: "flex", flexDirection: "column" }}>
                <Space direction="vertical">
                  <Button type="primary" block htmlType="submit">
                    <span>注册</span>
                  </Button>
                  <Button type="primary" block htmlType="submit" onClick={() => navigate("/login")}>
                    <span>返回</span>
                  </Button>
                  <div style={{ height: "30px" }}></div>
                </Space>
              </div>
            </Form.Item>

          </Form>
        </Card>
      </div>
    </div>
  );
}

