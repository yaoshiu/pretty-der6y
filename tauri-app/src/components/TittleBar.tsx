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

import { faMinus, faXmark } from "@fortawesome/free-solid-svg-icons";
import { getCurrentWindow, type Window } from "@tauri-apps/api/window";
import { onMount, type JSX } from "solid-js";
import Icon from "./Icon";
import { useLogger } from "./Logger";

const TittleButton = (props: {
  children: JSX.Element;
  onClick: (event: MouseEvent) => void;
  onKeyPress?: (event: KeyboardEvent) => void;
}) => {
  return (
    <div
      class="inline-flex justify-center items-center w-8 h-8"
      onClick={props.onClick}
      onKeyPress={props.onKeyPress}
    >
      {props.children}
    </div>
  );
};

export const TittleBar = () => {
  const logger = useLogger();

  let appWindow: Window | undefined;

  onMount(() => {
    try {
      appWindow = getCurrentWindow();
    } catch (error) {
      const message = error instanceof Error ? error.message : error;
      logger?.error(`Error getting current window: ${message}`);
    }
  });

  return (
    <div
      data-tauri-drag-region
      class="h-8 flex justify-end fixed top-0 left-0 right-0 z-[500]"
    >
      <TittleButton onClick={() => appWindow?.minimize()}>
        <Icon icon={faMinus} classes="transition-all hover:drop-shadow-lg" />
      </TittleButton>
      <TittleButton onClick={() => appWindow?.close()}>
        <Icon icon={faXmark} classes="transition-all hover:drop-shadow-lg" />
      </TittleButton>
    </div>
  );
};

export default TittleBar;
