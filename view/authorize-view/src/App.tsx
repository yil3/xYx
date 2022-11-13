import Router from "@/routers";
import AuthRouter from "./routers/utils/authRouter";

export default function App() {
  return (
    <div className="App">
      <AuthRouter>
        <Router />
      </AuthRouter>
    </div>
  );
}

