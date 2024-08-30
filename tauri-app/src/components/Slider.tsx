import { mergeProps, type JSX, type Signal } from "solid-js";

const Slider = (props: {
	value: Signal<number>;
	width?: number;
	padding?: number;
	minimum?: number;
	maximum?: number;
}) => {
	const merged = mergeProps(
		{
			width: 24,
			padding: 4,
			minimum: 0,
			maximum: 100,
		},
		props,
	);

	const [value, setValue] = merged.value;

	const startDrag: JSX.EventHandler<HTMLDivElement, MouseEvent> = (event) => {
		const rect = event.currentTarget.getBoundingClientRect();

		function onDrag(event: MouseEvent) {
			const availableLeft = rect.left + merged.width / 2;
			const availableWidth = rect.width - merged.width;
			const newValue = ((event.clientX - availableLeft) * 100) / availableWidth;
			setValue(Math.min(Math.max(newValue, 0), 100));
		}

		onDrag(event);

		function endDrag() {
			document.removeEventListener("mousemove", onDrag);
			document.removeEventListener("mouseup", endDrag);
		}

		document.addEventListener("mousemove", onDrag);
		document.addEventListener("mouseup", endDrag);
	};

	return (
		<div
			class="relative w-full"
			style={{
				height: `${merged.width}px`,
			}}
		>
			<div class="relative h-full w-full shadow-inner bg-gray-300 rounded-full overflow-hidden">
				<div class="drop-shadow h-full w-full" onMouseDown={startDrag}>
					<div
						class="h-full bg-gradient-to-r from-pink-300 to-cyan-300 rounded-full"
						style={{
							// 100% - ((100% - widthpx) * value / 100 + widthpx)
							"clip-path": `inset(0 calc(${100 - value()}% + ${
								(merged.width * (value() - 100)) / 100
							}px) 0 0 round 999px)`,
						}}
					>
						<div
							class="absolute shadow bg-white rounded-full cursor-pointer"
							style={{
								// (100% - widthpx) * value / 100 + widthpx / 2 - (widthpx - 2paddingpx) / 2
								left: `calc(${value()}% + ${
									merged.padding - (merged.width * value()) / 100
								}px)`,
								top: `${merged.padding}px`,
								height: `${merged.width - merged.padding * 2}px`,
								width: `${merged.width - merged.padding * 2}px`,
							}}
						/>
					</div>
				</div>
			</div>
		</div>
	);
};

export default Slider;
