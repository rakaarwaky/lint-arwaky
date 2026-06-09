use crate::layer_rules::contract_rule_protocol::IAnalyzer;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;

#[async_trait]
pub trait ICycleAnalysisProtocol: Send + Sync {
    async fn check_cycles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

pub struct DefaultCycleAnalysisProtocol {}

#[async_trait]
impl ICycleAnalysisProtocol for DefaultCycleAnalysisProtocol {
    async fn check_cycles(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        todo!()
    }
}
