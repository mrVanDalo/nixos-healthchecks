{ config, ... }:
{

  # make sure the grafana port is really closed
  healthchecks.closed.public.ports.grafana = [ config.services.grafana.settings.server.http_port ];

  # test if grafana is actually running
  healthchecks.http.grafana.url = "http://localhost:${toString config.services.grafana.settings.server.http_port}";

  services.grafana = {
    enable = true;
    settings = {
      server = {
        domain = "grafana.${config.networking.hostName}.private";
        http_port = 2342;
        http_addr = "127.0.0.1";
      };
      users.default_theme = "light";
      "auth.anonymous" = {
        enabled = true;
        org_name = "Chungus";
        org_role = "Viewer";
        hide_version = true;
      };
    };
  };

}
