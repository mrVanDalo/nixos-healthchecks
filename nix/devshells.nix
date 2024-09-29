_: {
  perSystem =
    { pkgs, self', ... }:
    {
      devShells = {
        default = pkgs.mkShell {
          inputsFrom = [ self'.packages.default ];
          packages = [
            self'.formatter.outPath
          ];
        };
      };
    };
}
