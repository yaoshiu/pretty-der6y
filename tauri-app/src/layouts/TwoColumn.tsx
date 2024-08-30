import type { JSX } from "solid-js";

export const TwoColumn = (props: {
  first: JSX.Element;
  second: JSX.Element;
  scrollOnMobile?: boolean;
}) => {
  return (
    <div class="flex flex-col md:flex-row h-screen">
      <div
        class="w-full md:w-1/2 h-1/2 md:h-full
        flex items-center justify-center bg-white"
      >
        {props.first}
      </div>
      <div
        class="w-full md:w-1/2 h-1/2 md:h-full
        flex items-center justify-center bg-white"
      >
        {props.second}
      </div>
    </div>
  );
};

export default TwoColumn;
