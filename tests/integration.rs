#[cfg(test)]
mod integration {

    use assert_cmd::Command;
    use predicates::str::is_match;
    use std::env::current_dir;
    use std::ffi::OsString;

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
            .stderr(is_match(r"which(\.exe)?: Too few arguments").unwrap());
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
            .stdout(is_match(r"Usage: .*which(\.exe)? \[options\] COMMAND").unwrap())
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
            .stderr(is_match(r"Usage: .*which(\.exe)? \[options\] COMMAND").unwrap())
            .stderr(is_match(r"which(\.exe)?: Unrecognized option: 'foo'").unwrap());
    }

    #[cfg(windows)]
    fn testdata_path(subdir: &str, dirs: Vec<&str>) -> OsString {
            let mut base = OsString::new();
            base.push(current_dir().unwrap().as_os_str());
            base.push("\\tests\\testdata\\");
            base.push(subdir);
            base.push("\\win\\");

            let mut path = OsString::new();
            path.push(base.clone());
            path.push(dirs[0]);

            for dir in dirs.iter().skip(1) {
                path.push(";");
                path.push(base.clone());
                path.push(dir);
            }

            path
    }

    #[cfg(unix)]
    fn testdata_path(subdir: &str, dirs: Vec<&str>) -> OsString {
            let mut base = OsString::new();
            base.push(current_dir().unwrap().as_os_str());
            base.push("/tests/testdata/");
            base.push(subdir);
            base.push("/unix/");

            let mut path = OsString::new();
            path.push(base.clone());
            path.push(dirs[0]);

            for dir in dirs.iter().skip(1) {
                path.push(":");
                path.push(base.clone());
                path.push(dir);
            }

            path
    }

    #[test]
    fn simple_found() {
        let mut cmd = Command::cargo_bin("which").unwrap();
        let assert = cmd
            .arg("foo")
            .env("PATH", testdata_path("simple", ["bin1"].to_vec()))
            .assert();
        assert
            .success()
            .stdout(is_match(r"tests[\\/]testdata[\\/]simple[\\/](win|unix)[\\/]bin1[\\/]foo(\.exe)?").unwrap())
            .stderr("");
    }

    #[test]
    fn simple_not_found() {
        let mut cmd = Command::cargo_bin("which").unwrap();
        let assert = cmd
            .arg("bar")
            .env("PATH", testdata_path("simple", ["bin1"].to_vec()))
            .assert();
        assert
            .failure()
            .code(1)
            .stdout("")
            .stderr("bar: Command not found\n");
    }

}
