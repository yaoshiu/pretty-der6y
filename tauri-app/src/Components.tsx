import type { JSX, Signal } from "solid-js";
import { mergeProps } from "solid-js/web";
import { children, createSignal, For, splitProps } from "solid-js";

export function Input(props: {
  value?: Signal<string>;
  type?: string;
  required?: boolean;
  maxlength?: number;
  placeholder?: string;
}) {
  const merged = mergeProps(
    {
      type: "text",
      value: createSignal(""),
    },
    props
  );
  const [local, others] = splitProps(merged, ["value"]);
  const [value, setValue] = local.value;

  return (
    <input
      class="w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:outline-none"
      {...others}
      value={value()}
      onInput={(e) => setValue(e.currentTarget.value)}
    />
  );
}

export function Button(props: JSX.ButtonHTMLAttributes<HTMLButtonElement>) {
  const merged = mergeProps(
    {
      type: "button" as const,
    },
    props
  );
  const [local, others] = splitProps(merged, ["onClick", "children"]);

  const [ripples, setRipples] = createSignal<
    { x: number; y: number; size: number }[]
  >([]);

  const onClick: JSX.EventHandler<HTMLButtonElement, MouseEvent> = (e) => {
    const rect = e.currentTarget.getBoundingClientRect();
    const size = Math.max(rect.width, rect.height);
    const x = e.clientX - rect.left - size / 2;
    const y = e.clientY - rect.top - size / 2;

    setRipples((prev) => [...prev, { x, y, size }]);
    setTimeout(() => setRipples((prev) => prev.slice(1)), 1000);

    if (typeof local.onClick === "function") {
      local.onClick(e);
    }
  };

  return (
    <button
      class="w-full bg-blue-500 text-white py-2 rounded-lg hover:bg-blue-600 relative overflow-hidden"
      onClick={onClick}
      {...others}
    >
      {children(() => local.children)()}

      <For each={ripples()}>
        {({ x, y, size }) => (
          <span
            class="animate-ripple absolute rounded-full bg-white bg-opacity-60 pointer-events-none"
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
}
