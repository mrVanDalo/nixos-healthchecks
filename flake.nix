{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    inputs@{ self, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        ./nix/formatter.nix
        ./nix/devshells.nix
        ./flake-module.nix
      ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      perSystem =
        {
          pkgs,
          self',
          lib,
          system,
          ...
        }:
        {
          packages.default = self'.packages.script-exec;
          packages.script-exec = pkgs.callPackage ./pkgs/script-exec { };
        };
      flake = {
        nixosConfigurations.my-example-machine = inputs.nixpkgs.lib.nixosSystem {
          #inherit system pkgs;
          system = "x86_64-linux";
          modules = [
            ./examples
            self.nixosModules.default
          ];
        };
        flakeModule = ./flake-module.nix;
        nixosModules.default = {
          imports = [
            ./modules/rawCommands.nix
            ./modules/localCommands.nix
            ./modules/http.nix
            ./modules/closedPorts.nix
          ];
        };
      };
    };
}
