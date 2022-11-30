import { lazy } from "react";
import { RouteStruct } from "../interface";

const route: RouteStruct[] = [{
  element: lazy(() => import("@/layouts")),
  children: [
    { path: "/", element: lazy(() => import("@/components/Home")), meta: { title: "Home", key:"" } },
  ],
}]

export default route;
