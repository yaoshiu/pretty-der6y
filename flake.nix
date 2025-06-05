{
  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ self.overlays.default ];
        };
      in
      {
        devShells.default = pkgs.callPackage ./shell.nix { };
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
