// PURPOSE: ICycleAnalysisProtocol + DefaultCycleAnalysisProtocol — port trait and default impl for circular dependency detection (AES015)
use import_rules::contract_rule_protocol::IAnalyzer;
use output_report::taxonomy_result_vo::LintResultList;
use source_parsing::taxonomy_path_vo::FilePath;
use source_parsing::taxonomy_paths_vo::FilePathList;
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
