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

