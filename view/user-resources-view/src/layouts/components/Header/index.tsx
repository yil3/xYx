import { Layout } from "antd";
import Me from "./Me";
import NavMenu from "./NavMenu";

export default function Header() {
  return (
    <Layout.Header
      style={{ padding: 0, display: "flex", backgroundColor: "#fff" }}
    >
      <div className="logo" style={{ width: "180px" }} />
      <NavMenu />
      <Me />
    </Layout.Header>
  );
};

