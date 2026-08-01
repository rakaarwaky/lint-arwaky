// PURPOSE: Stateless utility functions for reading lintable files — re-export shim
// All functions consolidated into filesystem::utility_filesystem_io

pub use crate::filesystem::utility_filesystem_io::{
    cache_clear_str as clear_cache, cache_contains_str, cache_get_by_str as get_cached,
    cache_populate_from_pairs as populate_cache, read_lintable_file, MAX_LINT_FILE_BYTES,
};
