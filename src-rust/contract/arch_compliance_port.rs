use async_trait::async_trait;
use super::*;

#[async_trait]
pub trait IArchCompliancePort: Send + Sync {
    async fn scan(&self, path: &FilePath) -> LintResultList;
    async fn apply_fix(&self, path: &FilePath) -> ComplianceStatus;
}
