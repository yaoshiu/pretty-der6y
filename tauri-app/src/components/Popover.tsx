import type { JSX } from "solid-js";

export const Popover = (props: JSX.HTMLAttributes<HTMLDivElement>) => {
	return (
		<div
			class="absolute w-max z-10 bg-white border border-gray-200 rounded-lg shadow-lg"
			{...props}
		>
			{props.children}
		</div>
	);
};

export default Popover;
