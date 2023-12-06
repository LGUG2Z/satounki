{
  lib,
  pkgs,
  config,
  ...
}: let
  inherit (lib) types;

  cfg = config.services.satounki;
in {
  options = {
    services.satounki = {
      enable = lib.mkOption {
        description = ''
          Whether to enable the Satounki service.
        '';
        type = types.bool;
        default = false;
      };

      port = lib.mkOption {
        description = ''
          Port to run the Satounki service on.
        '';
        type = types.int;
        default = 8080;
      };

      worker = lib.mkOption {
        description = ''
          Whether to enable the Satounki worker.
        '';
        type = types.bool;
        default = false;
      };

      dataDir = lib.mkOption {
        type = types.str;
        default = "/var/lib/satounki";
        description = lib.mdDoc "The directory where Satounki stores its data files.";
      };

      package = lib.mkOption {
        description = ''
          The package to use.
        '';
        type = types.package;
        default = pkgs.satounki;
      };

      serviceCredentialsFile = lib.mkOption {
        description = ''
          Path to an EnvironmentFile containing required environment
          variables:

          - GOOGLE_CLOUD_ID
          - GOOGLE_CLIENT_SECRET
          - SLACK_CLIENT_ID
          - SLACK_CLIENT_SECRET
          - SLACK_SIGNING_SECRET
          - SATOUNKIPLATFORM_API_TOKEN
        '';
        type = types.nullOr types.path;
        default = null;
      };

      workerCredentialsFile = lib.mkOption {
        description = ''
          Path to an EnvironmentFile containing required environment
          variables:

          - COMPANY_WORKER_KEY
        '';
        type = types.nullOr types.path;
        default = null;
      };

      workerConfigurationFile = lib.mkOption {
        description = ''
          Path to a YAML configuration file containing credentials for services managed by Satounki
        '';
        type = types.nullOr types.path;
        default = null;
      };

      user = lib.mkOption {
        description = ''
          The group under which Satounki runs.
        '';
        type = types.str;
        default = "satounki";
      };

      group = lib.mkOption {
        description = ''
          The user under which Satounki runs.
        '';
        type = types.str;
        default = "satounki";
      };

      settings = {
        databaseUrl = lib.mkOption {
          type = types.str;
          default = "${cfg.dataDir}/satounki.db";
          example = "${cfg.dataDir}/satounki.db";
          description = lib.mdDoc "Where the Satounki database will be stored.";
        };

        hostname = lib.mkOption {
          type = types.str;
          default = "";
          example = "team.satounki.com";
          description = lib.mdDoc "Hostname under which Satounki is served.";
        };

        port = lib.mkOption {
          type = types.int;
          default = 8080;
          example = 8080;
          description = lib.mdDoc "Port under which Satounki is served.";
        };

        companyDomain = lib.mkOption {
          type = types.str;
          default = "";
          example = "satounki.com";
          description = lib.mdDoc "Domain of the company Satounk is deployed for.";
        };
      };
    };
  };

  config = {
    assertions = lib.mkIf cfg.enable [
      {
        assertion = !lib.isStorePath cfg.serviceCredentialsFile;
        message = ''
          <option>services.satounki.serviceCredentialsFile</option> points to a path in the Nix store. The Nix store is globally readable.

          You should use a quoted absolute path to prevent this.
        '';
      }
      {
        assertion = !lib.isStorePath cfg.workerCredentialsFile;
        message = ''
          <option>services.satounki.workerCredentialsFile</option> points to a path in the Nix store. The Nix store is globally readable.

          You should use a quoted absolute path to prevent this.
        '';
      }
      {
        assertion = !lib.isStorePath cfg.workerConfigurationFile;
        message = ''
          <option>services.satounki.workerConfigurationFile</option> points to a path in the Nix store. The Nix store is globally readable.

          You should use a quoted absolute path to prevent this.
        '';
      }
    ];

    systemd.services.satounki = lib.mkIf cfg.enable {
      description = "Satounki API";
      wantedBy = ["multi-user.target"];
      after = ["network.target"];
      environment = {
        DATABASE_URL = cfg.settings.databaseUrl;
        SATOUNKI_URL = "https://${cfg.settings.hostname}";
        PORT = builtins.toString cfg.settings.port;
      };
      serviceConfig = {
        ExecStart = "${cfg.package}/bin/api";
        EnvironmentFile = cfg.serviceCredentialsFile;
        StateDirectory = "satounki";
        DynamicUser = true;
        User = cfg.user;
        Group = cfg.group;
        ProtectHome = true;
        ProtectHostname = true;
        ProtectKernelLogs = true;
        ProtectKernelModules = true;
        ProtectKernelTunables = true;
        ProtectProc = "invisible";
        ProtectSystem = "strict";
        Restart = "on-failure";
        RestrictAddressFamilies = ["AF_INET" "AF_INET6" "AF_UNIX"];
        RestrictNamespaces = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
      };
    };

    systemd.services.satounki-worker = lib.mkIf cfg.worker {
      description = "Satounki Worker";
      wantedBy = ["multi-user.target"];
      after = ["network.target" "satounki.service"];
      environment = {
        COMPANY_DOMAIN = cfg.settings.companyDomain;
        PORT = builtins.toString cfg.settings.port;
      };
      serviceConfig = {
        LoadCredential = [
          "config.yaml:${cfg.workerConfigurationFile}"
        ];
        ExecStart = "${cfg.package}/bin/client --config %d/config.yaml";
        EnvironmentFile = cfg.workerCredentialsFile;
        StateDirectory = "satounki";
        DynamicUser = true;
        User = cfg.user;
        Group = cfg.group;
        ProtectHome = true;
        ProtectHostname = true;
        ProtectKernelLogs = true;
        ProtectKernelModules = true;
        ProtectKernelTunables = true;
        ProtectProc = "invisible";
        ProtectSystem = "strict";
        Restart = "on-failure";
        RestrictAddressFamilies = ["AF_INET" "AF_INET6" "AF_UNIX"];
        RestrictNamespaces = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
      };
    };
  };
}
