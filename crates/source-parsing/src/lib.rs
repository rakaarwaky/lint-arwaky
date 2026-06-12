// PURPOSE: Module declarations and re-exports for source-parsing (scanners, parsers, collectors, providers, VOs)

// Re-export shared taxonomy and contract types from shared crate
pub use shared::source_parsing::contract_parser_port;
pub use shared::source_parsing::contract_parser_port::ISourceParserPort;
pub use shared::source_parsing::contract_path_normalization_port;
pub use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
pub use shared::source_parsing::contract_scanner_provider_port;
pub use shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
pub use shared::source_parsing::taxonomy_parser_error;
pub use shared::source_parsing::taxonomy_parser_error::{SourceParserError, SyntaxErrorVO};
pub use shared::source_parsing::taxonomy_paths_vo;
pub use shared::source_parsing::taxonomy_paths_vo::{FilePathList, RenamedFile, RenamedFileList};
pub use shared::source_parsing::taxonomy_path_vo;
pub use shared::source_parsing::taxonomy_path_vo::{DirectoryPath, FilePath};

// Local infrastructure modules
// BarrelImportResolver is a utility (taxonomy layer) — lives in shared crate
pub use shared::source_parsing::taxonomy_barrel_provider::BarrelImportResolver;
pub mod infrastructure_file_collector;
pub use infrastructure_file_collector::FileCollectorProvider;
pub mod infrastructure_js_scanner;
pub use infrastructure_js_scanner::ASTJSParserAdapter;
pub mod infrastructure_parser_adapter;
pub use infrastructure_parser_adapter::SourceParserOrchestrator;
pub mod infrastructure_path_provider;
pub use infrastructure_path_provider::PathNormalizationProvider;
pub mod infrastructure_py_scanner;
pub use infrastructure_py_scanner::ASTPythonParserAdapter;
pub mod infrastructure_rust_scanner;
pub use infrastructure_rust_scanner::ASTRustParserAdapter;
pub mod root_source_parsing_container;
pub use root_source_parsing_container::SourceParsingContainer;