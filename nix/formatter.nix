{ inputs, ... }:
{
  imports = [ inputs.treefmt-nix.flakeModule ];

  perSystem = _: {
    treefmt = {
      # https://flake.parts/options/treefmt-nix < for all options
      projectRootFile = ".git/config";
      programs.nixfmt.enable = true;
      programs.deno.enable = true;
      programs.rustfmt.enable = true;
    };
  };
}
