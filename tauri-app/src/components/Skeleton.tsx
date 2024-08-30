import { createSignal, mergeProps, onMount, Show, type JSX } from "solid-js";

export const Skeleton = (props: { height?: string }) => {
	const merged = mergeProps({ height: "1rem" }, props);

	return (
		<div
			class="animate-pulse bg-gray-300 w-full"
			style={{
				height: merged.height,
			}}
		/>
	);
};

/**
 * Renders a background component with a fallback skeleton while the image is loading.
 *
 * @param props - The component props.
 * @param props.src - The source URL of the background image.
 * @param props.children - The child elements to be rendered inside the background component.
 * @returns The rendered background component.
 */
export const Background = (props: { src: string; children: JSX.Element }) => {
	const [loaded, setLoaded] = createSignal(false);

	onMount(() => {
		const img = new Image();
		img.onload = () => setLoaded(true);
		img.src = props.src;
	});

	return (
		<Show when={loaded()} fallback={<Skeleton height="100%" />}>
			<div
				class="w-full h-full bg-cover bg-center transition-opacity duration-500"
				classList={{
					"opacity-100": loaded(),
					"opacity-0": !loaded(),
				}}
				style={{
					"background-image": `url('${props.src}')`,
				}}
			>
				{props.children}
			</div>
		</Show>
	);
};

export default Skeleton;
