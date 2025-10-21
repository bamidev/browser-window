{
  description = "Browser Window flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachSystem flake-utils.lib.allSystems (system:
      let
        lib = nixpkgs.lib;
        pkgs = nixpkgs.legacyPackages.${system};
        buildRustPackage = pkgs.rustPlatform.buildRustPackage;
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;

        browserWindowDefaults = {
          pname = manifest.name;
          version = manifest.version;
          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
          nativeBuildInputs = with pkgs; [
            pkg-config
            rustPlatform.bindgenHook
          ];

          # Install all examples' binaries in the bin folder
          installPhase = with pkgs; ''
            ${coreutils}/bin/mkdir -p $out/bin
            ${coreutils}/bin/cp -r target/${stdenv.targetPlatform.rust.rustcTargetSpec}/release/examples/* $out/bin
          '';
        };

        browserWindowWebkitGtk = buildRustPackage (browserWindowDefaults // {
          buildFeatures = ["webkitgtk"];

          buildInputs = with pkgs; [ zlib ];
          # Populate PKG_CONFIG_PATH because most Rust crates utilize pkg-config to find all system build flags.
          preBuild = with pkgs; ''
            export PKG_CONFIG_PATH=""\
            "${at-spi2-atk.dev}/lib/pkgconfig:"\
            "${cairo.dev}/lib/pkgconfig:"\
            "${gdk-pixbuf.dev}/lib/pkgconfig:"\
            "${glib.dev}/lib/pkgconfig:"\
            "${gtk3.dev}/lib/pkgconfig:"\
            "${harfbuzz.dev}/lib/pkgconfig:"\
            "${libsoup_3.dev}/lib/pkgconfig:"\
            "${pango.dev}/lib/pkgconfig:"\
            "${webkitgtk_4_1.dev}/lib/pkgconfig"
          '';

        });

        cef = pkgs.stdenv.mkDerivation {
          pname = "cef";
          version = "122.1.12";
          outputs = ["out"];

          nativeBuildInputs = with pkgs; [
            xorg.libX11
          ];

          src = fetchTarball {
            url = "https://cef-builds.spotifycdn.com/cef_binary_122.1.12+g6e69d20+chromium-122.0.6261.112_linux64_minimal.tar.bz2";
            sha256 = "sha256:0kqd2yx6xiblnp1davjfy3xfv8q69rd1b6nyir2abprlwn04rhh9";
          };

          buildPhase = with pkgs; ''
            ${coreutils}/bin/mv CMakeLists.txt CMakeLists.txt.old
            ${coreutils}/bin/echo "add_compile_definitions(DCHECK_ALWAYS_ON=1)" > CMakeLists.txt
            ${coreutils}/bin/cat CMakeLists.txt.old >> CMakeLists.txt

            ${cmake}/bin/cmake .
            ${cmake}/bin/cmake --build .
          '';
          installPhase = with pkgs; ''
            ${coreutils}/bin/mkdir -p $out/Release
            ${coreutils}/bin/mkdir -p $out/Resources
            ${coreutils}/bin/cp -r Release $out
            ${coreutils}/bin/cp -r Resources $out

            ${coreutils}/bin/cp -r include $out

            ${coreutils}/bin/mkdir $out/libcef_dll_wrapper
            ${coreutils}/bin/cp libcef_dll_wrapper/libcef_dll_wrapper.a $out/libcef_dll_wrapper
          '';
        };

        # TODO: The CEF derivation needs some work.
        browserWindowCef = buildRustPackage (browserWindowDefaults // {
          buildFeatures = ["cef"];

          buildInputs = with pkgs; [ dbus cups ];
          nativeBuildInputs = [
            cef
          ] ++ (with pkgs; [
            pkg-config
            rustPlatform.bindgenHook
          ]);

          preBuild = with pkgs; ''
            export CEF_PATH="${cef}"
            export PKG_CONFIG_PATH=""\
            "${at-spi2-atk.dev}/lib/pkgconfig:"\
            "${cairo.dev}/lib/pkgconfig:"\
            "${cups.dev}/lib/pkgconfig:"\
            "${dbus.dev}/lib/pkgconfig:"\
            "${glib.dev}/lib/pkgconfig:"\
            "${pango.dev}/lib/pkgconfig"
          '';
        });
      in {
        # The examples are made available through `nix run`
        apps = rec {
          default = terminal-webkitgtk;
          authentication-webkitgtk = {
            name = "authentication-example";
            type = "app";
            program = "${browserWindowWebkitGtk}/bin/authentication";
          };
          terminal-webkitgtk = {
            name = "terminal-example";
            type = "app";
            program = "${browserWindowWebkitGtk}/bin/terminal";
          };
        # TODO: Make the CEF apps available for other supported platforms as well
        } // lib.optionals (system == "x86_64-linux") {
          authentication-cef = {
            name = "authentication";
            type = "app";
            program = "${browserWindowCef}/bin/authentication";
          };
          terminal-cef = {
            name = "terminal";
            type = "app";
            program = "${browserWindowCef}/bin/terminal";
          };
        };

        devShells = rec {
          cef = pkgs.mkShell {
            packages = browserWindowCef.nativeBuildInputs ++ browserWindowCef.buildInputs;
            inputsFrom = browserWindowCef.buildInputs;
            shellHook = browserWindowCef.preBuild;
          };
          default = webkitgtk;
          webkitgtk = pkgs.mkShell {
            packages = browserWindowWebkitGtk.nativeBuildInputs ++ browserWindowWebkitGtk.buildInputs;
            inputsFrom = browserWindowWebkitGtk.buildInputs;
            shellHook = browserWindowWebkitGtk.preBuild;
          };
        };

        # `nix build` can be used to test the build and run the tests
        packages = rec {
          cef = browserWindowCef;
          default = webkitgtk;
          webkitgtk = browserWindowWebkitGtk;
        };
      });
}
