# Pretty Der6y

![image](./docs/images/image.png)

A third-party running data upload client.

## Installation

You can find the latest release [here](https://github.com/yaoshiu/pretty-der6y/releases/latest
).

### Build From Source

You can build the project from source by the following steps.

#### Prerequistes

- Rust toolchain including `rustc` and `cargo`.
- A JavaScript runtime ([bun](bun.sh) recommended).
- For more information, check the [Tauri v2 documentation](https://v2.tauri.app/start/prerequisites/).

#### Build Steps

```bash
git clone https://github.com/yaoshiu/pretty-der6y.git # Clone the registry

cd tauri-app # Move to the directory for the client application

bun install # Install frontend dependencies

bun tauri build # Build the application
```

## Usage

### Custom Route File

The route file is in [GEOJSON](geojson.org) format. Route files for *Pretty Der6y* should contain exactly **ONE** feature with a **SINGLE** `LineString`.

You can create your route in [georoute.io](georoute.io).

Check our example route file [here](./assets/map.geojson).

## Special Thanks To

- The [Tauri](tauri.app) toolkit.

- The [Solid](solidjs.com) framework.

- The [TailwindCSS](tailwindcss.com).

## Image Sources

The background image for the login page is from [Nardack - Pixiv](https://www.pixiv.net/artworks/89657320). Modification and distribution without the permission of the author is prohibited.

## License

This project is licensed under the AGPL-3.0 License.

You can modify and redistribute this software freely, but make sure the source code is available to public.

See the [LICENSE](./LICENSE) file for details.
