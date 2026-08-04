// PURPOSE: utility_parser_dispatcher — route file parsing to the correct language parser.
use crate::orphan_rules::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use std::path::Path;

pub fn parse_file_content(path: &str, content: &str) -> FileParseResultVO {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    match ext {
        "rs" => FileParseResultVO::Rust(super::utility_rust_parser::parse_rust(content)),
        "py" => FileParseResultVO::Python(super::utility_python_parser::parse_python(content)),
        "ts" | "tsx" | "js" | "jsx" => {
            FileParseResultVO::TypeScript(super::utility_ts_parser::parse_ts(content))
        }
        _ => FileParseResultVO::Unsupported,
    }
}

pub fn is_supported(path: &str) -> bool {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    matches!(ext, "rs" | "py" | "ts" | "tsx" | "js" | "jsx")
}
