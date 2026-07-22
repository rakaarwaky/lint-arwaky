// PURPOSE: Unit tests for ChangeAnalyzer — deduplication, lintability filtering.
// Layer: Capabilities (target ≥ 70% coverage)

use file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer;
use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
use shared::file_watch::taxonomy_watch_event_vo::{WatchEvent, WatchEventKind};

// ─── Helpers ────────────────────────────────────────────────

fn make_event(path: &str) -> WatchEvent {
    WatchEvent::new(path.to_string(), WatchEventKind::Modified)
}

fn make_event_with_kind(path: &str, kind: WatchEventKind) -> WatchEvent {
    WatchEvent::new(path.to_string(), kind)
}

// ─── new / default ──────────────────────────────────────────

#[test]
fn new_returns_instance() {
    let analyzer = ChangeAnalyzer::new();
    // Unit struct — just verify it compiles and runs.
    let _ = &analyzer;
}

#[test]
fn default_returns_instance() {
    let analyzer = ChangeAnalyzer::default();
    let _ = &analyzer;
}

// ─── is_lintable ────────────────────────────────────────────

#[test]
fn is_lintable_rust_file() {
    assert!(ChangeAnalyzer::is_lintable("src/main.rs"));
}

#[test]
fn is_lintable_python_file() {
    assert!(ChangeAnalyzer::is_lintable("app.py"));
}

#[test]
fn is_lintable_javascript_file() {
    assert!(ChangeAnalyzer::is_lintable("index.js"));
}

#[test]
fn is_lintable_typescript_file() {
    assert!(ChangeAnalyzer::is_lintable("app.ts"));
}

#[test]
fn is_lintable_tsx_file() {
    assert!(ChangeAnalyzer::is_lintable("Component.tsx"));
}

#[test]
fn is_lintable_jsx_file() {
    assert!(ChangeAnalyzer::is_lintable("Component.jsx"));
}

#[test]
fn is_lintable_mjs_file() {
    assert!(ChangeAnalyzer::is_lintable("module.mjs"));
}

#[test]
fn is_lintable_cjs_file() {
    assert!(ChangeAnalyzer::is_lintable("module.cjs"));
}

#[test]
fn is_lintable_json_file() {
    assert!(ChangeAnalyzer::is_lintable("package.json"));
}

#[test]
fn is_lintable_css_file() {
    assert!(ChangeAnalyzer::is_lintable("styles.css"));
}

#[test]
fn is_lintable_markdown_file() {
    assert!(ChangeAnalyzer::is_lintable("README.md"));
}

#[test]
fn is_lintable_toml_file() {
    assert!(ChangeAnalyzer::is_lintable("Cargo.toml"));
}

#[test]
fn is_lintable_yaml_file() {
    assert!(ChangeAnalyzer::is_lintable("config.yaml"));
}

#[test]
fn is_lintable_yml_file() {
    assert!(ChangeAnalyzer::is_lintable("config.yml"));
}

#[test]
fn is_lintable_rejects_binary() {
    assert!(!ChangeAnalyzer::is_lintable("image.png"));
}

#[test]
fn is_lintable_rejects_exe() {
    assert!(!ChangeAnalyzer::is_lintable("program.exe"));
}

#[test]
fn is_lintable_rejects_no_extension() {
    assert!(!ChangeAnalyzer::is_lintable("Makefile"));
}

#[test]
fn is_lintable_rejects_empty_string() {
    assert!(!ChangeAnalyzer::is_lintable(""));
}

#[test]
fn is_lintable_rejects_partial_extension() {
    // ".r" is not ".rs"
    assert!(!ChangeAnalyzer::is_lintable("file.r"));
}

#[test]
fn is_lintable_nested_path() {
    assert!(ChangeAnalyzer::is_lintable("crates/shared/src/common/mod.rs"));
}

// ─── analyze (deduplication) ────────────────────────────────

#[test]
fn analyze_empty_vec_returns_empty() {
    let analyzer = ChangeAnalyzer::new();
    let result = analyzer.analyze(vec![]);
    assert!(result.is_empty());
}

#[test]
fn analyze_single_event_returns_single() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![make_event("src/main.rs")];
    let result = analyzer.analyze(events);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].path, "src/main.rs");
}

#[test]
fn analyze_deduplicates_same_path() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        make_event("src/main.rs"),
        make_event("src/main.rs"),
        make_event("src/main.rs"),
    ];
    let result = analyzer.analyze(events);
    assert_eq!(result.len(), 1);
}

#[test]
fn analyze_keeps_distinct_paths() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        make_event("src/main.rs"),
        make_event("src/lib.rs"),
        make_event("tests/test.rs"),
    ];
    let result = analyzer.analyze(events);
    assert_eq!(result.len(), 3);
}

#[test]
fn analyze_last_event_wins_for_same_path() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        make_event_with_kind("src/main.rs", WatchEventKind::Created),
        make_event_with_kind("src/main.rs", WatchEventKind::Modified),
    ];
    let result = analyzer.analyze(events);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].kind, WatchEventKind::Modified);
}

#[test]
fn analyze_mixed_duplicates_and_unique() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        make_event("a.rs"),
        make_event("b.py"),
        make_event("a.rs"),
        make_event("c.ts"),
        make_event("b.py"),
    ];
    let result = analyzer.analyze(events);
    assert_eq!(result.len(), 3);
}

// ─── filter_lintable ────────────────────────────────────────

#[test]
fn filter_lintable_empty_vec() {
    let analyzer = ChangeAnalyzer::new();
    let result = analyzer.filter_lintable(vec![]);
    assert!(result.is_empty());
}

#[test]
fn filter_lintable_all_lintable() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        make_event("src/main.rs"),
        make_event("app.py"),
        make_event("index.ts"),
    ];
    let result = analyzer.filter_lintable(events);
    assert_eq!(result.len(), 3);
}

#[test]
fn filter_lintable_removes_non_lintable() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        make_event("src/main.rs"),
        make_event("image.png"),
        make_event("app.py"),
        make_event("binary.exe"),
    ];
    let result = analyzer.filter_lintable(events);
    assert_eq!(result.len(), 2);
    assert!(result.iter().all(|e| ChangeAnalyzer::is_lintable(&e.path)));
}

#[test]
fn filter_lintable_all_non_lintable_returns_empty() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        make_event("photo.jpg"),
        make_event("data.bin"),
        make_event("archive.zip"),
    ];
    let result = analyzer.filter_lintable(events);
    assert!(result.is_empty());
}

#[test]
fn filter_lintable_preserves_event_kind() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        make_event_with_kind("src/main.rs", WatchEventKind::Created),
        make_event_with_kind("src/lib.rs", WatchEventKind::Removed),
    ];
    let result = analyzer.filter_lintable(events);
    assert_eq!(result[0].kind, WatchEventKind::Created);
    assert_eq!(result[1].kind, WatchEventKind::Removed);
}
