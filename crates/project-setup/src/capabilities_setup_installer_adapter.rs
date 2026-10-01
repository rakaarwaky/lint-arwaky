// PURPOSE: SetupInstallerAdapter — capabilities adapter for executing npm/pip install commands
//
// Installs Python linters (ruff, mypy, bandit) via `pip install --user` and
// JS linters (eslint, prettier, typescript) via `npm install -g`.
//
// The Python installer retries with `--break-system-packages` on failure to
// handle PEP 668 (externally-managed environment) errors on modern Linux distros.
// The npm installer supports `sudo` prefix for global installations that need
// elevated permissions.

use shared_common::taxonomy_common_vo::PatternList;
use shared_common::taxonomy_job_vo::SuccessStatus;
use shared_project_setup::contract_setup_protocol::IAdapterInstallationProtocol;
use shared_project_setup::contract_setup_protocol::InstallPackagesResult;
use shared_project_setup::taxonomy_project_setup_vo::SetupError;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SetupInstallerAdapter;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IAdapterInstallationProtocol for SetupInstallerAdapter {
    fn install_python_packages(&self, packages: &PatternList) -> InstallPackagesResult {
        if packages.is_empty() {
            return Ok(());
        }

        let status = std::process::Command::new("pip")
            .args(["install", "--user"])
            .args(packages.values())
            .status()
            .map_err(|e| SetupError::io(e.to_string()))?;
        if status.success() {
            return Ok(());
        }

        // Retry with --break-system-packages if initial attempt fails (typically PEP 668 on modern Linux)
        let status2 = std::process::Command::new("pip")
            .args(["install", "--user", "--break-system-packages"])
            .args(packages.values())
            .status();

        match status2 {
            Ok(s) if s.success() => Ok(()),
            _ => Err(SetupError::other(format!(
                "pip install exited with status {:?}",
                status.code()
            ))),
        }
    }

    fn install_npm_packages(&self, packages: &PatternList, sudo: bool) -> InstallPackagesResult {
        if packages.is_empty() {
            return Ok(());
        }

        let (cmd, args) = if sudo {
            ("sudo", vec!["npm", "install", "-g"])
        } else {
            ("npm", vec!["install", "-g"])
        };

        let status = std::process::Command::new(cmd)
            .args(args)
            .args(packages.values())
            .status()
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

    /// Install the Python adapter set (ruff, mypy, bandit).
    fn install_python_adapters(&self) -> SuccessStatus {
        let res = self.install_python_packages(&PatternList::new(vec!["ruff", "mypy", "bandit"]));
        SuccessStatus::new(res.is_ok())
    }

    /// Install the JavaScript adapter set (eslint, prettier, typescript).
    fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus {
        let res = self.install_npm_packages(
            &PatternList::new(vec!["eslint", "prettier", "typescript"]),
            sudo,
        );
        SuccessStatus::new(res.is_ok())
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl SetupInstallerAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SetupInstallerAdapter {
    fn default() -> Self {
        Self::new()
    }
}
