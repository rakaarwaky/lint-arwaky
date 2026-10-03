// PURPOSE: Layer-scoped scan — run the whole pipeline, then keep only the
//          findings whose file belongs to one AES layer.
//
// A layer command (`taxonomy`, `contract`, `capabilities`, `utility`, `agent`,
// `surface`) answers "what is wrong with this layer?", so it must not re-derive
// rules per layer: every rule group still runs over the same workspace (import
// cycles and orphans are cross-file facts), and only the reporting is narrowed.

use shared_common::ViolationItem;
use shared_common::utility_layer_detector::detect_layer_from_prefix;

use crate::surface_check_action::{ScanOptions, collect_scan};

/// Run every rule group over `opts`, then keep the violations whose file sits
/// in `layer`. `Err(String)` carries the same user-facing message `scan` does.
pub fn collect_layer_scan(opts: ScanOptions, layer: &str) -> Result<Vec<ViolationItem>, String> {
    let violations = collect_scan(opts)?;
    Ok(retain_violations_for_layer(violations, layer))
}

/// Retain the violations whose file name carries `layer`'s prefix.
///
/// The layer of a file is its AES name prefix (`taxonomy_`, `contract_`,
/// `capabilities_`, `utility_`, `agent_`, `surface_`, `root_`) — the same
/// signal `detect_layer_from_prefix` gives every other rule. A finding that
/// names no layer-prefixed file (a folder-level structure finding, a Markdown
/// doc invariant) belongs to no single layer, so a layer command drops it and
/// `scan`/`structure`/`docs` stay the commands that report those.
pub fn retain_violations_for_layer(
    violations: Vec<ViolationItem>,
    layer: &str,
) -> Vec<ViolationItem> {
    violations
        .into_iter()
        .filter(|violation| violation_in_layer(violation, layer))
        .collect()
}

/// Whether one violation's file belongs to `layer`.
fn violation_in_layer(violation: &ViolationItem, layer: &str) -> bool {
    detect_layer_from_prefix(&violation.file.value).as_deref() == Some(layer)
}
