// PURPOSE: IOrphanParserProtocol — contract trait for language parser dispatching.
// Pure trait definition: zero implementation, zero layer imports.

use crate::orphan_detector::taxonomy_orphan_parse_result_vo::FileParseResultVO;

pub trait IOrphanParserProtocol: Send + Sync {
    /// Parse a file based on its extension, routing to the correct language parser.
    fn parse_file(&self, path: &str, content: &str) -> FileParseResultVO;

    /// Check if a file extension is supported by any parser.
    fn is_supported(&self, path: &str) -> bool;
}