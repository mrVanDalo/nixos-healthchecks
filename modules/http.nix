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
        expectedContent = mkOption {
          type = nullOr str;
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
        curl = lib.getExe pkgs.curl;
        grep = lib.getExe pkgs.gnugrep;
        # fixme expected Content checks seem not to work
        scriptWithExpectedContent = url: responseCode: expectedContent: ''
          if ${curl} -s -o /dev/null -w "%{http_code}" ${url} | ${grep} -q "${toString responseCode}"; then
            if ${curl} -s ${url} | ${grep} -q "${expectedContent}"; then
              echo -n ""
            else
              echo " [Fail] ${url} did return ${toString responseCode}, but did not contain the string '${expectedContent}'."
            fi
          else
            echo " [Fail] ${url} did not return ${toString responseCode}."
          fi
        '';

        scriptWithoutExpectedContent = url: responseCode: ''
          if ${curl} -s -o /dev/null -w "%{http_code}" ${url} | ${grep} -q "${toString responseCode}"; then
              echo -n ""
          else
            echo " [Fail] ${url} did not return ${toString responseCode}."
          fi
        '';
        script =
          url: responeCode: expectedContent:
          if (expectedContent == null) then
            scriptWithExpectedContent url responeCode expectedContent
          else
            scriptWithoutExpectedContent url responeCode;

      in
      mapAttrs' (
        service:
        {
          url,
          responseCode,
          expectedContent,
        }:
        nameValuePair service {
          title = "verify http for ${service}";
          script = pkgs.writers.writeBash "http-${service}" (script url responseCode expectedContent);
        }

      ) config.healthchecks.http;

  };

}
