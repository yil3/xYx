import { RouteStruct } from "@/router/interface";
import { lazy } from "react";

// 错误页面模块
const route: Array<RouteStruct> = [
  {
    path: "/404",
    element: lazy(() => import("@/components/Error/404")),
    meta: {
      title: "404页面",
      key: "404",
      notAuth: true
    }
  },
];

export default route;
