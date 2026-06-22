// PURPOSE: SetupInstallerAdapter — infrastructure adapter for executing npm/pip install commands
use async_trait::async_trait;
use shared::project_setup::contract_setup_protocol::ISetupInstallerPort;
use shared::project_setup::taxonomy_language_vo::ProjectLanguage;

pub struct SetupInstallerAdapter;

impl Default for SetupInstallerAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl SetupInstallerAdapter {
    pub fn new() -> Self {
        let _dummy = ProjectLanguage::new("rust");
        Self
    }
}

#[async_trait]
impl ISetupInstallerPort for SetupInstallerAdapter {
    async fn install_python_packages(&self, packages: &[String]) -> Result<(), String> {
        let status = tokio::process::Command::new("pip")
            .args(["install", "--user"])
            .args(packages)
            .status()
            .await
            .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err(format!(
                "pip install exited with status {:?}",
                status.code()
            ))
        }
    }

    async fn install_npm_packages(&self, packages: &[String], sudo: bool) -> Result<(), String> {
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
            .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err(format!(
                "npm install exited with status {:?}",
                status.code()
            ))
        }
    }
}
