// unused_import_checker — Capability for detecting unused imports.
// Implements IUnusedImportProtocol: find_unused_imports.

use crate::code_analysis::contract_unused_protocol::IUnusedProtocol;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use /* UNKNOWN: SymbolName */ crate::naming_rules::taxonomy_symbol_vo::SymbolName;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

static ALL_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r#"__all__\s*=\s*\[([^\]]*)\]"#).ok());

/// Business logic for identifying imports that are not utilized in the code.
pub struct UnusedImportRuleChecker {}

impl UnusedImportRuleChecker {
    pub fn new() -> Self {
        Self {}
    }

    fn extract_imported_aliases(content: &str) -> HashMap<String, String> {
        let mut aliases: HashMap<String, String> = HashMap::new();

        for line in content.lines() {
            let trimmed = line.trim();

            // `from x import Y as Z` or `from x import Y`
            if let Some(rest) = trimmed.strip_prefix("from ") {
                let parts: Vec<&str> = rest.splitn(2, " import ").collect();
                if parts.len() == 2 {
                    let module = parts[0].trim();
                    let names_str = parts[1].trim().trim_matches(|c| c == '(' || c == ')');
                    for name_part in names_str.split(',') {
                        let name_part = name_part.trim();
                        if name_part.contains(" as ") {
                            let alias_parts: Vec<&str> = name_part.splitn(2, " as ").collect();
                            let alias = alias_parts[1].trim().to_string();
                            let fullname = format!("{}.{}", module, alias_parts[0].trim());
                            aliases.insert(alias, fullname);
                        } else if !name_part.is_empty() {
                            let fullname = format!("{}.{}", module, name_part);
                            aliases.insert(name_part.to_string(), fullname);
                        }
                    }
                }
            }
            // `import X as Y` or `import X`
            else if let Some(rest) = trimmed.strip_prefix("import ") {
                for name_part in rest.split(',') {
                    let name_part = name_part.trim();
                    if name_part.contains(" as ") {
                        let alias_parts: Vec<&str> = name_part.splitn(2, " as ").collect();
                        let alias = alias_parts[1].trim().to_string();
                        let fullname = alias_parts[0].trim().to_string();
                        aliases.insert(alias, fullname);
                    } else if !name_part.is_empty() {
                        aliases.insert(name_part.to_string(), name_part.to_string());
                    }
                }
            }
        }

        aliases
    }

    fn extract_exported_symbols(content: &str) -> HashSet<String> {
        let mut exported: HashSet<String> = HashSet::new();
        if let Some(caps) = ALL_RE.as_ref().and_then(|re| re.captures(content)) {
            let inner = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            for item in inner.split(',') {
                let name = item.trim().trim_matches('"').trim_matches('\'').to_string();
                if !name.is_empty() {
                    exported.insert(name);
                }
            }
        }
        exported
    }

    fn extract_used_symbols(
        content: &str,
        all_imports: &HashMap<String, String>,
    ) -> HashSet<String> {
        let mut used: HashSet<String> = HashSet::new();

        // Strip import lines to avoid false positives
        let code_lines: String = content
            .lines()
            .filter(|l| {
                let t = l.trim();
                !t.starts_with("import ") && !t.starts_with("from ")
            })
            .collect::<Vec<_>>()
            .join("\n");

        for alias in all_imports.keys() {
            // Check if the alias is used as a word in the code
            let pattern = format!(r"\b{}\b", regex::escape(alias));
            if let Ok(re) = Regex::new(&pattern) {
                if re.is_match(&code_lines) {
                    used.insert(alias.clone());
                }
            }
        }

        used
    }

    /// Find unused imports in a Python file.
    pub fn find_unused_imports(&self, file_path: &str) -> Vec<String> {
        let Ok(content) = fs::read_to_string(file_path) else {
            return vec![];
        };

        let imported_aliases = Self::extract_imported_aliases(&content);
        let exported_symbols = Self::extract_exported_symbols(&content);
        let used_symbols = Self::extract_used_symbols(&content, &imported_aliases);

        imported_aliases
            .iter()
            .filter(|(alias, _fullname)| {
                // Unused if: not in used_symbols AND not in __all__ exports
                !used_symbols.contains(*alias) && !exported_symbols.contains(*alias)
            })
            .map(|(alias, _)| alias.clone())
            .collect()
    }
}

impl IUnusedProtocol for UnusedImportRuleChecker {
    fn find_unused_imports(&self, path: &FilePath) -> Vec<SymbolName> {
        self.find_unused_imports(path.value())
            .into_iter()
            .map(|s| SymbolName::new(s))
            .collect()
    }
}
