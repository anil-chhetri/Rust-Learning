use assert_cmd::Command;
use predicates::prelude::*;

use std::fs;

#[test]
fn dies_no_args() {
    let mut echor_command = Command::cargo_bin("echor").unwrap();
    echor_command
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn run_with_args() {
    let mut echor_command = Command::cargo_bin("echor").unwrap();
    echor_command.arg("hello world").assert().success();
}

#[test]
fn hello1() {
    let outputfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outputfile).unwrap();
    let mut result = Command::cargo_bin("echor").unwrap();
    result
        .arg("Hello there")
        .assert()
        .success()
        .stdout(expected);
}
