/// SyspathBootstrapHandler — ensures the project's src directory is on sys.path (equivalent).
/// In Rust, sys.path bootstrap is not needed. This is a structural stub matching the Python
/// architecture 1:1, providing the same API surface for compatibility.
use crate::contract::service_container_aggregate::ServiceContainerAggregate;
use crate::taxonomy::{BooleanVO, FilePath};
use std::sync::Arc;

pub struct SyspathBootstrapHandler {}

impl SyspathBootstrapHandler {
    /// Ensure the project's src directory is resolved.
    /// Returns a BooleanVO::True equivalent for structural consistency.
    pub fn execute(_container: Arc<dyn ServiceContainerAggregate>) -> BooleanVO {
        BooleanVO { value: true }
    }

    /// Return the resolved src directory path.
    pub fn get_src_dir() -> FilePath {
        FilePath {
            value: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("src-rust")
                .to_string_lossy()
                .to_string(),
        }
    }
}

// Auto-bootstrap equivalent: no-op in Rust
