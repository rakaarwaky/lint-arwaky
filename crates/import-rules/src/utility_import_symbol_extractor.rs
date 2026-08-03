// PURPOSE: utility_import_symbol_extractor — AST-based unused import detection.
use shared::common::taxonomy_layer_vo::Identity;
use shared::filesystem::taxonomy_filesystem_vo::ImportEntry;
use std::collections::{HashMap, HashSet};

// ─── Block 2: Usage Detection (AST-based) ─────────────────

pub fn extract_used_symbols(
    file_path: &str,
    _content: &str,
    imported_aliases: &HashMap<Identity, Identity>,
    used_identifiers: &[String],
) -> HashSet<Identity> {
    let mut used = HashSet::new();
    let id_set: HashSet<&str> = used_identifiers.iter().map(|s| s.as_str()).collect();
    for alias in imported_aliases.keys() {
        if id_set.contains(alias.value()) {
            used.insert(Identity::new(alias.value()));
        }
    }
    used
}

// ═══════════════════════════════════════════════════════════════
// ImportEntry-based functions (consume filesystem crate's parsed data)
// ═══════════════════════════════════════════════════════════════

/// Extract imported aliases from ImportEntry list (replaces extract_imported_aliases).
/// Returns HashMap<alias, raw_path> — same shape as the legacy function.
pub fn extract_imported_aliases_from_entries(
    entries: &[ImportEntry],
) -> HashMap<Identity, Identity> {
    let mut aliases = HashMap::new();
    for entry in entries {
        // Skip glob imports
        if entry.is_wildcard {
            continue;
        }
        // Skip crate::, super::, self:: (Rust internal)
        if entry.raw_path.starts_with("crate::")
            || entry.raw_path.starts_with("super::")
            || entry.raw_path.starts_with("self::")
        {
            continue;
        }
        // Skip __future__ imports (Python)
        if entry.raw_path.starts_with("__future__") {
            continue;
        }
        // For grouped imports (use foo::{A, B}), each symbol is a separate alias
        if !entry.symbols.is_empty() {
            for sym in &entry.symbols {
                if sym == "*" || sym == "self" || sym == "default" || sym.is_empty() {
                    continue;
                }
                aliases.insert(
                    Identity::new(sym.clone()),
                    Identity::new(entry.raw_path.clone()),
                );
            }
        } else {
            // Single import — use last segment of raw_path as alias
            let last = entry
                .raw_path
                .rsplit("::")
                .next()
                .unwrap_or(&entry.raw_path);
            if last.is_empty() || last == "*" || last == "self" || last == "default" {
                continue;
            }
            aliases.insert(
                Identity::new(last.to_string()),
                Identity::new(entry.raw_path.clone()),
            );
        }
    }
    aliases
}

/// Extract exported symbols from ImportEntry list (replaces extract_exported_symbols).
pub fn extract_exported_symbols_from_entries(entries: &[ImportEntry]) -> HashSet<Identity> {
    let mut exported = HashSet::new();
    for entry in entries {
        if !entry.is_reexport {
            continue;
        }
        if !entry.symbols.is_empty() {
            for sym in &entry.symbols {
                if sym == "*" || sym.is_empty() {
                    if entry.is_wildcard {
                        exported.insert(Identity::new("*"));
                    }
                    continue;
                }
                exported.insert(Identity::new(sym.clone()));
            }
        } else {
            let last = entry
                .raw_path
                .rsplit("::")
                .next()
                .unwrap_or(&entry.raw_path);
            if last == "*" {
                exported.insert(Identity::new("*"));
            } else if !last.is_empty() {
                exported.insert(Identity::new(last.to_string()));
            }
        }
    }
    exported
}
