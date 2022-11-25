import { Layout, Card } from "antd";
import { Outlet } from "react-router-dom";
import Header from "./components/Header";
import Footer from "./components/Footer";
import "./index.less";

const LayoutView = () => {
  return (
    <div id="parent-area">
      <Layout>
        <Header />
        <Card>
          <Layout.Content>
            <Outlet></Outlet>
          </Layout.Content>
        </Card>
      </Layout>
      <Footer />
    </div>
  );
};

export const LayoutBasic = () => {
  return (
    <div id="parent-area">
      <Layout.Content style={{ flex: 1,display: "flex" }}>
        <Outlet></Outlet>
      </Layout.Content>
      <Footer />
    </div>
  )
};

export default LayoutView;
