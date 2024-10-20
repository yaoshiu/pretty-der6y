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

import { createMemo, createSignal, onCleanup, onMount } from "solid-js";
import DatePicker from "@components/DatePicker";
import LeafletMap from "@components/LeafletMap";
import TwoColumn from "@layouts/TwoColumn";
import TimePicker from "@components/TimePicker";
import Slider from "@components/Slider";
import Uploader from "@components/Uploader";
import Button from "@components/Button";
import { useLogger } from "@components/Logger";
import * as L from "leaflet";
import { commands } from "@helpers/bindings";
import isDef from "@helpers/isDef";

export default function Main() {
  const logger = useLogger();
  const [time, setTime] = createSignal(new Date());
  const [percentage, setPercentage] = createSignal(0);
  const [file, setFile] = createSignal<File>();
  const [map, setMap] = createSignal<L.Map>();
  const [daily, setDaily] = createSignal(0);
  const [pending, setPending] = createSignal(false);

  const mileage = createMemo(() => (percentage() * daily()) / 100);

  let tick: number;

  onMount(() => {
    (function updateTime() {
      const now = new Date();
      setTime(now);

      const delay = 1000 - (now.getTime() % 1000);

      tick = setTimeout(updateTime, delay);
    })();

    commands
      .getDailyLimit()
      .then((res) =>
        res.status === "ok"
          ? setDaily(res.data)
          : logger?.error(`Error getting daily limit: ${res.error}`),
      )
      .catch((error) => {
        const message = error instanceof Error ? error.message : error;
        logger?.error(`Error getting daily limit: ${message}`);
      });
  });

  onCleanup(() => {
    clearTimeout(tick);
  });

  function passedSetTime(time: Date | ((prev: Date) => Date)) {
    clearInterval(tick);
    return setTime(time);
  }

  function updateFile(next?: File | ((prev?: File) => File)) {
    if (next) {
      setFile(next);
    } else {
      setFile();
    }

    if (isDef(file)) {
      const reader = new FileReader();
      reader.onload = (event) => {
        const data = event.target?.result;
        if (typeof data === "string") {
          const json = JSON.parse(data);
          const geojson = L.geoJSON(json);
          if (isDef(map)) {
            geojson.addTo(map());
            map().fitBounds(geojson.getBounds());
          }
        }
      };
      reader.readAsText(file());
    }

    return undefined;
  }

  return (
    <TwoColumn
      first={<LeafletMap class="w-full h-full" map={[map, setMap]} />}
      second={
        <div
          class="overflow-y-auto md:overflow-y-hidden max-w-sm 
          flex flex-col md:justify-center w-full h-full p-8"
        >
          <h2 class="text-3xl font-bold text-gray-800 mb-6">UPLOAD</h2>
          <form
            class="space-y-4"
            onSubmit={(event) => {
              event.preventDefault();
              if (!file()) {
                logger?.warn("No file selected!");
                return;
              }
              setPending(true);

              const reader = new FileReader();
              reader.onload = (event) => {
                const data = event.target?.result;
                if (typeof data === "string") {
                  commands
                    .upload(data, mileage(), time().getTime())
                    .then((res) =>
                      res.status === "ok"
                        ? logger?.info("Upload successful!")
                        : logger?.error(`Error uploading: ${res.error}`),
                    )
                    .catch((error) => {
                      logger?.error(`Error uploading: ${error}`);
                    })
                    .finally(() => setPending(false));
                } else {
                  logger?.error("Invalid file type!");
                  setPending(false);
                }
              };

              if (isDef(file)) {
                reader.readAsText(file());
              }
            }}
          >
            <label class="block">
              <span class="text-gray-500 font-bold">Date</span>
              <DatePicker date={[time, passedSetTime]} />
            </label>
            <label class="block">
              <span class="text-gray-500 font-bold">Time</span>
              <TimePicker time={[time, passedSetTime]} />
            </label>
            <label class="block">
              <div class="justify-between flex">
                <span class="text-gray-500 font-bold">Mileage</span>
                <span class="text-gray-700 text-sm">
                  {mileage().toFixed(2)}km
                </span>
              </div>
              <Slider value={[percentage, setPercentage]} />
            </label>
            <Uploader
              file={[file, updateFile]}
              accept=".geojson,application/geo+json"
            />
            <Button type="submit" disabled={pending()}>
              Upload
            </Button>
          </form>
        </div>
      }
    />
  );
}
