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

import { createSignal, For, mergeProps, splitProps, type JSX } from "solid-js";

export const Button = (
  props: JSX.ButtonHTMLAttributes<HTMLButtonElement> & {
    disabled?: boolean;
  },
) => {
  const merged = mergeProps(
    {
      type: "button" as const,
    },
    props,
  );
  const [local, others] = splitProps(merged, ["onClick", "children"]);

  const [ripples, setRipples] = createSignal<
    { x: number; y: number; size: number }[]
  >([]);

  return (
    <button
      {...others}
      class="w-full py-3 relative overflow-hidden rounded-lg transition-colors bg-indigo-600 disabled:bg-indigo-700 text-white hover:bg-indigo-700"
      onClick={(event) => {
        if (typeof local.onClick === "function") {
          local.onClick(event);
        } else if (Array.isArray(local.onClick)) {
          const [handler, data] = local.onClick;
          handler(data, event);
        }

        const rect = event.currentTarget.getBoundingClientRect();
        const size = Math.max(rect.width, rect.height);
        const x = event.clientX - rect.left - size / 2;
        const y = event.clientY - rect.top - size / 2;

        setRipples((prev) => [...prev, { x, y, size }]);
        setTimeout(() => setRipples((prev) => prev.slice(1)), 1000);
      }}
    >
      {local.children}

      <For each={ripples()}>
        {({ x, y, size }) => (
          <span
            class="animate-ripple absolute rounded-full bg-white/50 pointer-events-none"
            style={{
              top: `${y}px`,
              left: `${x}px`,
              width: `${size}px`,
              height: `${size}px`,
            }}
          />
        )}
      </For>
    </button>
  );
};

export default Button;
