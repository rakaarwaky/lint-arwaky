use crate::di_containers::taxonomy_routing_vo::ClassDefinitionMap;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::shared_common::taxonomy_source_vo::ContentString;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

pub trait IDispatchRoutingProtocol: Send + Sync {
    fn check_capability_routing(
        &self,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

pub trait IDispatchRoutingParserProtocol: Send + Sync {
    fn strip_docstrings(&self, text: &ContentString) -> ContentString;
    fn extract_class_methods(&self, text: &ContentString) -> ClassDefinitionMap;
}
