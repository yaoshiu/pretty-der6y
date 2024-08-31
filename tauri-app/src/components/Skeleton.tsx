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

import { createSignal, mergeProps, onMount, Show, type JSX } from "solid-js";

export const Skeleton = (props: { height?: string }) => {
  const merged = mergeProps({ height: "1rem" }, props);

  return (
    <div
      class="animate-pulse bg-gray-300 w-full"
      style={{
        height: merged.height,
      }}
    />
  );
};

/**
 * Renders a background component with a fallback skeleton while the image is loading.
 *
 * @param props - The component props.
 * @param props.src - The source URL of the background image.
 * @param props.children - The child elements to be rendered inside the background component.
 * @returns The rendered background component.
 */
export const Background = (props: { src: string; children: JSX.Element }) => {
  const [loaded, setLoaded] = createSignal(false);

  onMount(() => {
    const img = new Image();
    img.onload = () => setLoaded(true);
    img.src = props.src;
  });

  return (
    <Show when={loaded()} fallback={<Skeleton height="100%" />}>
      <div
        class="w-full h-full bg-cover bg-center transition-opacity duration-500"
        classList={{
          "opacity-100": loaded(),
          "opacity-0": !loaded(),
        }}
        style={{
          "background-image": `url('${props.src}')`,
        }}
      >
        {props.children}
      </div>
    </Show>
  );
};

export default Skeleton;
