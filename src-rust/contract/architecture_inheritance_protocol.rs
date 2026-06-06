use crate::contract::architecture_rule_protocol::IAnalyzer;
use crate::taxonomy::{FilePath, FilePathList, LintResultList};
use async_trait::async_trait;

#[async_trait]
pub trait IArchInheritanceProtocol: Send + Sync {
    async fn check_mandatory_inheritance(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
