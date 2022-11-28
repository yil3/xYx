import Router from "@/router";
import AuthRouter from "@/router/AuthRouter";

export default function App() {
  return (
    <div className="App">
      <AuthRouter>
        <Router />
      </AuthRouter>
    </div>
  );
}

