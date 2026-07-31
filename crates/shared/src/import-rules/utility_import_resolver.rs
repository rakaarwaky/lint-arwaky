use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::{Identity, LayerNameVO, LineContentVO};
use crate::common::taxonomy_path_vo::FilePath;
use crate::import_rules::taxonomy_resolved_import_vo::ResolvedImport;
use crate::import_rules::utility_path_normalizer;
use crate::orphan_detector::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use std::collections::HashMap;
use std::path::Path;

/// Convert a Result<FilePath, _> to FilePath, using default on error.
pub fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()
}

/// Convert an optional OsStr reference to a string slice.
pub fn os_str_to_str(opt: Option<&std::ffi::OsStr>) -> &str {
    opt.and_then(|o| o.to_str()).map_or("", |s| s)
}

/// Parse import lines from file content using AST.
/// Replaces regex/line-based parse_import_lines_helper.
pub fn parse_import_lines_helper(
    file_path: &str,
    content: &str,
) -> Vec<(LineNumber, LineContentVO)> {
    let mut result = Vec::new();
    match FileParseResultVO::parse_path_content(file_path, content) {
        FileParseResultVO::Rust(parse_result) => {
            for imp in &parse_result.imports {
                result.push((
                    LineNumber::new(imp.line as i64),
                    LineContentVO::new(imp.raw_path.clone()),
                ));
            }
        }
        FileParseResultVO::Python(parse_result) => {
            for imp in &parse_result.imports {
                result.push((
                    LineNumber::new(imp.line as i64),
                    LineContentVO::new(imp.raw_path.clone()),
                ));
            }
        }
        FileParseResultVO::TypeScript(parse_result) => {
            for imp in &parse_result.imports {
                result.push((
                    LineNumber::new(imp.line as i64),
                    LineContentVO::new(imp.raw_path.clone()),
                ));
            }
        }
        FileParseResultVO::Unsupported => {}
    }
    result
}

/// Parse a scope value (e.g. "contract(protocol)") into layer + suffix matches.
pub fn resolve_scope(scope: &Identity) -> (LayerNameVO, Vec<Identity>) {
    let scope_str = scope.value();
    if let Some(paren) = scope_str.find('(') {
        let layer = scope_str[..paren].trim();
        let inner = scope_str[paren + 1..].trim_end_matches(')').trim();
        let suffixes: Vec<Identity> = if inner.contains('|') {
            inner
                .split('|')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(Identity::new)
                .collect()
        } else {
            inner
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(Identity::new)
                .collect()
        };
        (LayerNameVO::new(layer), suffixes)
    } else {
        (LayerNameVO::new(scope_str.trim()), vec![])
    }
}

/// Check if an import line satisfies the given scope requirement.
pub fn import_matches_scope(
    import_line: &LineContentVO,
    layer: &LayerNameVO,
    suffixes: &[Identity],
) -> bool {
    let import_line_str = import_line.value();
    let segments: Vec<&str> = import_line_str
        .split(|c: char| {
            c == ':'
                || c == '.'
                || c == '/'
                || c == '\\'
                || c.is_whitespace()
                || c == '"'
                || c == '\''
                || c == '{'
                || c == '}'
                || c == ','
                || c == ';'
        })
        .filter(|s| !s.is_empty())
        .collect();
    let layer_lower = layer.value().to_lowercase();
    let layer_prefix = format!("{}_", layer_lower);
    let layer_match = segments.iter().any(|s| {
        let trimmed = s.trim().to_lowercase();
        trimmed == layer_lower || trimmed.starts_with(&layer_prefix)
    });
    if !layer_match || suffixes.is_empty() {
        return layer_match;
    }
    suffixes.iter().any(|s| {
        let s_val = s.value();
        segments.iter().any(|seg| {
            let cleaned = seg
                .trim_end_matches(';')
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim();
            cleaned.split(',').any(|t| {
                let name = t.trim();
                let name_lower = name.to_lowercase();
                if name_lower.ends_with(&format!("_{}", s_val)) {
                    return true;
                }
                if let Some(rest) = name_lower.strip_suffix(s_val) {
                    if rest.is_empty() || rest.ends_with('_') {
                        return true;
                    }
                    if name.len() >= s_val.len() {
                        let suffix_in_orig = &name[name.len() - s_val.len()..];
                        if suffix_in_orig.starts_with(|c: char| c.is_uppercase()) {
                            return true;
                        }
                    }
                }
                false
            })
        })
    })
}

/// Extract the module path from an import line.
pub fn extract_module_from_line(line: &LineContentVO) -> Option<Identity> {
    let trimmed = line.value().trim();
    if let Some(rest) = trimmed.strip_prefix("from ") {
        return Some(Identity::new(rest.split_whitespace().next()?.to_string()));
    }
    if trimmed.starts_with("import ") {
        if let Some(pos) = trimmed.rfind(" from ") {
            let module_part = trimmed[pos + 6..].trim();
            let cleaned = module_part
                .trim_end_matches(';')
                .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                .trim();
            return Some(Identity::new(cleaned.to_string()));
        }
        if let Some(rest) = trimmed.strip_prefix("import ") {
            if rest.contains('"') || rest.contains('\'') || rest.contains('`') {
                let cleaned = rest
                    .trim_end_matches(';')
                    .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                    .trim();
                return Some(Identity::new(cleaned.to_string()));
            }
            let first_token = rest.split_whitespace().next()?;
            return Some(Identity::new(first_token.to_string()));
        }
    }
    if let Some(rest) = trimmed
        .strip_prefix("pub(crate) use ")
        .or_else(|| trimmed.strip_prefix("pub use "))
        .or_else(|| trimmed.strip_prefix("use "))
    {
        let module = rest.trim_end_matches(';').trim().to_string();
        if let Some(brace_pos) = module.find("::{") {
            return Some(Identity::new(module[..brace_pos].to_string()));
        }
        return Some(Identity::new(module));
    }
    if !trimmed.is_empty() {
        return Some(Identity::new(trimmed.to_string()));
    }
    None
}

/// Extract layer name from an import segment.
pub fn extract_layer_from_import(segment: &Identity) -> Option<LayerNameVO> {
    let segment_str = segment.value();
    if let Some(layer) = utility_path_normalizer::extract_layer_from_prefix(segment_str) {
        return Some(LayerNameVO::new(layer));
    }
    match segment_str {
        "taxonomy" => Some(LayerNameVO::new("taxonomy")),
        "contract" => Some(LayerNameVO::new("contract")),
        "capabilities" => Some(LayerNameVO::new("capabilities")),
        "utility" => Some(LayerNameVO::new("utility")),
        "agent" => Some(LayerNameVO::new("agent")),
        "surfaces" | "surface" => Some(LayerNameVO::new("surfaces")),
        "root" => Some(LayerNameVO::new("root")),
        _ => None,
    }
}

/// Find the line number of an import statement containing the given alias.
pub fn find_import_line_number(content: &str, alias: &str) -> LineNumber {
    let first_part = alias.split('.').next().unwrap_or("");
    let pos_opt = content.lines().position(|l| {
        let t = l.trim();
        // Rust: use path::alias; / use path::{..., alias, ...};
        // Also handle "use ... as alias;"
        let is_use =
            t.starts_with("use ") || t.starts_with("pub use ") || t.starts_with("pub(crate) use ");
        let alias_in_use = is_use && (t.contains(alias));
        // Python: import X / from X import
        let py_import = t.contains(&format!("import {alias}"))
            || t.contains(&format!("from {first_part} import"));
        alias_in_use || py_import
    });
    let line = match pos_opt {
        Some(p) => p + 1,
        None => 1,
    };
    LineNumber::new(line as i64)
}

// ─── Barrel Import Resolution ─────────────────────────────
//
// Barrel files (__init__.py, index.ts, mod.rs) re-export symbols from

/// Check if a filename is a barrel/re-export file.
/// Barrel files are entry points that re-export symbols from submodules.
/// Import checkers should skip them for unused/dummy checks because
/// re-exports are intentional public API, not unused imports.
///
/// Recognized barrel files:
/// - Python: `__init__.py`
/// - Rust: `mod.rs`, `lib.rs`, `main.rs`
/// - TypeScript/JavaScript: `index.ts`, `index.js`, `index.tsx`, `index.jsx`
pub fn is_barrel_file(filename: &str) -> bool {
    matches!(
        filename,
        "__init__.py"
            | "mod.rs"
            | "lib.rs"
            | "main.rs"
            | "index.ts"
            | "index.js"
            | "index.tsx"
            | "index.jsx"
    )
}

// Barrel files (__init__.py, index.ts, mod.rs) re-export symbols from
// their original source files. When an import goes through a barrel file,
// the module path hides the original file name and its layer prefix.
//
// These functions resolve imported symbols back to their original source
// files so that layer detection works correctly.
//
// # Example
// ```text
// import:   from modules.shared.src.server import IBlenderConnectionProtocol
// barrel:   modules/shared/src/server/__init__.py
//           → from .contract_connection_protocol import IBlenderConnectionProtocol
// resolved: resolved_file = "contract_connection_protocol" → layer "contract" ✅
// ```

/// Normalize a module path for filesystem lookup.
/// Strips relative prefixes (`./`, `../`) and converts dots to path separators.
///
/// # Examples
/// - `"modules.shared.src.server"` → `"modules/shared/src/server"`
/// - `"./services"`                → `"services"`
/// - `"../utils"`                  → `"utils"`
fn normalize_module_path(module_path: &str) -> String {
    module_path
        .trim_start_matches("./")
        .trim_start_matches("../")
        .replace('.', "/")
        .replace("::", "/") // Rust path separator
}

/// Try to find a barrel file at the given base path with all candidate names.
fn try_barrel_candidates(dir: &Path, candidates: &[&str]) -> Option<String> {
    for candidate in candidates {
        let barrel_path = dir.join(candidate);
        if barrel_path.exists() {
            return Some(barrel_path.to_string_lossy().to_string());
        }
    }
    None
}

/// Find the barrel file (__init__.py, index.ts, mod.rs) for a module path.
///
/// Handles three path conventions:
/// - Python: `modules.shared.src.server` → `modules/shared/src/server/__init__.py`
/// - Rust:   `shared::import_rules::Type` → tries parent dir `shared/import_rules/`
/// - Rust crate paths: `shared::import_rules` → also tries `crates/shared/src/import-rules/`
///
/// # Examples
/// - `("modules.shared.src.server", "/workspace")` →
///   checks `/workspace/modules/shared/src/server/__init__.py`
/// - `("./services", "/workspace")` →
///   checks `/workspace/services/index.ts`
pub fn find_barrel_file(module_path: &str, root_dir: &str) -> Option<String> {
    let base = Path::new(root_dir);
    let clean_path = normalize_module_path(module_path);
    let module_dir = base.join(&clean_path);

    let barrel_candidates = [
        "__init__.py",
        "index.ts",
        "index.js",
        "index.tsx",
        "index.jsx",
        "mod.rs",
    ];

    // 1. Try direct path (for Python/TS paths: modules/shared/src/server/)
    if let Some(found) = try_barrel_candidates(&module_dir, &barrel_candidates) {
        return Some(found);
    }

    // 2. Try parent directory (for Rust paths ending with type name:
    //    "shared/import_rules/Type" → "shared/import_rules/")
    if let Some(parent) = module_dir.parent()
        && let Some(found) = try_barrel_candidates(parent, &barrel_candidates)
    {
        return Some(found);
    }

    // 3. Try under crates/{crate}/src/ for Rust workspace paths
    //    e.g. "shared::import_rules" → "crates/shared/src/import-rules/mod.rs"
    //    Also handles hyphens: Rust replaces `-` with `_` in module paths,
    //    so if the underscore path doesn't exist, try the hyphen variant.
    let segments: Vec<&str> = clean_path.split('/').collect();
    if let Some(first_seg) = segments.first() {
        // Try both underscore (Rust module name) and hyphen (filesystem name)
        let crate_names = [*first_seg, &first_seg.replace('_', "-")];

        for (idx, &crate_name) in crate_names.iter().enumerate() {
            // Skip duplicate (only matters when first_seg has no underscore)
            if idx == 1 && crate_name == crate_names[0] {
                continue;
            }
            let crate_src = base.join("crates").join(crate_name).join("src");
            let remainder: Vec<&str> = segments.iter().skip(1).copied().collect();

            // Try with full remainder
            if !remainder.is_empty() {
                // Try underscore remainder first, then hyphen
                let remainder_opts = [remainder.join("/"), remainder.join("/").replace('_', "-")];
                for (rem_idx, rem) in remainder_opts.iter().enumerate() {
                    // Skip duplicate remainder (when there's no underscore)
                    if rem_idx == 1 && *rem == remainder_opts[0] {
                        continue;
                    }
                    let full_dir = crate_src.join(rem);
                    if let Some(found) = try_barrel_candidates(&full_dir, &barrel_candidates) {
                        return Some(found);
                    }
                }
            }

            // Try parent of remainder (in case last segment is a type name)
            if remainder.len() > 1 {
                let remainder_str = remainder.join("/");
                let parent_path = Path::new(&remainder_str);
                if let Some(parent_dir) = parent_path.parent() {
                    let dir = crate_src.join(parent_dir);
                    if let Some(found) = try_barrel_candidates(&dir, &barrel_candidates) {
                        return Some(found);
                    }
                    // Also try hyphen variant of parent
                    let parent_hyphen = parent_dir.to_string_lossy().replace('_', "-");
                    let dir_hyphen = crate_src.join(&parent_hyphen);
                    if let Some(found) = try_barrel_candidates(&dir_hyphen, &barrel_candidates) {
                        return Some(found);
                    }
                }
            }
        }
    }

    None
}

/// Extract the file stem (last path component without extension) from a module path.
///
/// # Examples
/// - `"contract_connection_protocol"`           → `"contract_connection_protocol"`
/// - `"sub.contract_connection_protocol"`       → `"contract_connection_protocol"`
/// - `"./services/user-service"`                → `"user-service"`
/// - `"auth"`                                   → `"auth"`
fn extract_module_stem(module_path: &str) -> String {
    module_path
        .trim_start_matches("./")
        .trim_start_matches("../")
        .rsplit(['.', '/'])
        .next()
        .unwrap_or(module_path)
        .to_string()
}

/// Parse re-export mappings from a barrel file's content.
/// Returns a map: **symbol_name → source_file_stem**
///
/// The value is the **file stem** (not the full path, not the symbol name),
/// so that `extract_layer_from_prefix()` can detect the layer directly.
///
/// # Python `__init__.py`:
/// ```python
/// from .contract_connection_protocol import IBlenderConnectionProtocol
/// ```
/// → `{"IBlenderConnectionProtocol": "contract_connection_protocol"}`
///
/// # TS `index.ts`:
/// ```typescript
/// export { UserService } from './user-service';
/// ```
/// → `{"UserService": "user-service"}`
///
/// # Rust `mod.rs`:
/// ```rust,ignore
/// pub use auth::AuthOrchestrator;
/// ```
/// → `{"AuthOrchestrator": "auth"}`
pub fn parse_barrel_reexports(barrel_content: &str) -> HashMap<String, String> {
    let mut reexports: HashMap<String, String> = HashMap::new();

    for line in barrel_content.lines() {
        let trimmed = line.trim();

        // ── Python: from .module import X, Y ──
        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some((from_part, import_part)) = trimmed.split_once(" import ") {
                let module = from_part.strip_prefix("from ").unwrap_or("").trim();
                // Extract file stem: ".contract_connection_protocol" → "contract_connection_protocol"
                //                     ".sub.contract_protocol"      → "contract_protocol"
                let module_stem = extract_module_stem(module);

                for name in import_part.split(',') {
                    let name = name.trim();
                    if name.is_empty() || name == "*" {
                        continue;
                    }
                    // Handle "X as Y" → exported name is Y
                    let exported_name = name.split(" as ").last().unwrap_or(name).trim();
                    reexports.insert(exported_name.to_string(), module_stem.clone());
                }
            }
            continue;
        }

        // ── TS/JS: export { X, Y } from './module' ──
        if trimmed.starts_with("export ") && trimmed.contains(" from ") {
            if let Some(from_pos) = trimmed.rfind(" from ") {
                let module_part = trimmed[from_pos + 6..].trim();
                let module_clean = module_part
                    .trim_end_matches(';')
                    .trim_matches(|c: char| c == '\'' || c == '"' || c == '`');
                // Extract file stem: "./user-service" → "user-service"
                //                     "../types/index"  → "index"
                let module_stem = extract_module_stem(module_clean);

                if let Some(brace_start) = trimmed.find('{')
                    && let Some(brace_end) = trimmed.find('}')
                {
                    let inner = &trimmed[brace_start + 1..brace_end];
                    for part in inner.split(',') {
                        let part = part.trim();
                        if part.is_empty() {
                            continue;
                        }
                        let exported_name = part.split(" as ").last().unwrap_or(part).trim();
                        reexports.insert(exported_name.to_string(), module_stem.clone());
                    }
                }
            }
            continue;
        }

        // ── Rust: pub use submodule::Type; / pub use submodule::{A, B}; ──
        if trimmed.starts_with("pub use ") || trimmed.starts_with("pub(crate) use ") {
            let use_part = trimmed
                .trim_start_matches("pub(crate) use ")
                .trim_start_matches("pub use ")
                .trim_end_matches(';')
                .trim();

            if let Some(brace_pos) = use_part.find("::{") {
                // pub use submodule::{A, B};
                let prefix = &use_part[..brace_pos];
                // Extract module stem: "features::auth" → "auth"
                let module_stem = prefix.rsplit("::").next().unwrap_or(prefix).to_string();
                let inner = use_part[brace_pos + 3..].trim_end_matches('}');
                for name in inner.split(',') {
                    let name = name.trim().split(" as ").last().unwrap_or("").trim();
                    if !name.is_empty() && name != "*" {
                        reexports.insert(name.to_string(), module_stem.clone());
                    }
                }
            } else {
                // pub use submodule::Type;
                let name = use_part.rsplit("::").next().unwrap_or("").trim();
                // Extract module stem: "auth::AuthOrchestrator" → "auth"
                let module_stem = use_part
                    .rsplit_once("::")
                    .map(|x| x.0)
                    .unwrap_or(use_part)
                    .rsplit("::")
                    .next()
                    .unwrap_or(use_part)
                    .to_string();
                if !name.is_empty() && name != "*" {
                    reexports.insert(name.to_string(), module_stem);
                }
            }
        }
    }

    reexports
}

/// Resolve an import through a barrel file to its original source file.
///
/// # Arguments
/// * `module_path` - The module path from the import statement (e.g., "modules.shared.src.server")
/// * `symbol_name` - The imported symbol (e.g., "IBlenderConnectionProtocol")
/// * `root_dir` - Workspace root directory
///
/// # Returns
/// `Some(ResolvedImport)` if the symbol was found in a barrel file's re-exports,
/// `None` if no barrel file exists or symbol not found.
///
/// # Example
/// ```text
/// resolve_barrel_import("modules.shared.src.server", "IBlenderConnectionProtocol", "/workspace")
/// → Some(ResolvedImport {
///     original_module: "modules.shared.src.server",
///     resolved_file:   "contract_connection_protocol",
///     resolved_layer:  Some("contract"),
///     symbol:          "IBlenderConnectionProtocol",
/// })
/// ```
pub fn resolve_barrel_import(
    module_path: &str,
    symbol_name: &str,
    root_dir: &str,
) -> Option<ResolvedImport> {
    // Step 1: Find the barrel file
    let barrel_path = find_barrel_file(module_path, root_dir)?;

    // Step 2: Read barrel file content
    let barrel_content = crate::code_analysis::utility_file_reader::get_cached(&barrel_path)
        .or_else(|| std::fs::read_to_string(&barrel_path).ok())?;

    // Step 3: Parse re-export mappings (symbol → file_stem)
    let reexports = parse_barrel_reexports(&barrel_content);

    // Step 4: Look up the symbol → get source file stem
    let resolved_file = reexports.get(symbol_name)?.clone();

    // Step 5: Detect layer from resolved file stem
    // "contract_connection_protocol" → Some("contract")
    let resolved_layer = utility_path_normalizer::extract_layer_from_prefix(&resolved_file);

    Some(ResolvedImport {
        original_module: module_path.to_string(),
        resolved_file,
        resolved_layer,
        symbol: symbol_name.to_string(),
    })
}

/// Convenience wrapper: resolve a barrel import and return just the file stem.
///
/// # Example
/// ```text
/// resolve_barrel_symbol("modules.shared.src.server", "IBlenderConnectionProtocol", "/workspace")
/// → Some("contract_connection_protocol")
/// ```
pub fn resolve_barrel_symbol(module_path: &str, symbol: &str, root_dir: &str) -> Option<String> {
    resolve_barrel_import(module_path, symbol, root_dir).map(|r| r.resolved_file)
}

/// Extract imported symbol names from an import line.
/// Handles Python, Rust, and TS/JS import syntax.
///
/// # Examples
/// - `from X import A, B as C`       → ["A", "C"]
/// - `use crate::mod::{A, B};`       → ["A", "B"]
/// - `import { A, B } from './mod'`  → ["A", "B"]
/// - `import X from './mod'`         → ["X"]
pub fn extract_symbol_names(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    let mut names = Vec::new();

    // ── Python: from X import A, B ──
    if trimmed.starts_with("from ") {
        if let Some(import_part) = trimmed.split_once(" import ").map(|(_, p)| p) {
            let clean = import_part
                .trim_start_matches('(')
                .trim_end_matches(')')
                .trim_end_matches(';');
            for part in clean.split(',') {
                let name = part.trim().split(" as ").last().unwrap_or("").trim();
                if !name.is_empty() && name != "*" {
                    names.push(name.to_string());
                }
            }
        }
        return names;
    }

    // ── Rust: use crate::module::{A, B}; / use module::Type; ──
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

        if let Some(brace_start) = use_part.find("::{") {
            let inner = use_part[brace_start + 3..].trim_end_matches('}');
            for part in inner.split(',') {
                let name = part.trim().split(" as ").last().unwrap_or("").trim();
                if !name.is_empty() && name != "*" && name != "self" {
                    names.push(name.to_string());
                }
            }
        } else {
            let name = use_part.rsplit("::").next().unwrap_or("").trim();
            if !name.is_empty() && name != "*" {
                names.push(name.to_string());
            }
        }
        return names;
    }

    // ── TS/JS: import { A, B } from './module' ──
    if trimmed.starts_with("import ") && trimmed.contains('{') {
        if let Some(open) = trimmed.find('{')
            && let Some(close) = trimmed.find('}')
        {
            let inner = &trimmed[open + 1..close];
            for part in inner.split(',') {
                let name = part.trim().split(" as ").last().unwrap_or("").trim();
                if !name.is_empty() {
                    names.push(name.to_string());
                }
            }
        }
        return names;
    }

    // ── TS/JS: import X from './module' ──
    if trimmed.starts_with("import ")
        && trimmed.contains(" from ")
        && let Some(import_part) = trimmed.strip_prefix("import ")
    {
        let name = import_part.split(" from ").next().unwrap_or("").trim();
        if !name.is_empty() && name != "default" && name != "*" {
            names.push(name.to_string());
        }
    }

    names
}

/// Check if a symbol is a Python `__future__` import.
/// These affect parsing behavior, not runtime usage, and should be skipped
/// by both unused-import and dummy-import checkers.
pub fn is_future_import(content: &str, symbol: &str) -> bool {
    content.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with("from __future__ import ")
            && (trimmed == format!("from __future__ import {}", symbol)
                || trimmed.contains(format!(", {}", symbol).as_str())
                || trimmed.contains(format!(" {},", symbol).as_str()))
    })
}
