{ pkgs ? (import <nixpkgs> { }).pkgsCross.mingwW64, fenix ? import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") { } }:

pkgs.pkgsStatic.callPackage
  ({ mkShell, openssl, pkg-config, windows }: mkShell {
    nativeBuildInputs =
      let
        toolchain = with fenix; combine [
          stable.rustc
          stable.cargo
          targets.x86_64-pc-windows-gnu.stable.rust-std
        ]; in
      [
        openssl
        pkg-config
        toolchain
      ];

    buildInputs = [ windows.pthreads ];
  })
{ }
