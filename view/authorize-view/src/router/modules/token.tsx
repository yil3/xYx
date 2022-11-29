import lazyLoad from "@/utils/RouteUtils";
import { lazy } from "react";
import { RouteStruct } from "../interface";

const route: RouteStruct[] = [{
  element: lazy(() => import("@/layouts")),
  children: [
    { path: "/token", element: lazy(() => import("@/components/Token")), meta: { title: "Token", key: "token" } },
  ],
}]

export default lazyLoad(route);
