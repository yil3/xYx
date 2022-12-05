import { Menu, MenuProps } from "antd";
import { useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";
/**
 * @Author xYx
 * @Date 2022-10-27 15:35:39
 */

interface NavMenuProps {
}

const items = [
  { label: "首页", key: "/" },
  { label: "用户", key: "/user" },
  { label: "用户组", key: "/user/group" },
  { label: "角色组", key: "/role/group" },
  { label: "角色", key: "/role" },
  { label: "权限", key: "/permission" },
];

export default function NavMenu(_props: NavMenuProps) {
  const location = useLocation();
  const [current, setCurrent] = useState(location.pathname);
  const navigate = useNavigate();
  const onClick: MenuProps["onClick"] = e => {
    navigate(e.key);
    setCurrent(e.key);
  };
  return (
    <>
      <Menu style={{ flex: 1 }} onClick={onClick} mode="horizontal" defaultSelectedKeys={[current]} items={items} />
    </>
  )
};

