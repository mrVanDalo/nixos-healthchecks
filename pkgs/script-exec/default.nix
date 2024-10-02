{ rustPlatform, ... }:
rustPlatform.buildRustPackage rec {
  pname = "script-exec";
  version = "1.0.0";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
