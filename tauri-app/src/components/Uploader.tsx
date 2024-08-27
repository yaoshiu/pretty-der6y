import {
  createSignal,
  Show,
  splitProps,
  type JSX,
  type Signal,
} from "solid-js";
import { useLogger } from "./Logger";

const Uploader = (
  props: JSX.InputHTMLAttributes<HTMLInputElement> & {
    file: Signal<File | null>;
  }
) => {
  const [local, others] = splitProps(props, ["file"]);

  const [file, setFile] = local.file;
  const [dragging, setDragging] = createSignal(false);

  const logger = useLogger();

  const validateFileType = (file: File) => {
    if (props.accept) {
      const validTypes = props.accept.split(",").map((type) => type.trim());
      const isValid = validTypes.some((type) => {
        if (type.startsWith(".")) {
          return file.name.endsWith(type);
        }
        return (
          file.type === type || file.type.startsWith(type.replace("*", ""))
        );
      });
      return isValid;
    }
    return true;
  };

  function onChange(event: Event) {
    const target = event.target as HTMLInputElement;
    if (target.files?.length) {
      setFile(target.files[0]);
    }
  }

  function onDrop(event: DragEvent) {
    event.preventDefault();
    setDragging(false);
    if (event.dataTransfer?.files.length) {
      const file = event.dataTransfer.files[0];
      if (validateFileType(file)) {
        setFile(event.dataTransfer.files[0]);
      } else {
        logger?.error("Invalid file type!");
      }
    }
  }

  function onDragOver(event: DragEvent) {
    event.preventDefault();
    setDragging(true);
  }

  function handleDragLeave() {
    setDragging(false);
  }

  return (
    <div
      class="flex items-center justify-center border-2 border-dashed rounded-lg p-6 cursor-pointer transition"
      classList={{
        "border-blue-indigo bg-indigo-50": dragging(),
        "border-gray-300": !dragging(),
      }}
      onDragOver={onDragOver}
      onDragLeave={handleDragLeave}
      onDrop={onDrop}
    >
      <label class="cursor-pointer text-center text-gray-600">
        <Show
          when={file()}
          fallback={
            <span class="">Drag & Drop a file here or click to select</span>
          }
        >
          <span class="">{file()?.name}</span>
        </Show>
        <input {...others} type="file" class="hidden" onChange={onChange} />
      </label>
    </div>
  );
};

export default Uploader;
