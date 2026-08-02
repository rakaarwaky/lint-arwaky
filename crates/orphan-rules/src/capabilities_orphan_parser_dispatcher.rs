// PURPOSE: OrphanParserDispatcher — delegates to shared taxonomy parsers.
use shared::orphan_rules::FileParseResultVO;
use shared::orphan_rules::IOrphanParserProtocol;
use shared::orphan_rules::taxonomy_parser_dispatcher::{
    is_supported as shared_is_supported, parse_file_content as shared_parse_file_content,
};

pub struct OrphanParserDispatcher;

impl OrphanParserDispatcher {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OrphanParserDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl IOrphanParserProtocol for OrphanParserDispatcher {
    fn parse_file(&self, path: &str, content: &str) -> FileParseResultVO {
        shared_parse_file_content(path, content)
    }

    fn is_supported(&self, path: &str) -> bool {
        shared_is_supported(path)
    }
}

pub fn parse_file_content(path: &str, content: &str) -> shared::orphan_rules::FileParseResultVO {
    shared_parse_file_content(path, content)
}
