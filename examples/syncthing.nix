{ config, ... }:
let
  syncthingUIPort = 8384;
in
{
  # check if syncthing gui is running
  healthchecks.http.syncthing = {
    url = config.services.syncthing.guiAddress;
    expectedContent = "syncthing";
  };

  # check if synchting gui port is not accessible from outside
  healthchecks.closed.public.ports.syncthing = [ syncthingUIPort ];

  # (incomplete) syncthing configuration
  services.syncthing = {
    enable = true;
    guiAddress = "localhost:${toString syncthingUIPort}";
  };

}
