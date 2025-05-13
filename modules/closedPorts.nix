{
  lib,
  config,
  pkgs,
  ...
}:
with lib;
with types;
{
  options.healthchecks.closed = mkOption {
    default = { };
    example = {
      public = {
        host = "example.com";
        ports = {
          arr = [
            7878
            8989
            8686
          ];
        };
      };
    };
    description = ''
      Verify that ports the defined ports are closed for a specific interface.
      Verification is done by rustscan.
    '';
    type = attrsOf (submodule {
      options = {
        host = mkOption {
          type = str;
          description = ''
            The host against which the rustscan will be done.
            Needed because we have more than interface on the machine.
          '';
        };
        ports = mkOption {
          default = { };
          type = attrsOf (listOf int);
          description = ''
            service -> [port, ... ]
            Ports that should be verified as beeing closed.
          '';
        };
      };
    });
  };

  config = {
    healthchecks.rawCommands.closed-ports =
      let

        # todo : verify if host is reachable
        command =
          serviceName: interfaceName: host: ports:
          nameValuePair (interfaceName + serviceName) {
            title = "verify ${interfaceName} ports are closed for ${serviceName}";
            script = pkgs.writers.writeBashBin "verify-${interfaceName}-ports-are-closed-for-${serviceName}" ''
              # Run the rustscan command and capture the output
              output=$(
              ${pkgs.rustscan}/bin/rustscan \
                  --ports ${concatStringsSep "," (map toString ports)} \
                  --addresses ${host} \
                  --greppable
                  2>&1)

              # Check if there was any output
              if [ -n "$output" ]; then
                  echo "$output"
                  exit 1
              else
                  exit 0
              fi
            '';
          };

        interfaceCommands = mapAttrsToList (
          interfaceName: interfaceConfiguration:
          mapAttrsToList (
            serviceName: servicePorts:
            command serviceName interfaceName interfaceConfiguration.host servicePorts
          ) interfaceConfiguration.ports
        ) config.healthchecks.closed;

      in
      builtins.listToAttrs (flatten interfaceCommands);

  };

}
