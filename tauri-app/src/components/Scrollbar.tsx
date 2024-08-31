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
  createSignal,
  splitProps,
  type JSX,
  type ParentComponent,
} from "solid-js";

const Scrollbar: ParentComponent<JSX.HTMLAttributes<HTMLDivElement>> = (
  props,
) => {
  const [local, others] = splitProps(props, ["children", "ref"]);
  const [height, setHeight] = createSignal("");
  const [top, setTop] = createSignal("0");
  const [dragging, setDragging] = createSignal(false);
  const [show, setShow] = createSignal(false);

  let scrollableContent!: HTMLDivElement;

  const updateThumb: JSX.EventHandler<HTMLDivElement, Event> = (event) => {
    const scrollableContent = event.currentTarget;
    const contentHeight = scrollableContent.scrollHeight;
    const scrollTop = scrollableContent.scrollTop;
    const newThumbTop = (scrollTop / contentHeight) * 100;
    setTop(`${newThumbTop}%`);

    const visibleHeight = scrollableContent.clientHeight;
    const newThumbHeight = (visibleHeight / contentHeight) * 100;
    setHeight(`${newThumbHeight}%`);
  };

  const showThumb: JSX.EventHandler<HTMLDivElement, MouseEvent> = (event) => {
    updateThumb(event);
    setShow(true);
  };

  const startDrag: JSX.EventHandler<HTMLDivElement, MouseEvent> = (event) => {
    const startY = event.clientY;
    const scrollTop = scrollableContent.scrollTop;
    setDragging(true);

    function onDrag(event: MouseEvent) {
      const deltaY = event.clientY - startY;
      const contentHeight = scrollableContent.scrollHeight;
      const visibleHeight = scrollableContent.clientHeight;
      const newScrollTop = scrollTop + (deltaY / visibleHeight) * contentHeight;

      scrollableContent.scrollTo({ top: newScrollTop });
    }

    function stopDrag() {
      setDragging(false);
      document.removeEventListener("mousemove", onDrag);
      document.removeEventListener("mouseup", stopDrag);
    }

    document.addEventListener("mousemove", onDrag);
    document.addEventListener("mouseup", stopDrag);
  };

  function hideThumb() {
    setShow(false);
  }

  return (
    <div class="relative h-full overflow-hidden">
      <div
        {...others}
        ref={(ref) => {
          scrollableContent = ref;
          if (typeof local.ref === "function") {
            local.ref(ref);
          } else {
            local.ref = ref;
          }
        }}
        class="h-full overflow-y-scroll"
        style={{
          "scrollbar-width": "none",
        }}
        onScroll={updateThumb}
        onMouseEnter={showThumb}
        onMouseLeave={hideThumb}
      >
        {local.children}
        <div
          style={{ height: height(), top: top() }}
          class="absolute right-0 w-2 bg-gray-400/50 hover:bg-gray-600/50 transition-opacity duration-300 rounded"
          classList={{
            "opacity-100": show() || dragging(),
            "opacity-0": !show() && !dragging(),
          }}
          onMouseDown={startDrag}
        />
      </div>
    </div>
  );
};

export default Scrollbar;
