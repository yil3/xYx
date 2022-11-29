import { LayoutBasic } from "@/layouts";
import { Navigate, useRoutes } from "react-router-dom";
import { RouteStruct } from "./interface";

// * 导入所有router
const metaRouters = import.meta.glob("./modules/*.tsx", { eager: true });
export const routes: RouteStruct[] = [];

Object.keys(metaRouters).forEach((item) => {
  let metaRouter = metaRouters[item] as any;
  if (metaRouter.default) {
    routes.push(...metaRouter.default);
  }
});

routes.concat([
  {
    element: <LayoutBasic />,
    children: [],
  },
  {
    path: "*",
    element: <Navigate to="/404" />,
  },
]);

export default () => useRoutes(routes);
