// PURPOSE: taxonomy_parser_helper — pure utility functions for import parsing and syntax token extraction
use crate::common::taxonomy_name_vo::SymbolName;
use crate::import_rules::utility_import_resolver;

pub fn extract_import_modules(content: &str) -> Vec<SymbolName> {
    let mut modules = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("from ") {
            if let Some(module) = rest.split_whitespace().next() {
                modules.push(SymbolName::new(module));
            }
        } else if trimmed.starts_with("import ") {
            if let Some(pos) = trimmed.rfind(" from ") {
                let module_part = trimmed[pos + 6..].trim();
                let cleaned = module_part
                    .trim_end_matches(';')
                    .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                    .trim();
                modules.push(SymbolName::new(cleaned));
            } else if let Some(rest) = trimmed.strip_prefix("import ") {
                if rest.contains('"') || rest.contains('\'') || rest.contains('`') {
                    let cleaned = rest
                        .trim_end_matches(';')
                        .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                        .trim();
                    modules.push(SymbolName::new(cleaned));
                } else if let Some(first_token) = rest.split_whitespace().next() {
                    modules.push(SymbolName::new(first_token.trim_end_matches(',')));
                }
            }
        } else if let Some(rest) = trimmed.strip_prefix("use ") {
            let module = rest.trim_end_matches(';');
            modules.push(SymbolName::new(module));
        }
    }
    modules
}

/// Extract import modules with barrel file resolution.
/// When an import targets a barrel file (__init__.py, index.ts, mod.rs),
/// resolves the symbol to its original source file for accurate layer detection.
///
/// Handles ALL three language patterns:
///   - Python:  from mypackage import PaymentService
///   - Rust:    use crate::features::AuthOrchestrator;
///   - TS/JS:   import { UserService } from './services';
///
/// # Returns
/// Vec of (original_module, resolved_module) pairs.
/// If no barrel resolution needed, resolved_module == original_module.
pub fn extract_import_modules_resolved(
    content: &str,
    root_dir: &str,
) -> Vec<(SymbolName, SymbolName)> {
    let mut resolved_modules = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // ── Python: from X import Y ──
        if let Some(rest) = trimmed.strip_prefix("from ") {
            if let Some((module, import_part)) = rest.split_once(" import ") {
                let module = module.trim();
                for name in import_part.split(',') {
                    let name = name.trim().split(" as ").next().unwrap_or("").trim();
                    if name.is_empty() || name == "*" {
                        continue;
                    }
                    // Try barrel resolution
                    if let Some(resolved) =
                        utility_import_resolver::resolve_barrel_import(module, name, root_dir)
                    {
                        resolved_modules.push((
                            SymbolName::new(module),
                            SymbolName::new(resolved.resolved_file),
                        ));
                    } else {
                        resolved_modules.push((SymbolName::new(module), SymbolName::new(module)));
                    }
                }
            }
            continue;
        }

        // ── Rust: use crate::module::Type; / use module::{A, B}; ──
        if trimmed.starts_with("use ")
            || trimmed.starts_with("pub use ")
            || trimmed.starts_with("pub(crate) use ")
        {
            let use_part = trimmed
                .trim_start_matches("pub(crate) use ")
                .trim_start_matches("pub use ")
                .trim_start_matches("use ")
                .trim_end_matches(';')
                .trim();

            // Strip crate:: / super:: / self:: prefix for barrel lookup
            let module_path = use_part
                .strip_prefix("crate::")
                .or_else(|| use_part.strip_prefix("super::"))
                .or_else(|| use_part.strip_prefix("self::"))
                .unwrap_or(use_part);

            if let Some(brace_pos) = module_path.find("::{") {
                // use module::{A, B};
                let prefix = &module_path[..brace_pos];
                let inner = module_path[brace_pos + 3..].trim_end_matches('}');
                for name in inner.split(',') {
                    let name = name.trim().split(" as ").last().unwrap_or("").trim();
                    if name.is_empty() || name == "*" || name == "self" {
                        continue;
                    }
                    // Try barrel resolution
                    if let Some(resolved) =
                        utility_import_resolver::resolve_barrel_import(prefix, name, root_dir)
                    {
                        resolved_modules.push((
                            SymbolName::new(prefix),
                            SymbolName::new(resolved.resolved_file),
                        ));
                    } else {
                        resolved_modules.push((SymbolName::new(prefix), SymbolName::new(prefix)));
                    }
                }
            } else {
                // use module::Type;
                let name = module_path.rsplit("::").next().unwrap_or("").trim();
                let prefix = module_path
                    .rsplit_once("::")
                    .map(|x| x.0)
                    .unwrap_or(module_path);
                if !name.is_empty() && name != "*" {
                    // Try barrel resolution
                    if let Some(resolved) =
                        utility_import_resolver::resolve_barrel_import(prefix, name, root_dir)
                    {
                        resolved_modules.push((
                            SymbolName::new(prefix),
                            SymbolName::new(resolved.resolved_file),
                        ));
                    } else {
                        resolved_modules.push((SymbolName::new(prefix), SymbolName::new(prefix)));
                    }
                }
            }
            continue;
        }

        // ── TS/JS: import { X, Y } from './module'; ──
        if trimmed.starts_with("import ") && trimmed.contains(" from ") {
            if let Some(from_pos) = trimmed.rfind(" from ") {
                let module_part = trimmed[from_pos + 6..].trim();
                let module = module_part
                    .trim_end_matches(';')
                    .trim_matches(|c: char| c == '\'' || c == '"' || c == '`')
                    .trim_start_matches("./")
                    .trim_start_matches("../");

                let import_part = &trimmed[7..from_pos]; // after "import "
                if import_part.contains('{') {
                    // import { A, B } from './module'
                    if let Some(open) = import_part.find('{')
                        && let Some(close) = import_part.find('}') {
                            let inner = &import_part[open + 1..close];
                            for name in inner.split(',') {
                                let name = name.trim().split(" as ").last().unwrap_or("").trim();
                                if name.is_empty() {
                                    continue;
                                }
                                // Try barrel resolution
                                if let Some(resolved) =
                                    utility_import_resolver::resolve_barrel_import(
                                        module, name, root_dir,
                                    )
                                {
                                    resolved_modules.push((
                                        SymbolName::new(module),
                                        SymbolName::new(resolved.resolved_file),
                                    ));
                                } else {
                                    resolved_modules
                                        .push((SymbolName::new(module), SymbolName::new(module)));
                                }
                            }
                        }
                } else {
                    // import X from './module'
                    let name = import_part.trim();
                    if !name.is_empty() && name != "default" {
                        // Try barrel resolution
                        if let Some(resolved) =
                            utility_import_resolver::resolve_barrel_import(module, name, root_dir)
                        {
                            resolved_modules.push((
                                SymbolName::new(module),
                                SymbolName::new(resolved.resolved_file),
                            ));
                        } else {
                            resolved_modules
                                .push((SymbolName::new(module), SymbolName::new(module)));
                        }
                    }
                }
            }
            continue;
        }

        // ── JS: const { X } = require('./module'); ──
        if trimmed.starts_with("const ") && trimmed.contains("require(")
            && let Some(req_start) = trimmed.find("require(") {
                let after = &trimmed[req_start + 8..];
                if let Some(paren_end) = after.find(')') {
                    let req_module = after[..paren_end]
                        .trim_matches(|c: char| c == '\'' || c == '"' || c == '`')
                        .trim_start_matches("./")
                        .trim_start_matches("../");

                    if let Some(eq_pos) = trimmed.find('=') {
                        let left = trimmed[6..eq_pos].trim(); // after "const "
                        if left.starts_with('{') && left.ends_with('}') {
                            let inner = &left[1..left.len() - 1];
                            for name in inner.split(',') {
                                let name = name.trim().split(':').next_back().unwrap_or("").trim();
                                if name.is_empty() {
                                    continue;
                                }
                                // Try barrel resolution
                                if let Some(resolved) =
                                    utility_import_resolver::resolve_barrel_import(
                                        req_module, name, root_dir,
                                    )
                                {
                                    resolved_modules.push((
                                        SymbolName::new(req_module),
                                        SymbolName::new(resolved.resolved_file),
                                    ));
                                } else {
                                    resolved_modules.push((
                                        SymbolName::new(req_module),
                                        SymbolName::new(req_module),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
    }

    resolved_modules
}
