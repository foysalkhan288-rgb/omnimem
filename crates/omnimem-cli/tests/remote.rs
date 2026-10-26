//! Integration tests for `omnimem remote`.
//!
//! Tests cover the four subcommands and the security contract (token
//! never lands on disk):
//!
//! 1. `remote list` on a fresh repo prints `<no remotes>`.
//! 2. `remote add` + `remote list` + `remote show` + `remote remove`
//!    round-trip through `.omnimem/config.toml`.
//! 3. `remote add` refuses a duplicate name.
//! 4. `remote show` on a missing name fails actionably.
//! 5. `remote add --token-env VAR` records only the env var name in
//!    the file - never the actual token.

use std::path::Path;
use std::process::Command;

use assert_cmd::prelude::*;
use tempfile::TempDir;

fn omnimem(repo: &Path, args: &[&str]) -> Command {
    let mut cmd = Command::cargo_bin("omnimem").expect("built omnimem binary");
    cmd.current_dir(repo);
    cmd.arg("-R").arg(repo);
    for a in args {
        cmd.arg(a);
    }
    cmd
}

#[test]
fn remote_list_empty_repo() {
    let dir = TempDir::new().unwrap();
    omnimem(dir.path(), &["init", dir.path().to_str().unwrap()])
        .assert()
        .success();
    let out = omnimem(dir.path(), &["remote", "list"]).assert().success();
    let stdout = String::from_utf8_lossy(&out.get_output().stdout).to_string();
    assert!(
        stdout.contains("<no remotes>"),
        "empty repo should show no remotes, got: {stdout}"
    );
}

#[test]
fn remote_round_trip_through_config_toml() {
    let dir = TempDir::new().unwrap();
    omnimem(dir.path(), &["init", dir.path().to_str().unwrap()])
        .assert()
        .success();

    // Add two remotes.
    omnimem(
        dir.path(),
        &[
            "remote",
            "add",
            "origin",
            "https://alice.example/notes.car",
            "--token-env",
            "omnimem_ORIGIN_TOKEN",
        ],
    )
    .assert()
    .success();
    omnimem(
        dir.path(),
        &["remote", "add", "backup", "https://example.com/backup"],
    )
    .assert()
    .success();

    // List should see both.
    let listed = omnimem(dir.path(), &["remote", "list"]).assert().success();
    let listed_out = String::from_utf8_lossy(&listed.get_output().stdout).to_string();
    assert!(
        listed_out.contains("origin") && listed_out.contains("backup"),
        "list should surface both remotes, got: {listed_out}"
    );
    assert!(listed_out.contains("https://alice.example/notes.car"));
    assert!(listed_out.contains("https://example.com/backup"));

    // Show origin: must carry url + token_env, but NOT the token.
    let shown = omnimem(dir.path(), &["remote", "show", "origin"])
        .assert()
        .success();
    let shown_out = String::from_utf8_lossy(&shown.get_output().stdout).to_string();
    assert!(
        shown_out.contains("https://alice.example/notes.car"),
        "show should carry url: {shown_out}"
    );
    assert!(
        shown_out.contains("omnimem_ORIGIN_TOKEN"),
        "show should carry token env-var name: {shown_out}"
    );

    // CRITICAL: inspect `.omnimem/config.toml` on disk and assert no
    // secret-shaped value landed there. The env-var NAME is fine;
    // the value is not.
    let cfg = std::fs::read_to_string(dir.path().join(".omnimem/config.toml")).unwrap();
    assert!(
        cfg.contains("omnimem_ORIGIN_TOKEN"),
        "token_env name must persist: {cfg}"
    );
    // `token =` would be the disastrous shape. Make sure that never appears.
    assert!(
        !cfg.to_lowercase().contains("token ="),
        "config.toml must NOT carry a bare token field, got: {cfg}"
    );

    // Remove one, confirm it's gone.
    omnimem(dir.path(), &["remote", "remove", "backup"])
        .assert()
        .success();
    let after = omnimem(dir.path(), &["remote", "list"]).assert().success();
    let after_out = String::from_utf8_lossy(&after.get_output().stdout).to_string();
    assert!(
        after_out.contains("origin") && !after_out.contains("backup"),
        "backup should be gone, origin survive: {after_out}"
    );
}

#[test]
fn remote_add_duplicate_fails() {
    let dir = TempDir::new().unwrap();
    omnimem(dir.path(), &["init", dir.path().to_str().unwrap()])
        .assert()
        .success();
    omnimem(
        dir.path(),
        &["remote", "add", "origin", "https://x.example/x.car"],
    )
    .assert()
    .success();
    let out = omnimem(
        dir.path(),
        &["remote", "add", "origin", "https://y.example/y.car"],
    )
    .assert()
    .failure();
    let stderr = String::from_utf8_lossy(&out.get_output().stderr).to_string();
    assert!(
        stderr.contains("already exists"),
        "duplicate-remote error expected: {stderr}"
    );
}

#[test]
fn remote_show_missing_fails() {
    let dir = TempDir::new().unwrap();
    omnimem(dir.path(), &["init", dir.path().to_str().unwrap()])
        .assert()
        .success();
    let out = omnimem(dir.path(), &["remote", "show", "origin"])
        .assert()
        .failure();
    let stderr = String::from_utf8_lossy(&out.get_output().stderr).to_string();
    assert!(
        stderr.contains("not found"),
        "missing-remote error expected: {stderr}"
    );
}

/// audit-2026-04-25 P1-4: `omnimem remote add origin file:///path` must
/// fail up-front with a hint pointing at `omnimem clone`. Previously the
/// remote was added silently and `omnimem fetch origin` later failed
/// opaquely with a builder error from the HTTP transport.
#[test]
fn remote_add_rejects_file_scheme() {
    let dir = TempDir::new().unwrap();
    omnimem(dir.path(), &["init", dir.path().to_str().unwrap()])
        .assert()
        .success();
    let out = omnimem(
        dir.path(),
        &["remote", "add", "origin", "file:///tmp/local.car"],
    )
    .assert()
    .failure();
    let stderr = String::from_utf8_lossy(&out.get_output().stderr).to_string();
    assert!(
        stderr.contains("file://"),
        "rejection should call out the scheme: {stderr}"
    );
    assert!(
        stderr.contains("omnimem clone"),
        "rejection should point at omnimem clone: {stderr}"
    );
}
