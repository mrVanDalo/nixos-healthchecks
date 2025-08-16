{ pkgs, config, ... }:
{
  # check if nextcloud is up and running
  healthchecks.http.nextcloud = {
    url = "https://nextcloud.com"; # todo: your nextcloud url here
    expectedContent = "login";
    notExpectedContent = "upgrade";
  };

  # make sure the nextcloud and dependent service ports are really closed
  healthchecks.closed.public.ports.nextcloud = [
    config.services.postgresql.settings.port
  ];

  # (incomplete) nextcloud configuration
  services.nextcloud.enable = true;
  services.nextcloud.config.dbtype = "pgsql";
  services.nextcloud.hostName = "example";
  services.postgresql.enable = true;

services.nextcloud.config.adminpassFile = pkgs.writeText "password" "secret";
}
