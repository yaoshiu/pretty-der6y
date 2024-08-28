import { createMemo, createSignal, onCleanup, onMount } from "solid-js";
import DatePicker from "./components/DatePicker";
import Map from "./components/Map";
import TwoColumn from "./layouts/TwoColumn";
import TimePicker from "./components/TimePicker";
import Slider from "./components/Slider";
import Uploader from "./components/Uploader";
import Button from "./components/Button";
import { useLogger } from "./components/Logger";
import * as L from "leaflet";
import { invoke } from "@tauri-apps/api/core";

export default function Main() {
  const logger = useLogger();
  const [time, setTime] = createSignal(new Date());
  const [percentage, setPercentage] = createSignal(0);
  const [file, setFile] = createSignal<File | null>(null);
  const [map, setMap] = createSignal<L.Map>();
  const [daily, setDaily] = createSignal(0);
  const [pending, setPending] = createSignal(false);

  const mileage = createMemo(() => (percentage() * daily()) / 100);

  let tick: NodeJS.Timeout;

  onMount(() => {
    (function updateTime() {
      const now = new Date();
      setTime(now);

      const delay = 1000 - (now.getTime() % 1000);

      tick = setTimeout(updateTime, delay);
    })();

    invoke("get_daily_limit")
      .then(setDaily)
      .catch((e: any) => {
        logger?.error(e.toString());
      });
  });

  onCleanup(() => {
    clearTimeout(tick);
  });

  function passedSetTime(time: Date | ((prev: Date) => Date)) {
    clearInterval(tick);
    return setTime(time);
  }

  function updateFile(newFile: File | ((prev: File | null) => File)) {
    setFile(newFile);

    const file = typeof newFile === "function" ? newFile(null) : newFile;
    if (file) {
      const reader = new FileReader();
      reader.onload = (event) => {
        const data = event.target?.result;
        if (typeof data === "string") {
          const json = JSON.parse(data);
          const geojson = L.geoJSON(json);
          geojson.addTo(map()!);
          map()?.fitBounds(geojson.getBounds());
        }
      };
      reader.readAsText(file);
    }
  }

  return (
    <TwoColumn
      first={<Map class="w-full h-full" map={[map, setMap]} />}
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
                logger?.error("No file selected!");
                return;
              }
              setPending(true);

              const reader = new FileReader();
              reader.onload = (event) => {
                const data = event.target?.result;
                if (typeof data === "string") {
                  invoke("upload", {
                    geojson: data,
                    mileage: mileage(),
                    endTime: time().getTime(),
                  })
                    .then(() => {
                      logger?.info("Upload successful!");
                    })
                    .catch((e) => {
                      logger?.error(e.toString());
                    })
                    .finally(() => setPending(false));
                } else {
                  logger?.error("Invalid file type!");
                  setPending(false);
                }
              };

              reader.readAsText(file()!);
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
