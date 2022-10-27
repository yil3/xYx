import { Menu, MenuProps } from "antd";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
/**
 * @Author xYx
 * @Date 2022-10-27 15:35:39
 */

const items = [
  { label: "首页", key: "/" },
  { label: "令牌", key: "/token" },
];

const NavMenu = (_props: any) => {
  const [current, setCurrent] = useState("/");
  const navigate = useNavigate();
  const onClick: MenuProps["onClick"] = e => {
    navigate(e.key);
    setCurrent(e.key);
  };
  return <Menu style={{ flex: 1 }} onClick={onClick} mode="horizontal" defaultSelectedKeys={[current]} items={items} />;
};

export default NavMenu;
