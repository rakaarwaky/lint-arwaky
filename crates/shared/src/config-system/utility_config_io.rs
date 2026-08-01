// PURPOSE: Config I/O utility — re-export shim
// All functions consolidated into filesystem::utility_filesystem_io

pub use crate::filesystem::utility_filesystem_io::{
    read_file_async, read_text_within_canonical_root, MAX_CONFIG_FILE_SIZE,
};
