{ pkgs, config, lib, ... }:
with lib;
let
  serviceConfig = config.services.vf-backend;
in
{
  options.services.kraken = {
    enable = mkEnableOption "Kraken graphql sqlite backend";
    stateDir = mkOption {
      type = types.str;
      default = "/var/lib/kraken";
      example = "/var/lib/kraken";
      description = ''
        location of folder for the application dynamic changes
      '';
    };
    package = mkOption {
      type = types.package;
      description = "package to run the instance with";
    };
    logLevel = mkOption {
      type = types.str;
      default = "info";
      example = "info";
      description = ''
        log level you want for the web server
      '';
    };
    instances = mkOption {
      description = "instances of the union service to run. Typically one for each client";
      default = { };
      type = types.attrsOf (types.submodule {
        options = {
          dbName = mkOption {
            type = types.str;
            default = "temp";
            example = "temp";
            description = ''
              name of the sqlite db you want to use
            '';
          };
          port = mkOption {
            type = types.port;
            default = 5059;
            example = 5059;
            description = ''
              local port on which to run the server
            '';
          };
        };
      });
    };
  };

  config = mkIf serviceConfig.enable {
    users.groups.kraken = { };
    users.users.kraken = {
      description = "kraken user";
      group = "kraken";
      isSystemUser = true;
    };

    systemd.services = lib.mapAttrs'
      (name: instanceConfig: lib.nameValuePair "kraken-backend-${name}"
        {
          wantedBy = [ "multi-user.target" ];
          description = "A backend with graphql and sqlite for valueflows";
          serviceConfig = {
            Type = "exec";
            Restart = "on-failure";
            RestartSec = 5;

            ExecStartPre = pkgs.writeShellScript "db_create_and_migrate" ''
              # go into the directory where the migrations are
              cd ${serviceConfig.package}
              ${pkgs.sqlx-cli}/bin/sqlx db create
              ${pkgs.sqlx-cli}/bin/sqlx migrate run
            '';
            ExecStart = "${serviceConfig.package}/bin/backend";
            ExecStop = "${serviceConfig.package}/bin/backend";

            User = "kraken";
            Group = "kraken";

            StateDirectory = "kraken";

            PrivateTmp = true;
            ProtectSystem = "full";
            NoNewPrivileges = true;
            ReadWritePaths = "${serviceConfig.stateDir}";
          };
          environment = {
            DATABASE_URL = "sqlite:${serviceConfig.stateDir}/${instanceConfig.dbName}.db";
            HTTP_PORT = toString instanceConfig.port;
            RUST_LOG = "${serviceConfig.logLevel}";
          };
        })
      serviceConfig.instances;
  };
}
