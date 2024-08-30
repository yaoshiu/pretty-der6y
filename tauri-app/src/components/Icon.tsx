import {
	type IconName,
	type IconLookup,
	icon,
} from "@fortawesome/fontawesome-svg-core";
import { type JSX, splitProps } from "solid-js";

/**
 * Renders an icon component.
 *
 * @component
 * @param {JSX.HTMLAttributes<HTMLSpanElement>} props - The props for the icon component.
 * @param {IconName | IconLookup} props.icon - The name or lookup object for the icon.
 * @returns {JSX.Element} The rendered icon component.
 */
export const Icon = (
	props: JSX.HTMLAttributes<HTMLSpanElement> & {
		icon: IconName | IconLookup;
	},
) => {
	const [local, others] = splitProps(props, ["icon"]);
	const iconHTML = icon(local.icon).html[0];

	return <span {...others} innerHTML={iconHTML} />;
};

export default Icon;
