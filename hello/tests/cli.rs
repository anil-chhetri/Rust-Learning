use assert_cmd;
use std::process::Command;

#[test]
fn works() {
    assert!(true);
}

#[test]
fn runs() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn cli_runs() {
    let mut cmd = assert_cmd::Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}

#[test]
fn true_ok() {
    let mut cmd = assert_cmd::Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = assert_cmd::Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn cli_stdout() {
    let mut cmd = assert_cmd::Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}
