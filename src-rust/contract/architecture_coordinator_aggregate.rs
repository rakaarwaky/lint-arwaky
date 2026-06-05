use async_trait::async_trait;
use super::*;

#[async_trait]
pub trait ArchCoordinatorAggregate: Send + Sync {
    async fn check_compliance(&self, path: &FilePath) -> ComplianceStatus;
    async fn scan(&self, path: &FilePath) -> LintResultList;
    async fn apply_fix(&self, path: &FilePath) -> ComplianceStatus;
}
