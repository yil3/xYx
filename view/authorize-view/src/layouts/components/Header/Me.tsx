import { Avatar, Dropdown, Menu, Space } from "antd";
import { DownOutlined, UserOutlined } from "@ant-design/icons";
import { useNavigate } from "react-router-dom";
//
const Me = (_props: any) => {
  const navigate = useNavigate();

  const menu = (
    <Menu
      onClick={e => {
        let { key } = e;
        if (key === "/logout") {
          localStorage.removeItem("token");
          localStorage.removeItem("jwt_token");
          navigate("/login");
        } else {
          navigate(key);
        }
      }}
      items={[
        {
          key: "/account",
          label: "我的信息",
        },
        {
          key: "/logout",
          label: "退出",
        },
      ]}
    />
  );
  return (
    <Dropdown overlay={menu}>
      <Space style={{ margin: "0 40px 0 20px" }}>
        <Avatar icon={<UserOutlined />} />
        about me
        <DownOutlined />
      </Space>
    </Dropdown>
  );
};

export default Me;
