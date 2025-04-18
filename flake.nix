{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
    surrealdb-gh.url = "github:surrealdb/surrealdb/v2.1.4";
  };
  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      perSystem =
        {
          config,
          self',
          pkgs,
          lib,
          system,
          ...
        }:
        let
          enableAndroid = false;

          androidSdk =
            let
              androidComposition = pkgs.androidenv.composeAndroidPackages {
                cmdLineToolsVersion = "13.0";
                # INFO: toolsVersion is unused because the tools package is deprecated
                # toolsVersion = "26.1.1";
                platformToolsVersion = "35.0.2";
                buildToolsVersions = [
                  "34.0.0"
                  "35.0.0"
                ];
                includeEmulator = true;
                emulatorVersion = "35.1.4";
                platformVersions = [
                  "33"
                ];
                includeSources = false;
                includeSystemImages = true;
                systemImageTypes = [ "google_apis_playstore" ];
                abiVersions = [
                  "x86_64"
                  # "armeabi-v7a"
                  # "arm64-v8a"
                ];
                cmakeVersions = [ "3.6.4111459" ];
                includeNDK = true;
                ndkVersions = [ "27.0.12077973" ];
                useGoogleAPIs = true;
                useGoogleTVAddOns = false;
                includeExtras = [
                  "extras;google;gcm"
                ];
              };
            in
            androidComposition.androidsdk;

          androidDeps = with pkgs; [
            androidSdk
            openjdk
          ];

          dioxusDeps = with pkgs; [
            atkmm
            cairo
            fontconfig
            fribidi
            gdk-pixbuf
            glib
            glib-networking
            gtk3
            gsettings-desktop-schemas # Doesn't fix appimage bundle issue
            harfbuzz
            freetype
            libdrm
            libGL
            libgpg-error
            libsoup_3
            mesa
            openssl
            wrapGAppsHook
            webkitgtk_4_1
            xdotool
            xorg.libX11
            xorg.libxcb
            zlib
            sqlite
          ];

          runtimeDeps = with pkgs; [
          ];

          buildDeps =
            with pkgs;
            [
              clang
              lld
              lldb
              pkg-config
              rustPlatform.bindgenHook
              stdenv.cc.cc.lib
              (wasm-bindgen-cli.overrideAttrs (oldAttrs: rec {
                version = "0.2.100";
                src = fetchCrate {
                  pname = "wasm-bindgen-cli";
                  version = version;
                  hash = "sha256-3RJzK7mkYFrs7C/WkhW9Rr4LdP5ofb2FdYGz1P7Uxog=";
                };

                cargoDeps = rustPlatform.fetchCargoVendor {
                  inherit src;
                  inherit (src) pname version;
                  hash = "sha256-qsO12332HSjWCVKtf1cUePWWb9IdYUmT+8OPj/XP2WE=";
                };
              }))
            ]
            ++ dioxusDeps
            ++ (if enableAndroid then androidDeps else [ ]);

          devDeps =
            with pkgs;
            [
              # Libraries and programs needed for dev work; included in dev shell
              # NOT included in the nix build operation
              bacon
              bashInteractive
              bunyan-rs
              cargo-deny
              cargo-edit
              cargo-expand
              cargo-msrv
              cargo-nextest
              cargo-watch
              (cargo-whatfeatures.overrideAttrs (oldAttrs: rec {
                version = "0.9.13";
                src = fetchCrate {
                  pname = "cargo-whatfeatures";
                  version = "${version}";
                  hash = "sha256-Nbyr7u47c6nImzYJvPVLfbqgDvzyXqR1C1tOLximuHU=";
                };

                cargoDeps = rustPlatform.fetchCargoVendor {
                  inherit src;
                  inherit (src) pname version;
                  hash = "sha256-p95aYXsZM9xwP/OHEFwq4vRiXoO1n1M0X3TNbleH+Zw=";
                };
              }))
              dioxus-cli
              gdb
              just
              nushell
              panamax
              tailwindcss
              zellij
            ]
            ++ [
              inputs.surrealdb-gh.packages.${system}.default
            ];

          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          msrv = cargoToml.package.rust-version;

          rustPackage =
            features:
            (pkgs.makeRustPlatform {
              cargo = pkgs.rust-bin.stable.latest.minimal;
              rustc = pkgs.rust-bin.stable.latest.minimal;
            }).buildRustPackage
              {
                inherit (cargoToml.package) name version;
                src = ./.;
                cargoLock.lockFile = ./Cargo.lock;
                buildFeatures = features;
                buildInputs = runtimeDeps;
                nativeBuildInputs = buildDeps;
                # Uncomment if your cargo tests require networking or otherwise
                # don't play nicely with the nix build sandbox:
                # doCheck = false;
              };

          ldpath =
            with pkgs;
            [
              stdenv.cc.cc.lib
            ]
            ++ dioxusDeps;

          mkDevShell =
            rustc:
            pkgs.mkShell {
              shellHook = ''
                # TODO: figure out if it's possible to remove this or allow a user's preferred shell
                exec env SHELL=${pkgs.bashInteractive}/bin/bash zellij --layout ./zellij_layout.kdl
              '';
              LD_LIBRARY_PATH = lib.makeLibraryPath ldpath;

              ANDROID_HOME = if enableAndroid then "${androidSdk}/libexec/android-sdk" else "";
              ANDROID_NDK_HOME = if enableAndroid then "${androidSdk}/libexec/android-sdk/ndk-bundle" else "";

              GIO_MODULE_DIR = "${pkgs.glib-networking}/lib/gio/modules/";

              RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps ++ devDeps ++ [ rustc ];
            };

          rustTargets = [
            "x86_64-unknown-linux-gnu"
            "x86_64-linux-android"
            "aarch64-linux-android"
            "wasm32-unknown-unknown"
          ];

          rustExtensions = [
            "rust-analyzer"
            "rust-src"
          ];
        in
        {

          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
            config = {
              allowUnfreePredicate =
                pkg:
                builtins.elem (lib.getName pkg) (
                  [
                    "surrealdb"
                  ]
                  ++ (
                    if enableAndroid then
                      [
                        "android-sdk-tools"
                        "android-sdk-cmdline-tools"
                      ]
                    else
                      [ ]
                  )
                );
              android_sdk.accept_license = true;
            };
          };

          packages.default = self'.packages.base;
          devShells.default = self'.devShells.stable;

          packages.base = (rustPackage "");
          packages.bunyan = (rustPackage "bunyan");
          packages.tokio-console = (rustPackage "tokio-console");

          devShells.nightly = (
            mkDevShell (
              pkgs.rust-bin.selectLatestNightlyWith (
                toolchain:
                toolchain.default.override {
                  extensions = rustExtensions;
                  targets = rustTargets;
                }
              )
            )
          );
          devShells.stable = (
            mkDevShell (
              pkgs.rust-bin.stable.latest.default.override {
                extensions = rustExtensions;
                targets = rustTargets;
              }
            )
          );
          devShells.msrv = (
            mkDevShell (
              pkgs.rust-bin.stable.${msrv}.default.override {
                extensions = rustExtensions;
                targets = rustTargets;
              }
            )
          );
        };
    };
}
