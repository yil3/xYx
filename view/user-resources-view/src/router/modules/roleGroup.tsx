/**
* @Author xYx
* @Date 2022-12-05 16:50:36
*/

import { lazy } from "react";

export default [{
  element: lazy(() => import("@/layouts")),
  children: [{ path: "/role/group", element: lazy(() => import("@/components/RoleGroup")) }]
}]
