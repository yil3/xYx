import React from "react";
import lazyLoad from "@/routers/utils/lazyLoad";
import { RouteObject } from "@/routers/interface";

// 错误页面模块
const errorRouter: Array<RouteObject> = [
	{
		path: "/404",
		element: lazyLoad(React.lazy(() => import("@/components/Error/404"))),
		meta: {
			title: "404页面",
			key: "404",
      notRequiresAuth: true
		}
	},
];

export default errorRouter;