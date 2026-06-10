// PURPOSE: IImportParserPort — contract port trait for import parsing utilities (scope resolution, line parsing, layer extraction)

use crate::shared_common::{Identity, LineNumber, FileContentVO, LayerNameVO, LineContentVO};
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IImportParserPort: Send + Sync {
    /// Resolve a scope value (e.g. "contract(protocol)", "taxonomy(entity,error,event)")
    /// into layer + suffix matches. Returns (LayerNameVO, Vec<Identity>).
    fn resolve_scope(&self, scope: &Identity) -> (LayerNameVO, Vec<Identity>);

    /// Check if an import line satisfies the given scope requirement.
    fn import_matches_scope(&self, import_line: &LineContentVO, layer: &LayerNameVO, suffixes: &[Identity]) -> bool;

    fn get_basename(&self, file: &FilePath) -> Identity;

    fn read_import_lines(&self, file: &FilePath) -> Vec<(LineNumber, LineContentVO)>;

    fn parse_import_lines(&self, content: &FileContentVO) -> Vec<(LineNumber, LineContentVO)>;

    fn extract_module_from_line(&self, line: &LineContentVO) -> Option<Identity>;

    fn extract_layer_from_import(&self, segment: &Identity) -> Option<LayerNameVO>;
}
