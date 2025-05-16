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

          max-jobs = nixosConfiguration.options.healthchecks.config.max-jobs.value;
          rawCommandOptions = nixosConfiguration.options.healthchecks.rawCommands.value;

          commandScripts = mapAttrsToList (
            group: groupConfiguration:
            (mapAttrsToList (topic: { title, script, ... }: "\"${title}\"=${script}") groupConfiguration)
          ) rawCommandOptions;

        in
        ''
          ${scriptExec}/bin/script-exec ${optionalString useEmoji "--style=emoji"} \
          -j ${toString max-jobs} \
          ${concatStringsSep " " (flatten commandScripts)}
        '';

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
          ${machineHeader}
          ${rawCommands nixosConfiguration}
        '';
    in
    {

      # Define packages which can be installed so it's fast
      packages.healthchecks = pkgs.writers.writeBashBin "nixos-healthchecks" ''
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
          mapAttrsToList (machine: configuration: ''
            # Check each machine
            if [[ $machine == "${machine}" || $all_machines -eq 1 ]]; then
                machine_found=1
                ${verify machine configuration} || overall_status=1
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
