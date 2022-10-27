import Router from "@/routers";
import AuthRouter from "./routers/utils/authRouter";

function App() {
  return (
    <div className="App">
      <AuthRouter>
        <Router />
      </AuthRouter>
    </div>
  );
}

export default App;
