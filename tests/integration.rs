#[cfg(test)]
mod integration {

    use assert_cmd::Command;

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    #[test]
    fn minus_version() {
        let mut cmd = Command::cargo_bin("which").unwrap();
        let assert = cmd
            .arg("--version")
            .assert();
        assert
            .success()
            .stdout(format!("Rusty which {}, Copyright (c) 2019 Graham THE Ollis.\n", VERSION));
        
    }

    #[test]
    fn minus_v() {
        let mut cmd = Command::cargo_bin("which").unwrap();
        let assert = cmd
            .arg("-v")
            .assert();
        assert
            .success()
            .stdout(format!("Rusty which {}, Copyright (c) 2019 Graham THE Ollis.\n", VERSION));
        
    }

}
