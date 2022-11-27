import React, { Suspense } from "react";
import { Spin } from "antd";
import { RouteStruct } from "../interface";

/**
 * @description 路由懒加载
 * @param {Element} Comp 需要访问的组件
 * @returns element
 */
const intoLazy = (Comp: React.LazyExoticComponent<any>): React.ReactNode => {
	return (
		<Suspense
			fallback={
				<Spin
					size="large"
					style={{
						display: "flex",
						alignItems: "center",
						justifyContent: "center",
						height: "100%"
					}}
				/>
			}
		>
			<Comp />
		</Suspense>
	);
};

const lazyLoad = (arr: RouteStruct[]) => {
  return arr.map((item) => {
    if (!(item.element.type instanceof Function)) {
      item.element = intoLazy(item.element);
    }
    if (item.children && item.children.length) {
      item.children = lazyLoad(item.children);
    }
    return item;
  });
};

export default lazyLoad;
