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
        assert_cmd_snapshot!(cli());
    }

    #[test]
    fn test_main_help() {
        assert_cmd_snapshot!(cli().arg("--help"));
    }

    #[test]
    fn test_main_success() {
        assert_cmd_snapshot!(cli().arg("success=./examples/success.sh"));
    }

    #[test]
    fn test_main_failure() {
        assert_cmd_snapshot!(cli().arg("failing=./examples/failing.sh"));
    }

    #[test]
    fn test_main_success_systemd() {
        assert_cmd_snapshot!(
            cli()
                .arg("--style=systemd")
                .arg("success=./examples/success.sh")
        );
    }

    #[test]
    fn test_main_failure_systemd() {
        assert_cmd_snapshot!(
            cli()
                .arg("--style=systemd")
                .arg("failing=./examples/failing.sh")
        );
    }

    #[test]
    fn test_main_success_emoji() {
        assert_cmd_snapshot!(
            cli()
                .arg("--style=emoji")
                .arg("success=./examples/success.sh")
        );
    }

    #[test]
    fn test_main_failure_emoji() {
        assert_cmd_snapshot!(
            cli()
                .arg("--style=emoji")
                .arg("failing=./examples/failing.sh")
        );
    }

    #[test]
    fn test_main_multiple() {
        assert_cmd_snapshot!(
            cli()
                .arg("success=./examples/success.sh")
                .arg("success=./examples/success-1.sh")
                .arg("success=./examples/success-3.sh")
                .arg("fail=./examples/failing.sh")
                .arg("fail=./examples/failing-1.sh")
                .arg("fail=./examples/failing-3.sh")
        );
    }
}
