// PURPOSE: utility_symbol_renamer — stateless utility for in-place symbol renaming in files
use std::path::Path;

/// Rename a symbol in a file, returns number of replacements made.
pub fn rename_in_file(file_path: &str, old_name: &str, new_name: &str) -> usize {
    let path = Path::new(file_path);
    if !crate::filesystem::utility_filesystem_io::path_exists(path) || !crate::filesystem::utility_filesystem_io::is_file(path) {
        return 0;
    }
    let content = match crate::filesystem::utility_filesystem_io::read_file(path) {
        Ok(c) => c,
        Err(_) => return 0,
    };
    if !content.contains(old_name) {
        return 0;
    }
    let new_content = content.replace(old_name, new_name);
    if new_content != content && crate::filesystem::utility_filesystem_io::write_file(path, new_content.as_bytes()).is_ok() {
        return 1;
    }
    0
}

/// Check if a symbol exists in a file.
pub fn symbol_exists(file_path: &str, symbol: &str) -> bool {
    let path = Path::new(file_path);
    if !crate::filesystem::utility_filesystem_io::path_exists(path) || !crate::filesystem::utility_filesystem_io::is_file(path) {
        return false;
    }
    match crate::filesystem::utility_filesystem_io::read_file(path) {
        Ok(c) => c.contains(symbol),
        Err(_) => false,
    }
}
