use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::fs;

use import_rules_lint_arwaky::infrastructure_import_parser_adapter::ImportParserAdapter;
use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
use shared::common::taxonomy_path_vo::FilePath;
use shared::import_rules::taxonomy_language_vo::LanguageVO;

fn make_fp(s: &str) -> FilePath {
    FilePath::new(s.to_string()).unwrap_or_default()
}

fn make_identity(s: &str) -> Identity {
    Identity::new(s)
}

// ---------------------------------------------------------------------------
// resolve_scope
// ---------------------------------------------------------------------------

#[test]
fn resolve_scope_plain_layer() {
    let parser = ImportParserAdapter::new();
    let (layer, suffixes) = parser.resolve_scope(&make_identity("capabilities"));
    assert_eq!(layer.value(), "capabilities");
    assert!(suffixes.is_empty());
}

#[test]
fn resolve_scope_with_suffix() {
    let parser = ImportParserAdapter::new();
    let (layer, suffixes) = parser.resolve_scope(&make_identity("contract(protocol)"));
    assert_eq!(layer.value(), "contract");
    assert_eq!(suffixes.len(), 1);
    assert_eq!(suffixes[0].value(), "protocol");
}

#[test]
fn resolve_scope_multiple_suffixes() {
    let parser = ImportParserAdapter::new();
    let (layer, suffixes) = parser.resolve_scope(&make_identity("taxonomy(entity,vo,error)"));
    assert_eq!(layer.value(), "taxonomy");
    assert_eq!(suffixes.len(), 3);
}

#[test]
fn resolve_scope_whitespace_handling() {
    let parser = ImportParserAdapter::new();
    let (layer, suffixes) = parser.resolve_scope(&make_identity("  agent ( container | orchestrator )  "));
    assert_eq!(layer.value(), "agent");
    assert_eq!(suffixes.len(), 2);
}

// ---------------------------------------------------------------------------
// extract_module_from_line
// ---------------------------------------------------------------------------

#[test]
fn extract_module_from_rust_use() {
    let parser = ImportParserAdapter::new();
    let line = LineContentVO::new("use shared::common::Path;".to_string());
    let module = parser.extract_module_from_line(&line);
    assert!(module.is_some());
    assert_eq!(module.unwrap().value(), "shared::common::Path");
}

#[test]
fn extract_module_from_rust_use_with_braces() {
    let parser = ImportParserAdapter::new();
    let line = LineContentVO::new("use std::collections::{HashMap, HashSet};".to_string());
    let module = parser.extract_module_from_line(&line);
    assert!(module.is_some());
    assert_eq!(module.unwrap().value(), "std::collections");
}

#[test]
fn extract_module_from_js_import() {
    let parser = ImportParserAdapter::new();
    let line = LineContentVO::new("import { foo } from './bar';".to_string());
    let module = parser.extract_module_from_line(&line);
    assert!(module.is_some());
    let val = module.unwrap().value().to_string();
    assert!(val.contains("./bar"), "got: {val}");
}

#[test]
fn extract_module_from_python_import() {
    let parser = ImportParserAdapter::new();
    let line = LineContentVO::new("import os".to_string());
    let module = parser.extract_module_from_line(&line);
    assert!(module.is_some());
    assert_eq!(module.unwrap().value(), "os");
}

#[test]
fn extract_module_from_python_from_import() {
    let parser = ImportParserAdapter::new();
    let line = LineContentVO::new("from pathlib import Path".to_string());
    let module = parser.extract_module_from_line(&line);
    assert!(module.is_some());
    assert_eq!(module.unwrap().value(), "pathlib");
}

#[test]
fn extract_module_non_import_line() {
    let parser = ImportParserAdapter::new();
    let line = LineContentVO::new("fn main() {}".to_string());
    let module = parser.extract_module_from_line(&line);
    assert!(module.is_none());
}

// ---------------------------------------------------------------------------
// extract_layer_from_import
// ---------------------------------------------------------------------------

#[test]
fn extract_layer_exact_match() {
    let parser = ImportParserAdapter::new();
    assert_eq!(
        parser.extract_layer_from_import(&make_identity("taxonomy")).unwrap().value(),
        "taxonomy"
    );
    assert_eq!(
        parser.extract_layer_from_import(&make_identity("infrastructure")).unwrap().value(),
        "infrastructure"
    );
}

#[test]
fn extract_layer_prefix_match() {
    let parser = ImportParserAdapter::new();
    assert_eq!(
        parser.extract_layer_from_import(&make_identity("capabilities_checker")).unwrap().value(),
        "capabilities"
    );
}

#[test]
fn extract_layer_no_match() {
    let parser = ImportParserAdapter::new();
    assert!(parser.extract_layer_from_import(&make_identity("random_stuff")).is_none());
}

// ---------------------------------------------------------------------------
// get_language_from_path
// ---------------------------------------------------------------------------

#[test]
fn language_rust_from_rs() {
    let parser = ImportParserAdapter::new();
    assert_eq!(parser.get_language_from_path("main.rs"), LanguageVO::Rust);
}

#[test]
fn language_python_from_py() {
    let parser = ImportParserAdapter::new();
    assert_eq!(parser.get_language_from_path("main.py"), LanguageVO::Python);
}

#[test]
fn language_js_from_js() {
    let parser = ImportParserAdapter::new();
    assert_eq!(parser.get_language_from_path("index.js"), LanguageVO::JavaScript);
}

#[test]
fn language_unknown_for_unknown_ext() {
    let parser = ImportParserAdapter::new();
    assert_eq!(parser.get_language_from_path("data.bin"), LanguageVO::Unknown);
}

// ---------------------------------------------------------------------------
// read_import_lines (integration via temp file)
// ---------------------------------------------------------------------------

#[test]
fn read_import_lines_from_temp_file() {
    let dir = std::env::temp_dir().join(format!("import_test_{}", std::process::id()));
    let _ = fs::create_dir_all(&dir);
    let file_path = dir.join("test.rs");
    let content = "\
use std::collections::HashMap;
use std::sync::Arc;

fn main() {}
";
    fs::write(&file_path, content).unwrap();

    let parser = ImportParserAdapter::new();
    let fp = make_fp(&file_path.to_string_lossy());
    let lines = parser.read_import_lines(&fp);
    assert_eq!(lines.len(), 2, "should find 2 import lines");
    // First import
    assert!(lines[0].1.value().contains("use std::collections::HashMap"));
    // Second import
    assert!(lines[1].1.value().contains("use std::sync::Arc"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn read_import_lines_nonexistent_file() {
    let parser = ImportParserAdapter::new();
    let fp = make_fp("/nonexistent/path/file.rs");
    let lines = parser.read_import_lines(&fp);
    assert!(lines.is_empty());
}

// ---------------------------------------------------------------------------
// import_matches_scope
// ---------------------------------------------------------------------------

#[test]
fn import_matches_scope_exact_layer() {
    let parser = ImportParserAdapter::new();
    let line = LineContentVO::new("use capabilities_checker::run;".to_string());
    let layer = LayerNameVO::new("capabilities");
    assert!(parser.import_matches_scope(&line, &layer, &[]));
}

#[test]
fn import_matches_scope_with_suffix() {
    let parser = ImportParserAdapter::new();
    let line = LineContentVO::new("use contract_protocol::Interface;".to_string());
    let layer = LayerNameVO::new("contract");
    let suffixes = vec![Identity::new("protocol")];
    assert!(parser.import_matches_scope(&line, &layer, &suffixes));
}

#[test]
fn import_matches_scope_no_match() {
    let parser = ImportParserAdapter::new();
    let line = LineContentVO::new("use std::collections::HashMap;".to_string());
    let layer = LayerNameVO::new("infrastructure");
    assert!(!parser.import_matches_scope(&line, &layer, &[]));
}

// ---------------------------------------------------------------------------
// get_basename
// ---------------------------------------------------------------------------

#[test]
fn get_basename_from_filepath() {
    let parser = ImportParserAdapter::new();
    let fp = make_fp("/home/user/project/src/main.rs");
    assert_eq!(parser.get_basename(&fp).value(), "main.rs");
}

#[test]
fn get_basename_simple() {
    let parser = ImportParserAdapter::new();
    let fp = make_fp("lib.rs");
    assert_eq!(parser.get_basename(&fp).value(), "lib.rs");
}

// ---------------------------------------------------------------------------
// parse_import_lines
// ---------------------------------------------------------------------------

#[test]
fn parse_import_lines_simple() {
    let parser = ImportParserAdapter::new();
    let content = FileContentVO::new("\
use std::collections::HashMap;
use std::sync::Arc;

fn main() {}
".to_string());
    let lines = parser.parse_import_lines(&content);
    assert_eq!(lines.len(), 2);
}

#[test]
fn parse_import_lines_multi_line_use() {
    let parser = ImportParserAdapter::new();
    let content = FileContentVO::new("\
use std::collections::{
    HashMap,
    HashSet,
};

fn main() {}
".to_string());
    let lines = parser.parse_import_lines(&content);
    assert_eq!(lines.len(), 1, "multi-line use should be parsed as one import");
}
