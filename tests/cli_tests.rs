// tests/cli_tests.rs
//! CLI integration tests using assert_cmd.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_version_command() {
    let mut cmd = assert_cmd::cargo::cargo_bin("systrix");
    cmd.arg("version");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("systrix"));
}

#[test]
fn test_info_command() {
    let mut cmd = assert_cmd::cargo::cargo_bin("systrix");
    cmd.arg("info");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("CPU"))
        .stdout(predicate::str::contains("Memory"))
        .stdout(predicate::str::contains("Disk"));
}

#[test]
fn test_ps_command() {
    let mut cmd = assert_cmd::cargo::cargo_bin("systrix");
    cmd.arg("ps").arg("--limit").arg("5");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("PID"))
        .stdout(predicate::str::contains("NAME"));
}

#[test]
fn test_net_command() {
    let mut cmd = assert_cmd::cargo::cargo_bin("systrix");
    cmd.arg("net");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Network"));
}

#[test]
fn test_disk_command() {
    let mut cmd = assert_cmd::cargo::cargo_bin("systrix");
    cmd.arg("disk");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Disk"));
}

#[test]
fn test_kill_without_pid() {
    let mut cmd = assert_cmd::cargo::cargo_bin("systrix");
    cmd.arg("kill");
    
    // Should fail because PID is required
    cmd.assert().failure();
}

#[test]
fn test_kill_system_process_without_force() {
    let mut cmd = assert_cmd::cargo::cargo_bin("systrix");
    cmd.arg("kill").arg("1");
    
    // Should fail or warn about killing system process
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("system process"));
}

#[test]
fn test_report_command() {
    use std::fs;
    use tempfile::tempdir;
    
    let dir = tempdir().unwrap();
    let report_path = dir.path().join("test_report.json");
    
    let mut cmd = assert_cmd::cargo::cargo_bin("systrix");
    cmd.arg("report")
        .arg("--output")
        .arg(report_path.to_str().unwrap());
    
    cmd.assert().success();
    
    // Verify report file was created
    assert!(report_path.exists());
    
    // Verify it's valid JSON
    let content = fs::read_to_string(&report_path).unwrap();
    let _json: serde_json::Value = serde_json::from_str(&content).unwrap();
}
