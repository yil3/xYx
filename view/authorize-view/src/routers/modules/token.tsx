import LayoutView from "@/layouts";
import Token from "@/views/Token";
import { RouteStruct } from "../interface";

const route: RouteStruct[] = [{
  element: <LayoutView />,
  children: [
    { path: "/token", element: <Token />, meta: { title: "Token", key: "token" } },
  ],
}]

export default route;
