import type { Accessor } from "solid-js";

// I really hates doing this.
export default function isDef<T>(
  accessor: Accessor<T | undefined>,
): accessor is Accessor<T> {
  return accessor() !== undefined;
}
