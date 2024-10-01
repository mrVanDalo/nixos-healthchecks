{ self, ... }:
{
  imports = [ ];

  perSystem =
    {
      pkgs,
      self',
      lib,
      ...
    }:
    with lib;
    {
      apps.healthchecks = {
        type = "app";
        program =
          let
            nixosConfigurationsToVerify = filterAttrs (
              machine: configuration: builtins.hasAttr "healthchecks" configuration.options
            ) self.nixosConfigurations;

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
                      echo "${title}"
                      ${pkgs.moreutils}/bin/chronic ${script}
                    ''
                  ) groupConfiguration)
                  # todo handle exit code and stderr and such properly
                ) commandOptions;

              in
              flatten commandScripts;

            verify = machineName: nixosConfiguration: ''
              echo "${machineName}" | ${pkgs.boxes}/bin/boxes -d ansi
              ${concatStringsSep "\n" (rawCommands nixosConfiguration)}
            '';

            allCommands = concatStringsSep "\n\n" (mapAttrsToList verify nixosConfigurationsToVerify);

          in
          pkgs.writers.writeBashBin "verify" allCommands;
      };
    };

}
