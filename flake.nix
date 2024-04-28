{
  inputs = {
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      fenix,
      flake-utils,
      naersk,
      nixpkgs,
      self,
    }:
    {
      overlays.default =
        final: prev:
        let
          inherit (final.hostPlatform) system;
          toolchain =
            with fenix.packages.${system};
            combine [
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
            nativeBuildInputs =
              with final;
              (lib.optional stdenv.isDarwin [ darwin.apple_sdk.frameworks.SystemConfiguration ])
              ++ [ installShellFiles ];
            src = ./.;
            postFixup = ''
              for shell in bash fish zsh; do
                installShellCompletion --cmd pretty-derby --$shell <($out/bin/pretty-derby --completion $shell)
              done
            '';
          };
        };
    }
    // (flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [ self.overlays.default ];
        };
      in
      {
        packages = rec {
          pretty-derby = pkgs.pretty-derby;
          default = pretty-derby;
        };

        devShells.default =
          with pkgs;
          mkShell {
            buildInputs =
              let
                toolchain =
                  with fenix.packages.${system};
                  combine [
                    minimal.cargo
                    minimal.rustc
                  ];
              in
              [
                toolchain
                iconv
              ]
              ++ (lib.optional stdenv.isDarwin [ darwin.apple_sdk.frameworks.SystemConfiguration ]);
          };

        formatter = pkgs.nixfmt-rfc-style;
      }
    ));
}
