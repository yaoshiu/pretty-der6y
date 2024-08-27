import { createSignal, For, mergeProps, splitProps, type JSX } from "solid-js";

export const Button = (
  props: JSX.ButtonHTMLAttributes<HTMLButtonElement> & {
    pending?: boolean;
  }
) => {
  const merged = mergeProps(
    {
      type: "button" as const,
    },
    props
  );
  const [local, others] = splitProps(merged, [
    "onClick",
    "children",
    "pending",
  ]);

  const [ripples, setRipples] = createSignal<
    { x: number; y: number; size: number }[]
  >([]);

  return (
    <button
      class="w-full py-3 relative overflow-hidden rounded-lg transition-colors bg-indigo-600 disabled:bg-indigo-700 text-white hover:bg-indigo-700"
      disabled={local.pending}
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
      {...others}
    >
      {local.children}

      <For each={ripples()}>
        {({ x, y, size }) => (
          <span
            class="animate-ripple absolute rounded-full bg-white/50 pointer-events-none"
            style={{
              top: y + "px",
              left: x + "px",
              width: size + "px",
              height: size + "px",
            }}
          />
        )}
      </For>
    </button>
  );
};

export default Button;
