{ config, ... }:
{
  # check if gitea is up and running
  healthchecks.http.gitea = {
    url = "https://gitea.com"; # todo: your gitea url here
    expectedContent = "login";
    notExpectedContent = "upgrade";
  };

  # make sure the gitea and dependent service ports are really closed
  healthchecks.closed.public.ports.gitea = [
    config.services.postgresql.settings.port
  ];

  # (incomplete) gitea configuration
  services.gitea.enable = true;
  services.gitea.database.type = "postgres";
  services.postgresql.enable = true;

}
