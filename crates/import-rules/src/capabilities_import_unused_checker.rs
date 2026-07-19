// PURPOSE: UnusedImportRuleChecker — IUnusedImportProtocol for AES203: detect imports that are never used in the code (Rust/Python/JS)
// AES203 rule: Every import must be used at least once in the file that declares it.

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
use shared::import_rules::taxonomy_import_constant::DERIVE_MACROS;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

static ALL_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r#"__all__\s*=\s*\[([^\]]*)\]"#).ok());

// ─── Block 1: Struct Definition ───────────────────────────
/// Identifies imports that are declared but never used in the file (AES203).
pub struct UnusedImportRuleChecker {
    parser: Arc<dyn IImportParserPort>,
}

// ─── Block 2: Public Contract (IUnusedImportProtocol) ───

impl IUnusedImportProtocol for UnusedImportRuleChecker {
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage> {
        let Ok(content_msg) = self.parser.read_file_to_message(path) else {
            return vec![];
        };
        let content = content_msg.value().to_string();
        let imported_aliases = self.extract_imported_aliases(&content);
        let exported_symbols = self.extract_exported_symbols(&content);
        let used_symbols = self.extract_used_symbols(&content, &imported_aliases);
        let mut unused: Vec<String> = Vec::new();
        for alias in imported_aliases.keys() {
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                unused.push(alias.value().to_string());
            }
        }
        let rust_js_imports = self.extract_rust_js_imports(&content);
        for (name, line_idx) in rust_js_imports {
            if !self.is_name_used(&name, &content, line_idx) {
                unused.push(name.value().to_string());
            }
        }
        unused.into_iter().map(LintMessage::new).collect()
    }

    fn check_unused_imports(
        &self,
        file: &FilePath,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let imported_aliases = self.extract_imported_aliases(content);
        let exported_symbols = self.extract_exported_symbols(content);
        let used_symbols = self.extract_used_symbols(content, &imported_aliases);
        for alias in imported_aliases.keys() {
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                let line_num = self
                    .parser
                    .find_import_line_number(content, alias.value())
                    .value() as usize;
                violations.push(LintResult::new_arch(
                    file.value(),
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
        let rust_js_imports = self.extract_rust_js_imports(content);
        for (name, line_idx) in rust_js_imports {
            if !self.is_name_used(&name, content, line_idx.clone()) {
                violations.push(LintResult::new_arch(
                    file.value(),
                    line_idx.value() as usize,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(LintMessage::new(format!(
                            "Import '{}' is declared but never used in this file.",
                            name.value()
                        ))),
                    }
                    .to_string(),
                ));
            }
        }
    }
}

// ─── Block 3: Constructors & Private Helpers ───

impl UnusedImportRuleChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
    }

    fn is_rust_trait_import(&self, name: &SymbolName) -> bool {
        let name_str = name.value();
        if name_str.starts_with('I')
            && name_str.len() > 1
            && name_str.chars().nth(1).unwrap_or(' ').is_uppercase()
        {
            return true;
        }
        if name_str.ends_with("Protocol")
            || name_str.ends_with("Port")
            || name_str.ends_with("Trait")
            || name_str.ends_with("Aggregate")
            || name_str.ends_with("Ext")
        {
            return true;
        }
        matches!(
            name_str,
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

    fn extract_imported_aliases(&self, content: &str) -> HashMap<Identity, Identity> {
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
                                && !self.is_rust_trait_import(&SymbolName::new(name))
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
                        if !name.is_empty()
                            && name != "*"
                            && !self.is_rust_trait_import(&SymbolName::new(name))
                        {
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

    fn extract_exported_symbols(&self, content: &str) -> HashSet<Identity> {
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

    fn extract_used_symbols(
        &self,
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

    fn extract_rust_js_imports(&self, content: &str) -> Vec<(SymbolName, LineNumber)> {
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
                if (s.starts_with('I')
                    && s.len() > 1
                    && s.chars().nth(1).unwrap_or(' ').is_uppercase())
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

    fn is_name_used(&self, name: &SymbolName, content: &str, exclude_line: LineNumber) -> bool {
        if self.is_rust_trait_import(name) || DERIVE_MACROS.contains(&name.value()) {
            return true;
        }

        let rest = content
            .lines()
            .enumerate()
            .filter(|(j, _)| *j != exclude_line.value() as usize)
            .map(|(_, l)| l)
            .collect::<Vec<_>>()
            .join("\n");
        rest.contains(name.value())
    }
}
