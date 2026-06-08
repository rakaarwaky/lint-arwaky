use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::layer_rules::contract_rule_protocol::IAnalyzer;
use async_trait::async_trait;

#[async_trait]
pub trait IRoleCheckerProtocol: Send + Sync {
    async fn check_agent_roles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    async fn check_surface_roles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
