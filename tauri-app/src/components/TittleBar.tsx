import { faMinus, faXmark } from "@fortawesome/free-solid-svg-icons";
import { getCurrentWindow, type Window } from "@tauri-apps/api/window";
import { onMount, type JSX } from "solid-js";
import Icon from "./Icon";
import { useLogger } from "./Logger";

const TittleButton = (props: {
  children: JSX.Element;
  onClick: () => void;
}) => {
  return (
    <div
      class="inline-flex justify-center items-center w-8 h-8"
      onClick={props.onClick}
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
    } catch (msg: any) {
      logger?.error(msg.toString());
    }
  });

  return (
    <div
      data-tauri-drag-region
      class="h-8 flex justify-end fixed top-0 left-0 right-0 z-[500]"
    >
      <TittleButton onClick={() => appWindow?.minimize()}>
        <Icon icon={faMinus} class="transition-all hover:drop-shadow-lg" />
      </TittleButton>
      <TittleButton onClick={() => appWindow?.close()}>
        <Icon icon={faXmark} class="transition-all hover:drop-shadow-lg" />
      </TittleButton>
    </div>
  );
};

export default TittleBar;
