import { Avatar, Dropdown, Space } from "antd";
import { DownOutlined, UserOutlined } from "@ant-design/icons";

/**
* @Author xYx
* @Date 2022-11-26 22:54:11
*/

interface MeProps { }

export default function Me(_props: MeProps) {
  const items = [
    {
      key: "account",
      label: "我的信息",
    },
    {
      key: "logout",
      label: <a onClick={() => {
        localStorage.removeItem("token");
        window.location.href = window.location.origin;
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

