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

      rawCommands =
        { nixosConfiguration, style, ... }:
        let

          max-jobs = nixosConfiguration.options.healthchecks.config.max-jobs.value;

          labels = concatStringsSep " " (
            mapAttrsToList (
              label: value: "--label=${label}:${value}"
            ) nixosConfiguration.options.healthchecks.config.labels.value
          );

          rawCommandOptions = nixosConfiguration.options.healthchecks.rawCommands.value;

          commandScripts = mapAttrsToList (
            group: groupConfiguration:
            (mapAttrsToList (topic: { title, script, ... }: "\"${title}\"=${script}") groupConfiguration)
          ) rawCommandOptions;

        in
        ''
          ${scriptExec}/bin/script-exec \
          --style=${style} \
          -j ${toString max-jobs} \
          ${labels} \
          ${concatStringsSep " " (flatten commandScripts)}
        '';

      verify =
        {
          machine,
          nixosConfiguration,
          style,
          ...
        }:
        let
          machineHeader = {
            "emoji" = ''
              echo ""
              echo "🖥️ ${machine}"
            '';
            "prometheus" = "";
            "systemd" = ''
              echo ""
              echo "{Machine} ${machine}"'';
          };
        in
        pkgs.writers.writeBash "verify-${machine}" ''
          ${lib.getAttr style machineHeader}
          ${rawCommands { inherit nixosConfiguration style; }}
        '';
    in
    {

      # Define packages which can be installed so it's fast
      packages =
        let

          packageGenerator =
            { style, name, ... }:
            pkgs.writers.writeBashBin name ''
              overall_status=0
              machine_found=0
              all_machines=0

              # Help message function
              show_help() {
                  cat ${toString ./help.txt}
                  exit 0
              }

              # Parse the optional arguments
              if [[ $1 == "--help" ]]; then
                  show_help
              elif [[ $1 == --machine=* ]]; then
                  machine="''${1#--machine=}"
              else
                  all_machines=1  # If no valid argument is provided, set flag for all machines
              fi


              ${concatStringsSep "\n\n" (
                mapAttrsToList (machine: nixosConfiguration: ''
                  # Check each machine
                  if [[ $machine == "${machine}" || $all_machines -eq 1 ]]; then
                      machine_found=1
                      ${verify { inherit machine nixosConfiguration style; }} || overall_status=1
                  fi
                '') nixosConfigurationsToVerify
              )}

              # If no machine was found and a specific machine was requested
              if [[ $machine_found -eq 0 && $all_machines -eq 0 ]]; then
                  echo "Error: Machine '$machine' does not exist."
                  exit 1
              fi

              exit $overall_status
            '';
        in
        {
          healthchecks = packageGenerator {
            style = "emoji";
            name = "nixos-healthchecks";
          };
          healthchecks-prometheus = packageGenerator {
            style = "prometheus";
            name = "nixos-healthchecks-prometheus";
          };
        };

      apps =
        {
          healthchecks = {
            type = "app";
            meta.description =
              let
                amountOfMachines = length (attrNames nixosConfigurationsToVerify);
              in
              "run healthchecks for all ${toString amountOfMachines} defined nixosConfigurations";
            program = pkgs.writers.writeBashBin "verify" ''
              overall_status=0
              ${concatStringsSep "\n\n" (
                mapAttrsToList (
                  machine: nixosConfiguration:
                  "${
                    verify {
                      inherit machine nixosConfiguration;
                      style = "emoji";
                    }
                  } || overall_status=1"
                ) nixosConfigurationsToVerify
              )}
              exit $overall_status
            '';
          };
        }
        // mapAttrs' (machine: nixosConfiguration: {
          name = "healthchecks-${machine}";
          value = {
            type = "app";
            meta.description = "run healthchecks for ${machine}";
            program = pkgs.writers.writeBashBin "verify-${machine}" ''
              ${verify {
                inherit machine nixosConfiguration;
                style = "emoji";
              }}
              exit $?
            '';
          };
        }) nixosConfigurationsToVerify;
    };

}
