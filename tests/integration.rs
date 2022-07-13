use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn help() {
    let mut cmd = Command::cargo_bin("cli").unwrap();
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("rust-cli 🔐 Rust CLI Template"));
}

#[test]
fn file_not_found() {
    let mut cmd = Command::cargo_bin("cli").unwrap();
    cmd.arg("-f").arg("file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));
}

#[test]
fn read_file() {
    let mut cmd = Command::cargo_bin("cli").unwrap();
    cmd.arg("-f").arg("tests/fixtures/test.txt");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello!"));
}
