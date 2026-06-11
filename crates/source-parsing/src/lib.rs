// PURPOSE: source-parsing — scanners, parsers, collectors, providers
// Depends on: shared-common (taxonomy/contract types)

pub use shared_common::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
pub use shared_common::contract_parser_port::ISourceParserPort;
pub use shared_common::contract_scanner_provider_port::IScannerProviderPort;
pub use shared_common::taxonomy_path_vo::{DirectoryPath, FilePath};
pub use shared_common::taxonomy_paths_vo::{FilePathList, RenamedFile, RenamedFileList};

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
pub mod infrastructure_barrel_provider;
pub use infrastructure_barrel_provider::BarrelImportResolver;
pub mod source_parsing_container;
