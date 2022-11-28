/**
* @Author xYx
* @Date 2022-11-26 17:32:16
*/

import { lazy } from "react";
import { RouteStruct } from "../interface";

const route: RouteStruct[] = [{
  element: lazy(() => import("@/layouts")),
  children: [{ path: "/user", element: lazy(() => import("@/components/User")), meta: { title: "用户管理", key: "user" } }]
}]

export default route;
