{pkgs ? import (fetchTarball "https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz") {}}:
with pkgs; let
  curlylint = with python39Packages;
    buildPythonPackage rec {
      pname = "curlylint";
      version = "0.13.1";

      src = fetchPypi {
        inherit pname version;
        sha256 = "008b9d160f3920404ac12efb05c0a39e209cb972f9aafd956b79c5f4e2162752";
      };

      pythonRelaxDeps = true;
      nativeBuildInputs = [
        pythonRelaxDepsHook
      ];

      propagatedBuildInputs = [
        pathspec
        click
        toml
        parsy
        attrs
      ];

      doCheck = false;

      meta = with lib; {
        homepage = "https://github.com/thibaudcolas/curlylint";
        description = "Experimental HTML templates linting for Jinja, Nunjucks, Django templates, Twig, Liquid";
        license = licenses.mit;
      };
    };

  PROJECT_ROOT = builtins.getEnv "PWD";
  yaml = pkgs.formats.yaml {};

  process_compose_config = yaml.generate "process-compose.yaml" {
    version = "0.5";
    processes = {
      api = {
        command = "${pkgs.just}/bin/just api";
        readiness_probe.http_get = {
          host = "127.0.0.1";
          scheme = "http";
          path = "/health";
          port = 8080;
        };
      };
      worker = {
        command = "${pkgs.just}/bin/just worker";
        depends_on.api.condition = "process_healthy";
      };
      tunnel = {
        command = "${pkgs.just}/bin/just tunnel";
        disabled = true;
      };
    };
  };

  rust-bin-from-toolchain = rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in
  mkShell {
    name = "satounki";

    PC_PORT_NUM = 9955;

    shellHook = ''
      ${pkgs.coreutils}/bin/ln -sf "${process_compose_config}" "${PROJECT_ROOT}/process-compose.yaml"
    '';

    buildInputs =
      [
        alejandra
        bacon
        cloudflared
        curlylint
        diesel-cli
        djhtml
        go
        golangci-lint
        google-cloud-sdk
        httpie
        jq
        just
        nodePackages_latest.prettier
        nodePackages_latest.quicktype
        nodePackages_latest.typescript
        nodejs_20
        openssl.dev
        overmind
        pkg-config
        process-compose
        rust-bin.nightly.latest.rustfmt
        rust-bin-from-toolchain
        sd
        sops
        sqlite
        sqlitebrowser
        ssh-to-age
        terraform
        yq-go
      ]
      # maybe needed for darwin? don't have a macbook to test
      ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
        libiconv
        darwin.apple_sdk.frameworks.Security
      ];
  }
