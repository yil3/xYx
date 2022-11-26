import { Menu, MenuProps } from "antd";
import { useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";
/**
 * @Author xYx
 * @Date 2022-10-27 15:35:39
 */

interface NavMenuProps { }

const items = [
  { label: "首页", key: "/" },
  { label: "客户端", key: "/client" },
  { label: "令牌", key: "/token" },
];

export default function NavMenu(_props: NavMenuProps) {
  const location = useLocation();
  const [current, setCurrent] = useState(location.pathname);
  const navigate = useNavigate();
  const onClick: MenuProps["onClick"] = e => {
    navigate(e.key);
    setCurrent(e.key);
  };
  return <Menu style={{ flex: 1 }} onClick={onClick} mode="horizontal" defaultSelectedKeys={[current]} items={items} />;
};

