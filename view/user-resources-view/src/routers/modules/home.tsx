import LayoutView from "@/layouts";
import Home from "@/views/home";
import { RouteObject } from "../interface";

const route: RouteObject[] = [{
  element: <LayoutView />,
  children: [
    { path: "/", element: <Home />, meta: { title: "Home", key:"" } },
  ],
}]

export default route;
