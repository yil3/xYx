import { useLocation, Navigate } from "react-router-dom";
import { AxiosCanceler } from "@/api/helper/axiosCancel";
import { searchRoute } from "@/utils/util";
import { router } from "@/routers/index";

const axiosCanceler = new AxiosCanceler();

/**
 * @description 路由守卫组件
 * */
const AuthRouter = (props: { children: JSX.Element }) => {
  const { pathname } = useLocation();
  const route = searchRoute(pathname, router);
  // * 在跳转路由之前，清除所有的请求
  axiosCanceler.removeAllPending();
  // * 判断当前路由是否需要访问权限(不需要权限直接放行)
  if (route.meta?.notRequiresAuth) return props.children;

  // * 判断是否有Token
  // const token = store.getState().global.token;
  // const token = localStorage.getItem('token')
  // if (!token) return <Navigate to="/login" replace />;

  // * 当前账号有权限返回 Router，正常访问页面
  // TODO: 判断当前账号是否有权限访问当前路由

  return props.children;
};

export default AuthRouter;
