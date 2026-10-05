use shared_common::taxonomy_tool_name_vo::ToolName;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_maintenance::contract_maintenance_protocol::ISelfUpdateProtocol;
use shared_maintenance::taxonomy_maintenance_constant::GITHUB_REPO;
use shared_maintenance::taxonomy_maintenance_vo::SelfUpdateResultVO;
use shared_maintenance::utility_maintenance_helpers;
use std::path::{Path, PathBuf};
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct SelfUpdateChecker {
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Implementation ─────────────────────
impl ISelfUpdateProtocol for SelfUpdateChecker {
    fn self_update(&self, check_only: bool) -> SelfUpdateResultVO {
        let current =
            utility_maintenance_helpers::normalize_version(shared_common::RELEASE_VERSION);
        let tag = match self.fetch_latest_tag() {
            Ok(t) => t,
            Err(e) => {
                tracing::warn!(error = %e, "self-update: cannot reach GitHub releases API");
                return SelfUpdateResultVO::error(&current, &e);
            }
        };
        let latest = utility_maintenance_helpers::normalize_version(&tag);
        if !utility_maintenance_helpers::is_newer_version(&latest, &current) {
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
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl SelfUpdateChecker {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { io }
    }

    fn fetch_latest_tag(&self) -> Result<String, String> {
        let url = format!(
            "https://api.github.com/repos/{}/releases/latest",
            GITHUB_REPO
        );
        let (stdout, stderr, success) = self.io.run_external_command_in(
            &ToolName::new("curl"),
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
        if !utility_maintenance_helpers::is_valid_tag(&tag) {
            return Err(format!("release tag {tag:?} is not a plain version string"));
        }
        Ok(tag)
    }

    fn install_dir(&self) -> Result<PathBuf, String> {
        if let Ok(home) = std::env::var("CARGO_HOME") {
            let cargo_bin = Path::new(&home).join("bin");
            if cargo_bin.is_dir() {
                return Ok(cargo_bin);
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            let cargo_bin = Path::new(&home).join(".cargo/bin");
            if cargo_bin.is_dir() {
                return Ok(cargo_bin);
            }
        }
        Ok(PathBuf::from("."))
    }

    fn install_release_binary(&self, tag: &str) -> Result<String, String> {
        let asset_url = format!(
            "https://github.com/{}/releases/download/{}/lint-arwaky-cli",
            GITHUB_REPO, tag
        );
        let checksum_url = format!(
            "https://github.com/{}/releases/download/{}/lint-arwaky-cli.sha256",
            GITHUB_REPO, tag
        );
        tracing::info!(
            target: "lint_arwaky::audit",
            event = "self_update_attempt",
            release_tag = %tag,
            source_url = %asset_url,
            "self-update download started"
        );
        let dir = self.install_dir()?;
        let target = dir.join("lint-arwaky-cli");
        let tmp = dir.join(".lint-arwaky-cli.download");
        let checksum_tmp = dir.join(".lint-arwaky-cli.sha256.tmp");
        let dir_str = dir.to_string_lossy().to_string();
        let target_str = target.to_string_lossy().to_string();
        let tmp_str = tmp.to_string_lossy().to_string();
        let checksum_tmp_str = checksum_tmp.to_string_lossy().to_string();
        let (_, stderr, success) = self.io.run_external_command_in(
            &ToolName::new("curl"),
            &["-fsSL", "-o", &tmp_str, &asset_url],
            &dir_str,
        );
        if !success {
            let _ = self.io.remove_file(&tmp);
            tracing::info!(
                target: "lint_arwaky::audit",
                event = "self_update_result",
                release_tag = %tag,
                action = "failed",
                reason = "asset_download_failed",
                "self-update download failed"
            );
            let detail = if stderr.trim().is_empty() {
                "release asset download failed"
            } else {
                stderr.trim()
            };
            return Err(detail.to_string());
        }
        let (_, _, ok) = self.io.run_external_command_in(
            &ToolName::new("curl"),
            &["-fsSL", "-o", &checksum_tmp_str, &checksum_url],
            &dir_str,
        );
        if !ok {
            let _ = self.io.remove_file(&tmp);
            tracing::info!(
                target: "lint_arwaky::audit",
                event = "self_update_result",
                release_tag = %tag,
                action = "refused",
                reason = "checksum_unavailable",
                "self-update refused installation"
            );
            return Err(
                "release has no published SHA-256 checksum; refusing to install".to_string(),
            );
        }
        let verification = self.verify_download_checksum(&tmp, &checksum_tmp);
        let _ = self.io.remove_file(&checksum_tmp);
        tracing::info!(
            target: "lint_arwaky::audit",
            event = "self_update_checksum",
            release_tag = %tag,
            checksum_verified = verification.is_ok(),
            "self-update checksum verification completed"
        );
        if let Err(error) = verification {
            let _ = self.io.remove_file(&tmp);
            tracing::info!(
                target: "lint_arwaky::audit",
                event = "self_update_result",
                release_tag = %tag,
                action = "refused",
                reason = "checksum_mismatch",
                "self-update refused installation"
            );
            return Err(error);
        }
        let (_, _, moved) = self.io.run_external_command_in(
            &ToolName::new("mv"),
            &[&tmp_str, &target_str],
            &dir_str,
        );
        if !moved {
            let _ = self.io.remove_file(&tmp);
            tracing::info!(
                target: "lint_arwaky::audit",
                event = "self_update_result",
                release_tag = %tag,
                action = "failed",
                reason = "replacement_failed",
                "self-update replacement failed"
            );
            return Err("failed to move downloaded binary into place".to_string());
        }
        let (_, _, chmodded) = self.io.run_external_command_in(
            &ToolName::new("chmod"),
            &["+x", &target_str],
            &dir_str,
        );
        if !chmodded {
            tracing::info!(
                target: "lint_arwaky::audit",
                event = "self_update_result",
                release_tag = %tag,
                action = "failed",
                reason = "chmod_failed",
                "self-update installed binary but chmod failed"
            );
            return Err("failed to make downloaded binary executable".to_string());
        }
        tracing::info!(
            target: "lint_arwaky::audit",
            event = "self_update_result",
            release_tag = %tag,
            action = "installed",
            destination = %target_str,
            "self-update replacement completed"
        );
        Ok(target_str)
    }

    /// Verify the published hash against the downloaded temporary file itself.
    /// The checksum asset may name the final binary, so it must never be passed
    /// directly to `sha256sum --check` from the install directory.
    pub fn verify_download_checksum(
        &self,
        downloaded: &Path,
        published_checksum: &Path,
    ) -> Result<(), String> {
        let checksum = self
            .io
            .read_to_string(published_checksum)
            .map_err(|e| format!("cannot read published checksum: {e}"))?;
        let expected = checksum
            .value
            .split_whitespace()
            .next()
            .filter(|hash| hash.len() == 64 && hash.bytes().all(|b| b.is_ascii_hexdigit()))
            .ok_or_else(|| "published checksum is not a valid SHA-256 digest".to_string())?;
        let directory = downloaded.parent().unwrap_or_else(|| Path::new("."));
        let filename = downloaded
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "download path has no valid filename".to_string())?;
        let rewritten = directory.join(".lint-arwaky-cli.sha256.verify");
        self.io
            .write_string(&rewritten, &format!("{expected}  {filename}\n"))
            .map_err(|e| format!("cannot prepare checksum verification: {e}"))?;

        let rewritten_filename = rewritten
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "checksum verification path has no valid filename".to_string())?;
        let directory_str = directory.to_string_lossy().to_string();
        let (stdout, stderr, verified) = self.io.run_external_command_in(
            &ToolName::new("sha256sum"),
            &["--check", rewritten_filename],
            &directory_str,
        );
        let _ = self.io.remove_file(&rewritten);
        if verified {
            Ok(())
        } else {
            let detail = if stderr.trim().is_empty() {
                stdout.trim()
            } else {
                stderr.trim()
            };
            Err(format!("checksum verification failed: {detail}"))
        }
    }
}
