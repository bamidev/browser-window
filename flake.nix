{
  description = "browser-window";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachSystem flake-utils.lib.allSystems (system:
      let
        lib = nixpkgs.lib;
        pkgs = nixpkgs.legacyPackages.${system};
        stdenv = pkgs.stdenv;
        browser-window = stdenv.mkDerivation {
          pname = "stonenet";
          version = "0.0.0";
          src = ./.;
          buildPhase = ''
            ${pkgs.cargo}/bin/cargo build
          '';
          installPhase = ''
            mkdir -p $out/bin
            cp a.out $out/bin
          '';
        };
      in {
        apps.default = {
          name = "browser-window";
          type = "app";
          program = "${browser-window}/bin/stonenetd";
        };
      });
}
