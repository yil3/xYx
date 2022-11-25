import { Button, Layout, Result } from "antd";
import { useNavigate } from "react-router-dom";
import Footer from "@/layouts/components/Footer";
import "./index.less";

const NotFound = () => {
  const navigate = useNavigate();
  const goHome = () => {
    navigate("/");
  };
  return (
    <Layout style={{ minHeight: "100vh" }}>
      <Result style={{ flex: 1 }}
        status="404"
        title="404"
        subTitle="Sorry, the page you visited does not exist."
        extra={
          <Button type="primary" onClick={goHome}>
            Back Home
          </Button>
        }
      />
      <Footer />
    </Layout>
  );
};

export default NotFound;
