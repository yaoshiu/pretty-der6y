/*
    Pretty Der6y - A third-party running data upload client.
    Copyright (C) 2024  Fay Ash

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

import {
  type IconLookup,
  type IconName,
  type IconParams,
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
export default function Icon(props: IconParams & {
  icon: IconName | IconLookup;
}): JSX.Element {
  const [local, others] = splitProps(props, ["icon"]);
  const iconNode = icon(local.icon, others).node[0];

  return iconNode;
}
