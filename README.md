# NixOS health checks

NixOS flake to write health checks as options, intended to be written right next
to the service definitions to verify right after deployment or whenever you like
if your services are running correctly.

## How to run

```shell
nix run .#healthchecks
```

## How to set up with flake parts

First you have to import the `healthchecks.flakeModule` and the
`healthchecks.nixosModules.default`.

```nix
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
```

Now you can define healthchecks to check the http responses

```nix
healthchecks.http.nextcloud = {
  url = "https://example.com/login";
  expectedContent = "Login";
};
```

or define healthchecks to check if a port is actually closed

```nix
healthchecks.closed.public.host = "example.com";
healthchecks.closed.public.ports.opentelemetry = [ 4317 ];
```

or define a healthcheck with a custom command

```nix
healthchecks.localCommand.test = ''
echo "this is a test"
'';
```
