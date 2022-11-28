import LayoutView from "@/layouts";
import { lazy } from "react";
import { RouteStruct } from "../interface";

const route: RouteStruct[] = [{
  element: <LayoutView />,
  children: [
    { path: "/token", element: lazy(() => import("@/components/Token")), meta: { title: "Token", key: "token" } },
  ],
}]

export default route;
