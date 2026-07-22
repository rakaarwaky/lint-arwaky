use shared::project_setup::contract_setup_protocol::ISetupInstallerProtocol;
use shared::project_setup::taxonomy_setup_contract_vo::SetupError;

// PURPOSE: SetupInstallerAdapter — capabilities adapter for executing npm/pip install commands
//
// Installs Python linters (ruff, mypy, bandit) via `pip install --user` and
// JS linters (eslint, prettier, typescript) via `npm install -g`.
//
// The Python installer retries with `--break-system-packages` on failure to
// handle PEP 668 (externally-managed environment) errors on modern Linux distros.
// The npm installer supports `sudo` prefix for global installations that need
// elevated permissions.
use async_trait::async_trait;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SetupInstallerAdapter;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl ISetupInstallerProtocol for SetupInstallerAdapter {
    async fn install_python_packages(&self, packages: &[String]) -> Result<(), SetupError> {
        let status = tokio::process::Command::new("pip")
            .args(["install", "--user"])
            .args(packages)
            .status()
            .await
            .map_err(|e| SetupError::io(e.to_string()))?;
        if status.success() {
            return Ok(());
        }

        // Retry with --break-system-packages if initial attempt fails (typically PEP 668 on modern Linux)
        let status2 = tokio::process::Command::new("pip")
            .args(["install", "--user", "--break-system-packages"])
            .args(packages)
            .status()
            .await;

        match status2 {
            Ok(s) if s.success() => Ok(()),
            _ => Err(SetupError::other(format!(
                "pip install exited with status {:?}",
                status.code()
            ))),
        }
    }

    async fn install_npm_packages(
        &self,
        packages: &[String],
        sudo: bool,
    ) -> Result<(), SetupError> {
        let (cmd, args) = if sudo {
            ("sudo", vec!["npm", "install", "-g"])
        } else {
            ("npm", vec!["install", "-g"])
        };
        let status = tokio::process::Command::new(cmd)
            .args(args)
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for SetupInstallerAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl SetupInstallerAdapter {
    pub fn new() -> Self {
        Self
    }
}
