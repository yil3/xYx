import { LayoutBasic } from "@/layouts";
import Login from "@/pages/login";
import SignUp from "@/pages/signup";
import Authorize from "@/pages/authorize";
import { Navigate, useRoutes } from "react-router-dom";
import { RouteStruct } from "./interface";

// * 导入所有router
const metaRouters = import.meta.glob("./modules/*.tsx", { eager: true });

export const routerArray: RouteStruct[] = [];
Object.keys(metaRouters).forEach(item => {
  let metaRouter = metaRouters[item] as any;
  if (metaRouter.default) {
    routerArray.push(...metaRouter.default);
  }
});

export const rootRouter: RouteStruct[] = [
  {
    element: <LayoutBasic />,
    children: [
      { path: "/login", element: <Login />, meta: { title: 'login', notRequiresAuth: true } },
      { path: "/signup", element: <SignUp />, meta: { title: 'signup', notRequiresAuth: true } },
      { path: "/authorize", element: <Authorize />, meta: { title: 'authorize', notRequiresAuth: true } },
    ],
  },
  ...routerArray,
  {
    path: "*",
    element: <Navigate to="/404" />
  }
];

const Router = () => {
  // @ts-ignore
  return useRoutes(rootRouter);
};
export default Router;
