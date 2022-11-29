/**
* @Author xYx
* @Date 2022-11-04 16:33:51
*/
import lazyLoad from "@/utils/RouteUtils";
import { lazy } from "react";
import { RouteStruct } from "../interface";

const route: RouteStruct[] = [{
  element: lazy(() => import("@/layouts")),
  children: [
    { path: "/client", element: lazy(() => import("@/components/Client")), meta: { title: "Client", key:"client" } },
  ],
}]

export default lazyLoad(route);
