use super::*;

pub trait IImportViolationProtocol: Send + Sync {
    fn scan(&self, path: &FilePath) -> LintResultList;
    fn apply_fix(&self, path: &FilePath) -> ComplianceStatus;
    fn name(&self) -> AdapterName;
}
