import { Avatar, Dropdown, Menu, Space } from "antd";
import { UserOutlined } from "@ant-design/icons";

const menu = (
  <Menu
    onClick={e => console.log(e)}
    items={[
      {
        key: "/account",
        label: "我的信息",
      },
      {
        key: "/signout",
        label: "退出",
      },
    ]}
  />
);


const Me = () => {
  return (
    <div>
      <Dropdown overlay={menu}>
        <Space style={{ margin: "0 40px 0 20px" }}>
          <Avatar icon={<UserOutlined />} />
          about me
        </Space>
      </Dropdown>
    </div>
  );
};

export default Me;
