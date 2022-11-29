/**
* @Author xYx
* @Date 2022-11-26 17:48:29
*/
import { NonIndexRouteObject } from "react-router-dom";

export interface MetaProps {
	keepAlive?: boolean;
	notAuth?: boolean;
	title: string;
	key?: string;
}

export interface RouteStruct extends NonIndexRouteObject {
	meta?: MetaProps;
	children?: RouteStruct[];
  element?: any;
}
