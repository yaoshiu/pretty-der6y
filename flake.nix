{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { fenix, flake-utils, naersk, nixpkgs, self }:
    {
      overlays = rec {
        dev = final: prev:
          let
            system = final.hostPlatform.system;
            toolchain = with fenix.packages.${system}; combine [
              minimal.cargo
              minimal.rustc
            ];
            naersk-lib = naersk.lib.${system}.override {
              cargo = toolchain;
              rustc = toolchain;
            };
          in
          {
            pretty-derby = naersk-lib.buildPackage {
              nativeBuildInputs = with final; lib.optional stdenv.isDarwin [
                darwin.apple_sdk.frameworks.SystemConfiguration
              ];
              src = ./.;
            };

            pretty-derby-shell = with final; mkShell {
              buildInputs = [
                toolchain
                iconv
              ] ++ (lib.optional stdenv.isDarwin [
                darwin.apple_sdk.frameworks.SystemConfiguration
              ]);
            };
          };

        default = final: prev: { inherit (dev) pretty-derby; };
      };
    } //
    (flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [ self.overlays.dev ];
        };
      in
      {
        packages = rec {
          pretty-derby = pkgs.pretty-derby;
          default = pretty-derby;
        };


        devShells.default = pkgs.pretty-derby-shell;
      }));
}
