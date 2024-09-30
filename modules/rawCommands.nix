{ lib, ... }:
with lib;
with types;
{

  options.healthchecks.rawCommands = mkOption {
    default = { };
    description = ''
      This is a raw command and should not be used,
      It's what all other healthchecks end up to be.
      Use `localCommands` or `remoteCommands` if you want to run a script.
    '';

    # group -> topic -> script-definition
    # e.g.: http -> serivce -> script
    # e.g.: closedPorts -> interface -> serivce -> script

    type = attrsOf (
      attrsOf (submodule {
        options = {
          title = mkOption {
            type = str;
            description = ''
              Title to print when this particial script is running.
            '';
          };
          script = mkOption {
            type = path;
            description = ''
              The path to the script that should be run.
            '';
          };
        };
      })
    );

  };

}
