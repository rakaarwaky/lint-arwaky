// source-parsing — re-exports parser contracts and path types from common
// The actual types are defined in shared::common

pub use crate::common::contract_parser_port::ISourceParserPort;
pub use crate::common::contract_path_normalization_port::IPathNormalizationPort;
pub use crate::common::contract_scanner_provider_port::IScannerProviderPort;
pub use crate::common::taxonomy_parser_error::SourceParserError;
pub use crate::common::taxonomy_paths_vo::{FilePathList, RenamedFile, RenamedFileList};
pub use crate::common::taxonomy_path_vo::{DirectoryPath, FilePath};