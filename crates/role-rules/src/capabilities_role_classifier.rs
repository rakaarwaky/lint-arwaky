// PURPOSE: RoleClassifier — IClassificationProtocol for FR-RoleRules-001:
// stateless filename-prefix → layer classification.
//
// This map used to live on the role-rules agent. An agent must not implement a
// contract protocol (AES405), so it moved here: the agent keeps the behaviour by
// holding this capability as a dependency and delegating, never by implementing
// the trait itself.

use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;
use shared_role_rules::contract_role_protocol::IClassificationProtocol;
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct RoleClassifier {}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl IClassificationProtocol for RoleClassifier {
    fn classify_layer(&self, file: &FileEntry) -> Option<LayerNameVO> {
        let filename = file
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        let stem = Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        // The segment only counts when the stem actually contains a `_`.
        // FR-RoleRules-001 scopes the map to `layer_concern_role` filenames and
        // says a file with no underscore has no prefix match; without this guard
        // `split('_').next()` hands back the whole stem, so a bare `agent.rs`
        // would be classified — and then audited — as an agent.
        if !stem.contains('_') {
            return None;
        }
        match stem.split('_').next().unwrap_or_default() {
            "taxonomy" => Some(LayerNameVO::new("taxonomy")),
            "contract" => Some(LayerNameVO::new("contract")),
            "capabilities" | "capability" => Some(LayerNameVO::new("capabilities")),
            "utility" => Some(LayerNameVO::new("utility")),
            "agent" => Some(LayerNameVO::new("agent")),
            "surface" | "surfaces" => Some(LayerNameVO::new("surfaces")),
            // `root` is pure DI wiring; unrecognised prefixes are skipped.
            _ => None,
        }
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ───────────
impl Default for RoleClassifier {
    fn default() -> Self {
        Self::new()
    }
}

impl RoleClassifier {
    pub fn new() -> Self {
        Self {}
    }
}
