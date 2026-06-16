use assert_cmd::Command;
use predicates::prelude::*;


#[test]
fn test_scan_command_output() {
    let mut cmd = Command::cargo_bin("logguardian").unwrap();

    cmd.arg("scan")
       .arg("tests/top_errors.log")
       .assert()
       .success()
       .stdout(predicates::str::contains("Lines"))
       .stdout(predicates::str::contains("Errors"))
       .stdout(predicates::str::contains("Warnings"))
       .stdout(predicates::str::contains("Info"));
}

#[test]
fn test_top_command_output() {
    let mut cmd = Command::cargo_bin("logguardian").unwrap();

    cmd.arg("top")
       .arg("tests/top_errors.log")
       .assert()
       .success()
       .stdout(predicates::str::contains("Top Errors"))
       .stdout(predicates::str::contains("Disk failure"))
       .stdout(predicates::str::contains("Network timeout"));
}

#[test]
fn test_invalid_file_error() {
    let mut cmd = Command::cargo_bin("logguardian").unwrap();

    cmd.arg("scan")
       .arg("does_not_exist.log")
       .assert()
       .failure()
       .stdout(predicates::str::contains("❌ File not found"));
}
