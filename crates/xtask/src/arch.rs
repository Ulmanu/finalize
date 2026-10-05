//! Workspace dependency-direction enforcement.

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// A workspace member and the workspace-local crates it may depend on.
#[derive(Debug, Clone, Copy)]
pub struct MemberSpec {
    /// Cargo package name.
    pub name: &'static str,
    /// Member directory relative to the workspace root.
    pub dir: &'static str,
    /// Allowed regular dependencies (workspace-local package names).
    pub allowed_deps: &'static [&'static str],
    /// Allowed dev-dependencies (workspace-local package names).
    pub allowed_dev_deps: &'static [&'static str],
}

/// The expected workspace layout and allowed dependency edges.
pub const MEMBERS: &[MemberSpec] = &[
    MemberSpec {
        name: "momentum-domain",
        dir: "crates/domain",
        allowed_deps: &[],
        allowed_dev_deps: &[],
    },
    MemberSpec {
        name: "momentum-app",
        dir: "crates/app",
        allowed_deps: &["momentum-domain"],
        allowed_dev_deps: &["momentum-testkit"],
    },
    MemberSpec {
        name: "momentum-infra",
        dir: "crates/infra",
        allowed_deps: &["momentum-domain", "momentum-app"],
        allowed_dev_deps: &["momentum-testkit"],
    },
    MemberSpec {
        name: "momentum-web",
        dir: "crates/web",
        allowed_deps: &["momentum-domain", "momentum-app", "momentum-infra"],
        allowed_dev_deps: &["momentum-testkit"],
    },
    MemberSpec {
        name: "momentum-testkit",
        dir: "crates/testkit",
        allowed_deps: &["momentum-domain", "momentum-app"],
        allowed_dev_deps: &["momentum-xtask"],
    },
    MemberSpec {
        name: "momentum-xtask",
        dir: "crates/xtask",
        allowed_deps: &[],
        allowed_dev_deps: &[],
    },
];

/// A single dependency-direction violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    /// Member whose manifest contains the offending dependency.
    pub member: String,
    /// Offending workspace-local dependency.
    pub dependency: String,
    /// Whether the edge appeared in `[dev-dependencies]`.
    pub dev: bool,
    /// Human-readable explanation.
    pub detail: String,
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} -> {} ({}): {}",
            self.member,
            self.dependency,
            if self.dev {
                "dev-dependency"
            } else {
                "dependency"
            },
            self.detail
        )
    }
}

/// Verifies that every workspace member only depends on allowed workspace-local crates and
/// that every listed member manifest exists.
///
/// # Errors
///
/// Returns the collected [`Violation`] list when a manifest is missing, unreadable, or
/// declares a forbidden workspace-local dependency.
pub fn check_workspace(root: &Path) -> Result<(), Vec<Violation>> {
    let mut violations = Vec::new();
    let member_names: BTreeSet<&str> = MEMBERS.iter().map(|spec| spec.name).collect();

    for spec in MEMBERS {
        let manifest_path = manifest_path(root, spec);
        let manifest = match fs::read_to_string(&manifest_path) {
            Ok(contents) => contents,
            Err(error) => {
                violations.push(Violation {
                    member: spec.name.to_string(),
                    dependency: String::new(),
                    dev: false,
                    detail: format!("cannot read {}: {error}", manifest_path.display()),
                });
                continue;
            }
        };

        let parsed: toml::Value = match toml::from_str(&manifest) {
            Ok(value) => value,
            Err(error) => {
                violations.push(Violation {
                    member: spec.name.to_string(),
                    dependency: String::new(),
                    dev: false,
                    detail: format!("cannot parse {}: {error}", manifest_path.display()),
                });
                continue;
            }
        };

        check_section(
            &parsed,
            spec,
            "dependencies",
            false,
            &member_names,
            &mut violations,
        );
        check_section(
            &parsed,
            spec,
            "dev-dependencies",
            true,
            &member_names,
            &mut violations,
        );
    }

    check_root_members(root, &member_names, &mut violations);

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

fn check_root_members(root: &Path, member_names: &BTreeSet<&str>, violations: &mut Vec<Violation>) {
    let root_manifest = root.join("Cargo.toml");
    let Ok(contents) = fs::read_to_string(&root_manifest) else {
        violations.push(Violation {
            member: "workspace".to_string(),
            dependency: String::new(),
            dev: false,
            detail: format!("cannot read {}", root_manifest.display()),
        });
        return;
    };
    let Ok(parsed) = toml::from_str::<toml::Value>(&contents) else {
        violations.push(Violation {
            member: "workspace".to_string(),
            dependency: String::new(),
            dev: false,
            detail: format!("cannot parse {}", root_manifest.display()),
        });
        return;
    };

    let Some(members) = parsed
        .get("workspace")
        .and_then(|workspace| workspace.get("members"))
        .and_then(toml::Value::as_array)
    else {
        violations.push(Violation {
            member: "workspace".to_string(),
            dependency: String::new(),
            dev: false,
            detail: "[workspace].members is missing".to_string(),
        });
        return;
    };

    let declared: BTreeSet<String> = members
        .iter()
        .filter_map(toml::Value::as_str)
        .map(ToString::to_string)
        .collect();
    let expected: BTreeSet<String> = MEMBERS.iter().map(|spec| spec.dir.to_string()).collect();

    for missing in expected.difference(&declared) {
        violations.push(Violation {
            member: "workspace".to_string(),
            dependency: missing.clone(),
            dev: false,
            detail: "expected workspace member is not declared".to_string(),
        });
    }
    for extra in declared.difference(&expected) {
        if !member_names.iter().any(|name| extra.contains(name)) {
            violations.push(Violation {
                member: "workspace".to_string(),
                dependency: extra.clone(),
                dev: false,
                detail: "unexpected workspace member".to_string(),
            });
        }
    }
}

/// Returns the manifest path for a member spec.
#[must_use]
pub fn manifest_path(root: &Path, spec: &MemberSpec) -> PathBuf {
    root.join(spec.dir).join("Cargo.toml")
}

fn check_section(
    parsed: &toml::Value,
    spec: &MemberSpec,
    section: &str,
    dev: bool,
    member_names: &BTreeSet<&str>,
    violations: &mut Vec<Violation>,
) {
    let Some(table) = parsed.get(section).and_then(toml::Value::as_table) else {
        return;
    };

    let allowed = if dev {
        spec.allowed_dev_deps
    } else {
        spec.allowed_deps
    };

    for (dependency, value) in table {
        if !member_names.contains(dependency.as_str()) {
            continue;
        }
        if allowed.contains(&dependency.as_str()) {
            continue;
        }
        // Optional dependencies that appear in the dev set are feature-gated adapters
        // (for example the in-memory stores behind `local` mode): they are allowed because
        // they never become hard production edges.
        let optional = value
            .as_table()
            .and_then(|entry| entry.get("optional"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);
        if !dev && optional && spec.allowed_dev_deps.contains(&dependency.as_str()) {
            continue;
        }
        violations.push(Violation {
            member: spec.name.to_string(),
            dependency: dependency.clone(),
            dev,
            detail: format!(
                "not in the allowed {} set {:?} for this crate",
                if dev { "dev-dependency" } else { "dependency" },
                allowed
            ),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workspace_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("workspace root")
            .to_path_buf()
    }

    #[test]
    fn repository_workspace_respects_dependency_direction() {
        let root = workspace_root();
        if let Err(violations) = check_workspace(&root) {
            let rendered: Vec<String> = violations.iter().map(ToString::to_string).collect();
            panic!("dependency direction violations:\n{}", rendered.join("\n"));
        }
    }

    #[test]
    fn missing_manifest_is_reported() {
        let temp = std::env::temp_dir().join("momentum-arch-missing-manifest");
        let _ = fs::remove_dir_all(&temp);
        let violations = check_workspace(&temp).expect_err("missing manifests must fail");
        assert!(violations.len() >= MEMBERS.len());
    }
}
