import LayoutView from "@/layouts";
import Home from "@/views/Home";
import { RouteStruct } from "../interface";

const route: RouteStruct[] = [{
  element: <LayoutView />,
  children: [
    { path: "/", element: <Home />, meta: { title: "Home", key:"" } },
  ],
}]

export default route;
