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
    let
      devOverlay = final: prev: rec {
        darwinOptions = final.lib.optionalAttrsfinalal.stdenv.isDarwin {
          buildInputs = with final.darwin.apple_sdk.frameworks; [
            SystemConfiguration
          ];
        };
        system = final.hostPlatform.system;
        toolchain = with fenix.packages.${system}; combine [
          minimal.cargo
          minimal.rustc
        ];
        naersk-lib = naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        };
        pretty-derby = naersk-lib.buildPackage (darwinOptions // {
          src = ./.;
        });
      };
    in
    {
      overlays.default = final: prev: {
        inherit (devOverlay) pretty-derby;
      };
    } //
    (flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [ devOverlay ];
        };
      in
      {
        packages = rec {
          pretty-derby = pkgs.pretty-derby;
          default = pretty-derby;
        };


        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
          ];
        };
      }));
}
