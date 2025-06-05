{
  outputs =
    {
      self,
      nixpkgs,
      fenix,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config = {
            allowUnfree = true;
          };
          overlays = [ self.overlays.default ];
        };

        extraTargets = [
          "aarch64-linux-android"
          "armv7-linux-androideabi"
          "i686-linux-android"
          "x86_64-linux-android"
        ];

        rust =
          with (import fenix {
            inherit pkgs system;
          });
          combine (
            [
              stable.toolchain
            ]
            ++ builtins.map (platform: targets.${platform}.stable.rust-std) extraTargets
          );
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            gobject-introspection
            rust
            cargo-tauri # Optional, Only needed if Tauri doesn't work through the traditional way.
            nodejs

            pkgs.android-studio
            glibc
            openjdk
          ];

          buildInputs = with pkgs; [
            at-spi2-atk
            atkmm
            cairo
            gdk-pixbuf
            glib
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            pango
            webkitgtk_4_1
            openssl
          ];
        };

        packages = rec {
          default = pretty-der6y;
          pretty-der6y = pkgs.callPackage ./tauri-app { };
        };
      }
    )
    // {
      overlays = {
        default = final: prev: {
          pretty-der6y = final.callPackage ./tauri-app { };
        };
      };
    };
}
