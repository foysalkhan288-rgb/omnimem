//! How to spawn the `omnimem` binary from Rust and capture its output.
//!
//! Many integration builders (build scripts, AI-agent shims, custom
//! wrappers) don't want to depend on the library surface of
//! `omnimem-core` directly; they want the friendly, stable `omnimem` CLI as
//! a subprocess and a way to feed it flags + parse its stdout.
//!
//! This example shows the pattern: locate the freshly-built binary,
//! invoke it with `--help`, and print the first few lines. Under
//! `cargo run --example shell_integration -p omnimem-cli` the binary is
//! at `target/debug/omnimem[.exe]` next to the example. In production
//! the caller should resolve it via `$PATH` or pass an absolute path
//! the operator supplied.
//!
//! See also:
//! - `docs/guide/cli.md` - user-facing command reference.
//! - `docs/guide/installation.md` - how to get `omnimem` on `$PATH`.
//! - `omnimem completions <shell>` - bash/zsh/fish/powershell/elvish.
//!
//! Run:
//! ```console
//! cargo run --example shell_integration -p omnimem-cli
//! ```

use std::path::PathBuf;
use std::process::Command;

fn locate_omnimem() -> PathBuf {
    // `cargo run --example` drops examples alongside the primary
    // binary, so resolving "../omnimem[.exe]" from the current exe's
    // directory is the reliable Windows + POSIX trick.
    let ext = if cfg!(windows) { ".exe" } else { "" };
    let exe = std::env::current_exe().expect("current_exe");
    exe.parent()
        .expect("examples/ parent")
        .parent()
        .expect("debug/ parent")
        .join(format!("omnimem{ext}"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let omnimem = locate_omnimem();
    if !omnimem.exists() {
        eprintln!(
            "error: {} not found. Run `cargo build -p omnimem-cli` before this example.",
            omnimem.display()
        );
        std::process::exit(1);
    }
    println!("locating omnimem at: {}", omnimem.display());

    // `omnimem --help` is a harmless, read-only call. No repo required.
    let output = Command::new(&omnimem).arg("--help").output()?;
    if !output.status.success() {
        eprintln!(
            "omnimem --help failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    }
    let help = String::from_utf8_lossy(&output.stdout);
    println!("---- omnimem --help (first 6 lines) ----");
    for line in help.lines().take(6) {
        println!("{line}");
    }
    println!("---- ...");
    println!("OK");
    Ok(())
}
