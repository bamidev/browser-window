{
  description = "Browser Window flake for testing and development.";

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

          # Install all examples' binaries in the bin folder, and the seperate executable as well
          installPhase = with pkgs; ''
            ${coreutils}/bin/mkdir -p $out/bin
            ${coreutils}/bin/cp target/${stdenv.targetPlatform.rust.rustcTargetSpec}/release/examples/authentication $out/bin
            ${coreutils}/bin/cp target/${stdenv.targetPlatform.rust.rustcTargetSpec}/release/examples/terminal $out/bin
          '';
        };

        browserWindowWebkitGtk = buildRustPackage (browserWindowDefaults // {
          buildFeatures = ["webkitgtk" "no-gui-tests"];

          buildInputs = with pkgs; [
            at-spi2-atk
            cairo
            gdk-pixbuf
            glib
            gtk3
            harfbuzz
            libsoup_3
            pango
            webkitgtk_4_1
            zlib
          ];
        });

        cef = pkgs.stdenv.mkDerivation rec {
          pname = "cef";
          version = "122.1.12";
          outputs = ["out"];

          src = fetchTarball {
            url = "https://cef-builds.spotifycdn.com/cef_binary_122.1.12+g6e69d20+chromium-122.0.6261.112_linux64_minimal.tar.bz2";
            sha256 = "sha256:0kqd2yx6xiblnp1davjfy3xfv8q69rd1b6nyir2abprlwn04rhh9";
          };

          buildInputs = with pkgs; [
            alsa-lib
            at-spi2-atk
            cairo
            cups
            dbus
            expat
            libdrm
            harfbuzz
            glib
            gtk3
            libgbm
            libGL
            libxkbcommon
            nspr
            nss_3_115
            pango
            xorg.libXcomposite
            xorg.libXdamage
            xorg.libXext
            xorg.libXfixes
            xorg.libXrandr
            xorg.libxcb
            xorg.libX11
            xorg.xorgproto
          ];

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
            #${coreutils}/bin/cp -r Resources $out
            # The resources need to live in the same directory as libcef.so,
            # so lets put everything in the Release folder then.
            ${coreutils}/bin/cp -r Resources/* $out/Release

            ${coreutils}/bin/cp -r include $out

            ${coreutils}/bin/mkdir $out/libcef_dll_wrapper
            ${coreutils}/bin/cp libcef_dll_wrapper/libcef_dll_wrapper.a $out/libcef_dll_wrapper
          '';

          fixupPhase = ''
            # Patch libcef.so because it has been precompiled
            patchelf --set-rpath "${pkgs.lib.makeLibraryPath buildInputs}" $out/Release/libcef.so
          '';
        };

        browserWindowCef = buildRustPackage (browserWindowDefaults // {
          buildFeatures = ["cef" "no-gui-tests"];

          buildInputs = cef.buildInputs;
          nativeBuildInputs = [
            cef
          ] ++ (with pkgs; [
            pkg-config
            rustPlatform.bindgenHook
          ]);

          env = {
            CEF_PATH = "${cef}";
          };

          installPhase = browserWindowDefaults.installPhase + (with pkgs; ''
            ${coreutils}/bin/cp target/${stdenv.targetPlatform.rust.rustcTargetSpec}/release/browser-window-se $out/bin
          '');
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

            shellHook = ''
              export CEF_PATH="${browserWindowCef.CEF_PATH}"
            '';
          };
          default = webkitgtk;
          webkitgtk = pkgs.mkShell {
            packages = browserWindowWebkitGtk.nativeBuildInputs ++ browserWindowWebkitGtk.buildInputs;
            inputsFrom = browserWindowWebkitGtk.buildInputs;
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
