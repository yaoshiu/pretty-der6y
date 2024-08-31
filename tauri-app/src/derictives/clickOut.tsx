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
