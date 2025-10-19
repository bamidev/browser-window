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
          pname = "browser-window";
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
          name = "terminal-example";
          type = "app";
          program = "${browser-window}/bin/terminal-example";
        };

        devShells = {
          default = pkgs.mkShell {
            packages = with pkgs; [
              llvmPackages_21.libcxxClang
            ];
          };

          cef = pkgs.mkShell {
            packages = with pkgs; [
              cmake
              llvmPackages_21.libcxxClang
            ];

            shellHook = ''
              if [ ! -d cef ]; then
                echo Preparing CEF... 
                ./get-cef.sh
              fi

              export CEF_PATH="cef/$(ls cef)"
            '';
          };
        };
      });
}
