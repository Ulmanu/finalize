//! WASM client build task.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Outcome of a web-client build attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebBuildOutcome {
    /// The client compiled for `wasm32-unknown-unknown`.
    Built {
        /// Path of the produced wasm artifact.
        artifact: PathBuf,
    },
    /// The target is not installed in the active toolchain; this is an environmental
    /// limitation, not a code failure.
    SkippedMissingTarget {
        /// Diagnostic text explaining what was detected.
        detail: String,
    },
}

/// Builds the `momentum-web` client crate for `wasm32-unknown-unknown`.
///
/// # Errors
///
/// Returns the compiler diagnostic when a real compilation failure occurs (as opposed to a
/// missing target toolchain, which is reported as
/// [`WebBuildOutcome::SkippedMissingTarget`]).
pub fn build_web_client(root: &Path) -> Result<WebBuildOutcome, String> {
    if let Some(detail) = missing_target_detail() {
        return Ok(WebBuildOutcome::SkippedMissingTarget { detail });
    }

    let output = Command::new(cargo_binary())
        .current_dir(root)
        .args([
            "build",
            "-p",
            "momentum-web",
            "--no-default-features",
            "--features",
            "hydrate",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .output()
        .map_err(|error| format!("failed to spawn cargo: {error}"))?;

    if output.status.success() {
        Ok(WebBuildOutcome::Built {
            artifact: root.join("target/wasm32-unknown-unknown/debug/momentum_web.wasm"),
        })
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if looks_like_missing_target(&stderr) {
            Ok(WebBuildOutcome::SkippedMissingTarget {
                detail: stderr.trim().to_string(),
            })
        } else {
            Err(stderr.trim().to_string())
        }
    }
}

fn cargo_binary() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}

fn looks_like_missing_target(stderr: &str) -> bool {
    stderr.contains("target may not be installed")
        || stderr.contains("can't find crate for `std`")
        || stderr.contains("is not installed")
        || stderr.contains("rustup")
}

/// Detects whether the active toolchain has the wasm target available.
///
/// Returns `Some(diagnostic)` when the target is unavailable.
#[must_use]
pub fn missing_target_detail() -> Option<String> {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let sysroot = Command::new(rustc)
        .args(["--print", "sysroot"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| PathBuf::from(String::from_utf8_lossy(&output.stdout).trim().to_string()))?;

    let target_dir = sysroot
        .join("lib")
        .join("rustlib")
        .join("wasm32-unknown-unknown");
    if target_dir.is_dir() {
        None
    } else {
        Some(format!(
            "wasm32-unknown-unknown is not installed in sysroot {}",
            sysroot.display()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_missing_target_diagnostic_text() {
        assert!(looks_like_missing_target(
            "error: target may not be installed"
        ));
        assert!(looks_like_missing_target("can't find crate for `std`"));
        assert!(!looks_like_missing_target("error[E0308]: mismatched types"));
    }
}
