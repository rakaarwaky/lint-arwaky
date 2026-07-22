// PURPOSE: Stateless path resolution utilities

/// Walk up from `path` to find the workspace root (parent of `crates/`, `packages/`, or `modules/`).
pub fn find_workspace_root(path: &str) -> Option<std::path::PathBuf> {
    shared::common::utility_file_handler::find_workspace_root(path)
}
