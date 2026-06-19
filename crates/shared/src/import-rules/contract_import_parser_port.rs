// PURPOSE: IImportParserPort — contract port trait for import parsing utilities
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::taxonomy_common_vo::LineNumber;
use crate::taxonomy_layer_vo::FileContentVO;
use crate::taxonomy_layer_vo::Identity;
use crate::taxonomy_layer_vo::LayerNameVO;
use crate::taxonomy_layer_vo::LineContentVO;

pub trait IImportParserPort: Send + Sync {
    fn resolve_scope(&self, scope: &Identity) -> (LayerNameVO, Vec<Identity>);
    fn import_matches_scope(
        &self,
        import_line: &LineContentVO,
        layer: &LayerNameVO,
        suffixes: &[Identity],
    ) -> bool;
    fn get_basename(&self, file: &FilePath) -> Identity;
    fn read_import_lines(&self, file: &FilePath) -> Vec<(LineNumber, LineContentVO)>;
    fn parse_import_lines(&self, content: &FileContentVO) -> Vec<(LineNumber, LineContentVO)>;
    fn extract_module_from_line(&self, line: &LineContentVO) -> Option<Identity>;
    fn extract_layer_from_import(&self, segment: &Identity) -> Option<LayerNameVO>;
}
