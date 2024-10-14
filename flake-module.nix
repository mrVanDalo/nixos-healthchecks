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
    in
    {
      apps.healthchecks = {
        type = "app";
        program =
          let
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

          in
          pkgs.writers.writeBashBin "verify" allCommands;
      };
    };

}
