// PURPOSE: utility_external_lint_io — re-export shim
// All functions consolidated into filesystem::utility_filesystem_io

pub use crate::filesystem::utility_filesystem_io::{
    canonicalize_path, has_cargo_lock, has_cargo_toml, has_config_file, has_local_bin,
    has_python_files, is_executable_in_path, scan_directory,
};
