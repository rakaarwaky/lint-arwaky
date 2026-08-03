// PURPOSE: taxonomy_parser_helper — pure utility functions for import parsing and syntax token extraction
use crate::utility_import_resolver;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::filesystem::taxonomy_filesystem_vo::ImportEntry;

// ═══════════════════════════════════════════════════════════════
// ImportEntry-based functions (consume filesystem crate's parsed data)
// ═══════════════════════════════════════════════════════════════

/// Extract import modules with barrel file resolution from ImportEntry list.
/// Replaces extract_import_modules_resolved.
pub fn extract_import_modules_from_entries_resolved(
    entries: &[ImportEntry],
    root_dir: &str,
) -> Vec<(SymbolName, SymbolName)> {
    let mut resolved_modules = Vec::new();
    for entry in entries {
        let module_path = &entry.raw_path;
        if !entry.symbols.is_empty() {
            // Grouped import — resolve each symbol
            for sym in &entry.symbols {
                if sym.is_empty() || sym == "*" || sym == "self" {
                    continue;
                }
                if let Some(resolved) =
                    utility_import_resolver::resolve_barrel_import(module_path, sym, root_dir, None)
                {
                    resolved_modules.push((
                        SymbolName::new(module_path.clone()),
                        SymbolName::new(resolved.resolved_file),
                    ));
                } else {
                    resolved_modules.push((
                        SymbolName::new(module_path.clone()),
                        SymbolName::new(module_path.clone()),
                    ));
                }
            }
        } else {
            // Single import
            let name = module_path.rsplit("::").next().unwrap_or(module_path);
            let prefix = module_path
                .rsplit_once("::")
                .map(|x| x.0)
                .unwrap_or(module_path);
            if !name.is_empty() && name != "*" {
                if let Some(resolved) =
                    utility_import_resolver::resolve_barrel_import(prefix, name, root_dir, None)
                {
                    resolved_modules.push((
                        SymbolName::new(prefix.to_string()),
                        SymbolName::new(resolved.resolved_file),
                    ));
                } else {
                    resolved_modules.push((
                        SymbolName::new(module_path.clone()),
                        SymbolName::new(module_path.clone()),
                    ));
                }
            }
        }
    }
    resolved_modules
}
