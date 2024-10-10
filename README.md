# NixOS health checks

NixOS flake to write health checks as options, intended to be written right next
to the service definitions to verify right after deployment or whenever you like
if your services are running correctly.

## How to run

```
nix run .#healthchecks
🖥️ myMachine
✅ verify public ports are closed for opentelemetry
✅ verify http for nextcloud
...

🖥️ myServer
✅ verify public ports are closed for opentelemetry
✅ verify http for nextcloud
...
```

## How to define checks

You can define checks using the (newly introduced) `healthchecks`
[NixOS Option](https://search.nixos.org/options).

### Check the http responses

```nix
healthchecks.http.nextcloud = {
  url = "https://example.com/login";
  expectedContent = "Login";
};
```

### check if a port is actually closed

```nix
healthchecks.closed.public.host = "example.com";
healthchecks.closed.public.ports.opentelemetry = [ 4317 ];
```

### custom command

```nix
healthchecks.localCommand.test = ''
echo "this is a test"
'';
```

## How to set up with flake parts

You have to import the `healthchecks.flakeModule` and the
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
            nixosConfigurations.my-machine = lib.nixosSetup {
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
