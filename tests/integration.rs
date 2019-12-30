#[cfg(test)]
mod integration {

    use assert_cmd::Command;
    use predicates::str::is_match;

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    #[test]
    fn no_arguments() {
        let mut cmd = Command::cargo_bin("which").unwrap();
        let assert = cmd
            .assert();
        assert
            .failure()
            .code(1)
            .stdout("")
            .stderr(is_match("which(.exe)?: Too few arguments").unwrap());
    }

    #[test]
    fn minus_version() {
        let mut cmd = Command::cargo_bin("which").unwrap();
        let assert = cmd
            .arg("--version")
            .assert();
        assert
            .success()
            .stdout(format!("Rusty which {}, Copyright (c) 2019 Graham THE Ollis.\n", VERSION))
            .stderr("");
    }

    #[test]
    fn minus_v() {
        let mut cmd = Command::cargo_bin("which").unwrap();
        let assert = cmd
            .arg("-v")
            .assert();
        assert
            .success()
            .stdout(format!("Rusty which {}, Copyright (c) 2019 Graham THE Ollis.\n", VERSION))
            .stderr("");
    }

    #[test]
    fn minus_help() {
        let mut cmd = Command::cargo_bin("which").unwrap();
        let assert = cmd
            .arg("--help")
            .assert();
        assert
            .success()
            .stdout(is_match("Usage: .*which(.exe)? \\[options\\] COMMAND").unwrap())
            .stderr("");
    }

    #[test]
    fn minus_foo() {
        let mut cmd = Command::cargo_bin("which").unwrap();
        let assert = cmd
            .arg("--foo")
            .assert();
        assert
            .failure()
            .code(1)
            .stdout("")
            .stderr(is_match("Usage: .*which(.exe)? \\[options\\] COMMAND").unwrap())
            .stderr(is_match("which(.exe)?: Unrecognized option: 'foo'").unwrap());
    }

}
