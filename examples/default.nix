{

  # public domain/ip address to test for closed ports
  healthchecks.closed.public.host = "example.com";
  healthchecks.config.max-jobs = 2; # to see something on the example.gif

  imports = [
    ./gitea.nix
    ./nextcloud.nix
    ./syncthing.nix
  ];

}
