import {
  createContext,
  createSignal,
  lazy,
  onMount,
  Show,
  useContext,
} from "solid-js";
import Login from "./Login.tsx";
import { LoggerProvider, useLogger } from "./components/Logger.tsx";
import TittleBar from "./components/TittleBar.tsx";
import { check } from "@tauri-apps/plugin-updater";

const Main = lazy(() => import("./Main.tsx"));

const LoginedContext = createContext(createSignal(false));

export function useLogined() {
  return useContext(LoginedContext);
}

const Body = () => {
  const [logined] = useLogined();
  const logger = useLogger();

  onMount(async () => {
    try {
      const update = await check();
      logger?.info(
        update?.available
          ? `${update?.version} is available! Downloading...`
          : `Newest release!`
      );
      await update?.downloadAndInstall((event) => {
        if (event.event === "Finished") {
          logger?.info("Update downloaded, restart to apply!");
        }
      });
    } catch (error: any) {
      logger?.error(error.toString());
    }
  });

  return (
    <Show when={logined()} fallback={<Login />}>
      <Main />
    </Show>
  );
};

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
          <Body />
        </LoginedContext.Provider>
      </div>
    </LoggerProvider>
  );
};

export default App;
