{ config, ... }:
{

  services.netdata = {
    enable = true;
    config.global."memory mode" = "ram";
  };

  # make sure the netdata port is really closed
  healthchecks.closed.public.ports.netdata = [ 19999 ];

  # test if netdata is actually running
  healthchecks.http.netdata = {
    url = "http://localhost:19999";
    expectedContent = "netdata dashboard";
  };

}
