// PURPOSE: SetupInstallerAdapter — infrastructure adapter for executing npm/pip install commands
//
// Installs Python linters (ruff, mypy, bandit) via `pip install --user` and
// JS linters (eslint, prettier, typescript) via `npm install -g`.
//
// The Python installer retries with `--break-system-packages` on failure to
// handle PEP 668 (externally-managed environment) errors on modern Linux distros.
// The npm installer supports `sudo` prefix for global installations that need
// elevated permissions.
use async_trait::async_trait;
use shared::project_setup::contract_setup_protocol::ISetupInstallerPort;
use shared::project_setup::taxonomy_language_vo::ProjectLanguage;
use shared::project_setup::taxonomy_setup_contract_vo::SetupError;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct SetupInstallerAdapter;

// ─── Block 2: Public Contract ─────────────────────────────
impl Default for SetupInstallerAdapter {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl SetupInstallerAdapter {
    pub fn new() -> Self {
        let _dummy = ProjectLanguage::new("rust");
        Self
    }

    async fn run_pip_command(
        &self,
        cmd: &str,
        base_args: &[&str],
        packages: &[String],
    ) -> Result<(), SetupError> {
        let status = tokio::process::Command::new(cmd)
            .args(base_args)
            .args(packages)
            .status()
            .await
            .map_err(|e| SetupError::io(e.to_string()))?;
        if status.success() {
            Ok(())
        } else {
            Err(SetupError::other(format!(
                "Command {} exited with status {:?}",
                cmd,
                status.code()
            )))
        }
    }
}

#[async_trait]
impl ISetupInstallerPort for SetupInstallerAdapter {
    async fn install_python_packages(&self, packages: &[String]) -> Result<(), SetupError> {
        let mut errors = Vec::new();

        // Try pip3, pip, python3 -m pip, python -m pip (with and without --break-system-packages)
        let candidates = [
            ("pip3", vec!["install", "--user"]),
            ("pip3", vec!["install", "--user", "--break-system-packages"]),
            ("pip", vec!["install", "--user"]),
            ("pip", vec!["install", "--user", "--break-system-packages"]),
            ("python3", vec!["-m", "pip", "install", "--user"]),
            (
                "python3",
                vec!["-m", "pip", "install", "--user", "--break-system-packages"],
            ),
            ("python", vec!["-m", "pip", "install", "--user"]),
            (
                "python",
                vec!["-m", "pip", "install", "--user", "--break-system-packages"],
            ),
        ];

        for (cmd, args) in candidates {
            match self.run_pip_command(cmd, &args, packages).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    errors.push(format!("{} {:?}: {}", cmd, args, e));
                }
            }
        }

        Err(SetupError::other(format!(
            "Failed to install Python packages. Attempted commands failed: {}",
            errors.join("; ")
        )))
    }

    async fn install_npm_packages(
        &self,
        packages: &[String],
        sudo: bool,
    ) -> Result<(), SetupError> {
        let (cmd, args) = if sudo {
            let path_val = std::env::var("PATH").unwrap_or_default();
            (
                "sudo".to_string(),
                vec![
                    "env".to_string(),
                    format!("PATH={}", path_val),
                    "npm".to_string(),
                    "install".to_string(),
                    "-g".to_string(),
                ],
            )
        } else {
            (
                "npm".to_string(),
                vec!["install".to_string(), "-g".to_string()],
            )
        };

        let status = tokio::process::Command::new(&cmd)
            .args(&args)
            .args(packages)
            .status()
            .await
            .map_err(|e| SetupError::io(e.to_string()))?;
        if status.success() {
            Ok(())
        } else {
            Err(SetupError::other(format!(
                "npm install exited with status {:?}",
                status.code()
            )))
        }
    }
}
