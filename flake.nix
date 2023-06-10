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
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };
        toolchain = with fenix.packages.${system}; combine [
          stable.cargo
          stable.rustc
        ];
      in
      {
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
      });
}

