use crate::contract::IAnalyzer;
use crate::taxonomy::{
    ErrorMessage, FilePath, FilePathList, LayerNameVO, LintResultList, PatternList,
};
use async_trait::async_trait;

#[async_trait]
pub trait IArchImportProtocol: Send + Sync {
    async fn process_file_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        file_path: &FilePath,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    async fn validate_imports_present(
        &self,
        analyzer: &dyn IAnalyzer,
        file_path: &FilePath,
        root_dir: &FilePath,
        required_layers: &PatternList,
        results: &mut LintResultList,
        message_template: &ErrorMessage,
        layer_name: &LayerNameVO,
        layers_display: &PatternList,
    );
    async fn check_mandatory_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    async fn check_forbidden_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    async fn check_legacy_import_rules(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

pub struct DefaultArchImportProtocol {}

#[async_trait]
impl IArchImportProtocol for DefaultArchImportProtocol {
    async fn process_file_imports(
        &self,
        _analyzer: &dyn IAnalyzer,
        _file_path: &FilePath,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        todo!()
    }

    async fn validate_imports_present(
        &self,
        _analyzer: &dyn IAnalyzer,
        _file_path: &FilePath,
        _root_dir: &FilePath,
        _required_layers: &PatternList,
        _results: &mut LintResultList,
        _message_template: &ErrorMessage,
        _layer_name: &LayerNameVO,
        _layers_display: &PatternList,
    ) {
        todo!()
    }

    async fn check_mandatory_imports(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        todo!()
    }

    async fn check_forbidden_imports(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        todo!()
    }

    async fn check_legacy_import_rules(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        todo!()
    }
}
