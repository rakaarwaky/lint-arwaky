// PURPOSE: utility_orphan_parser_dispatch — route file parsing to the correct language parser.
// Stateless standalone function. Depends on 3 language utilities + taxonomy VOs.
// This is the SINGLE entry point that all capabilities files call.

use super::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use super::utility_orphan_python_parser;
use super::utility_orphan_rust_parser;
use super::utility_orphan_ts_parser;
use std::path::Path;

/// Parse a file based on its extension, routing to the correct language parser.
///
/// This is the only function capabilities files need to call.
///
/// # Examples
/// ```ignore
/// let result = utility_orphan_parser_dispatch::parse_file("foo.rs", &content);
/// match result {
///     FileParseResultVO::Rust(r) => { /* use r.trait_impls */ }
///     FileParseResultVO::Python(p) => { /* use p.class_bases */ }
///     FileParseResultVO::TypeScript(t) => { /* use t.class_implements */ }
///     FileParseResultVO::Unsupported => { /* skip */ }
/// }
/// ```
pub fn parse_file(path: &str, content: &str) -> FileParseResultVO {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match ext {
        "rs" => FileParseResultVO::Rust(utility_orphan_rust_parser::parse_rust(content)),
        "py" => FileParseResultVO::Python(utility_orphan_python_parser::parse_python(content)),
        "ts" | "tsx" | "js" | "jsx" => {
            FileParseResultVO::TypeScript(utility_orphan_ts_parser::parse_ts(content))
        }
        _ => FileParseResultVO::Unsupported,
    }
}

/// Check if a file extension is supported by any parser.
pub fn is_supported(path: &str) -> bool {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    matches!(ext, "rs" | "py" | "ts" | "tsx" | "js" | "jsx")
}
