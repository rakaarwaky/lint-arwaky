// PURPOSE: ISetupInstallerProtocol — protocol trait for package installation
// AES402: `Result<(), String>` is replaced with `Result<(), SetupError>`
// so callers can pattern-match on specific failure modes (Io vs
// InvalidState vs Other) instead of inspecting free-form error strings.
use crate::common::taxonomy_common_vo::PatternList;
use crate::project_setup::taxonomy_setup_contract_vo::SetupError;

pub type InstallPackagesResult = Result<(), SetupError>;

pub trait ISetupInstallerProtocol: Send + Sync {
    fn install_python_packages(&self, packages: &PatternList) -> InstallPackagesResult;
    fn install_npm_packages(&self, packages: &PatternList, sudo: bool) -> InstallPackagesResult;
}
