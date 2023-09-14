{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix-flake = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { fenix-flake, flake-utils, naersk, nixpkgs, self }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };
        fenix = fenix-flake.packages.${system};
        common-toolchain = with fenix; [
          minimal.cargo
          minimal.rustc
        ];
        mkToolchain = target: with fenix;
          combine ([ targets.${target}.latest.rust-std ] ++ common-toolchain);
        mkNaersk = toolchain: naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        };
      in
      rec {
        packages.default =
          let
            toolchain = fenix.combine common-toolchain;
          in
          (mkNaersk toolchain).buildPackage {
            nativeBuildInputs = with pkgs; [
              pkg-config
            ];

            buildInputs = with pkgs; [
              openssl
            ];
            src = ./.;
          };

        packages.x86_64-unknown-linux-musl =
          let
            target = "x86_64-unknown-linux-musl";
            toolchain = mkToolchain target;
          in
          (mkNaersk toolchain).buildPackage {
            src = ./.;
            strictDeps = true;

            depsBuildBuild = with pkgs.pkgsStatic; [
              stdenv.cc
            ];

            nativeBuildInputs = with pkgs; [
              pkg-config
            ];

            buildInputs = with pkgs.pkgsStatic; [
              openssl
            ];

            CARGO_BUILD_TARGET = target;
            CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
          };

        packages.x86_64-pc-windows-gnu =
          let
            target = "x86_64-pc-windows-gnu";
            toolchain = mkToolchain target;
          in
          (mkNaersk toolchain).buildPackage {
            src = ./.;
            strictDeps = true;

            depsBuildBuild = with pkgs.pkgsCross.mingwW64; [
              stdenv.cc
              windows.pthreads
            ];

            nativeBuildInputs = with pkgs; [
              pkg-config
              openssl
            ];

            CARGO_BUILD_TARGET = target;
          };

        packages.aarch64-unknown-linux-musl =
          let
            target = "aarch64-unknown-linux-musl";
            toolchain = mkToolchain target;
            targetCc = pkgs.pkgsCross.aarch64-multiplatform-musl.pkgsStatic.stdenv.cc;
          in
          (mkNaersk toolchain).buildPackage {
            src = ./.;
            strictDeps = true;

            depsBuildBuild = [
              targetCc
            ];

            nativeBuildInputs = with pkgs; [
              pkg-config
            ];

            buildInputs = with pkgs.pkgsCross.aarch64-multiplatform-musl.pkgsStatic; [
              openssl
            ];

            CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER = "rust-lld";
            CARGO_BUILD_TARGET = target;
            CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
          };

        devShells.default =
          let
            toolchain = common-toolchain;
          in
          pkgs.mkShell {
            nativeBuildInputs = with pkgs;
              [
                toolchain
                pkg-config
                openssl
              ];
          };

        overlays.default = (self: super: {
          pretty-derby = packages.default;
        });
      });
}

