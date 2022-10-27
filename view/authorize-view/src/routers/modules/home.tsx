import LayoutView from "@/layouts";
import Home from "@/views/home";
import Token from "@/views/token";

export default {
  path: "/",
  element: <LayoutView />,
  children: [
    { path: "/", element: <Home />, meta: { title: "Home", name: "home" } },
    { path: "/token", element: <Token />, meta: { title: "Token", name: "token" } },
  ],
};
