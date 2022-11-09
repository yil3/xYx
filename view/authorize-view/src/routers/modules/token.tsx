import LayoutView from "@/layouts";
import Token from "@/views/token";
import { RouteObject } from "../interface";

const route: RouteObject[] = [{
  element: <LayoutView />,
  children: [
    { path: "/token", element: <Token />, meta: { title: "Token", key: "token" } },
  ],
}]

export default route;
