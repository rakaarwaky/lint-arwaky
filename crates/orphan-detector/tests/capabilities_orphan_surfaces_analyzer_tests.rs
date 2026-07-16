use orphan_detector_lint_arwaky::capabilities_orphan_filename_extractor::OrphanFilenameExtractor;
use orphan_detector_lint_arwaky::capabilities_orphan_surfaces_analyzer::{
    get_surface_suffix, surface_category,
};
use shared::orphan_detector::contract_orphan_protocol::IOrphanFilenameExtractorProtocol;
use std::sync::Arc;

fn get_extractor() -> Arc<dyn IOrphanFilenameExtractorProtocol> {
    Arc::new(OrphanFilenameExtractor::new())
}

#[test]
fn suffix_from_command_file() {
    let extractor = get_extractor();
    let suffix = get_surface_suffix("surface_check_command.rs", &extractor);
    assert_eq!(suffix, "command");
}

#[test]
fn suffix_from_controller_file() {
    let extractor = get_extractor();
    let suffix = get_surface_suffix("surface_user_controller.rs", &extractor);
    assert_eq!(suffix, "controller");
}

#[test]
fn suffix_from_component_file() {
    let extractor = get_extractor();
    let suffix = get_surface_suffix("surface_header_component.tsx", &extractor);
    assert_eq!(suffix, "component");
}

#[test]
fn suffix_from_hook_file() {
    let extractor = get_extractor();
    let suffix = get_surface_suffix("surface_use_auth_hook.ts", &extractor);
    assert_eq!(suffix, "hook");
}

#[test]
fn suffix_from_entry_file() {
    let extractor = get_extractor();
    let suffix = get_surface_suffix("root_cli_entry.rs", &extractor);
    assert_eq!(suffix, "entry");
}

#[test]
fn category_smart_for_command() {
    assert_eq!(surface_category("command"), "smart");
    assert_eq!(surface_category("controller"), "smart");
    assert_eq!(surface_category("page"), "smart");
}

#[test]
fn category_utility_for_hook() {
    assert_eq!(surface_category("hook"), "utility");
    assert_eq!(surface_category("store"), "utility");
    assert_eq!(surface_category("action"), "utility");
    assert_eq!(surface_category("screen"), "utility");
    assert_eq!(surface_category("router"), "utility");
}

#[test]
fn category_passive_for_component() {
    assert_eq!(surface_category("component"), "passive");
    assert_eq!(surface_category("view"), "passive");
    assert_eq!(surface_category("layout"), "passive");
}

#[test]
fn category_unknown_for_unrecognized() {
    assert_eq!(surface_category("xyz"), "unknown");
    assert_eq!(surface_category(""), "unknown");
}
