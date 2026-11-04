// PURPOSE: Acceptance tests — AES201 forbidden import detection.
// Verifies: taxonomy→capabilities, surface→agent, capabilities→agent, etc.
use import_rules_lint_arwaky::capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
use shared_common::NamingConfig;
use shared_common::taxonomy_common_vo::{BooleanVO, Count, PatternList};
use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_common::taxonomy_layer_vo::{LayerDefinition, LayerMapVO};
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_path_vo::FilePathList;
use shared_config_system::ArchitectureConfig;
use shared_import_rules::IImportForbiddenProtocol;
use std::collections::HashMap;

/// Full AES config with 3 layers and their forbidden/allowed rules.
fn aes_config() -> (ArchitectureConfig, LayerMapVO) {
    let mut layers = HashMap::new();
    // taxonomy cannot import capabilities, agent, or surface
    layers.insert(
        LayerNameVO::new("taxonomy"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["capabilities", "agent", "surface"]),
            allowed: PatternList::new(vec!["utility"]),
            ..Default::default()
        },
    );
    // capabilities cannot import agent or surface
    layers.insert(
        LayerNameVO::new("capabilities"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["agent", "surface"]),
            allowed: PatternList::new(vec!["taxonomy", "contract", "utility"]),
            ..Default::default()
        },
    );
    // surface cannot import agent
    layers.insert(
        LayerNameVO::new("surface"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["agent"]),
            allowed: PatternList::new(vec!["taxonomy", "contract", "utility", "capabilities"]),
            ..Default::default()
        },
    );
    // agent cannot import surface
    layers.insert(
        LayerNameVO::new("agent"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["surface"]),
            allowed: PatternList::new(vec!["taxonomy", "contract", "utility", "capabilities"]),
            ..Default::default()
        },
    );

    let layer_map = LayerMapVO::new(layers.clone());
    let config = ArchitectureConfig::new(
        BooleanVO::new(true),
        layers,
        vec![],
        NamingConfig::new(Count::new(3)),
        FilePathList::new(vec![]),
        BooleanVO::new(false),
    );
    (config, layer_map)
}

fn detect(content: &str, file: &str) -> usize {
    check_file(content, file).len()
}

fn check_file(content: &str, file: &str) -> Vec<shared_cli_commands::LintResult> {
    let checker = ArchImportForbiddenChecker::new();
    let (config, layer_map) = aes_config();
    let result = checker
        .check_single_file(file, content, "/tmp/project", &config, &layer_map)
        .unwrap();
    result.values
}

// ─── AES201: taxonomy layer violations ────────────────────

#[test]
fn aes201_taxonomy_importing_capabilities_is_forbidden() {
    let count = detect(
        "use capabilities::orchestrator;\n",
        "/tmp/project/taxonomy_vo.rs",
    );
    assert!(
        count > 0,
        "taxonomy importing capabilities must be forbidden"
    );
}

#[test]
fn aes201_taxonomy_importing_agent_is_forbidden() {
    let count = detect(
        "use agent::runner;\n",
        "/tmp/project/taxonomy_definition.rs",
    );
    assert!(count > 0, "taxonomy importing agent must be forbidden");
}

#[test]
fn aes201_taxonomy_importing_surface_is_forbidden() {
    let count = detect(
        "use surface::ui_handler;\n",
        "/tmp/project/taxonomy_value.rs",
    );
    assert!(count > 0, "taxonomy importing surface must be forbidden");
}

// ─── AES201: capabilities layer violations ────────────────

#[test]
fn aes201_capabilities_importing_agent_is_forbidden() {
    let count = detect(
        "use agent::import_orchestrator;\n",
        "/tmp/project/capabilities_checker.rs",
    );
    assert!(count > 0, "capabilities importing agent must be forbidden");
}

#[test]
fn aes201_capabilities_importing_surface_is_forbidden() {
    let count = detect(
        "use surface::ui_component;\n",
        "/tmp/project/capabilities_handler.rs",
    );
    assert!(
        count > 0,
        "capabilities importing surface must be forbidden"
    );
}

// ─── AES201: surface layer violations ─────────────────────

#[test]
fn aes201_surface_importing_agent_is_forbidden() {
    let count = detect("use agent::runner;\n", "/tmp/project/surface_handler.rs");
    assert!(count > 0, "surface importing agent must be forbidden");
}

// ─── AES201: agent layer violations ───────────────────────

#[test]
fn aes201_agent_importing_surface_is_forbidden() {
    let count = detect("use surface::ui;\n", "/tmp/project/agent_orchestrator.rs");
    assert!(count > 0, "agent importing surface must be forbidden");
}

// ─── AES201: allowed imports (no violations) ──────────────

#[test]
fn aes201_taxonomy_importing_utility_is_allowed() {
    let count = detect("use utility::helper;\n", "/tmp/project/taxonomy_vo.rs");
    assert_eq!(count, 0, "taxonomy importing utility should be allowed");
}

#[test]
fn aes201_capabilities_importing_taxonomy_is_allowed() {
    let count = detect(
        "use taxonomy::definition_vo;\n",
        "/tmp/project/capabilities_checker.rs",
    );
    assert_eq!(
        count, 0,
        "capabilities importing taxonomy should be allowed"
    );
}

#[test]
fn aes201_capabilities_importing_utility_is_allowed() {
    let count = detect(
        "use utility::helper;\n",
        "/tmp/project/capabilities_handler.rs",
    );
    assert_eq!(count, 0, "capabilities importing utility should be allowed");
}

// ─── AES201: source line reporting ────────────────────────

#[test]
fn aes201_reports_source_line_not_import_index() {
    let content = "use taxonomy::definition_vo;\nuse utility::helper;\n\n\nuse agent::runner;\n";
    let violations = check_file(content, "/tmp/project/surface_handler.rs");
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES201");
    // Source line of `use agent::runner;` is 5, whereas its 1-based index in the import list is 3.
    assert_eq!(violations[0].line.value(), 5);
}

#[test]
fn aes201_reports_source_line_for_python_forbidden_imports() {
    let content = "\
from __future__ import annotations

import contextlib
from typing import Any

from modules.agent.src.agent_browser_adapter import AgentBrowserAdapter

def some_func():
    pass

from modules.agent.src.agent_session_health_checker import AgentSessionHealthChecker
";
    let violations = check_file(content, "/tmp/project/surface_tui_sessions_store.py");
    assert_eq!(violations.len(), 2);
    assert_eq!(violations[0].line.value(), 6);
    assert_eq!(violations[1].line.value(), 11);
}

#[test]
fn aes201_falls_back_to_index_when_entry_line_is_zero() {
    let checker = ArchImportForbiddenChecker::new();
    let (config, layer_map) = aes_config();
    let file_path = "/tmp/project/surface_handler.rs";
    let root = FilePath::new("/tmp/project".to_string()).unwrap();
    let files = FilePathList::new(vec![FilePath::new(file_path.to_string()).unwrap()]);
    let mut content_map = HashMap::new();
    content_map.insert(file_path.to_string(), "placeholder".to_string());
    let mut imports_map = HashMap::new();
    imports_map.insert(
        file_path.to_string(),
        vec![shared_filesystem::taxonomy_filesystem_vo::ImportEntry {
            source_file: std::path::PathBuf::from(file_path),
            raw_path: "agent::runner".to_string(),
            resolved_path: None,
            import_type: shared_filesystem::taxonomy_filesystem_vo::ImportType::Use,
            language: shared_filesystem::taxonomy_filesystem_vo::Language::Rust,
            is_dynamic: false,
            is_resolved: false,
            symbols: Vec::new(),
            is_reexport: false,
            is_wildcard: false,
            line: 0,
        }],
    );

    let result = checker
        .check_forbidden_imports(
            &config,
            &layer_map,
            &files,
            &root,
            &content_map,
            &imports_map,
        )
        .unwrap();
    assert_eq!(result.values.len(), 1);
    // When line == 0, fallback to idx + 1 (1 in this case)
    assert_eq!(result.values[0].line.value(), 1);
}
