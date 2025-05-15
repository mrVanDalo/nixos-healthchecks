{ inputs, ... }:
{
  perSystem =
    {
      pkgs,
      self',
      system,
      ...
    }:
    {
      # allow unfree packages
      _module.args.pkgs = import inputs.nixpkgs {
        inherit system;
        config.allowUnfree = true;
      };

      devShells = {
        default = pkgs.mkShell {
          packages = [
            self'.formatter.outPath
            pkgs.jetbrains.rust-rover
            pkgs.rustup
            pkgs.cargo-insta
          ];
        };
      };
    };
}
