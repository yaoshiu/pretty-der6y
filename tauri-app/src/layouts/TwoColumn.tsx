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

export function TwoColumn(props: {
  first: JSX.Element;
  second: JSX.Element;
  scrollOnMobile?: boolean;
}) {
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
}

export default TwoColumn;
