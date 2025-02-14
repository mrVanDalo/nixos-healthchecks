{ self, inputs, ... }:
{
  imports = [ ];

  perSystem =
    {
      pkgs,
      self',
      lib,
      system,
      ...
    }:
    with lib;
    let

      scriptExec = pkgs.callPackage ./pkgs/script-exec { };

      nixosConfigurationsToVerify = filterAttrs (
        machine: configuration: builtins.hasAttr "healthchecks" configuration.options
      ) self.nixosConfigurations;

      useEmoji = true;

      rawCommands =
        nixosConfiguration:
        let

          commandOptions = nixosConfiguration.options.healthchecks.rawCommands.value;

          commandScripts = mapAttrsToList (
            group: groupConfiguration:
            (mapAttrsToList (
              topic:
              { title, script }:
              ''
                ${scriptExec}/bin/script-exec --title "${title}" ${optionalString useEmoji "--emoji"} --time ${script}
              ''
            ) groupConfiguration)
          ) commandOptions;

        in
        flatten commandScripts;

      verify =
        machineName: nixosConfiguration:
        let
          machineHeader =
            if useEmoji then
              ''
                echo ""
                echo "🖥️ ${machineName}"
              ''
            else
              ''
                echo ""
                echo "{Machine} ${machineName}"
              '';
        in
        ''
          ${machineHeader}
          ${concatStringsSep "\n" (rawCommands nixosConfiguration)}
        '';

      allCommands = concatStringsSep "\n\n" (mapAttrsToList verify nixosConfigurationsToVerify);
      healthcheck-script-all = pkgs.writers.writeBashBin "nixos-healthchecks" allCommands;
      healthcheck-script-machine =
        machine: configuration:
        pkgs.writers.writeBashBin "nixos-healthchecks-${machine}" (verify machine configuration);

    in

    {
      apps =
        {
          healthchecks = {
            type = "app";
            program = healthcheck-script-all;
          };
        }
        // mapAttrs' (machine: configuration: {
          name = "healthchecks-${machine}";
          value = {
            type = "app";
            program = healthcheck-script-machine machine configuration;
          };
        }) nixosConfigurationsToVerify;

      packages.healthchecks = pkgs.buildEnv {
        name = "nixos-healthchecks";
        paths = [
          healthcheck-script-all
        ] ++ (mapAttrsToList healthcheck-script-machine nixosConfigurationsToVerify);
      };
    };

}
