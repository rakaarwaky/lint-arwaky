// call_chain_analyzer — Call chain analysis capability for JS/TS files.
// Implements ISemanticTracerProtocol: trace_call_chain, project_wide_rename.

use crate::naming_rules::taxonomy_name_vo::SymbolName;
use crate::naming_rules::taxonomy_naming_list_vo::SymbolNameList;
use regex::Regex;
use std::fs;

const JS_EXTENSIONS: &[&str] = &[".js", ".jsx", ".ts", ".tsx", ".mjs"];

/// Call chain analyzer for JavaScript/TypeScript files.
pub struct CallChainAnalyzer {
    // Capabilities only import from taxonomy and contract(protocol)
}

impl Default for CallChainAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CallChainAnalyzer {
    pub fn new() -> Self {
        Self {
        }
    }

    /// Build all naming variants for a symbol.
    pub fn build_variants(&self, name: &SymbolName) -> SymbolNameList {
        // Simple variant builder using basic string operations
        let mut list = SymbolNameList::new();
        list.push(name.clone());
        list
    }

    /// Resolve enclosing scope using basic scope detection.
    pub fn get_enclosing_scope(&self, file_path: &str, line: usize) -> Option<String> {
        // Basic scope detection based on brace counting
        if let Ok(content) = std::fs::read_to_string(file_path) {
            let lines: Vec<&str> = content.lines().collect();
            if line == 0 || line > lines.len() {
                return None;
            }
            let mut depth = 0i32;
            let mut last_scope: Option<String> = None;
            for (i, l) in lines.iter().enumerate() {
                for c in l.chars() {
                    match c {
                        '{' => depth += 1,
                        '}' => depth -= 1,
                        _ => {}
                    }
                }
                if depth > 0 && i < line {
                    if let Some(scope_name) = Self::detect_scope_name(l) {
                        last_scope = Some(scope_name);
                    }
                }
            }
            last_scope
        } else {
            None
        }
    }

    /// Get data flow for a variable in a file.
    pub fn find_flow(
        &self,
        file_path: &str,
        var_name: &str,
        start_line: Option<usize>,
    ) -> Vec<String> {
        if let Ok(content) = std::fs::read_to_string(file_path) {
            let mut result = Vec::new();
            let pattern = match Regex::new(&format!(r"\b{}\b", regex::escape(var_name))) {
                Ok(r) => r,
                Err(_) => return result,
            };
            for (i, line) in content.lines().enumerate() {
                if let Some(sl) = start_line {
                    if i + 1 < sl {
                        continue;
                    }
                }
                if pattern.is_match(line) {
                    result.push(format!("Line {} [Usage]: {}", i + 1, line.trim()));
                }
            }
            result
        } else {
            Vec::new()
        }
    }

    /// Heuristically detect a scope name (function/class name) from a line.
    fn detect_scope_name(line: &str) -> Option<String> {
        let trimmed = line.trim();
        // Match patterns like: function foo(, class Foo, fn foo(
        let patterns = [
            (r"^\s*(?:pub\s+)?fn\s+([a-zA-Z_]\w*)", true),
            (r"^\s*(?:pub\s+)?function\s+([a-zA-Z_]\w*)", true),
            (r"^\s*(?:pub\s+)?class\s+([a-zA-Z_]\w*)", true),
            (r"^\s*impl\s+([a-zA-Z_]\w*)", true),
            (r"^\s*def\s+([a-zA-Z_]\w*)", true),
        ];
        for (pattern, _) in &patterns {
            if let Ok(re) = Regex::new(pattern) {
                if let Some(caps) = re.captures(trimmed) {
                    if let Some(name) = caps.get(1) {
                        return Some(name.as_str().to_string());
                    }
                }
            }
        }
        None
    }

    fn collect_js_files(root_dir: &str) -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(entries) = std::fs::read_dir(root_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let subfiles = Self::collect_js_files(&path.to_string_lossy());
                    files.extend(subfiles);
                } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let dot_ext = format!(".{}", ext);
                    if JS_EXTENSIONS.contains(&dot_ext.as_str()) {
                        files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
        files
    }

    /// Find all call sites for the target name within the project.
    pub fn trace_call_chain(&self, root_dir: &str, target_name: &str) -> Vec<String> {
        let call_pattern = match Regex::new(&format!(r"\b{}\s*\(", regex::escape(target_name))) {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };
        let def_pattern = match Regex::new(&format!(
            r"(?:function|class)\s+{}\b",
            regex::escape(target_name)
        )) {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };

        let js_files = Self::collect_js_files(root_dir);
        let mut callers: Vec<String> = Vec::new();

        for filepath in &js_files {
            let Ok(content) = fs::read_to_string(filepath) else {
                continue;
            };

            for (i, line) in content.lines().enumerate() {
                if call_pattern.is_match(line) && !def_pattern.is_match(line) {
                    // Compute relative path
                    let rel = filepath
                        .strip_prefix(root_dir)
                        .unwrap_or(filepath)
                        .trim_start_matches('/');
                    let call_site = format!("{}:{} -> {}", rel, i + 1, line.trim());
                    callers.push(call_site);
                }
            }
        }

        callers
    }

    /// Rename a symbol across all JS/TS files in the project.
    pub fn project_wide_rename(&self, root_dir: &str, old_name: &str, new_name: &str) -> usize {
        let pattern = match Regex::new(&format!(
            r#"(`(?:\\.|[^`\\])*`|"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|//[^\n]*|/\*(?:.|\n)*?\*/)|(\b{}\b)"#,
            regex::escape(old_name)
        )) {
            Ok(r) => r,
            Err(_) => return 0,
        };

        let js_files = Self::collect_js_files(root_dir);
        let mut modified_count = 0;

        for filepath in &js_files {
            let Ok(source) = fs::read_to_string(filepath) else {
                continue;
            };
            if !source.contains(old_name) {
                continue;
            }

            let new_source = pattern
                .replace_all(&source, |caps: &regex::Captures| {
                    if caps.get(1).is_some() {
                        caps[0].to_string()
                    } else {
                        new_name.to_string()
                    }
                })
                .to_string();

            if new_source != source && fs::write(filepath, &new_source).is_ok() {
                modified_count += 1;
            }
        }

        modified_count
    }
}
