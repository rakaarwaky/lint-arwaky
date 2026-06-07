//! Port trait for architecture compliance checking.
//!
//! Defines the interface for scanning source trees for architecture
//! violations and applying automated fixes.

use async_trait::async_trait;
use crate::taxonomy::{FilePath, LintResultList, ComplianceStatus};


#[async_trait]
pub trait IArchCompliancePort: Send + Sync {
    async fn scan(&self, path: &FilePath) -> LintResultList;
    async fn apply_fix(&self, path: &FilePath) -> ComplianceStatus;
}
