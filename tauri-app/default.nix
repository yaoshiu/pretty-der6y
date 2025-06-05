{
  lib,
  rustPlatform,
  importNpmLock,
  cargo-tauri,
  nodejs,
  pkg-config,
  stdenv,
  wrapGAppsHook4,
  glib-networking,
  openssl,
  webkitgtk_4_1,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "pretty-der6y";
  version = "1.3.0";

  src = ./..;

  patches = [
    ./disable_updater.patch
  ];

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  npmDeps = importNpmLock.buildNodeModules {
    npmRoot = ./.;
    inherit nodejs;
  };

  npmWorkspace = "tauri-app";

  nativeBuildInputs =
    [
      cargo-tauri.hook

      nodejs
      importNpmLock.hooks.linkNodeModulesHook

      pkg-config
    ]
    ++ lib.optionals stdenv.hostPlatform.isLinux [
      wrapGAppsHook4
    ];

  buildInputs = [
    glib-networking
    openssl
    webkitgtk_4_1
  ];

  buildAndTestSubdir = "tauri-app/src-tauri";
})
