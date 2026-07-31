// filesystem crate — centralized file I/O, AST parsing, and dependency graph

pub mod capabilities_filesystem_service;
pub mod utility_ast_parser;
pub mod utility_dependency_graph;
pub mod utility_file_cache;
pub mod utility_file_walker;
pub mod utility_import_extractor;
pub mod utility_io;

pub use capabilities_filesystem_service::FilesystemService;
pub use utility_io::{
    find_workspace_root, is_dir, is_file, is_path_ignored, is_source_file, path_exists,
    read_file, read_file_safe, walk_directory, DEFAULT_SKIP_DIRS, MAX_LINT_FILE_BYTES,
};
