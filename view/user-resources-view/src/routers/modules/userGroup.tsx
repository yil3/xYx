import { lazy } from "react";
import { RouteStruct } from "../interface";

const route: RouteStruct[] = [{
  element: lazy(() => import("@/layouts")),
  children: [{ path: "/user/group", element: lazy(() => import("@/views/UserGroup")) }]
}]

export default route;
