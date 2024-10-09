{ config, lib, ... }:
with lib;
with types;
{

  options.healthchecks.localCommands = mkOption {
    default = { };
    type = attrsOf path;
    description = ''
      service -> path-to-command
      command to run on local machine to test remote server.
      exit code 0 will result in success
      all other exit codes will result in failure
    '';
  };

  config = {
    healthchecks.rawCommands.local = mapAttrs (name: script: {
      title = "running local command ${name}";
      inherit script;
    }) config.healthchecks.localCommands;

  };
}
