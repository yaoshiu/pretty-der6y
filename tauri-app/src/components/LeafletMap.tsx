import * as L from "leaflet";
import { type JSX, onMount, type Signal, splitProps } from "solid-js";
import "leaflet/dist/leaflet.css";
import { useLogger } from "./Logger";

export const LeafletMap = (
  props: JSX.HTMLAttributes<HTMLDivElement> & {
    map: Signal<L.Map | undefined>;
  },
) => {
  const [local, others] = splitProps(props, ["ref", "map"]);
  const [, setMap] = local.map;
  const logger = useLogger();

  let ref!: HTMLDivElement;

  onMount(() => {
    const map = L.map(ref);
    setMap(map);
    navigator.geolocation.getCurrentPosition(
      (position) => {
        map.setView([position.coords.latitude, position.coords.longitude], 13);
      },
      async (error) => {
        // This can be annoying. Maybe we should just remove it.
        logger?.warn(
          `Error getting location: ${error.message}, falling back to IP location`,
        );

        const response = await fetch("https://ipapi.co/json/");
        if (response.ok) {
          const data = await response.json();

          const { latitude, longitude } = data;

          map.setView([latitude, longitude], 13);
        } else {
          logger?.error(
            `Error getting location: ${response.statusText}, falling back to default location`,
          );
          map.setView([51.505, -0.09], 13);
        }
      },
    );
    L.tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
      maxZoom: 19,
      attribution:
        '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>',
    }).addTo(map);
  });

  return (
    <div
      ref={(el) => {
        ref = el;
        if (typeof local.ref === "function") {
          local.ref(el);
        } else {
          local.ref = el;
        }
      }}
      {...others}
    />
  );
};

export default LeafletMap;
