/**
* @Author xYx
* @Date 2022-11-27 17:33:47
*/
import { lazy } from "react";
import { RouteStruct } from "../interface";

const route: RouteStruct[] = [{
  element: lazy(() => import("@/layouts")),
  children: [
    { path: "/", element: lazy(() => import("@/views/home")), meta: { title: "Home", key:"home" } },
  ],
}]

export default route;
