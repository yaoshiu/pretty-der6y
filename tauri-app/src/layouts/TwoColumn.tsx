import type { JSX } from "solid-js";

export const TwoColumn = (props: { left: JSX.Element; right: JSX.Element }) => {
  return (
    <div class="flex h-screen">
      <div class="w-1/2 flex items-center justify-center bg-white">
        {props.left}
      </div>
      <div class="w-1/2 flex items-center justify-center bg-white">
        {props.right}
      </div>
    </div>
  );
};

export default TwoColumn;
