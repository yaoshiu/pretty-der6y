import { createContext, createSignal, Show, useContext } from "solid-js";
import Login from "./Login.tsx";
import { LoggerProvider } from "./components/Logger.tsx";
import TittleBar from "./components/TittleBar.tsx";
import Main from "./Main.tsx";

const LoginedContext = createContext(createSignal(false));

export function useLogined() {
  return useContext(LoginedContext);
}

const App = () => {
  const [logined, setLogined] = createSignal(false);

  return (
    <LoggerProvider>
      <div
        onContextMenu={(event) => {
          if (window.location.hostname == "tauri.localhost") {
            event.preventDefault();
          }
        }}
        class="select-none"
      >
        <TittleBar />
        <LoginedContext.Provider value={[logined, setLogined]}>
          <Show when={logined()} fallback={<Login />}>
            <Main />
          </Show>
        </LoginedContext.Provider>
      </div>
    </LoggerProvider>
  );
};

export default App;
