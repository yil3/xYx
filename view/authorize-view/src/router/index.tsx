import { LayoutBasic } from "@/layouts";
import Login from "@/pages/Login";
import SignUp from "@/pages/Signup";
import Authorize from "@/pages/Authorize";
import { Navigate, useRoutes } from "react-router-dom";
import { RouteStruct } from "./interface";
import lazyLoad from "@/utils/RouteUtils";

// * 导入所有router
const metaRouters = import.meta.glob("./modules/*.tsx", { eager: true });
export const routes: RouteStruct[] = [];

Object.keys(metaRouters).forEach((item) => {
  let metaRouter = metaRouters[item] as any;
  if (metaRouter.default) {
    routes.push.apply(routes, metaRouter.default);
  }
});

routes.push.apply(routes, [
  {
    element: <LayoutBasic />,
    children: [
      {
        path: "/login",
        element: <Login />,
        meta: { title: "login", notAuth: true },
      },
      {
        path: "/signup",
        element: <SignUp />,
        meta: { title: "signup", notAuth: true },
      },
      {
        path: "/authorize",
        element: <Authorize />,
        meta: { title: "authorize", notAuth: true },
      },
    ],
  },
  {
    path: "*",
    element: <Navigate to="/404" />,
  },
]);

export default () => useRoutes(lazyLoad(routes));
