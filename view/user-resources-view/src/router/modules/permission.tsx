/**
* @Author xYx
* @Date 2022-11-28 09:42:09
*/

import lazyLoad from "@/utils/RouteUtils";
import { lazy } from "react";
import { RouteStruct } from "../interface";

const routes: RouteStruct[] = [{
  element: lazy(() => import("@/layouts")),
  children: [
    { path: "/permission", element: lazy(() => import("@/components/Permission")) }
  ]
}]

export default lazyLoad(routes);
