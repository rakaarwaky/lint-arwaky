// Responsibilities: forward links, definitions, implementations, cycles, orphans

use crate::filesystem::taxonomy_filesystem_vo::{
    DefinitionEntry, FileEntry, ImplEntry, ImportEntry,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub trait IGraphProtocol: Send + Sync {
    fn build(
        &mut self,
        imports: &[ImportEntry],
        files: &[FileEntry],
        definitions: &[DefinitionEntry],
        implementations: &[ImplEntry],
    );
    fn dependency_graph(&self) -> &HashMap<PathBuf, Vec<PathBuf>>;
    fn symbol_definitions(&self) -> &HashMap<String, Vec<PathBuf>>;
    fn implementations(&self) -> &HashMap<String, Vec<PathBuf>>;
    fn dependents(&self, path: &Path) -> Vec<PathBuf>;
    fn dependencies(&self, path: &Path) -> Vec<PathBuf>;
    fn reachable(&self, from: &Path, to: &Path) -> bool;
    fn reverse_links(&self) -> &HashMap<PathBuf, Vec<PathBuf>>;
}
