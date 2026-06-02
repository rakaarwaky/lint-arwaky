use super::*;

pub trait ICycleAnalysisProtocol: Send + Sync {
    fn scan(&self, path: &FilePath) -> LintResultList;
    fn apply_fix(&self, path: &FilePath) -> ComplianceStatus;
    fn name(&self) -> AdapterName;
}
