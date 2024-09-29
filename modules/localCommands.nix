{ config, lib, ... }:
with lib;
with types;
{

  options.healthchecks.localCommands = mkOption {
    default = { };
    type = attrsOf str;
    description = ''
      service -> command
      command to run on local machine to test remote server.
    '';
  };

  config = {
    healthchecks.rawCommands.local = mapAttrs (name: script: {
      title = "running local command ${name}";
      inherit script;
    }) config.healthchecks.localCommands;

  };
}
