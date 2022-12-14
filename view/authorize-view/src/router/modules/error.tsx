import React from "react";
import { RouteStruct } from "@/router/interface";

// 错误页面模块
const route: Array<RouteStruct> = [
	{
		path: "/404",
		element: React.lazy(() => import("@/components/Error/404")),
		meta: {
			title: "404页面",
			key: "404",
      notAuth: true
		}
	},
];

export default route;
