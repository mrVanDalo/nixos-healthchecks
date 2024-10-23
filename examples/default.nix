{

  # public domain/ip address to test for closed ports
  healthchecks.closed.public.host = "example.com";

  imports = [
    ./netdata.nix
    ./grafana.nix
  ];

}
