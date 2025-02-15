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
                ${scriptExec}/bin/script-exec --title "${title}" ${optionalString useEmoji "--emoji"} --time ${script} || overall_status=1
              ''
            ) groupConfiguration)
          ) commandOptions;

        in
        flatten commandScripts;

      verify =
        machine: nixosConfiguration:
        let
          machineHeader =
            if useEmoji then
              ''
                echo ""
                echo "🖥️ ${machine}"
              ''
            else
              ''
                echo ""
                echo "{Machine} ${machine}"
              '';
        in
        pkgs.writers.writeBash "verify-${machine}" ''
          overall_status=0
          ${machineHeader}
          ${concatStringsSep "\n" (rawCommands nixosConfiguration)}
          exit $overall_status
        '';
    in
    {
      apps =
        {
          healthchecks = {
            type = "app";
            program = pkgs.writers.writeBashBin "verify" ''
              overall_status=0
              ${concatStringsSep "\n\n" (
                mapAttrsToList (
                  machine: configuration: "${verify machine configuration} || overall_status=1"
                ) nixosConfigurationsToVerify
              )}
              exit $overall_status
            '';
          };
        }
        // mapAttrs' (machine: configuration: {
          name = "healthchecks-${machine}";
          value = {
            type = "app";
            program = pkgs.writers.writeBashBin "verify-${machine}" ''
              ${verify machine configuration}
              exit $?
            '';
          };
        }) nixosConfigurationsToVerify;
    };

}
