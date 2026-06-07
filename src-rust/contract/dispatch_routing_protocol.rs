use crate::taxonomy::{ClassDefinitionMap, ContentString, FilePath, FilePathList, LintResultList};

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
