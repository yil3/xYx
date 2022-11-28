/**
* @Author xYx
* @Date 2022-11-04 16:33:51
*/
import LayoutView from "@/layouts";
import { lazy } from "react";
import { RouteStruct } from "../interface";

const route: RouteStruct[] = [{
  element: <LayoutView />,
  children: [
    { path: "/client", element: lazy(() => import("@/components/Client")), meta: { title: "Client", key:"client" } },
  ],
}]

export default route;
