//! Port trait for architecture compliance checking.
//!
//! Defines the interface for scanning source trees for architecture
//! violations and applying automated fixes.

use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::shared_common::taxonomy_message_vo::ComplianceStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IArchCompliancePort: Send + Sync {
    async fn scan(&self, path: &FilePath) -> LintResultList;
    async fn apply_fix(&self, path: &FilePath) -> ComplianceStatus;
}
