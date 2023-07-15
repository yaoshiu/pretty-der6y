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
        toolchain = with fenix; combine [
          stable.cargo
          stable.rustc
        ];
      in
      rec {
        packages.default =

          (naersk.lib.${system}.override {
            cargo = toolchain;
            rustc = toolchain;
          }).buildPackage {
            nativeBuildInputs = with pkgs; [
              pkg-config
              openssl
            ];
            src = ./.;
          };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            toolchain
            pkg-config
            openssl
          ];
        };

        devShells.x86_64-windows = pkgs.pkgsCross.mingwW64.callPackage
          ({ mkShell, openssl, pkg-config, windows }:
            let toolchain = with fenix; combine [
              stable.rustc
              stable.cargo
              targets.x86_64-pc-windows-gnu.stable.rust-std
            ]; in
            mkShell {
              nativeBuildInputs = [
                openssl
                pkg-config
                toolchain
              ];

              buildInputs = [ windows.pthreads ];
            })
          { };

        overlays.default = (self: super: {
          pretty-derby = packages.default;
        });
      });
}

