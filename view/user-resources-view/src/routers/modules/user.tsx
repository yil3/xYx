/**
* @Author xYx
* @Date 2022-11-26 17:32:16
*/

import LayoutView from "@/layouts";
import User from "@/views/user";
import { RouteStruct } from "../interface";

const route: RouteStruct[] = [{
  element: <LayoutView />,
  children: [{ path: "/user", element: <User />, meta: { title: "用户管理", key: "user" } }]
}]

export default route;
