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

import { createSignal } from "solid-js";
import { Background } from "@components/Skeleton";
import Button from "@components/Button";
import Input from "@components/Input";
import Icon from "@components/Icon";
import { useLogined } from "./App";
import { commands } from "@helpers/bindings";
import { faLock, faUser } from "@fortawesome/free-solid-svg-icons";
import { useLogger } from "@components/Logger";
import TwoColumn from "@layouts/TwoColumn";

const image = "https://s2.loli.net/2024/08/26/gZOYyS7aECHuF9z.webp";

export default function Login() {
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [, setLogined] = useLogined();
  const [pending, setPending] = createSignal(false);
  const logger = useLogger();

  return (
    <TwoColumn
      first={
        <Background src={image}>
          <div class="h-full flex items-center justify-center bg-black/30">
            <h1 class="text-4xl font-bold text-white">Pretty Der6y</h1>
          </div>
        </Background>
      }
      second={
        <div class="max-w-sm flex flex-col justify-center w-full h-full p-8">
          <h2 class="text-3xl font-bold text-gray-800 mb-6">LOGIN</h2>
          <form
            class="space-y-6"
            onSubmit={(event) => {
              event.preventDefault();
              setPending(true);
              commands
                .login(username(), password())
                .then(() => setLogined(true))
                .catch((error) => {
                  logger?.error(`Error logging in: ${error}`);
                })
                .finally(() => setPending(false));
            }}
          >
            <Input
              name="username"
              autocomplete="on"
              value={username()}
              placeholder="Username"
              type="tel"
              maxLength={11}
              required={true}
              onInvalid={(event) =>
                event.currentTarget.setCustomValidity("Username is required")
              }
              onInput={(event) => {
                setUsername(event.target.value);
                event.currentTarget.setCustomValidity("");
              }}
              prefixContent={<Icon icon={faUser} classes="text-gray-400" />}
            />
            <Input
              name="password"
              value={password()}
              type="password"
              placeholder="Password"
              maxLength={16}
              required={true}
              onInvalid={(event) =>
                event.currentTarget.setCustomValidity("Password is required")
              }
              onInput={(event) => {
                setPassword(event.target.value);
                event.currentTarget.setCustomValidity("");
              }}
              prefixContent={<Icon icon={faLock} classes="text-gray-400" />}
            />
            <Button type="submit" disabled={pending()}>
              Login
            </Button>
          </form>
        </div>
      }
    />
  );
}
