import Login from "@/pages/login";
import SignUp from "@/pages/signup";
import { Navigate, useRoutes } from "react-router-dom";
import { RouteObject } from "./interface";

// * 导入所有router
const metaRouters = import.meta.glob("./modules/*.tsx", { eager: true });

export const routerArray: RouteObject[] = [];
Object.keys(metaRouters).forEach(item => {
  let metaRouter = metaRouters[item] as any;
  if (metaRouter.default) {
    routerArray.push(...metaRouter.default);
  }
});

export const router: RouteObject[] = [
  { path: "/login", element: <Login />, meta: { title: 'login', notRequiresAuth: true } },
  { path: "/signup", element: <SignUp />, meta: { title: 'signup', notRequiresAuth: true } },
  ...routerArray,
	{
		path: "*",
		element: <Navigate to="/404" />
	}
];

const Router = () => {
  // @ts-ignore
  return useRoutes(router);
};
export default Router;
