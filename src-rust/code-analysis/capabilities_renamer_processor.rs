// symbol_renamer_processor — Capability for project-wide symbol renaming.
// Implements ISymbolRenamerProtocol: rename_symbol across the codebase.

use regex::Regex;
use std::fs;

/// Business logic for renaming symbols across the entire codebase.
pub struct SymbolRenamerProcessor {}

impl Default for SymbolRenamerProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolRenamerProcessor {
    pub fn new() -> Self {
        Self {}
    }

    fn collect_files(root_dir: &str) -> Vec<String> {
        let mut files = Vec::new();
        Self::walk_dir(root_dir, &mut files);
        files
    }

    fn walk_dir(dir: &str, files: &mut Vec<String>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let sub = path.to_string_lossy().to_string();
                    // Skip hidden dirs and common non-source dirs
                    let dirname = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
                    if !dirname.starts_with('.')
                        && !matches!(dirname, "node_modules" | "target" | "__pycache__" | ".git")
                    {
                        Self::walk_dir(&sub, files);
                    }
                } else {
                    files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    /// Rename a symbol across all files in root_dir.
    /// Ignores strings, comments, and template literals.
    /// Returns count of modified files.
    pub fn rename_symbol(&self, root_dir: &str, old_name: &str, new_name: &str) -> usize {
        // Pattern that skips strings and comments, captures word boundary matches
        let pattern = match Regex::new(&format!(
            r#"(?x)
            (
                \"\"\"(?:\\.|[^\\])*?\"\"\"  |   # Python triple-double
                \'\'\'(?:\\.|[^\\])*?\'\'\'  |   # Python triple-single
                \"(?:\\.|[^\"\\])*\"         |   # Double-quoted string
                \'(?:\\.|[^\'\\])*\'         |   # Single-quoted string
                `(?:\\.|[^`\\])*`            |   # Template literal
                \#[^\n]*                     |   # Python comment
                //[^\n]*                     |   # JS/Rust line comment
                /\*(?:.|\n)*?\*/                 # Block comment
            )
            |
            \b({old})\b
            "#,
            old = regex::escape(old_name)
        )) {
            Ok(r) => r,
            Err(_) => return 0,
        };

        let files = Self::collect_files(root_dir);
        let mut modified_count = 0;

        for file_path in &files {
            let Ok(source) = fs::read_to_string(file_path) else {
                continue;
            };

            if !source.contains(old_name) {
                continue;
            }

            let new_source = pattern
                .replace_all(&source, |caps: &regex::Captures| {
                    if caps.get(1).is_some() {
                        // Matched a string/comment — preserve as-is
                        caps[0].to_string()
                    } else {
                        // Matched the symbol — replace it
                        new_name.to_string()
                    }
                })
                .to_string();

            if new_source != source
                && fs::write(file_path, &new_source).is_ok() {
                    modified_count += 1;
                }
        }

        modified_count
    }
}
