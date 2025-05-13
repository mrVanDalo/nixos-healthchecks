{ lib, ... }:
with lib;
{

  options.healthchecks.config = {
    max-jobs = mkOption {
      default = 6;
      description = ''
        how many test jobs should run at the same time
      '';
      type = types.int;

    };
  };

}
