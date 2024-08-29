import {
	faAngleLeft,
	faAngleRight,
	faCalendar,
} from "@fortawesome/free-solid-svg-icons";
import { createMemo, createSignal, For, Show, type Signal } from "solid-js";
import clickOut from "../derictives/clickOut";
import Icon from "./Icon";
import Input from "./Input";
import Popover from "./Popover";

clickOut; // avoid unused import warning

const DatePicker = (props: { date: Signal<Date> }) => {
	type page = "year" | "month" | "day";

	const [date, setDate] = props.date;

	const value = createMemo(() => {
		const offset = date().getTimezoneOffset() * 60 * 1000;
		return new Date(date().getTime() - offset).toISOString().slice(0, 10);
	});

	const [show, setShow] = createSignal(false);
	const [page, setPage] = createSignal<page>("day");

	const Switcher = (props: { date: Date }) => {
		const decrease = createMemo(() => props.date.getTime() < date().getTime());

		return (
			<button
				type="button"
				tabindex="-1"
				onClick={() => {
					setDate(props.date);
				}}
				class="px-2 hover:text-indigo-500"
				classList={{
					"pr-4": decrease(),
					"pl-4": !decrease(),
				}}
			>
				<Icon icon={decrease() ? faAngleLeft : faAngleRight} />
			</button>
		);
	};

	const DayPage = () => {
		const week = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

		const calendar = createMemo(() => {
			const firstDay = new Date(date());
			firstDay.setDate(1);
			const lastDay = new Date(date());
			lastDay.setMonth(date().getMonth() + 1, 0);

			const startDay = firstDay.getDay();

			const days = Array.from({ length: lastDay.getDate() }, (_, i) => {
				const newDate = new Date(date());
				newDate.setDate(i + 1);
				return newDate;
			});
			const prevDays = Array.from({ length: startDay }, (_, i) => {
				const newDate = new Date(date());
				newDate.setDate(-i);
				return newDate;
			});

			return [...prevDays.reverse(), ...days];
		});

		const MonthYearButton = (props: { type: page }) => {
			return (
				<button
					type="button"
					tabindex="-1"
					onClick={() => setPage(props.type)}
					class="py-1 text-sm px-2 rounded-lg hover:bg-gray-500/50"
				>
					{props.type === "year"
						? date().getFullYear()
						: date().toLocaleString("en-US", { month: "short" })}
				</button>
			);
		};

		return (
			<div>
				<div class="flex justify-between py-1">
					<Switcher
						date={(() => {
							const newDate = new Date(date());
							newDate.setMonth(date().getMonth() - 1);
							return newDate;
						})()}
					/>
					<div class="flex justify-center">
						<MonthYearButton type={"month"} />
						<MonthYearButton type={"year"} />
					</div>
					<Switcher
						date={(() => {
							const newDate = new Date(date());
							newDate.setMonth(date().getMonth() + 1);
							return newDate;
						})()}
					/>
				</div>
				<div class="grid text-sm grid-cols-7 gap-1 pb-1 border-b border-gray-300">
					<For each={week}>{(day) => <div class="text-center">{day}</div>}</For>
				</div>
				<div class="grid grid-cols-7 gap-1 pt-1 pb-2">
					<For each={calendar()}>
						{(day) => {
							const current = createMemo(
								() =>
									day.getDate() === date().getDate() &&
									day.getMonth() === date().getMonth(),
							);

							return (
								<button
									type="button"
									tabindex="-1"
									onClick={() => {
										setDate(day);
										setShow(false);
									}}
									class="text-center size-8 rounded-lg"
									classList={{
										"text-gray-400": day.getMonth() !== date().getMonth(),
										"hover:bg-gray-500/50": !current(),
										"bg-indigo-500 text-white": current(),
									}}
								>
									{day.getDate()}
								</button>
							);
						}}
					</For>
				</div>
			</div>
		);
	};

	const Page = (props: { type: page }) => {
		const year = createMemo(() => date().getFullYear());
		const start = createMemo(() => year() - (year() % 10));
		const decade = createMemo(() =>
			Array.from({ length: 12 }, (_, i) => start() + i),
		);

		const months = [
			"Jan",
			"Feb",
			"Mar",
			"Apr",
			"May",
			"Jun",
			"Jul",
			"Aug",
			"Sep",
			"Oct",
			"Nov",
			"Dec",
		];

		const prev = createMemo(() =>
			props.type === "year" ? year() - 10 : year() - 1,
		);
		const next = createMemo(() =>
			props.type === "year" ? year() + 10 : year() + 1,
		);

		return (
			<div>
				<div class="flex justify-between py-1 border-b border-gray-300">
					<Switcher
						date={(() => {
							const newDate = new Date(date());
							newDate.setFullYear(prev());
							return newDate;
						})()}
					/>
					<button
						type="button"
						tabindex="-1"
						onClick={() => setPage("day")}
						class="py-1 px-2 rounded-lg hover:bg-gray-500/50"
					>
						{props.type === "year" ? `${start()} - ${start() + 9}` : year()}
					</button>
					<Switcher
						date={(() => {
							const newDate = new Date(date());
							newDate.setFullYear(next());
							return newDate;
						})()}
					/>
				</div>
				<div class="grid grid-cols-4 gap-1 pt-1 pb-2">
					{(props.type === "year" ? decade() : months).map((value, index) => {
						const current = createMemo(() =>
							props.type === "year"
								? value === year()
								: index === date().getMonth(),
						);

						return (
							<button
								type="button"
								tabindex="-1"
								onClick={() => {
									if (props.type === "year") {
										const newDate = new Date(date());
										newDate.setFullYear(value as number);
										setDate(newDate);
										setPage("month");
									} else {
										const newDate = new Date(date());
										newDate.setMonth(index);
										setDate(newDate);
										setPage("day");
									}
								}}
								class="text-center p-1 rounded-lg"
								classList={{
									"hover:bg-gray-500/50": !current(),
									"bg-indigo-500 text-white": current(),
								}}
							>
								{value}
							</button>
						);
					})}
				</div>
			</div>
		);
	};

	let ref!: HTMLDivElement;

	return (
		<div
			class="relative"
			use:clickOut={() => {
				setShow(false);
			}}
			ref={ref}
		>
			<Input
				type="text"
				value={value()}
				pattern="^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])$"
				onInput={(event) => {
					const input = event.target.value;
					if (/^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])$/.test(input)) {
						const newDate = new Date(input);
						newDate.setHours(
							date().getHours(),
							date().getMinutes(),
							date().getSeconds(),
							date().getMilliseconds(),
						);
						if (!Number.isNaN(newDate.getTime())) {
							setDate(newDate);
						}
					}
				}}
				onFocus={() => {
					setPage("day");
					setShow(true);
				}}
				onFocusOut={(event) => {
					if (
						event.relatedTarget &&
						!ref.contains(event.relatedTarget as HTMLElement)
					) {
						setShow(false);
					}
				}}
				placeholder="YYYY-MM-DD"
				suffixContent={<Icon icon={faCalendar} class="text-gray-400" />}
			/>

			<Show when={show()}>
				<Popover>
					<div class="px-4">
						<Show when={page() === "day"} fallback={<Page type={page()} />}>
							<DayPage />
						</Show>
					</div>
				</Popover>
			</Show>
		</div>
	);
};

export default DatePicker;
