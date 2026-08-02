// PURPOSE: SetupInstallerAdapter — capabilities adapter for executing npm/pip install commands
//
// Installs Python linters (ruff, mypy, bandit) via `pip install --user` and
// JS linters (eslint, prettier, typescript) via `npm install -g`.
//
// The Python installer retries with `--break-system-packages` on failure to
// handle PEP 668 (externally-managed environment) errors on modern Linux distros.
// The npm installer supports `sudo` prefix for global installations that need
// elevated permissions.

use shared::project_setup::{ISetupInstallerProtocol, InstallPackagesResult, SetupError};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SetupInstallerAdapter;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ISetupInstallerProtocol for SetupInstallerAdapter {
    fn install_python_packages(&self, packages: &[String]) -> InstallPackagesResult {
        if packages.is_empty() {
            return Ok(());
        }

        let status = std::process::Command::new("pip")
            .args(["install", "--user"])
            .args(packages)
            .status()
            .map_err(|e| SetupError::io(e.to_string()))?;
        if status.success() {
            return Ok(());
        }

        // Retry with --break-system-packages if initial attempt fails (typically PEP 668 on modern Linux)
        let status2 = std::process::Command::new("pip")
            .args(["install", "--user", "--break-system-packages"])
            .args(packages)
            .status();

        match status2 {
            Ok(s) if s.success() => Ok(()),
            _ => Err(SetupError::other(format!(
                "pip install exited with status {:?}",
                status.code()
            ))),
        }
    }

    fn install_npm_packages(&self, packages: &[String], sudo: bool) -> InstallPackagesResult {
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
            .args(packages)
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
