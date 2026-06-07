use crate::contract::IAnalyzer;
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

pub struct DefaultArchInheritanceProtocol;

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
