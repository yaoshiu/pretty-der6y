// uno.config.ts
import { defineConfig } from "unocss";

export default defineConfig({
  theme: {
    animation: {
      keyframes: {
        ripple: `{
          0% { transform: scale(0); opacity: 1; }
          100% { transform: scale(4); opacity: 0; }
        }`,
      },
      durations: {
        ripple: "1s",
      },
      timingFns: {
        ripple: "linear",
      },
    },
  },
});
