// PURPOSE: OrphanParserDispatcher — route file parsing to the correct language parser.
// Capabilities layer: implements IOrphanParserProtocol contract.

use crate::utility_orphan_python_parser;
use crate::utility_orphan_rust_parser;
use crate::utility_orphan_ts_parser;
use shared::orphan_detector::FileParseResultVO;
use shared::orphan_detector::IOrphanParserProtocol;
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct OrphanParserDispatcher {}

impl OrphanParserDispatcher {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for OrphanParserDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IOrphanParserProtocol for OrphanParserDispatcher {
    fn parse_file(&self, path: &str, content: &str) -> FileParseResultVO {
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

    fn is_supported(&self, path: &str) -> bool {
        let ext = Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        matches!(ext, "rs" | "py" | "ts" | "tsx" | "js" | "jsx")
    }
}

/// Convenience: parse file content using default dispatcher.
pub fn parse_file_content(path: &str, content: &str) -> shared::orphan_detector::FileParseResultVO {
    let dispatcher = OrphanParserDispatcher::new();
    dispatcher.parse_file(path, content)
}
