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
