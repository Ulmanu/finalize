//! `cargo xtask`-style workspace automation entry point.

use std::path::Path;
use std::process::ExitCode;

use momentum_xtask::arch;
use momentum_xtask::webbuild::{build_web_client, WebBuildOutcome};

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        print_usage();
        return ExitCode::from(2);
    };

    match command.as_str() {
        "arch-check" => {
            let root = workspace_root();
            match arch::check_workspace(&root) {
                Ok(()) => {
                    println!("architecture check passed: dependency direction is valid");
                    ExitCode::SUCCESS
                }
                Err(violations) => {
                    eprintln!("architecture check failed:");
                    for violation in violations {
                        eprintln!("  - {violation}");
                    }
                    ExitCode::FAILURE
                }
            }
        }
        "web-build" => {
            let root = workspace_root();
            match build_web_client(&root) {
                Ok(WebBuildOutcome::Built { artifact }) => {
                    println!("wasm client built: {}", artifact.display());
                    ExitCode::SUCCESS
                }
                Ok(WebBuildOutcome::SkippedMissingTarget { detail }) => {
                    println!("SKIPPED: wasm32-unknown-unknown unavailable ({detail})");
                    ExitCode::SUCCESS
                }
                Err(detail) => {
                    eprintln!("web build failed:\n{detail}");
                    ExitCode::FAILURE
                }
            }
        }
        "--help" | "-h" | "help" => {
            print_usage();
            ExitCode::SUCCESS
        }
        other => {
            eprintln!("unknown xtask command: {other}");
            print_usage();
            ExitCode::from(2)
        }
    }
}

fn print_usage() {
    println!(
        "usage: cargo xtask <command>\n\ncommands:\n  arch-check   verify workspace dependency direction\n  web-build    build the WASM client (reports missing target as SKIPPED)"
    );
}

fn workspace_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("xtask lives in crates/xtask")
        .to_path_buf()
}
