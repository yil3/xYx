/**
* @Author xYx
* @Date 2022-11-28 09:37:34
*/

import lazyLoad from "@/utils/RouteUtils";
import { lazy } from "react";
import { RouteStruct } from "../interface";

const routes: RouteStruct[] = [{
  element: lazy(() => import("@/layouts")),
  children: [
    { path: "/role", element: lazy(() => import("@/components/Role")) }
  ]
}]

export default lazyLoad(routes);
