{ config, lib, ... }:
with lib;
with types;
{

  options.healthchecks.localCommands = mkOption {
    default = { };
    type = attrsOf path;
    description = ''
      Command to run on local machine to test remote server.
      exit code 0 will result in success
      all other exit codes will result in failure
    '';
    example = lib.literalExpression ''
      {
        ping-wireguard = pkgs.writers.writeBash "ping-wireguard" '''
          # ping this machine via wireguard network
          ping -c 1 -W 5 10.5.23.42
        ''';
        ping-tinc = pkgs.writers.writeBash "ping-tinc" '''
          # ping this machine via tinc vpn
          ping -c 1 -W 5 10.5.23.42
        ''';
      };
    '';
  };

  config = {
    healthchecks.rawCommands.local = mapAttrs (name: script: {
      title = "running local command ${name}";
      inherit script;
    }) config.healthchecks.localCommands;

  };
}
