/*
    Pretty Der6y - A third-party running data upload client.
    Copyright (C) 2024  Fay Ash

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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
import { check, type Update } from "@tauri-apps/plugin-updater";

const Main = lazy(() => import("./Main.tsx"));

const LoginedContext = createContext(createSignal(false));

export function useLogined() {
  return useContext(LoginedContext);
}

function Body() {
  const [logined] = useLogined();
  const logger = useLogger();
  const [update, setUpdate] = createSignal<Update | null>(null);

  onMount(async () => {
    try {
      logger?.info("Checking for updates...");
      setUpdate(await check());
      logger?.info(
        update()?.available
          ? `${update()?.version} is available! Downloading...`
          : "Newest release!",
      );
      await update()?.downloadAndInstall((event) => {
        if (event.event === "Finished") {
          logger?.info("Update installed, restart to apply!");
        }
      });
    } catch (error) {
      const message = error instanceof Error ? error.message : error;
      logger?.error(`Error checking for updates: ${message}`);
    }
  });

  return (
    <Show when={logined()} fallback={<Login />}>
      <Main />
    </Show>
  );
}

function App() {
  const [logined, setLogined] = createSignal(false);

  return (
    <LoggerProvider>
      <div
        onContextMenu={(event) => {
          // tricky way to enable right click on dev server
          if (window.location.hostname === "tauri.localhost") {
            event.preventDefault();
          }
        }}
        class="select-none md:rounded-xl overflow-hidden"
      >
        <div class="hidden md:block">
          <TittleBar />
        </div>
        <LoginedContext.Provider value={[logined, setLogined]}>
          <Body />
        </LoginedContext.Provider>
      </div>
    </LoggerProvider>
  );
}

export default App;
