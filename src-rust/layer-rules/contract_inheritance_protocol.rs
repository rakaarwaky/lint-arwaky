// PURPOSE: IArchInheritanceProtocol + DefaultArchInheritanceProtocol — port trait for AES014: mandatory contract inheritance checks
use crate::layer_rules::contract_rule_protocol::IAnalyzer;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
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

pub struct DefaultArchInheritanceProtocol {}

#[async_trait]
impl IArchInheritanceProtocol for DefaultArchInheritanceProtocol {
    async fn check_mandatory_inheritance(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        todo!()
    }
}
