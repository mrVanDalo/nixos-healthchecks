{

  # public domain/ip address to test for closed ports
  healthchecks.closed.public.host = "example.com";

  imports = [
    ./gitea.nix
    ./nextcloud.nix
    ./syncthing.nix
  ];

}
