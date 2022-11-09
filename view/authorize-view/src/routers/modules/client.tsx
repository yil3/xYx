/**
* @Author xYx
* @Date 2022-11-04 16:33:51
*/
import LayoutView from "@/layouts";
import { RouteObject } from "../interface";
import Client from "@/views/client";

const route: RouteObject[] = [{
  element: <LayoutView />,
  children: [
    { path: "/client", element: <Client />, meta: { title: "Client", key:"client" } },
  ],
}]

export default route;
