{ lib, config, ... }:
with lib;
with types;
{

  options.healthchecks.config = {
    max-jobs = mkOption {
      default = 6;
      description = ''
        How many test jobs should run at the same time
      '';
      type = types.int;
    };
    labels = mkOption {
      default = {
        machine = config.networking.hostName;
      };
      description = ''
        Additional labels used in prometheus line output
      '';
      type = attrsOf str;
    };

  };

}
