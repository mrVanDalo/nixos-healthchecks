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
              --emoji        use emojis to print response code
              --time         measure script execution and show it
          -j, --jobs <JOBS>  Number of parallel jobs [default: 3]
          -h, --help         Print help
          -V, --version      Print version

        ----- stderr -----
        ");
    }

    #[test]
    fn test_main_success() {
        assert_cmd_snapshot!(cli().arg("success=./examples/success.sh"), @r"
        success: true
        exit_code: 0
        ----- stdout -----
        [Wait] success
        [1A[2K[ OK ] success

        ----- stderr -----
        ");
    }

    #[test]
    fn test_main_failure() {
        assert_cmd_snapshot!(cli().arg("failing=./examples/failing.sh"), @r"
        success: false
        exit_code: 1
        ----- stdout -----
        [Wait] failing
        [1A[2K[Fail] failing
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
        [Wait] success
        [1A[2K[Wait] success
        [Wait] success
        [1A[2K[1A[2K[Wait] success
        [Wait] success
        [Wait] success
        [1A[2K[1A[2K[1A[2K[ OK ] success
        [Wait] fail
        [1A[2K[Fail] fail
        Output:
        should fail
        [Wait] fail
        [1A[2K[ OK ] success
        [Wait] fail
        [1A[2K[Wait] fail
        [Wait] fail
        [1A[2K[1A[2K[Fail] fail
        Output:
        should fail
        [ OK ] success
        [Fail] fail
        Output:
        should fail

        ----- stderr -----
        ");
    }
}
