import { NonIndexRouteObject } from "react-router-dom";

/**
* @Author xYx
* @Date 2022-11-26 17:37:54
*/
export interface MetaProps {
  keepAlive?: boolean;
  notRequiresAuth?: boolean;
  title: string;
  key?: string;
}

export interface RouteStruct extends NonIndexRouteObject {
  meta?: MetaProps;
	children?: RouteStruct[];
}
