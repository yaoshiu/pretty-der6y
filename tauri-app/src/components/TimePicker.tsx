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
  createEffect,
  createMemo,
  createSignal,
  For,
  onMount,
  Show,
  type ParentComponent,
  type Signal,
} from "solid-js";
import Input from "./Input";
import Icon from "./Icon";
import { faClock } from "@fortawesome/free-solid-svg-icons";
import Popover from "./Popover";
import clickOut from "@directives/clickOut";
import Scrollbar from "./Scrollbar";

clickOut; // avoid unused import warning

export function TimePicker(props: { time: Signal<Date> }) {
  const [time, setTime] = props.time;
  const [show, setShow] = createSignal(false);

  const value = createMemo(() => {
    const offset = time().getTimezoneOffset() * 60 * 1000;
    return new Date(time().getTime() - offset).toISOString().slice(11, 19);
  });

  const TimeButton: ParentComponent<{
    hour?: number;
    minute?: number;
    second?: number;
  }> = (props) => {
    const same = createMemo(
      () =>
        time().getHours() === (props.hour ?? time().getHours()) &&
        time().getMinutes() === (props.minute ?? time().getMinutes()) &&
        time().getSeconds() === (props.second ?? time().getSeconds()),
    );

    let self!: HTMLButtonElement;

    function scrollToCenter(behavior: ScrollBehavior) {
      // Dirty trick the get the scrollable parent
      // Not a good practice, but it works, for now
      if (!self.parentElement) return;
      const parent = self.parentElement;

      const scrollHeight = parent.clientHeight;
      const selfHeight = self.clientHeight;

      const scrollTop = self.offsetTop - (scrollHeight - selfHeight) / 2;

      parent.scrollTo({ top: scrollTop, behavior });
    }

    onMount(() => {
      if (same()) {
        scrollToCenter("instant");
      }
      createEffect(() => same() && scrollToCenter("smooth"));
    });

    function handleClick() {
      const newDate = new Date(time());
      newDate.setHours(props.hour ?? time().getHours());
      newDate.setMinutes(props.minute ?? time().getMinutes());
      newDate.setSeconds(props.second ?? time().getSeconds());
      setTime(newDate);
    }

    return (
      <button
        type="button"
        class="block rounded-lg text-center w-12 p-1 my-1 hover:bg-gray-300"
        tabindex="-1"
        classList={{
          "bg-gray-300": same(),
          "bg-transparent": !same(),
          "text-indigo-500": same(),
        }}
        onClick={handleClick}
        ref={self}
      >
        {props.children}
      </button>
    );
  };

  let ref!: HTMLDivElement;

  return (
    <div class="relative" use:clickOut={() => setShow(false)} ref={ref}>
      <Input
        type="text"
        value={value()}
        pattern="^([01]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$"
        onInput={(event) => {
          const input = event.target.value;
          if (/^([01]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$/.test(input)) {
            const [hour, minute, second] = input.split(":").map(Number);
            setTime((prev) => {
              const date = new Date(prev);
              date.setHours(hour);
              date.setMinutes(minute);
              date.setSeconds(second);
              return date;
            });
          }
        }}
        onFocus={() => setShow(true)}
        onFocusOut={(event) => {
          if (
            event.relatedTarget &&
            !ref.contains(event.relatedTarget as HTMLElement)
          ) {
            setShow(false);
          }
        }}
        placeholder="HH:MM"
        suffixContent={<Icon icon={faClock} classes="text-gray-400" />}
      />
      <Show when={show()}>
        <Popover>
          <div class="px-2">
            <div class="flex h-48 space-x-1 justify-between">
              <Scrollbar>
                <For each={[...Array(24).keys()]}>
                  {(hour) => {
                    const newDate = new Date(time());
                    newDate.setHours(hour);

                    return (
                      <TimeButton hour={hour}>
                        {hour.toString().padStart(2, "0")}
                      </TimeButton>
                    );
                  }}
                </For>
              </Scrollbar>
              <Scrollbar>
                <For each={[...Array(60).keys()]}>
                  {(minute) => {
                    const newDate = new Date(time());
                    newDate.setMinutes(minute);

                    return (
                      <TimeButton minute={minute}>
                        {minute.toString().padStart(2, "0")}
                      </TimeButton>
                    );
                  }}
                </For>
              </Scrollbar>
              <Scrollbar>
                <For each={[...Array(60).keys()]}>
                  {(second) => {
                    const newDate = new Date(time());
                    newDate.setSeconds(second);

                    return (
                      <TimeButton second={second}>
                        {second.toString().padStart(2, "0")}
                      </TimeButton>
                    );
                  }}
                </For>
              </Scrollbar>
            </div>
            <div class="flex justify-between py-1 border-t border-gray-300">
              <button
                type="button"
                class="rounded-lg px-2 py-1 border border-gray-300 hover:bg-gray-300"
                onClick={() => {
                  setTime(new Date());
                }}
              >
                Now
              </button>
              <button
                type="button"
                class="rounded-lg px-2 py-1 border border-gray-300 hover:bg-gray-300"
                onClick={() => {
                  setShow(false);
                }}
              >
                OK
              </button>
            </div>
          </div>
        </Popover>
      </Show>
    </div>
  );
}

export default TimePicker;
