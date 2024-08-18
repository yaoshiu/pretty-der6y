import { createSignal } from "solid-js";
import { Button, Input } from "./Components";

export function Login() {
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");

  return (
    <form
      class="w-full max-w-sm"
      onSubmit={(e) => {
        e.preventDefault();
        console.log(`Username: ${username()}, Password: ${password()}`);
      }}
    >
      <div class="bg-white p-8 rounded-lg shadow-lg space-y-4">
        <Input
          type="tel"
          placeholder="Username"
          maxlength={11}
          value={[username, setUsername]}
          required={true}
        />
        <Input
          type="password"
          placeholder="Password"
          maxlength={16}
          value={[password, setPassword]}
          required={true}
        />
        <Button type="submit" onClick={() => console.log("Clicked")}>
          Login
        </Button>
      </div>
    </form>
  );
}
