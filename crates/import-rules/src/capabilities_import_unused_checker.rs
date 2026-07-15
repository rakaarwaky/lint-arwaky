// PURPOSE: UnusedImportRuleChecker — IUnusedImportProtocol for AES203: detect imports that are never used in the code (Rust/Python/JS)
// AES203 rule: Every import must be used at least once in the file that declares it.
// Detection strategies:
//   - Python/standard imports: extract imported aliases → find used symbols → diff.
//   - Rust/JS imports: extract named imports → check `is_name_used` for each.
//   - Respects __all__ exports (Python) and self-use patterns.
//
// Merged from capabilities_unused_analyzer.rs — all computation logic is now inline.

use once_cell::sync::Lazy;
use regex::Regex;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_layer_vo::Identity;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Identifies imports that are declared but never used in the file (AES203).
///
/// Algorithm:
///   1. Extract all imported aliases/symbols from the source (Python `import X` / `from Y import Z`,
///      Rust `use X::Y`, JS `import X from Y`).
///   2. Extract all used symbols by scanning the file content (inline computation).
///   3. If a symbol is exported (e.g., Python `__all__`), it is NOT unused (re-export pattern).
///   4. For Rust/JS: additional extraction of named imports + per-name usage check.
///   5. Each unused import becomes an AES203 MEDIUM violation.
pub struct UnusedImportRuleChecker {
    parser: Arc<dyn IImportParserPort>,
}

impl UnusedImportRuleChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
    }
}

impl IUnusedImportProtocol for UnusedImportRuleChecker {
    /// Find all unused imports in a file (returns list of symbol names).
    ///
    /// Steps:
    ///   1. Read file content. Return empty if file can't be read.
    ///   2. Extract all imported symbols/aliases (Python standard `import X`, `from Y import Z`).
    ///   3. Extract all exported symbols (Python `__all__`, Rust `pub use`, JS `export`).
    ///   4. Analyze which imported aliases are actually used in the code body.
    ///   5. For each alias: if it's NOT used AND NOT exported → add to unused list.
    ///   6. For Rust/JS named imports (e.g., `use foo::Bar`, `import { Bar }`):
    ///      extract and check each name individually via `is_name_used`.
    ///   7. Return the collected list of unused symbol names.
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage> {
        // Step 1: Read file content
        let Ok(content_msg) = self.parser.read_file_to_message(path) else {
            return vec![];
        };
        let content = content_msg.value().to_string();

        // Step 2-4: Inline computation (merged from UnusedAnalyzer)
        let imported_aliases = self.do_extract_imported_aliases(&content);
        let exported_symbols = self.do_extract_exported_symbols(&content);
        let used_symbols = self.do_extract_used_symbols(&content, &imported_aliases);

        let mut unused: Vec<String> = Vec::new();

        // Step 5: Identify unused Python/standard imports
        for alias in imported_aliases.keys() {
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                unused.push(alias.value().to_string());
            }
        }

        // Step 6: Handle Rust/JS specific imports
        let rust_js_imports = self.do_extract_rust_js_imports(&content);
        for (name, line_idx) in rust_js_imports {
            let name_str = name.value();
            if !self.do_is_name_used(name_str, &content, line_idx.value() as usize) {
                unused.push(name_str.to_string());
            }
        }

        // AES402: return VOs, not raw strings.
        unused
            .into_iter()
            .map(LintMessage::new)
            .collect()
    }

    /// Check for unused imports and record them as lint violations.
    ///
    /// Steps:
    ///   1. Extract all imported aliases (Python-style imports).
    ///   2. Extract all exported symbols.
    ///   3. Find which aliases are actually used in the code.
    ///   4. For each unused alias (not used, not exported): find its line number and emit MEDIUM violation.
    ///   5. For Rust/JS named imports: check each name and emit MEDIUM violation if unused.
    fn check_unused_imports(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        // 1-3: Inline computation (merged from UnusedAnalyzer)
        let imported_aliases = self.do_extract_imported_aliases(content);
        let exported_symbols = self.do_extract_exported_symbols(content);
        let used_symbols = self.do_extract_used_symbols(content, &imported_aliases);

        // 4: Identify unused Python/standard imports and record violations
        for alias in imported_aliases.keys() {
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                let line_num = self
                    .parser
                    .find_import_line_number(content, alias.value())
                    .value() as usize;
                violations.push(LintResult::new_arch(
                    file,
                    line_num,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(LintMessage::new(format!(
                            "Import '{}' is declared but never used in this file.",
                            alias
                        ))),
                    }
                    .to_string(),
                ));
            }
        }

        // 5: Handle Rust/JS specific imports and record violations
        let rust_js_imports = self.do_extract_rust_js_imports(content);
        for (name, line_idx) in rust_js_imports {
            let name_str = name.value().to_string();
            let line_no_us = line_idx.value() as usize;
            if !self.do_is_name_used(&name_str, content, line_no_us) {
                violations.push(LintResult::new_arch(
                    file,
                    line_no_us,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(LintMessage::new(format!(
                            "Import '{}' is declared but never used in this file.",
                            name_str
                        ))),
                    }
                    .to_string(),
                ));
            }
        }
    }
}

// ─── Internal Methods (merged from UnusedAnalyzer) ───

static ALL_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r#"__all__\s*=\s*\[([^\]]*)\]"#).ok());

// Known derive-macro imports that Rust compiler consumes implicitly.
// These are never "used" as ordinary symbols — they're consumed by #[derive(...)]
// attributes, so they must never be flagged as unused.
const DERIVE_MACROS: &[&str] = &[
    "async_trait",
    "Serialize",
    "Deserialize",
    "Clone",
    "Debug",
    "Default",
    "PartialEq",
    "Eq",
    "Hash",
    "Ord",
    "PartialOrd",
    "Copy",
    "EnumIter",
    "Display",
    "EnumString",
    "AsRefStr",
];

fn is_rust_trait_import(name: &str) -> bool {
    if name.starts_with('I') && name.len() > 1 && name.chars().nth(1).unwrap_or(' ').is_uppercase()
    {
        return true;
    }
    if name.ends_with("Protocol")
        || name.ends_with("Port")
        || name.ends_with("Trait")
        || name.ends_with("Aggregate")
        || name.ends_with("Ext")
    {
        return true;
    }
    matches!(
        name,
        "Default"
            | "Debug"
            | "Display"
            | "Clone"
            | "Copy"
            | "PartialEq"
            | "Eq"
            | "PartialOrd"
            | "Ord"
            | "Hash"
            | "From"
            | "Into"
            | "TryFrom"
            | "TryInto"
            | "AsRef"
            | "AsMut"
            | "Deref"
            | "DerefMut"
            | "Iterator"
            | "IntoIterator"
            | "ExactSizeIterator"
            | "FusedIterator"
            | "Future"
            | "Stream"
            | "Read"
            | "Write"
            | "BufRead"
            | "Seek"
            | "Send"
            | "Sync"
            | "Unpin"
            | "Sized"
            | "Drop"
            | "Fn"
            | "FnMut"
            | "FnOnce"
            | "async_trait"
            | "Digest"
            | "Manager"
            | "Emitter"
            | "Serialize"
            | "Deserialize"
            | "EnumIter"
            | "EnumString"
            | "AsRefStr"
            | "Parser"
    )
}

impl UnusedImportRuleChecker {
    fn do_extract_imported_aliases(&self, content: &str) -> HashMap<Identity, Identity> {
        extract_imported_aliases(content)
    }

    fn do_extract_exported_symbols(&self, content: &str) -> HashSet<Identity> {
        extract_exported_symbols(content)
    }

    fn do_extract_used_symbols(
        &self,
        content: &str,
        imported_aliases: &HashMap<Identity, Identity>,
    ) -> HashSet<Identity> {
        extract_used_symbols(content, imported_aliases)
    }

    fn do_extract_rust_js_imports(&self, content: &str) -> Vec<(SymbolName, LineNumber)> {
        extract_rust_js_imports(content)
    }

    fn do_is_name_used(&self, name: &str, content: &str, exclude_line: usize) -> bool {
        is_name_used(name, content, exclude_line)
    }
}

// ─── Free Functions (merged from UnusedAnalyzer) ───

pub fn extract_imported_aliases(content: &str) -> HashMap<Identity, Identity> {
    let mut aliases = HashMap::new();
    let mut in_cfg_test = false;
    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("#[cfg(test)]") {
            in_cfg_test = true;
            continue;
        }
        if in_cfg_test {
            if trimmed == "}" || trimmed.starts_with("}") {
                in_cfg_test = false;
            }
            continue;
        }

        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some((from_part, import_part)) = trimmed.split_once(" import ") {
                let module = from_part[5..].trim();
                for name in import_part.split(',') {
                    let name = name.trim();
                    if name.is_empty() || name == "*" {
                        continue;
                    }
                    if let Some((sym, alias)) = name.split_once(" as ") {
                        aliases.insert(
                            Identity::new(alias.trim()),
                            Identity::new(format!("{}.{}", module, sym.trim())),
                        );
                    } else {
                        aliases.insert(
                            Identity::new(name),
                            Identity::new(format!("{}.{}", module, name)),
                        );
                    }
                }
            }
            continue;
        }

        // Rust `use` statements: `use std::collections::HashMap;` or `use serde::{A, B};`
        if let Some(use_part) = trimmed.strip_prefix("use ") {
            let use_part = use_part.trim_end_matches(';').trim();
            if !use_part.is_empty()
                && !use_part.starts_with("crate::")
                && !use_part.starts_with("super::")
                && !use_part.starts_with("self::")
            {
                if let Some(brace_pos) = use_part.find("::{") {
                    let prefix = &use_part[..brace_pos];
                    let inner = use_part[brace_pos + 3..].trim_end_matches('}');
                    for name in inner.split(',') {
                        let name = name.trim().split(" as ").last().unwrap_or("").trim();
                        if !name.is_empty()
                            && name != "_"
                            && name != "*"
                            && !is_rust_trait_import(name)
                        {
                            aliases.insert(
                                Identity::new(name),
                                Identity::new(format!("{}::{}", prefix, name)),
                            );
                        }
                    }
                } else {
                    let raw_name = use_part.rsplit("::").next().unwrap_or(use_part);
                    let name = raw_name.split(" as ").last().unwrap_or(raw_name).trim();
                    if !name.is_empty() && name != "*" && !is_rust_trait_import(name) {
                        aliases.insert(Identity::new(name), Identity::new(use_part));
                    }
                }
            }
            continue;
        }

        if let Some(import_part) = trimmed.strip_prefix("import ") {
            for name in import_part.split(',') {
                let name = name.trim();
                if name.is_empty() {
                    continue;
                }
                if let Some((sym, alias)) = name.split_once(" as ") {
                    aliases.insert(Identity::new(alias.trim()), Identity::new(sym.trim()));
                } else {
                    let alias = name.rsplit('.').next().unwrap_or(name);
                    aliases.insert(Identity::new(alias), Identity::new(name));
                }
            }
        }
    }
    aliases
}

pub fn extract_exported_symbols(content: &str) -> HashSet<Identity> {
    let mut exported = HashSet::new();
    let code_lines = content
        .lines()
        .filter(|l| !l.trim().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(ref re) = *ALL_RE {
        if let Some(caps) = re.captures(&code_lines) {
            if let Some(matched) = caps.get(1) {
                for item in matched.as_str().split(',') {
                    let item = item.trim().trim_matches(|c| c == '\'' || c == '"');
                    if !item.is_empty() {
                        exported.insert(Identity::new(item));
                    }
                }
            }
        }
    }
    exported
}

pub fn extract_used_symbols(
    content: &str,
    imported_aliases: &HashMap<Identity, Identity>,
) -> HashSet<Identity> {
    let mut used = HashSet::new();
    let code_lines = content
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.starts_with("import ")
                && !t.starts_with("from ")
                && !t.starts_with("use ")
                && !t.starts_with("pub use ")
                && !t.starts_with("pub(crate) use ")
        })
        .collect::<Vec<_>>()
        .join("\n");

    for alias in imported_aliases.keys() {
        let alias_str = alias.value();
        if DERIVE_MACROS.contains(&alias_str) {
            used.insert(Identity::new(alias_str));
        }
    }

    let non_derive_aliases: Vec<&str> = imported_aliases
        .keys()
        .map(|a| a.value())
        .filter(|a| !DERIVE_MACROS.contains(a))
        .collect();

    if !non_derive_aliases.is_empty() && !code_lines.is_empty() {
        let patterns: Vec<String> = non_derive_aliases
            .iter()
            .map(|a| regex::escape(a))
            .collect();
        let combined = format!(r"\b({})\b", patterns.join("|"));
        if let Ok(re) = Regex::new(&combined) {
            let matched_set: HashSet<&str> =
                re.find_iter(&code_lines).map(|m| m.as_str()).collect();
            for alias in non_derive_aliases {
                if matched_set.contains(alias) {
                    used.insert(Identity::new(alias));
                }
            }
        }
    }

    used
}

pub fn extract_rust_js_imports(content: &str) -> Vec<(SymbolName, LineNumber)> {
    let mut imports = Vec::new();
    let mut in_cfg_test = false;
    for (i, line) in content.lines().enumerate() {
        let t = line.trim();
        if t.starts_with("#[cfg(test)]") {
            in_cfg_test = true;
            continue;
        }
        if in_cfg_test {
            if t == "}" || t.starts_with("}") {
                in_cfg_test = false;
            }
            continue;
        }

        let names: Vec<SymbolName> = if t.starts_with("use ")
            || t.starts_with("pub use ")
            || t.starts_with("pub(crate) use ")
        {
            let target = t.trim_end_matches(';').trim_start_matches("use ").trim();
            if target.starts_with("std::")
                || target.starts_with("core::")
                || target.starts_with("alloc::")
            {
                continue;
            }
            if let Some(brace_pos) = target.find("::{") {
                let inner = target[brace_pos + 3..].trim_end_matches('}').trim();
                inner
                    .split(',')
                    .map(|s| {
                        s.trim()
                            .split(" as ")
                            .last()
                            .unwrap_or("")
                            .trim()
                            .to_string()
                    })
                    .filter(|n| !n.is_empty() && n != "_" && n != "*")
                    .map(SymbolName::new)
                    .collect()
            } else {
                let name = target
                    .split("::")
                    .last()
                    .unwrap_or("")
                    .split(" as ")
                    .last()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                if name.is_empty() || name == "_" || name == "*" {
                    continue;
                }
                vec![SymbolName::new(name)]
            }
        } else if t.starts_with("import ") {
            if let Some(from_idx) = t.find(" from ") {
                let import_part = t[7..from_idx].trim();
                let names: Vec<SymbolName> = if import_part.starts_with('{') {
                    import_part[1..import_part.len() - 1]
                        .split(',')
                        .map(|s| {
                            s.trim()
                                .split(" as ")
                                .last()
                                .unwrap_or("")
                                .trim()
                                .to_string()
                        })
                        .filter(|n| !n.is_empty())
                        .map(SymbolName::new)
                        .collect()
                } else {
                    vec![SymbolName::new(import_part.trim())]
                };
                names
            } else {
                continue;
            }
        } else {
            continue;
        };

        for name in names {
            let s = name.value();
            if (s.starts_with('I') && s.len() > 1 && s.chars().nth(1).unwrap_or(' ').is_uppercase())
                || s.ends_with("Protocol")
                || s.ends_with("Port")
                || s.ends_with("Trait")
                || s.ends_with("Aggregate")
                || s == "Parser"
            {
                continue;
            }
            imports.push((name, LineNumber::new(i as i64 + 1)));
        }
    }
    imports
}

pub fn is_name_used(name: &str, content: &str, exclude_line: usize) -> bool {
    if is_rust_trait_import(name) || DERIVE_MACROS.contains(&name) {
        return true;
    }

    let rest = content
        .lines()
        .enumerate()
        .filter(|(j, _)| *j != exclude_line)
        .map(|(_, l)| l)
        .collect::<Vec<_>>()
        .join("\n");
    rest.contains(name)
}

// ─── Tests (merged from UnusedAnalyzer) ───

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_macro_serialize_always_used() {
        let content = r#"
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(
            Identity::new("Serialize"),
            Identity::new("serde::Serialize"),
        );
        aliases.insert(
            Identity::new("Deserialize"),
            Identity::new("serde::Deserialize"),
        );

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("Serialize")),
            "Serialize should always be considered used"
        );
        assert!(
            used.contains(&Identity::new("Deserialize")),
            "Deserialize should always be considered used"
        );
    }

    #[test]
    fn derive_macro_async_trait_always_used() {
        let content = r#"
use async_trait::async_trait;

#[async_trait]
trait MyTrait {
    async fn do_something();
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(
            Identity::new("async_trait"),
            Identity::new("async_trait::async_trait"),
        );

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("async_trait")),
            "async_trait should always be considered used"
        );
    }

    #[test]
    fn derive_macro_enum_iter_always_used() {
        // EnumIter was NOT previously in is_rust_trait_import — only DERIVE_MACROS catches it
        let content = r#"
use strum::{EnumIter, Display};

#[derive(EnumIter, Display)]
enum Color {
    Red,
    Green,
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(Identity::new("EnumIter"), Identity::new("strum::EnumIter"));
        aliases.insert(Identity::new("Display"), Identity::new("strum::Display"));

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("EnumIter")),
            "EnumIter should always be considered used"
        );
        assert!(
            used.contains(&Identity::new("Display")),
            "Display should always be considered used"
        );
    }

    #[test]
    fn derive_macro_as_ref_str_always_used() {
        // AsRefStr was NOT previously in is_rust_trait_import — only DERIVE_MACROS catches it
        let content = r#"
use strum::AsRefStr;

#[derive(AsRefStr)]
enum Status {
    Active,
    Inactive,
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(Identity::new("AsRefStr"), Identity::new("strum::AsRefStr"));

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("AsRefStr")),
            "AsRefStr should always be considered used"
        );
    }

    #[test]
    fn non_derive_import_still_checked_normally() {
        // Regular imports should NOT be auto-marked as used
        let content = r#"
use std::collections::HashMap;

fn main() {
    let _x = 42;
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(
            Identity::new("HashMap"),
            Identity::new("std::collections::HashMap"),
        );

        let used = extract_used_symbols(content, &aliases);
        assert!(
            !used.contains(&Identity::new("HashMap")),
            "HashMap should not be auto-marked as used"
        );
    }
}
