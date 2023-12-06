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
          htmlFilter = path: _type: builtins.match ".*html$" path != null;
          pngFilter = path: _type: builtins.match ".*png$" path != null;
          webmanifestFilter = path: _type: builtins.match ".*webmanifest$" path != null;
          cssFilter = path: _type: builtins.match ".*css$" path != null;
          scssFilter = path: _type: builtins.match ".*scss$" path != null;
          rolescraperFilter = path: _type: builtins.match ".*json$" path != null;
          customOrCargo = path: type:
            (sqlFilter path type)
            || (pngFilter path type)
            || (webmanifestFilter path type)
            || (cssFilter path type)
            || (scssFilter path type)
            || (rolescraperFilter path type)
            || (htmlFilter path type)
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
                cp -r ./rolescraper_*.json $out/bin
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
            vendorHash = "sha256-5McUZeBJuVTQ6ygDXEKkU9HnOc7mQgnwaUo7f7ATCok=";
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
            vendorHash = "sha256-LvS1eFkplj0IaA4YhpzA37TutQ0iYGbs3OoEAdeuMOc=";
          };
        in {
          devShells = flake-utils.lib.flattenTree {
            default = import ./shell.nix {inherit pkgs;};
          };

          packages = flake-utils.lib.flattenTree rec {
            inherit satounki terraform-provider-satounki terraform-provider-satounkiplatform;

            default = all;

            all = pkgs.symlinkJoin {
              name = "all";
              paths = [
                satounki
                terraform-provider-satounki
                terraform-provider-satounkiplatform
              ];
            };
          };
        }
      )
      // {
        overlays = {
          default = _: prev: rec {
            inherit (self.packages.${prev.system}) satounki;
          };
        };

        nixosModules = {
          satounki = {
            imports = [
              ./nixos/satounki.nix
            ];
          };

          nixpkgs.overlays = [
            self.overlays.default
          ];
        };
      };
}
