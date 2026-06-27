// PURPOSE: taxonomy_symbol_renamer_util — utility for in-place symbol renaming in files
use std::path::Path;

/// Simple in-place symbol renamer — replaces old_name with new_name in a single file.
/// Relaxed taxonomy rules: can be used by any layer.
pub struct SymbolRenamer;

impl SymbolRenamer {
    /// Rename a symbol in a file, returns number of replacements made
    pub fn rename_in_file(file_path: &str, old_name: &str, new_name: &str) -> usize {
        let path = Path::new(file_path);
        if !path.exists() || !path.is_file() {
            return 0;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return 0,
        };
        if !content.contains(old_name) {
            return 0;
        }
        let new_content = content.replace(old_name, new_name);
        if new_content != content && std::fs::write(path, &new_content).is_ok() {
            return 1;
        }
        0
    }

    /// Check if a symbol exists in a file
    pub fn symbol_exists(file_path: &str, symbol: &str) -> bool {
        let path = Path::new(file_path);
        if !path.exists() || !path.is_file() {
            return false;
        }
        match std::fs::read_to_string(path) {
            Ok(c) => c.contains(symbol),
            Err(_) => false,
        }
    }
}
