// PURPOSE: taxonomy_unused_helper — pure utility functions for unused import detection
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_name_vo::SymbolName;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

static ALL_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r#"__all__\s*=\s*\[([^\]]*)\]"#).ok());

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
            !t.starts_with("import ") && !t.starts_with("from ") && !t.starts_with('#')
        })
        .collect::<Vec<_>>()
        .join("\n");

    for alias in imported_aliases.keys() {
        let alias_str = alias.value();
        let pattern = format!(r"\b{}\b", regex::escape(alias_str));
        if let Ok(re) = Regex::new(&pattern) {
            if re.is_match(&code_lines) {
                used.insert(Identity::new(alias_str));
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

        let names: Vec<SymbolName> = if t.starts_with("use ") {
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
    if is_rust_trait_import(name) {
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

// ─── Private Helpers ───

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
            | "Parser"
    )
}
