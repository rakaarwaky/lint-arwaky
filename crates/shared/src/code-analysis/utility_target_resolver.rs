// PURPOSE: taxonomy_target_utility — re-export shim
// All functions consolidated into filesystem::utility_filesystem_io

pub use crate::filesystem::utility_filesystem_io::{
    collect_source_files, detect_source_dir, resolve_target,
};
