// PURPOSE: ExternalLintExecutor — implements ICargoDirProtocol.
// Resolves the directory containing `Cargo.toml` or `Cargo.lock`.

use std::sync::Arc;

use shared_common::taxonomy_path_vo::FilePath;
use shared_external_lint::ICargoDirProtocol;
use shared_filesystem::contract_filesystem_protocol::IToolResolutionProtocol;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ExternalLintExecutor {
    tool_resolution: Arc<dyn IToolResolutionProtocol>,
}

// ─── Block 2: FR-008 cargo working directory resolution ──

impl ICargoDirProtocol for ExternalLintExecutor {
    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath {
        self.tool_resolution.resolve_cargo_working_dir(path)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ExternalLintExecutor {
    pub fn new(tool_resolution: Arc<dyn IToolResolutionProtocol>) -> Self {
        Self { tool_resolution }
    }
}
