let
  nixpkgs = import <nixpkgs> {};
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "wall-env";
    buildInputs = [
      pkgconfig
      xorg.libX11.dev
      nix
    ];
  }
