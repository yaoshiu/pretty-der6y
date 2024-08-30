import { type JSX, mergeProps, splitProps } from "solid-js";

/**
 * Renders an input component with optional prefix and suffix content.
 *
 * @component
 * @param {JSX.InputHTMLAttributes<HTMLInputElement>} props - The input element props.
 * @param {JSX.Element} props.prefixContent - The content to be displayed before the input.
 * @param {JSX.Element} props.suffixContent - The content to be displayed after the input.
 * @returns {JSX.Element} The rendered input component.
 */
export const Input = (
  props: JSX.InputHTMLAttributes<HTMLInputElement> & {
    prefixContent?: JSX.Element;
    suffixContent?: JSX.Element;
  },
) => {
  const merged = mergeProps(
    {
      type: "text",
    },
    props,
  );

  const [local, others] = splitProps(merged, [
    "prefixContent",
    "suffixContent",
  ]);

  return (
    <div class="relative">
      <label>
        <div class="absolute flex justify-center items-center h-full w-10 top-1/2 transform -translate-y-1/2">
          {local.prefixContent}
        </div>
        <div class="absolute flex justify-center items-center h-full w-10 top-1/2 right-0 transform -translate-y-1/2">
          {local.suffixContent}
        </div>
        <input
          class="w-full px-4 p-3 border rounded-lg border-gray-300 focus:ring-2 focus:ring-indigo-500 focus:outline-none invalid:ring-rose-500"
          classList={{
            "pl-10": local.prefixContent !== undefined,
            "pr-10": local.suffixContent !== undefined,
          }}
          {...others}
        />
      </label>
    </div>
  );
};

export default Input;
