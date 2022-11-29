import { RouteStruct } from "@/router/interface";
import React, { Suspense } from "react";
import { Spin } from "antd";
import { isFunction } from "./isUtils";
/**
* @Author xYx
* @Date 2022-11-28 10:05:36
*/

/**
 * @description 递归查询对应的路由
 * @param {String} path 当前访问地址
 * @param {Array} routes 路由列表
 * @returns array
 */
export const searchRoute = (path: string, routes: RouteStruct[] = []): RouteStruct => {
  let result: RouteStruct = {};
  for (let item of routes) {
    if (item.path === path) return item;
    if (item.children) {
      const res = searchRoute(path, item.children);
      if (Object.keys(res).length) result = res;
    }
  }
  return result;
};

export const intoLazy = (Comp: React.LazyExoticComponent<any>): React.ReactNode => {
  return (
    <Suspense
      fallback={
        <Spin
          size="large"
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            height: "100vh"
          }}
        />
      }
    >
      <Comp />
    </Suspense>
  );
};

const lazyLoad = (arr: RouteStruct[]) => {
  return arr.map(item => {
    if (item.element && !isFunction(item.element.type)) {
      item.element = intoLazy(item.element);
    }
    if (item.children) {
      lazyLoad(item.children);
    }
    return item;
  });
};


export default lazyLoad;
