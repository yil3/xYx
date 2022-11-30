import { lazy } from "react";
import { RouteStruct } from "../interface";

/**
* @Author xYx
* @Date 2022-11-29 17:09:19
*/
const route: RouteStruct[] = [{
  element: lazy(() => import("@/layouts")),
  children: [{ path: "/user/group", element: lazy(() => import("@/components/UserGroup")) }]
}]

export default route;
