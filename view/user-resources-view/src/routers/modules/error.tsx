import React from "react";
import { RouteStruct } from "@/routers/interface";

// 错误页面模块
const errorRouter: Array<RouteStruct> = [
  {
    path: "/404",
    element: React.lazy(() => import("@/components/Error/404")),
    meta: {
      title: "404页面",
      key: "404",
      notRequiresAuth: true
    }
  },
];

export default errorRouter;
