import LayoutView from "@/layouts";
import Home from "@/views/home";
import { RouteStruct } from "../interface";

const route: RouteStruct[] = [{
  element: <LayoutView />,
  children: [
    { path: "/", element: <Home />, meta: { title: "Home", key:"home" } },
  ],
}]

export default route;
