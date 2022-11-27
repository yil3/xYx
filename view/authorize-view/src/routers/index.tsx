import { LayoutBasic } from "@/layouts";
import Login from "@/pages/Login";
import SignUp from "@/pages/Signup";
import Authorize from "@/pages/Authorize";
import { Navigate, useRoutes } from "react-router-dom";
import { RouteStruct } from "./interface";
import lazyLoad from "./utils/lazyLoad";

// * 导入所有router
const metaRouters = import.meta.glob("./modules/*.tsx", { eager: true });

export const router: RouteStruct[] = [];
Object.keys(metaRouters).forEach(item => {
  let metaRouter = metaRouters[item] as any;
  if (metaRouter.default) {
    lazyLoad(metaRouter.default);
    router.push(...metaRouter.default);
  }
});

export const routes: RouteStruct[] = [
  {
    element: <LayoutBasic />,
    children: [
      { path: "/login", element: <Login />, meta: { title: 'login', notRequiresAuth: true } },
      { path: "/signup", element: <SignUp />, meta: { title: 'signup', notRequiresAuth: true } },
      { path: "/authorize", element: <Authorize />, meta: { title: 'authorize', notRequiresAuth: true } },
    ],
  },
  ...router,
  {
    path: "*",
    element: <Navigate to="/404" />
  }
];

export default () => useRoutes(routes);
