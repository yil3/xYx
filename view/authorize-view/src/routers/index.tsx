import Login from "@/pages/login";
import SignUp from "@/pages/signup";
import { useRoutes } from "react-router-dom";
import home from "./modules/home";


const router = [
  { path: "/login", element: <Login /> },
  { path: "/signup", element: <SignUp /> },
  home,
];

const Router = () => {
  return useRoutes(router);
};
export default Router;
