// filesystem crate — centralized file I/O, AST parsing, and dependency graph

pub mod capabilities_filesystem_service;
pub mod utility_ast_parser;
pub mod utility_dependency_graph;
pub mod utility_file_cache;
pub mod utility_file_walker;
pub mod utility_import_extractor;
pub mod utility_io;

pub use capabilities_filesystem_service::FilesystemService;
