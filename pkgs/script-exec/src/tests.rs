#[cfg(test)]
mod tests {
    use insta_cmd::assert_cmd_snapshot;
    use insta_cmd::get_cargo_bin;
    use std::process::Command;

    fn cli() -> Command {
        Command::new(get_cargo_bin("script-exec"))
    }

    #[test]
    fn test_main_no_arguments() {
        assert_cmd_snapshot!(cli(), @r"
        success: false
        exit_code: 1
        ----- stdout -----

        ----- stderr -----
        No paths provided
        ");
    }

    #[test]
    fn test_main_help() {
        assert_cmd_snapshot!(cli().arg("--help"), @r"
        success: true
        exit_code: 0
        ----- stdout -----
        print out healthcheck script lines

        Usage: script-exec [OPTIONS] [PAIRS]...

        Arguments:
          [PAIRS]...  The alternating titles and paths to the scripts (title=path)

        Options:
              --style <STYLE>  The style of output to use [default: emoji] [possible values: emoji, systemd]
              --time           measure script execution and show it
          -j, --jobs <JOBS>    Number of parallel jobs [default: 3]
          -h, --help           Print help
          -V, --version        Print version

        ----- stderr -----
        ");
    }

    #[test]
    fn test_main_success() {
        assert_cmd_snapshot!(cli().arg("success=./examples/success.sh"), @r"
        success: true
        exit_code: 0
        ----- stdout -----
        ⏳ success
        [1A[2K✅ success [0.01s]

        ----- stderr -----
        ");
    }

    #[test]
    fn test_main_failure() {
        assert_cmd_snapshot!(cli().arg("failing=./examples/failing.sh"), @r"
        success: false
        exit_code: 1
        ----- stdout -----
        ⏳ failing
        [1A[2K❌ failing [0.01s]
        Output:
        should fail

        ----- stderr -----
        ");
    }

    #[test]
    fn test_main_multiple() {
        assert_cmd_snapshot!(cli()
            .arg("success=./examples/success.sh")
            .arg("success=./examples/success-1.sh")
            .arg("success=./examples/success-2.sh")
            .arg("fail=./examples/failing.sh")
            .arg("fail=./examples/failing-1.sh")
            .arg("fail=./examples/failing-2.sh")
            , @r"
        success: false
        exit_code: 1
        ----- stdout -----
        ⏳ success
        [1A[2K⏳ success
        ⏳ success
        [1A[2K[1A[2K⏳ success
        ⏳ success
        ⏳ success
        [1A[2K[1A[2K[1A[2K✅ success [0.01s]
        ⏳ fail
        [1A[2K❌ fail [0.01s]
        Output:
        should fail
        ⏳ fail
        [1A[2K✅ success [1.01s]
        ⏳ fail
        [1A[2K⏳ fail
        ⏳ fail
        [1A[2K[1A[2K❌ fail [1.01s]
        Output:
        should fail
        ✅ success [2.01s]
        ❌ fail [2.01s]
        Output:
        should fail

        ----- stderr -----
        ");
    }
}
