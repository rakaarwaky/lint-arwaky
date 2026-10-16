// PURPOSE: ConfigFileReader — implements IConfigFileReaderAggregate.
//
// Coordinates the config reader protocol and the filesystem I/O seam so
// surface layers (dispatcher, TUI) can read the nearest
// `lint_arwaky.config.yaml` without importing `contract(protocol)` traits
// (AES201).
use shared_config_system::{
    ConfigFileRead, IConfigFileReadFs, IConfigFileReaderAggregate, IConfigReadProtocol,
    utility_config_parser,
};
use shared_config_system::taxonomy_config_system_vo::AdapterEntry;

/// Parsed adapter entries from the nearest `adapters:` block (taxonomy VO).
pub type AdapterEntries = Vec<AdapterEntry>;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use std::path::Path;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

/// Walk-up reader over the nearest `lint_arwaky.config.yaml`.
///
/// Coordinates the filesystem I/O seam and the config reader protocol,
/// so the caller gets both the raw file walk and the parsed settings in one
/// `read_config_files` call.
pub struct ConfigFileReader {
    fs_io: Arc<dyn IFileSystemIOProtocol>,
    reader: Arc<dyn IConfigReadProtocol>,
    /// Aggregate used to verify workspace-root existence before the walk,
    /// so the reader can refuse to walk outside a known project boundary.
    fs_aggregate: Option<Arc<dyn IFilesystemAggregate>>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl IConfigFileReaderAggregate for ConfigFileReader {
    fn read_config_files(
        &self,
        root_path: &Path,
        fs: &dyn IConfigFileReadFs,
    ) -> ConfigFileRead {
        let start = root_path
            .parent()
            .filter(|_| root_path.is_file())
            .unwrap_or(root_path);
        // When the aggregate is injected, confirm the walk start is inside a
        // known project boundary so the reader never crosses into a sibling
        // workspace from a member-only target.
        if let Some(ref aggregate) = self.fs_aggregate {
            let ws_root = aggregate
                .execute(
                    shared_filesystem::FilesystemRequest::find_workspace_root(start),
                )
                .into_root();
            if ws_root.is_none() {
                return ConfigFileRead::default();
            }
        }
        let mut current: Option<&Path> = Some(start);
        while let Some(dir) = current {
            let cfg_path = dir.join("lint_arwaky.config.yaml");
            let content = fs.read_file(&cfg_path)
                .or_else(|| {
                    // Fallback: read via the injected I/O seam when the
                    // caller's seam does not carry a read path (e.g. a mock).
                    self.fs_io.read_to_string(&cfg_path).map(|c| c.value).ok()
                });
            if let Some(content) = content
                && !content.is_empty()
            {
                let adapter_entries =
                    utility_config_parser::parse_adapter_entries_from_yaml(&content);
                let ignored_rules =
                    utility_config_parser::parse_ignored_rules_from_yaml(&content);
                // Coordinate the reader protocol so a config with no `adapters:`
                // block still returns the merged defaults instead of an empty list.
                let normalized =
                    self.reader.read_config_from_content(&content);
                let adapter_entries: AdapterEntries =
                    if normalized.is_empty() { adapter_entries } else { normalized };
                tracing::debug!(
                    target: "lint_arwaky::audit",
                    event = "config_file_read",
                    path = %cfg_path.display(),
                    adapters = adapter_entries.len(),
                    ignored = ignored_rules.len(),
                    "nearest config file resolved"
                );
                return ConfigFileRead {
                    adapter_entries,
                    ignored_rules,
                };
            }
            current = dir.parent().filter(|&p| p != dir);
        }
        ConfigFileRead::default()
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────

impl ConfigFileReader {
    pub fn new(
        fs_io: Arc<dyn IFileSystemIOProtocol>,
        reader: Arc<dyn IConfigReadProtocol>,
        fs_aggregate: Option<Arc<dyn IFilesystemAggregate>>,
    ) -> Self {
        Self { fs_io, reader, fs_aggregate }
    }
}

