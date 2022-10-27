import LayoutView from "@/layouts";
import Home from "@/views/home";
import Token from "@/views/token";
import { RouteObject } from "../interface";

const route: RouteObject[] = [{
  path: "/",
  element: <LayoutView />,
  children: [
    { path: "/", element: <Home />, meta: { title: "Home" } },
    { path: "/token", element: <Token />, meta: { title: "Token" } },
  ],
}]

export default route;
