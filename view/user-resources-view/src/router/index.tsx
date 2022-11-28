import { LayoutBasic } from "@/layouts";
import { Navigate, useRoutes } from "react-router-dom";
import { RouteStruct } from "./interface";
import lazyLoad from "@/utils/RouteUtils";

// * 导入所有router
const metaRouters = import.meta.glob("./modules/*.tsx", { eager: true });


export const routeArray: RouteStruct[] = [];
Object.keys(metaRouters).forEach(item => {
  let metaRouter = metaRouters[item] as any;
  if (metaRouter.default) {
    lazyLoad(metaRouter.default);
    routeArray.push(...metaRouter.default);
  }
});

export const routes: RouteStruct[] = [
  {
    element: <LayoutBasic />,
    children: [
    ],
  },
  ...routeArray,
  {
    path: "*",
    element: <Navigate to="/404" />
  }
];

export default () => useRoutes(routes);
