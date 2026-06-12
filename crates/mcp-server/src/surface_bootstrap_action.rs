// PURPOSE: handle_bootstrap — MCP surface for server initialization and capability listing

use shared::common::contract_service_aggregate::ServiceContainerAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_common_vo::BooleanVO;
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
