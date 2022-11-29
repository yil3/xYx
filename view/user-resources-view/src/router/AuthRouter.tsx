import { useLocation } from "react-router-dom";
import { AxiosCanceler } from "@/api";
import { searchRoute } from "@/utils/RouteUtils";
import { routes } from "@/router";
import { parse } from 'qs'
import { token } from '@/api/modules/authorize'

const axiosCanceler = new AxiosCanceler();

/**
 * @description 路由守卫组件
 * */
const AuthRouter = (props: { children: JSX.Element }) => {
  // * 在跳转路由之前，清除所有的请求
  axiosCanceler.removeAllPending();

  // * 末授权/授权过期 获取token
  const params = parse(window.location.search, { ignoreQueryPrefix: true });
  if (params && params.code && params.state && params.state == 'x') {
    token({
      grant_type: 'authorization_code',
      code: params.code,
      client_id: "00000000-0000-0000-0000-000000000001",
      client_secret: "aa332211"
    }).then(res => {
      localStorage.setItem('token', JSON.stringify(res.data))
      // 删除url 全部参数
      window.history.replaceState(null, "", "/");
    })
  }

  // * 判断当前路由是否需要访问权限(不需要权限直接放行)
  const { pathname } = useLocation();
  const route = searchRoute(pathname, routes);
  if (route.meta?.notAuth) return props.children;

  // * 当前账号有权限返回 Router，正常访问页面
  // TODO: 判断当前账号是否有权限访问当前路由

  return props.children;
};

export default AuthRouter;
