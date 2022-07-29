{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs:
    with inputs;
      flake-utils.lib.eachDefaultSystem (
        system: let
          overlays = [
            (import rust-overlay)
          ];

          pkgs = (import nixpkgs) {
            inherit system overlays;
          };

          inherit (pkgs) lib;

          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

          sqlFilter = path: _type: builtins.match ".*sql$" path != null;
          teraFilter = path: _type: builtins.match ".*tera$" path != null;
          pngFilter = path: _type: builtins.match ".*png$" path != null;
          webmanifestFilter = path: _type: builtins.match ".*webmanifest$" path != null;
          cssFilter = path: _type: builtins.match ".*css$" path != null;
          scssFilter = path: _type: builtins.match ".*scss$" path != null;
          customOrCargo = path: type:
            (sqlFilter path type)
            || (pngFilter path type)
            || (webmanifestFilter path type)
            || (cssFilter path type)
            || (scssFilter path type)
            || (teraFilter path type)
            || (craneLib.filterCargoSources path type);

          pname = "satounki-workspace";
          version = "0.1.0";

          commonArgs = {
            inherit pname version;
            nativeBuildInputs = with pkgs; [
              pkg-config
            ];
            doCheck = false;
            buildInputs = with pkgs; [
              openssl
              sqlite
            ];
          };

          cargoArtifacts = craneLib.buildDepsOnly (commonArgs
            // {
              src = craneLib.cleanCargoSource (craneLib.path ./.);
            });

          satounki = craneLib.buildPackage (commonArgs
            // {
              inherit cargoArtifacts;
              src = lib.cleanSourceWith {
                src = ./.;
                filter = customOrCargo;
              };
              postInstall = ''
                cp -r ./api/templates $out/bin
              '';
            });

          terraform-provider-satounkiplatform = pkgs.buildGoModule {
            pname = "terraform-provider-satounkiplatform";
            version = "0.1";
            preBuild = ''
              sed -i 's|../../satounki-platform-go|./satounki-platform-go|' go.mod
            '';
            src = pkgs.symlinkJoin {
              name = "source-with-local-deps";
              paths = [./terraform-providers/satounkiplatform ./.];
            };
            vendorHash = "sha256-xP8g5IhY1Ia8BmPiIQMEaiLxEMKL96bNhrA+U+X4vcQ=";
          };

          terraform-provider-satounki = pkgs.buildGoModule {
            pname = "terraform-provider-satounki";
            version = "0.1";
            preBuild = ''
              sed -i 's|../../satounki-go|./satounki-go|' go.mod
            '';
            src = pkgs.symlinkJoin {
              name = "source-with-local-deps";
              paths = [./terraform-providers/satounki ./.];
            };
            vendorHash = "sha256-IFE28HNVKg+2XrmNGWXyADWkMgnVG0D90d+nv2o6jIY=";
          };
        in {
          devShells = flake-utils.lib.flattenTree {
            default = import ./shell.nix {inherit pkgs;};
          };

          packages = flake-utils.lib.flattenTree rec {
            all = pkgs.symlinkJoin {
              name = "all";
              paths = [
                satounki
                terraform-provider-satounki
                terraform-provider-satounkiplatform
              ];
            };
            default = all;
          };
        }
      );
}
