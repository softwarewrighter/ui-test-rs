//! Integration tests for CLI functionality

#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_short() {
    Command::cargo_bin("ui-test-rs")
        .unwrap()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("CLI tool for UI testing"));
}

#[test]
fn test_help_long() {
    Command::cargo_bin("ui-test-rs")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("AI CODING AGENT INSTRUCTIONS"));
}

#[test]
fn test_version_short() {
    Command::cargo_bin("ui-test-rs")
        .unwrap()
        .arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_version_long() {
    Command::cargo_bin("ui-test-rs")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_dry_run() {
    Command::cargo_bin("ui-test-rs")
        .unwrap()
        .arg("--dry-run")
        .arg("tests/")
        .assert()
        .success()
        .stdout(predicate::str::contains("Dry-run mode"));
}

#[test]
fn test_verbose_mode() {
    Command::cargo_bin("ui-test-rs")
        .unwrap()
        .arg("-v")
        .arg("tests/")
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose mode enabled"));
}

#[test]
fn test_default_path() {
    Command::cargo_bin("ui-test-rs")
        .unwrap()
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("would execute tests at ."));
}

#[test]
fn test_custom_path() {
    Command::cargo_bin("ui-test-rs")
        .unwrap()
        .arg("--dry-run")
        .arg("/custom/path")
        .assert()
        .success()
        .stdout(predicate::str::contains("/custom/path"));
}
