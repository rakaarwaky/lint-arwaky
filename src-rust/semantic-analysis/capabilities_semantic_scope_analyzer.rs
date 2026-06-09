// aes: wired-by-dispatch
// semantic_scope_analyzer — AST-based semantic scope analysis capability.
// Implements ISemanticTracerProtocol for Python code analysis.
// Uses regex-based analysis (no Python AST dependency).
use crate::naming_rules::taxonomy_name_vo::SymbolName;
use crate::naming_rules::taxonomy_naming_list_vo::SymbolNameList;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::shared_common::taxonomy_common_vo::DataFlowList;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_lint_vo::ScopeRef;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

/// Split a name into lowercase words by camelCase, PascalCase, underscores, hyphens.
/// Does NOT use regex look-around (unsupported in Rust's regex crate).
fn split_name_into_words(name: &str) -> Vec<String> {
    if name.is_empty() {
        return Vec::new();
    }
    let name = name.trim_matches(|c: char| c == '_' || c == '-');
    if name.is_empty() {
        return Vec::new();
    }

    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut chars = name.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '_' || ch == '-' {
            if !current.is_empty() {
                words.push(current.clone().to_lowercase());
                current.clear();
            }
            continue;
        }

        if ch.is_uppercase() {
            // End current word if it's not empty
            if !current.is_empty() {
                // Check if this uppercase starts a new word or continues an acronym
                let next_is_lower = chars.peek().is_some_and(|n| n.is_lowercase());
                let prev_is_upper = current.chars().last().is_some_and(|c| c.is_uppercase());

                if next_is_lower || (prev_is_upper && current.len() > 1) {
                    words.push(current.clone().to_lowercase());
                    current.clear();
                }
            }
            current.push(ch);
        } else if ch.is_lowercase() || ch.is_numeric() {
            current.push(ch);
        } else {
            // Skip other characters
            if !current.is_empty() {
                words.push(current.clone().to_lowercase());
                current.clear();
            }
        }
    }

    if !current.is_empty() {
        words.push(current.to_lowercase());
    }

    words
}

/// Regex to detect Python function definitions.
static PY_DEF_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^(?:async\s+)?def\s+(\w+)\s*\(").ok());

/// Regex to detect Python class definitions.
static PY_CLASS_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r"^class\s+(\w+)").ok());

/// AST-based semantic scope analyzer for Python code.
pub struct SemanticScopeAnalyzer {}

impl Default for SemanticScopeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticScopeAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Split a name into lowercase words by camelCase, PascalCase, underscores, hyphens.
    fn split_words(name: &str) -> Vec<String> {
        split_name_into_words(name)
    }

    /// Capitalize the first character of a string.
    fn capitalize(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + &c.collect::<String>(),
        }
    }

    /// Generate snake_case, camelCase, PascalCase, SCREAMING_SNAKE variants.
    pub fn get_variant_dict(&self, name: &SymbolName) -> serde_json::Value {
        let n = &name.value;
        let words = Self::split_words(n);

        if words.is_empty() {
            return serde_json::json!({
                "snake_case": n,
                "pascal_case": n,
                "camel_case": n,
                "screaming_snake": n.to_uppercase()
            });
        }

        let snake_case = words.join("_");
        let pascal_case: String = words.iter().map(|w| Self::capitalize(w)).collect();
        let camel_case = if words.len() > 1 {
            let mut c = words[0].clone();
            for w in &words[1..] {
                c.push_str(&Self::capitalize(w));
            }
            c
        } else {
            words[0].clone()
        };
        let screaming_snake = snake_case.to_uppercase();

        serde_json::json!({
            "snake_case": snake_case,
            "camel_case": camel_case,
            "pascal_case": pascal_case,
            "screaming_snake": screaming_snake
        })
    }

    /// Returns a SymbolNameList of all possible naming variants.
    pub fn build_variants(&self, name: &SymbolName) -> SymbolNameList {
        let dict = self.get_variant_dict(name);
        let mut results: HashSet<String> = HashSet::new();
        results.insert(name.value.clone());

        if let Some(v) = dict.get("snake_case").and_then(|v| v.as_str()) {
            results.insert(v.to_string());
        }
        if let Some(v) = dict.get("camel_case").and_then(|v| v.as_str()) {
            results.insert(v.to_string());
        }
        if let Some(v) = dict.get("pascal_case").and_then(|v| v.as_str()) {
            results.insert(v.to_string());
        }
        if let Some(v) = dict.get("screaming_snake").and_then(|v| v.as_str()) {
            results.insert(v.to_string());
        }
        // Add kebab-case variant
        if let Some(v) = dict.get("snake_case").and_then(|v| v.as_str()) {
            results.insert(v.replace('_', "-"));
        }

        SymbolNameList {
            values: results.into_iter().map(SymbolName::new).collect(),
        }
    }

    /// AST-based enclosing scope lookup.
    pub fn get_enclosing_scope(&self, file_path: &FilePath, line: LineNumber) -> Option<ScopeRef> {
        let target_line = line.value;
        if target_line <= 0 {
            return None;
        }

        let content = std::fs::read_to_string(file_path.to_string()).ok()?;
        let lines: Vec<&str> = content.lines().collect();
        let py_class_re = match &*PY_CLASS_RE {
            Some(r) => r,
            None => return None,
        };
        let py_def_re = match &*PY_DEF_RE {
            Some(r) => r,
            None => return None,
        };

        // Stack of (scope_name, start_line, end_line)
        let mut scope_stack: Vec<(String, i64, i64)> = Vec::new();
        let mut best_match: Vec<String> = Vec::new();

        for (i, raw_line) in lines.iter().enumerate() {
            let current_line = (i as i64) + 1;
            let stripped = raw_line.trim();

            // Pop expired scopes
            scope_stack.retain(|&(_, _, end)| end >= current_line);

            // Detect new scope
            if let Some(cap) = py_class_re.captures(stripped) {
                if let Some(name) = cap.get(1) {
                    let scope_name = format!("class {}", name.as_str());
                    let indent = raw_line.len() - raw_line.trim_start().len();
                    let mut end_line = lines.len() as i64;
                    for (j, l) in lines.iter().enumerate().skip(i + 1) {
                        if !l.trim().is_empty() {
                            let l_indent = l.len() - l.trim_start().len();
                            if l_indent <= indent
                                && (py_class_re.is_match(l.trim()) || py_def_re.is_match(l.trim()))
                            {
                                end_line = j as i64;
                                break;
                            }
                        }
                    }
                    scope_stack.push((scope_name, current_line, end_line));
                    if current_line <= target_line && target_line <= end_line {
                        best_match = scope_stack.iter().map(|(n, _, _)| n.clone()).collect();
                    }
                }
            } else if let Some(cap) = py_def_re.captures(stripped) {
                if let Some(name) = cap.get(1) {
                    let scope_name = format!("def {}", name.as_str());
                    let indent = raw_line.len() - raw_line.trim_start().len();
                    let mut end_line = lines.len() as i64;
                    for (j, l) in lines.iter().enumerate().skip(i + 1) {
                        if !l.trim().is_empty() {
                            let l_indent = l.len() - l.trim_start().len();
                            if l_indent <= indent
                                && (py_class_re.is_match(l.trim()) || py_def_re.is_match(l.trim()))
                            {
                                end_line = j as i64;
                                break;
                            }
                        }
                    }
                    scope_stack.push((scope_name, current_line, end_line));
                    if current_line <= target_line && target_line <= end_line {
                        best_match = scope_stack.iter().map(|(n, _, _)| n.clone()).collect();
                    }
                }
            }
        }

        if !best_match.is_empty() {
            Some(ScopeRef {
                name: crate::shared_common::taxonomy_suggestion_vo::DescriptionVO::new(
                    best_match.join(" -> "),
                ),
                kind: crate::shared_common::taxonomy_suggestion_vo::DescriptionVO::new(
                    String::new(),
                ),
                file: Some(file_path.clone()),
                start_line: None,
                end_line: None,
            })
        } else {
            None
        }
    }

    /// Stub for symbol location retrieval.
    pub fn get_symbol_locations(
        &self,
        _file_path: &FilePath,
        _symbol: &SymbolName,
    ) -> Vec<serde_json::Value> {
        Vec::new()
    }

    /// AST-based data flow analysis (assignments, usages, mutations).
    pub fn find_flow(
        &self,
        file_path: &FilePath,
        var_name: &SymbolName,
        start_line: LineNumber,
    ) -> DataFlowList {
        let content = match std::fs::read_to_string(file_path.to_string()) {
            Ok(c) => c,
            Err(_) => return DataFlowList { values: Vec::new() },
        };

        let lines: Vec<&str> = content.lines().collect();
        let vn = &var_name.value;
        let py_def_re = match &*PY_DEF_RE {
            Some(r) => r,
            None => return DataFlowList { values: Vec::new() },
        };
        let py_class_re = match &*PY_CLASS_RE {
            Some(r) => r,
            None => return DataFlowList { values: Vec::new() },
        };
        let sl = start_line.value as usize;

        // Determine target scope bounds
        let (scope_start, scope_end) = if sl > 0 {
            let mut start = sl;
            let mut end = lines.len();

            for (i, raw_line) in lines.iter().enumerate() {
                let line_no = i + 1;
                let stripped = raw_line.trim();
                if line_no <= sl {
                    if py_def_re.is_match(stripped) || py_class_re.is_match(stripped) {
                        start = line_no;
                    }
                } else if !stripped.is_empty() {
                    let indent = raw_line.len() - raw_line.trim_start().len();
                    if indent == 0
                        && (py_def_re.is_match(stripped) || py_class_re.is_match(stripped))
                    {
                        end = line_no;
                        break;
                    }
                }
            }

            (Some(start), Some(end))
        } else {
            (None, None)
        };

        let assign_re = match Regex::new(&format!(r"\b{}\s*=", regex::escape(vn))) {
            Ok(r) => r,
            Err(_) => return DataFlowList { values: Vec::new() },
        };
        let mutation_re = match Regex::new(&format!(r"\b{}\.\w+", regex::escape(vn))) {
            Ok(r) => r,
            Err(_) => return DataFlowList { values: Vec::new() },
        };
        let word_re = match Regex::new(&format!(r"\b{}\b", regex::escape(vn))) {
            Ok(r) => r,
            Err(_) => return DataFlowList { values: Vec::new() },
        };

        let mut flows: Vec<ErrorMessage> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();

        for (i, line_str) in lines.iter().enumerate() {
            let line_no = (i as i64) + 1;

            if let Some(s) = scope_start {
                if (line_no as usize) < s {
                    continue;
                }
            }
            if let Some(e) = scope_end {
                if (line_no as usize) > e {
                    break;
                }
            }

            if !word_re.is_match(line_str) {
                continue;
            }

            let stripped = line_str.trim();
            let entry_str = if mutation_re.is_match(line_str) {
                let method = mutation_re
                    .find(line_str)
                    .and_then(|m| m.as_str().split('.').nth(1))
                    .unwrap_or("mutation");
                format!("Line {} [Mutation '{}']: {}", line_no, method, stripped)
            } else if assign_re.is_match(line_str) {
                format!("Line {} [Assignment]: {}", line_no, stripped)
            } else {
                format!("Line {} [Usage]: {}", line_no, stripped)
            };

            if seen.contains(&entry_str) {
                continue;
            }
            seen.insert(entry_str.clone());
            flows.push(ErrorMessage::new(entry_str));
        }

        flows.sort_by_key(|a| extract_lineno(&a.value));
        DataFlowList { values: flows }
    }

    /// Project-wide call chain tracing.
    pub fn trace_call_chain(
        &self,
        root_dir: &DirectoryPath,
        target_name: &SymbolName,
    ) -> Vec<SymbolName> {
        let name = &target_name.value;
        let call_re = match Regex::new(&format!(r"\b{}\s*\(", regex::escape(name))) {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };
        let def_re = match Regex::new(&format!(r"def\s+{}\s*\(", regex::escape(name))) {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };

        let mut callers: Vec<SymbolName> = Vec::new();

        if let Ok(py_files) = collect_py_files(&root_dir.value) {
            for filepath in &py_files {
                let content = match std::fs::read_to_string(filepath) {
                    Ok(c) => c,
                    Err(_) => continue,
                };

                for (i, line) in content.lines().enumerate() {
                    if call_re.is_match(line) && !def_re.is_match(line) {
                        let rel_path = filepath.strip_prefix(&root_dir.value).unwrap_or(filepath);
                        callers.push(SymbolName::new(format!(
                            "{}:{} -> {}",
                            rel_path,
                            i + 1,
                            line.trim()
                        )));
                    }
                }
            }
        }

        callers
    }

    /// Project-wide symbol rename with string/comment awareness.
    pub fn project_wide_rename(
        &self,
        root_dir: &DirectoryPath,
        old_name: &SymbolName,
        new_name: &SymbolName,
    ) -> Count {
        let old = &old_name.value;
        let new = &new_name.value;

        let pattern = match Regex::new(&format!(
            r#"(?s)("""[\s\S]*?"""|'''[\s\S]*?'''|"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|#[^\n]*)|\b{}\b"#,
            regex::escape(old)
        )) {
            Ok(r) => r,
            Err(_) => return Count::new(0),
        };

        let mut modified_count: i64 = 0;

        if let Ok(py_files) = collect_py_files(&root_dir.value) {
            for filepath in &py_files {
                if let Ok(content) = std::fs::read_to_string(filepath) {
                    if content.contains(old) {
                        let new_source = pattern
                            .replace_all(&content, |caps: &regex::Captures| {
                                if caps.get(1).is_some() {
                                    caps.get(0).map(|m| m.as_str()).unwrap_or("").to_string()
                                } else {
                                    new.to_string()
                                }
                            })
                            .to_string();
                        if new_source != content && std::fs::write(filepath, &new_source).is_ok() {
                            modified_count += 1;
                        }
                    }
                }
            }
        }

        Count::new(modified_count)
    }
}

/// Extract line number from a flow string like "Line 5 [Assignment]: ..."
fn extract_lineno(fstr: &str) -> i64 {
    fstr.split("Line ")
        .nth(1)
        .and_then(|s| s.split(' ').next())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

/// Recursively collect all .py files under a directory.
fn collect_py_files(dir: &str) -> Result<Vec<String>, std::io::Error> {
    let mut files = Vec::new();
    collect_py_files_recursive(dir, &mut files)?;
    Ok(files)
}

fn collect_py_files_recursive(dir: &str, files: &mut Vec<String>) -> Result<(), std::io::Error> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_py_files_recursive(&path.to_string_lossy(), files)?;
        } else if path.extension().map(|e| e == "py").unwrap_or(false) {
            files.push(path.to_string_lossy().to_string());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{extract_lineno, SemanticScopeAnalyzer, SymbolName};

    #[test]
    fn test_split_words() {
        let words = SemanticScopeAnalyzer::split_words("getVariantDict");
        assert_eq!(words, vec!["get", "variant", "dict"]);
    }

    #[test]
    fn test_split_words_underscore() {
        let words = SemanticScopeAnalyzer::split_words("snake_case_name");
        assert_eq!(words, vec!["snake", "case", "name"]);
    }

    #[test]
    fn test_extract_lineno() {
        assert_eq!(extract_lineno("Line 5 [Assignment]: x = 1"), 5);
        assert_eq!(extract_lineno("no line here"), 0);
    }

    #[test]
    fn test_get_variant_dict() {
        let analyzer = SemanticScopeAnalyzer::new();
        let name = SymbolName::new("getUserName");
        let dict = analyzer.get_variant_dict(&name);
        assert_eq!(dict["snake_case"], "get_user_name");
        assert_eq!(dict["camel_case"], "getUserName");
        assert_eq!(dict["pascal_case"], "GetUserName");
        assert_eq!(dict["screaming_snake"], "GET_USER_NAME");
    }
}
