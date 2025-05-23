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
        headers = mkOption {
          type = attrsOf str;
          description = ''
            HTTP Headers
          '';
          example = {
            "Host" = "example.com";
          };
          default = { };
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
          url: responeCode: expectedContent: notExpectedContent: service: headers:
          pkgs.writers.writePython3 "verify-http-for-${service}"
            {
              libraries = [ pkgs.python3Packages.requests ];
              flakeIgnore = [
                "E231"
                "E302"
                "E303"
                "E305"
                "E501"
              ];
            }
            ''
              import requests
              import sys

              def ensure_http_prefix(url):
                  if not url.startswith(("http://", "https://")):
                      return "http://" + url
                  return url

              headers = ${builtins.toJSON headers}

              try:
                  response = requests.get(ensure_http_prefix("${url}"), headers=headers)
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
          headers,
        }:
        nameValuePair service {
          title = "verify http for ${service}";
          script = (script url responseCode expectedContent notExpectedContent service headers);
        }

      ) config.healthchecks.http;

  };

}
