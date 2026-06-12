// PURPOSE: SourceParsingContainer — source-parsing feature container (root_container layer)
use shared::source_parsing::contract_parser_port::ISourceParserPort;
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use std::sync::Arc;

pub struct SourceParsingContainer {
    source_parser: Arc<dyn ISourceParserPort>,
    path_normalization: Arc<dyn IPathNormalizationPort>,
    scanner_provider: Arc<dyn IScannerProviderPort>,
}

impl SourceParsingContainer {
    pub fn new() -> Self {
        let path_norm: Arc<dyn IPathNormalizationPort> =
            Arc::new(crate::infrastructure_path_provider::PathNormalizationProvider {});
        let source_parser: Arc<dyn ISourceParserPort> = Arc::new(
            crate::infrastructure_parser_adapter::SourceParserOrchestrator::new(
                Box::new(crate::infrastructure_py_scanner::ASTPythonParserAdapter::new()),
                Box::new(crate::infrastructure_rust_scanner::ASTRustParserAdapter::new()),
                Box::new(crate::infrastructure_js_scanner::ASTJSParserAdapter::new()),
            ),
        );
        Self {
            source_parser,
            path_normalization: path_norm,
            scanner_provider: Arc::new(
                crate::infrastructure_file_collector::FileCollectorProvider::new(),
            ),
        }
    }

    pub fn source_parser(&self) -> Arc<dyn ISourceParserPort> {
        self.source_parser.clone()
    }

    pub fn path_normalization(&self) -> Arc<dyn IPathNormalizationPort> {
        self.path_normalization.clone()
    }

    pub fn scanner_provider(&self) -> Arc<dyn IScannerProviderPort> {
        self.scanner_provider.clone()
    }
}
impl Default for SourceParsingContainer {
    fn default() -> Self {
        Self::new()
    }
}
