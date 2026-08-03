// PURPOSE: taxonomy_parser_helper — pure utility functions for import parsing
// Uses resolved_path from filesystem's barrel resolution — no local barrel logic.
use shared::common::taxonomy_name_vo::SymbolName;
use shared::filesystem::taxonomy_filesystem_vo::ImportEntry;

// ═══════════════════════════════════════════════════════════════
// ImportEntry-based functions (consume filesystem crate's parsed data)
// ═══════════════════════════════════════════════════════════════

/// Extract resolved import modules from ImportEntry list.
/// Uses `resolved_path` from filesystem's barrel resolution.
/// Falls back to `raw_path` when no barrel resolution was possible.
pub fn extract_import_modules_from_entries_resolved(
    entries: &[ImportEntry],
) -> Vec<(SymbolName, SymbolName)> {
    entries
        .iter()
        .map(|entry| {
            let raw = SymbolName::new(entry.raw_path.clone());
            let resolved = if let Some(ref resolved_path) = entry.resolved_path {
                SymbolName::new(resolved_path.to_string_lossy().to_string())
            } else {
                raw.clone()
            };
            (raw, resolved)
        })
        .collect()
}
