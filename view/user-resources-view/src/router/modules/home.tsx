/**
* @Author xYx
* @Date 2022-11-27 17:33:47
*/
import lazyLoad from "@/utils/RouteUtils";
import { lazy } from "react";
import { RouteStruct } from "../interface";

const route: RouteStruct[] = [{
  element: lazy(() => import("@/layouts")),
  children: [
    { path: "/", element: lazy(() => import("@/components/Home")), meta: { title: "Home", key:"home" } },
  ],
}]

export default lazyLoad(route);
