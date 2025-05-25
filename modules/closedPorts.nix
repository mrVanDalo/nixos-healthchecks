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
          synthing-gui = [ 8384 ];
        };
      };
    };
    description = ''
      Verify that specified ports are closed for specific network interfaces.
      Port verification is performed using rustscan, which checks if the ports
      are unreachable from the network.
    '';
    type = attrsOf (submodule {
      options = {
        host = mkOption {
          type = str;
          description = ''
            The target host address to scan with rustscan.
          '';
          example = "example.com";
        };
        ports = mkOption {
          default = { };
          type = attrsOf (listOf int);
          description = ''
            A mapping of service names to lists of port numbers that should be verified as closed.
            Each port in these lists will be checked to ensure it is not accessible.
          '';
          example = {
            ports = {
              arr = [
                7878
                8989
                8686
              ];
              synthing-gui = [ 8384 ];
            };
          };
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
            script = pkgs.writers.writeBash "verify-${interfaceName}-ports-are-closed-for-${serviceName}" ''
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
