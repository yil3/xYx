import { Avatar, Dropdown, Space } from "antd";
import { DownOutlined, UserOutlined } from "@ant-design/icons";
import { useNavigate } from "react-router-dom";

interface MeProps { }

export default function Me(_props: MeProps) {
  const navigate = useNavigate();
  const items = [
    {
      key: "account",
      label: "我的信息",
    },
    {
      key: "logout",
      label: <a onClick={() => {
        localStorage.removeItem("token");
        navigate("/login");
      }}>
        退出
      </a>,
    }
  ];
  return (
    <Dropdown menu={{ items }}>
      <Space style={{ margin: "0 40px 0 20px" }}>
        <Avatar icon={<UserOutlined />} />
        about me
        <DownOutlined />
      </Space>
    </Dropdown >
  );
};

