//! Architecture tests for the Cargo workspace topology.

use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("testkit lives in crates/testkit")
        .to_path_buf()
}

#[test]
fn workspace_respects_dependency_direction() {
    let root = workspace_root();
    if let Err(violations) = momentum_xtask::arch::check_workspace(&root) {
        let rendered: Vec<String> = violations.iter().map(ToString::to_string).collect();
        panic!("dependency direction violations:\n{}", rendered.join("\n"));
    }
}
