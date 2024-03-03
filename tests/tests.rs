use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_main_with_no_arguments() {
    let mut cmd = Command::cargo_bin("guit").unwrap();
    
    cmd.assert()
        .success()
        .stderr(predicate::eq("Nenhum argumento enviado.\n"));
}

#[test]
fn test_main_with_valid_command() {
    let mut cmd = Command::cargo_bin("guit").unwrap();
    let assert = cmd.arg("estado").assert();
    
    assert.success()
        .stdout(predicate::str::contains("On branch"));
}

