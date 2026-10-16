// PURPOSE: IConfigFileReaderAggregate — walk-up config file reader for surface layers.
//
// AES201 compliance: surface layers (dispatcher, TUI) may only import
// `contract(aggregate)` traits. The nearest-config-file walk that the
// dispatcher and TUI surfaces need (adapters + ignored_rules from
// `lint_arwaky.config.yaml`) was previously hand-rolled in
// `surface_external_action` with direct file reads; it now lives in the
// config-system crate (`config_system::ConfigFileReader`) behind this
// aggregate contract.
// ─── Block 1: Struct Definitions ──────────────────────────
use crate::taxonomy_config_system_vo::AdapterEntry;

/// Result of locating the nearest config files above a target path.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConfigFileRead {
    /// Adapter entries parsed from the nearest `adapters:` block, empty when
    /// no config file with adapters was found.
    pub adapter_entries: Vec<AdapterEntry>,
    /// `ignored_rules` parsed from the nearest `lint_arwaky.config.yaml`,
    /// empty when no config file was found.
    pub ignored_rules: Vec<String>,
}

/// A filesystem view the config reader needs: existence + read.
pub trait IConfigFileReadFs: Send + Sync {
    fn path_exists(&self, path: &std::path::Path) -> bool;
    fn read_file(&self, path: &std::path::Path) -> Option<String>;
}

/// Walk-up reader for the two config file families surface layers consume.
pub trait IConfigFileReaderAggregate: Send + Sync {
    /// Find the nearest `lint_arwaky.config.yaml` at or above `root_path`
    /// and return its adapter entries plus ignored rules.
    ///
    /// Mirrors the old `surface_external_action::load_config_entries` +
    /// `load_ignored_rules` pair: both settings come from one nearest file,
    /// and the walk continues past missing/empty files so a miss in one
    /// directory does not silently drop every entry from a parent config.
    fn read_config_files(
        &self,
        root_path: &std::path::Path,
        fs: &dyn IConfigFileReadFs,
    ) -> ConfigFileRead;
}
