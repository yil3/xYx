import LayoutView from "@/layouts";
import { lazy } from "react";
import { RouteStruct } from "../interface";

const route: RouteStruct[] = [{
  element: <LayoutView />,
  children: [
    { path: "/", element: lazy(() => import("@/components/Home")), meta: { title: "Home", key:"" } },
  ],
}]

export default route;
