use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::maintenance::contract_maintenance_protocol::IDependencyReportProtocol;
use shared::maintenance::taxonomy_maintenance_vo::{DependencyInfo, DependencyReport};
use std::path::Path;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct DependencyReportChecker {
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Implementation ─────────────────────
impl IDependencyReportProtocol for DependencyReportChecker {
    fn run_dependency_report(&self, project_path: &FilePath) -> Result<DependencyReport, String> {
        let root = &project_path.value;
        let cargo_lock = Path::new(root).join("Cargo.lock");
        if cargo_lock.exists() {
            let content = self
                .io
                .read_to_string(&cargo_lock)
                .map_err(|e| e.to_string())?;
            let mut dependencies = Vec::new();
            let mut in_package = false;
            let mut pkg_name = String::new();
            let mut pkg_version = String::new();
            for line in content.value.lines() {
                let trimmed = line.trim();
                if trimmed == "[[package]]" {
                    if !pkg_name.is_empty() && !pkg_version.is_empty() {
                        dependencies.push(DependencyInfo {
                            name: pkg_name.clone(),
                            version: pkg_version.clone(),
                            dep_type: "transitive".to_string(),
                        });
                    }
                    pkg_name.clear();
                    pkg_version.clear();
                    in_package = true;
                    continue;
                }
                if in_package {
                    if let Some(v) = trimmed.strip_prefix("name = ") {
                        pkg_name = v.trim_matches('"').to_string();
                    } else if let Some(v) = trimmed.strip_prefix("version = ") {
                        pkg_version = v.trim_matches('"').to_string();
                    }
                }
            }
            if !pkg_name.is_empty() && !pkg_version.is_empty() {
                dependencies.push(DependencyInfo {
                    name: pkg_name,
                    version: pkg_version,
                    dep_type: "transitive".to_string(),
                });
            }
            Ok(DependencyReport {
                language: "Rust".to_string(),
                dependencies,
            })
        } else {
            Err("No Cargo.lock found".to_string())
        }
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl DependencyReportChecker {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { io }
    }
}
