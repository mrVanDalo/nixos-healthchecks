{
  lib,
  config,
  pkgs,
  ...
}:
with lib;
with types;
{
  options.healthchecks.http = mkOption {
    default = { };
    example = {
      github = {
        url = "https://github.com";
        expectedContent = "GitHub";
      };
    };
    description = ''
      Run curl commands to verify if response code is as expected and expectedContent is part of the body.
    '';
    type = attrsOf (submodule {
      options = {
        url = mkOption {
          type = str;
          description = ''
            URL to  analyze.
          '';
        };
        responseCode = mkOption {
          type = int;
          default = 200;
          description = ''
            Expected response code
          '';
        };
        notExpectedContent = mkOption {
          type = nullOr str;
          default = null;
          description = ''
            Not expected string in the response
          '';
        };
        expectedContent = mkOption {
          type = nullOr str;
          default = null;
          description = ''
            Expected string in the response
          '';
        };
      };
    });
  };

  config = {

    healthchecks.rawCommands.http =
      let
        script =
          url: responeCode: expectedContent: notExpectedContent: service:
          pkgs.writers.writePython3Bin "verify-http-for-${service}"
            {
              libraries = [ pkgs.python3Packages.requests ];
              flakeIgnore = [
                "E302"
                "E305"
                "E501"
                "E303"
              ];
            }
            ''
              import requests
              import sys

              def ensure_http_prefix(url):
                  if not url.startswith(("http://", "https://")):
                      return "http://" + url
                  return url

              try:
                  response = requests.get(ensure_http_prefix("${url}"))
              except requests.exceptions.RequestException as e:
                  print(f"Request failed: {e}")
                  sys.exit(1)

              if response.status_code != ${toString responeCode}:
                  print(f"Received unexpected status code: {response.status_code}")
                  sys.exit(1)

              response_text = response.text

              ${optionalString (expectedContent != null) ''
                if "${expectedContent}" not in response_text:
                    print("'${expectedContent}' does not appear in response body.")
                    sys.exit(1)
              ''}

              ${optionalString (notExpectedContent != null) ''
                if "${notExpectedContent}" in response_text:
                    print("'${notExpectedContent}' does appear in response body.")
                    sys.exit(1)
              ''}

              sys.exit(0)
            '';

      in
      mapAttrs' (
        service:
        {
          url,
          responseCode,
          expectedContent,
          notExpectedContent,
        }:
        nameValuePair service {
          title = "verify http for ${service}";
          script = (script url responseCode expectedContent notExpectedContent service);
        }

      ) config.healthchecks.http;

  };

}
