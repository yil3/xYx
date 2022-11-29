import { RouteStruct } from "@/router/interface";
import lazyLoad from "@/utils/RouteUtils";
import { lazy } from "react";

// 错误页面模块
const errorRouter: Array<RouteStruct> = [
  {
    path: "/404",
    element: lazy(() => import("@/components/Error/404")),
    meta: {
      title: "404页面",
      key: "404",
      notRequiresAuth: true
    }
  },
];

export default lazyLoad(errorRouter);
