{

  inputs = {
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";
    flake-parts.url = "github:hercules-ci/flake-parts";
    healthchecks.inputs.nixpkgs.follows = "nixpkgs";
    healthchecks.url = "github:mrvandalo/nixos-healthchecks";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    inputs@{
      flake-parts,
      healthchecks,
      nixpkgs,
      self,
    }:

    flake-parts.lib.mkFlake { inherit inputs; } (
      { }:
      {
        systems = [ "x86_64-linux" ]; # feel free to use other systems

        # 1. import healthchecks flakeModule
        imports = [ healthchecks.flakeModule ];

        perSystem =
          { pkgs, system, ... }:
          with pkgs;
          {
            nixosConfigurations.my-tool = lib.nixosSetup {
              inherit system pkgs;
              modules = [
                ./configuration.nix

                # 2. import healthchecks nixosModule
                healthchecks.nixosModules.default

              ];
            };
          };
      }
    );
}
