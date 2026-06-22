// PURPOSE: Module declarations for source-parsing (scanners, parsers, collectors, providers)

pub mod infrastructure_file_collector;
pub use infrastructure_file_collector::{collect_all_source_files, count_loc, walk_rs_files, FileCollectorProvider};
pub mod infrastructure_language_detector;
pub use infrastructure_language_detector::LanguageDetector;
pub mod infrastructure_js_scanner;
pub use infrastructure_js_scanner::ASTJSParserAdapter;
pub mod infrastructure_parser_adapter;
pub use infrastructure_parser_adapter::SourceParserOrchestrator;
pub mod infrastructure_path_provider;
pub use infrastructure_path_provider::{normalize_project_root, PathNormalizationProvider};
pub mod infrastructure_py_scanner;
pub use infrastructure_py_scanner::ASTPythonParserAdapter;
pub mod infrastructure_rust_scanner;
pub use infrastructure_rust_scanner::ASTRustParserAdapter;
pub mod root_source_parsing_container;
pub use root_source_parsing_container::SourceParsingContainer;
