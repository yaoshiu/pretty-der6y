import { onCleanup, type Accessor } from "solid-js";

/**
 * Attaches a click outside event listener to the specified element.
 *
 * @param value - The callback function to be executed when a click outside the element occurs.
 *
 * @example
 * ```tsx
 * import { createSignal } from "solid-js";
 * import clickOut from "./derictives/clickOut";
 *
 * clickOut; // avoid unused import warning
 *
 * function App() {
 *   const [isOpen, setIsOpen] = createSignal(false);
 *
 *   function handleClickOutside() {
 *     setIsOpen(false);
 *   }
 *
 *   return (
 *     <div>
 *       <button onClick={() => setIsOpen(!isOpen())}>Toggle</button>
 *       {isOpen() && (
 *         <div use:clickOut={handleClickOutside}>
 *           Click outside to close
 *         </div>
 *       )}
 *     </div>
 *   );
 * }
 * ```
 */
export default function clickOut(
  el: HTMLElement,
  value: Accessor<(event: MouseEvent) => void>,
) {
  function onClick(event: MouseEvent) {
    if (el.contains(event.target as HTMLElement)) return;
    value()?.(event);
  }

  document.body.addEventListener("click", onClick);

  onCleanup(() => document.body.removeEventListener("click", onClick));
}

declare module "solid-js" {
  namespace JSX {
    interface Directives {
      clickOut: (event: MouseEvent) => void;
    }
  }
}
