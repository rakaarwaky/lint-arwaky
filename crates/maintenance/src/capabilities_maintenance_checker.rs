use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::{Count, Score};
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_suggestion_vo::DescriptionVO;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::maintenance::contract_maintenance_protocol::IMaintenanceCheckerProtocol;
use shared::maintenance::taxonomy_maintenance_vo::MaintenanceStatsVO;
use shared::maintenance::taxonomy_maintenance_vo::{
    DependencyInfo, DependencyReport, DoctorResultVO, HealthCheckAdapterVO, HealthCheckResult,
    SecurityFinding, SecurityScanReport, SelfUpdateResultVO, ToolStatus, ToolchainDiagnostics,
};
use std::collections::HashMap;
use std::sync::Arc;

const GITHUB_REPO: &str = "rakaarwaky/lint-arwaky";

pub struct MaintenanceChecker {
    io: Arc<dyn IFileSystemIOProtocol>,
}

impl MaintenanceChecker {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { io }
    }

    fn check_tool(&self, name: &str, args: &[&str], required: bool) -> ToolStatus {
        let (stdout, _, success) = self.io.run_external_command_in(name, args, ".");
        let (status, version) = if success {
            let ver = stdout.lines().next().unwrap_or("").trim().to_string();
            ("OK".to_string(), ver)
        } else if required {
            ("FAIL".to_string(), "NOT FOUND".to_string())
        } else {
            ("WARN".to_string(), "NOT FOUND".to_string())
        };
        ToolStatus {
            name: name.to_string(),
            status,
            version,
        }
    }

    /// Query the GitHub releases API for the latest published tag.
    fn fetch_latest_tag(&self) -> Result<String, String> {
        let url = format!(
            "https://api.github.com/repos/{}/releases/latest",
            GITHUB_REPO
        );
        let (stdout, stderr, success) = self.io.run_external_command_in(
            "curl",
            &["-fsSL", "-H", "Accept: application/vnd.github+json", &url],
            ".",
        );
        if !success {
            let detail = if stderr.trim().is_empty() {
                "curl request failed"
            } else {
                stderr.trim()
            };
            return Err(detail.to_string());
        }
        let json: serde_json::Value = serde_json::from_str(&stdout).map_err(|e| e.to_string())?;
        let tag = json
            .get("tag_name")
            .and_then(|t| t.as_str())
            .ok_or_else(|| "tag_name missing from GitHub response".to_string())?
            .to_string();
        if !Self::is_valid_tag(&tag) {
            return Err(format!("release tag {tag:?} is not a plain version string"));
        }
        Ok(tag)
    }

    /// Accept only a plain version string: optional `v` prefix, dot-separated
    /// numeric components, optional `-prerelease` suffix.
    ///
    /// The tag reaches a download URL and a `mv` target, so anything carrying a
    /// path separator, a parent-directory segment, or a shell metacharacter is
    /// rejected rather than passed through to a subprocess argument.
    pub fn is_valid_tag(tag: &str) -> bool {
        let body = tag.strip_prefix('v').unwrap_or(tag);
        let (core, suffix) = match body.split_once('-') {
            Some((core, suffix)) => (core, Some(suffix)),
            None => (body, None),
        };
        let parts: Vec<&str> = core.split('.').collect();
        if parts.is_empty() || parts.len() > 4 {
            return false;
        }
        let core_ok = parts
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()));
        let suffix_ok = suffix.is_none_or(|s| {
            !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '.')
        });
        core_ok && suffix_ok
    }

    /// Strip a leading `v` so tags and CARGO_PKG_VERSION compare as dotted numbers.
    pub fn normalize_version(tag: &str) -> String {
        tag.trim_start_matches('v').to_string()
    }

    /// Returns true when `candidate` is a strictly newer dotted version than `current`.
    pub fn is_newer_version(candidate: &str, current: &str) -> bool {
        let parse = |v: &str| -> Vec<u64> {
            v.split('.')
                .map(|part| part.parse::<u64>().unwrap_or(0))
                .collect()
        };
        let (mut cand, mut cur) = (parse(candidate), parse(current));
        let width = cand.len().max(cur.len());
        cand.resize(width, 0);
        cur.resize(width, 0);
        cand > cur
    }

    /// Resolve the directory holding the running binary.
    ///
    /// Uses `$CARGO_HOME/bin` first (the standard cargo install location),
    /// then `$HOME/.cargo/bin`, then the current working directory.
    /// `current_exe` is intentionally avoided — its return value can be
    /// influenced by the process environment and must not be trusted for
    /// deciding where to write files.
    fn install_dir(&self) -> Result<std::path::PathBuf, String> {
        if let Ok(home) = std::env::var("CARGO_HOME") {
            let cargo_bin = std::path::Path::new(&home).join("bin");
            if cargo_bin.is_dir() {
                return Ok(cargo_bin);
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            let cargo_bin = std::path::Path::new(&home).join(".cargo/bin");
            if cargo_bin.is_dir() {
                return Ok(cargo_bin);
            }
        }
        Ok(std::path::PathBuf::from("."))
    }

    /// Download the release asset for this platform, verify its SHA-256
    /// checksum, and replace the running binary.
    ///
    /// The checksum file (`lint-arwaky-cli.sha256`) is published alongside
    /// the release asset by the release workflow. It contains one line in the
    /// format produced by `sha256sum` and is verified before any replacement.
    fn install_release_binary(&self, tag: &str) -> Result<String, String> {
        let asset_url = format!(
            "https://github.com/{}/releases/download/{}/lint-arwaky-cli",
            GITHUB_REPO, tag
        );
        let checksum_url = format!(
            "https://github.com/{}/releases/download/{}/lint-arwaky-cli.sha256",
            GITHUB_REPO, tag
        );
        let dir = self.install_dir()?;
        let target = dir.join("lint-arwaky-cli");
        let tmp = dir.join(".lint-arwaky-cli.download");
        let checksum_tmp = dir.join(".lint-arwaky-cli.sha256.tmp");
        let dir_str = dir.to_string_lossy().to_string();
        let target_str = target.to_string_lossy().to_string();
        let tmp_str = tmp.to_string_lossy().to_string();
        let checksum_tmp_str = checksum_tmp.to_string_lossy().to_string();

        // 1. Download the release asset.
        let (_, stderr, success) = self.io.run_external_command_in(
            "curl",
            &["-fsSL", "-o", &tmp_str, &asset_url],
            &dir_str,
        );
        if !success {
            let _ = self.io.remove_file(&tmp);
            let detail = if stderr.trim().is_empty() {
                "release asset download failed"
            } else {
                stderr.trim()
            };
            return Err(detail.to_string());
        }

        // 2. Download the published checksum.
        let (_, _, ok) = self.io.run_external_command_in(
            "curl",
            &["-fsSL", "-o", &checksum_tmp_str, &checksum_url],
            &dir_str,
        );
        if !ok {
            let _ = self.io.remove_file(&tmp);
            return Err(
                "release has no published SHA-256 checksum; refusing to install".to_string(),
            );
        }

        // 3. Verify the downloaded asset against the published checksum.
        let (stdout, _, verified) =
            self.io
                .run_external_command_in("sha256sum", &["--check", &checksum_tmp_str], &dir_str);
        let _ = self.io.remove_file(&checksum_tmp);
        if !verified {
            let _ = self.io.remove_file(&tmp);
            return Err(format!("checksum verification failed: {}", stdout.trim()));
        }

        // 4. Move the verified binary into place and mark it executable.
        let (_, _, moved) =
            self.io
                .run_external_command_in("mv", &[&tmp_str, &target_str], &dir_str);
        if !moved {
            let _ = self.io.remove_file(&tmp);
            return Err("failed to move downloaded binary into place".to_string());
        }

        let (_, _, chmodded) =
            self.io
                .run_external_command_in("chmod", &["+x", &target_str], &dir_str);
        if !chmodded {
            return Err("failed to make downloaded binary executable".to_string());
        }

        Ok(target_str)
    }
}

impl IMaintenanceCheckerProtocol for MaintenanceChecker {
    fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        let mut rust_tools = vec![self.check_tool("rustc", &["--version"], true)];
        rust_tools.push(self.check_tool("cargo", &["--version"], true));
        let mut clippy_status = self.check_tool("cargo", &["clippy", "--version"], true);
        clippy_status.name = "clippy".to_string();
        rust_tools.push(clippy_status);
        rust_tools.push(self.check_tool("rustfmt", &["--version"], true));
        let python_tools = vec![
            self.check_tool("python3", &["--version"], false),
            self.check_tool("ruff", &["--version"], false),
            self.check_tool("mypy", &["--version"], false),
        ];
        let mut js_tools = vec![self.check_tool("node", &["--version"], false)];
        js_tools.push(self.check_tool("eslint", &["--version"], false));
        let vcs_tools = vec![self.check_tool("git", &["--version"], true)];
        let binary_path = std::env::current_exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        ToolchainDiagnostics {
            rust_tools,
            python_tools,
            js_tools,
            vcs_tools,
            binary_path,
        }
    }

    fn health_check(&self) -> HealthCheckResult {
        // FRD FR-004: all 9 adapters must be checked
        let mut adapters = Vec::new();
        for (name, bin, args, lang) in &[
            (
                "clippy",
                "cargo",
                &["clippy", "--version"] as &[&str],
                "Rust",
            ),
            ("rustfmt", "rustfmt", &["--version"] as &[&str], "Rust"),
            (
                "cargo-audit",
                "cargo",
                &["audit", "--version"] as &[&str],
                "Rust",
            ),
            ("ruff", "ruff", &["--version"] as &[&str], "Python"),
            ("mypy", "mypy", &["--version"] as &[&str], "Python"),
            ("bandit", "bandit", &["--version"] as &[&str], "Python"),
            ("eslint", "eslint", &["--version"] as &[&str], "JS/TS"),
            ("prettier", "prettier", &["--version"] as &[&str], "JS/TS"),
            ("tsc", "tsc", &["--version"] as &[&str], "JS/TS"),
        ] {
            let status = self.check_tool(bin, args, false);
            adapters.push(HealthCheckAdapterVO {
                name: name.to_string(),
                language: lang.to_string(),
                available: status.status == "OK",
            });
        }
        HealthCheckResult { adapters }
    }

    fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport {
        let root = &project_path.value;
        let cargo_lock = std::path::Path::new(root).join("Cargo.lock");
        if cargo_lock.exists() {
            let (s, _, _) = self
                .io
                .run_external_command_in("cargo", &["audit", "--json"], root);
            let mut findings = Vec::new();
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&s) {
                if let Some(list) = json
                    .get("vulnerabilities")
                    .and_then(|v| v.get("list"))
                    .and_then(|l| l.as_array())
                {
                    for adv in list {
                        let pkg = adv
                            .get("package")
                            .and_then(|p| p.get("name"))
                            .and_then(|n| n.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        let severity = adv
                            .get("severity")
                            .and_then(|s| s.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        let cve = adv
                            .get("advisory")
                            .and_then(|a| a.get("id"))
                            .and_then(|i| i.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        findings.push(SecurityFinding {
                            severity,
                            test_id: cve,
                            file: pkg,
                            line: 0,
                            issue: "Advisory vulnerability".to_string(),
                        });
                    }
                }
            }
            SecurityScanReport {
                language: "Rust".to_string(),
                tool_name: "cargo-audit".to_string(),
                findings,
                tool_installed: true,
            }
        } else {
            SecurityScanReport {
                language: "Unknown".to_string(),
                tool_name: "none".to_string(),
                findings: Vec::new(),
                tool_installed: false,
            }
        }
    }

    fn run_dependency_report(&self, project_path: &FilePath) -> Result<DependencyReport, String> {
        let root = &project_path.value;
        let cargo_lock = std::path::Path::new(root).join("Cargo.lock");
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

    fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO {
        let root = &project_path.value;
        let root_path = std::path::Path::new(root);
        let mut total_files = 0u64;
        let mut test_files = 0u64;
        let mut python_files = 0u64;
        let mut rust_files = 0u64;
        let mut js_files = 0u64;
        for entry_path in self
            .io
            .read_dir_entries_as_pathbuf(root_path)
            .unwrap_or_default()
        {
            if entry_path.is_file() {
                total_files += 1;
                let name = entry_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                if name.contains("test") || name.contains("spec") {
                    test_files += 1;
                }
                if let Some(ext) = entry_path.extension().and_then(|e| e.to_str()) {
                    match ext {
                        "rs" => rust_files += 1,
                        "py" => python_files += 1,
                        "ts" | "js" | "jsx" | "tsx" => js_files += 1,
                        _ => {}
                    }
                }
            }
        }
        let source_count = rust_files + python_files + js_files;
        MaintenanceStatsVO {
            project_path: project_path.clone(),
            total_files: Count::new(total_files as i64),
            test_files: Count::new(test_files as i64),
            test_ratio: Score::new(if source_count > 0 {
                test_files as f64 / source_count as f64
            } else {
                0.0
            }),
            python_files: Count::new(python_files as i64),
            rust_files: Count::new(rust_files as i64),
            js_files: Count::new(js_files as i64),
        }
    }

    fn clean(&self) {
        for dir in &[
            ".pytest_cache",
            "__pycache__",
            "node_modules/.cache",
            "target",
        ] {
            let path = std::path::Path::new(".").join(dir);
            if path.exists() {
                let _ = self.io.remove_dir_all(&path);
            }
        }
    }

    fn update(&self) {
        let _ = self.io.run_external_command_in(
            "pip",
            &["install", "--upgrade", "ruff", "mypy", "bandit"],
            ".",
        );
    }

    fn self_update(&self, check_only: bool) -> SelfUpdateResultVO {
        let current = Self::normalize_version(env!("CARGO_PKG_VERSION"));
        let tag = match self.fetch_latest_tag() {
            Ok(t) => t,
            Err(e) => {
                tracing::warn!(error = %e, "self-update: cannot reach GitHub releases API");
                return SelfUpdateResultVO::error(&current, &e);
            }
        };
        let latest = Self::normalize_version(&tag);

        if !Self::is_newer_version(&latest, &current) {
            return SelfUpdateResultVO::success(&current, &tag, true);
        }
        if check_only {
            let status = format!("Update available: {current} → {tag}");
            return SelfUpdateResultVO {
                current_version: current,
                latest_version: tag,
                already_up_to_date: false,
                upgraded: false,
                status,
            };
        }
        match self.install_release_binary(&tag) {
            Ok(path) => {
                let status = format!("Installed {tag} to {path}");
                SelfUpdateResultVO {
                    current_version: current,
                    latest_version: tag,
                    already_up_to_date: false,
                    upgraded: true,
                    status,
                }
            }
            Err(e) => {
                tracing::warn!(error = %e, "self-update: install failed");
                SelfUpdateResultVO::error(&current, &e)
            }
        }
    }

    fn doctor(&self) -> DoctorResultVO {
        let tools = self.diagnose_toolchain();
        let rust_ver = tools
            .rust_tools
            .first()
            .map(|t| t.version.clone())
            .unwrap_or_default();
        let python_ver = tools
            .python_tools
            .first()
            .map(|t| t.version.clone())
            .unwrap_or_default();
        let node_ver = tools
            .js_tools
            .first()
            .map(|t| t.version.clone())
            .unwrap_or_default();
        let all_ok = tools.rust_tools.iter().all(|t| t.status == "OK");
        let mut adapter_statuses = HashMap::new();
        for t in &tools.rust_tools {
            if let Ok(name) = AdapterName::new(t.name.clone()) {
                adapter_statuses.insert(name, t.status.clone());
            }
        }
        for t in &tools.python_tools {
            if let Ok(name) = AdapterName::new(t.name.clone()) {
                adapter_statuses.insert(name, t.status.clone());
            }
        }
        DoctorResultVO {
            python_version: DescriptionVO::new(python_ver),
            rust_version: DescriptionVO::new(rust_ver),
            node_version: DescriptionVO::new(node_ver),
            is_installed: if all_ok {
                ComplianceStatus::new(true)
            } else {
                ComplianceStatus::new(false)
            },
            config_found: FilePathList::new(Vec::new()),
            adapter_statuses,
            issues: Vec::new(),
            healthy: if all_ok {
                ComplianceStatus::new(true)
            } else {
                ComplianceStatus::new(false)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_version_strips_v_prefix() {
        assert_eq!(MaintenanceChecker::normalize_version("v3.7.0"), "3.7.0");
        assert_eq!(MaintenanceChecker::normalize_version("3.7.0"), "3.7.0");
    }

    #[test]
    fn is_newer_version_basic() {
        assert!(MaintenanceChecker::is_newer_version("3.8.0", "3.7.0"));
        assert!(!MaintenanceChecker::is_newer_version("3.7.0", "3.7.0"));
        assert!(!MaintenanceChecker::is_newer_version("3.6.0", "3.7.0"));
        assert!(MaintenanceChecker::is_newer_version("4.0.0", "3.7.0"));
        assert!(MaintenanceChecker::is_newer_version("3.7.1", "3.7.0"));
    }

    #[test]
    fn is_newer_version_handles_different_lengths() {
        assert!(MaintenanceChecker::is_newer_version("3.7", "3.6.0"));
        assert!(!MaintenanceChecker::is_newer_version("3.7", "3.7.0"));
        assert!(MaintenanceChecker::is_newer_version("10.0.0", "9.0.0"));
    }

    #[test]
    fn is_valid_tag_accepts_plain_versions() {
        assert!(MaintenanceChecker::is_valid_tag("v3.7.0"));
        assert!(MaintenanceChecker::is_valid_tag("3.7.0"));
        assert!(MaintenanceChecker::is_valid_tag("v3.7"));
        assert!(MaintenanceChecker::is_valid_tag("v1.0.0-alpha.1"));
    }

    #[test]
    fn is_valid_tag_rejects_traversal_and_metacharacters() {
        // A tag that reaches a download URL and a `mv` target must never carry
        // a path separator, a parent-directory segment, or a shell metacharacter.
        assert!(!MaintenanceChecker::is_valid_tag("../../etc/passwd"));
        assert!(!MaintenanceChecker::is_valid_tag("v3.7.0/../../bin/sh"));
        assert!(!MaintenanceChecker::is_valid_tag("v3.7.0; rm -rf /"));
        assert!(!MaintenanceChecker::is_valid_tag("v3.7.0 && curl evil"));
        assert!(!MaintenanceChecker::is_valid_tag("v3.7.0$(whoami)"));
        assert!(!MaintenanceChecker::is_valid_tag("v3.7.0|nc"));
        assert!(!MaintenanceChecker::is_valid_tag(""));
        assert!(!MaintenanceChecker::is_valid_tag("v"));
        assert!(!MaintenanceChecker::is_valid_tag("v3..0"));
        assert!(!MaintenanceChecker::is_valid_tag("v3.7.0-"));
    }
}
