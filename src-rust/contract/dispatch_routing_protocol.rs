use crate::taxonomy::{FilePath, FilePathList, LintResultList, ClassDefinitionMap};

pub trait IDispatchRoutingProtocol: Send + Sync {
    fn check_capability_routing(&self, files: &FilePathList, root_dir: &FilePath, results: &mut LintResultList);
}

pub trait IDispatchRoutingParserProtocol: Send + Sync {
    fn strip_docstrings(&self, text: &str) -> String;
    fn extract_class_methods(&self, text: &str) -> ClassDefinitionMap;
}
