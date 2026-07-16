use file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer;
use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
use shared::file_watch::taxonomy_watch_event_vo::{WatchEvent, WatchEventKind};

// ---------------------------------------------------------------------------
// is_lintable — which file extensions should trigger a re-lint
// ---------------------------------------------------------------------------

#[test]
fn lintable_rust_file() {
    assert!(ChangeAnalyzer::new().is_lintable("src/main.rs"));
}

#[test]
fn lintable_python_file() {
    assert!(ChangeAnalyzer::new().is_lintable("app.py"));
}

#[test]
fn lintable_js_file() {
    assert!(ChangeAnalyzer::new().is_lintable("index.js"));
}

#[test]
fn lintable_ts_file() {
    assert!(ChangeAnalyzer::new().is_lintable("component.ts"));
}

#[test]
fn lintable_tsx_file() {
    assert!(ChangeAnalyzer::new().is_lintable("component.tsx"));
}

#[test]
fn lintable_jsx_file() {
    assert!(ChangeAnalyzer::new().is_lintable("component.jsx"));
}

#[test]
fn lintable_yaml_file() {
    assert!(ChangeAnalyzer::new().is_lintable("config.yaml"));
}

#[test]
fn lintable_yml_file() {
    assert!(ChangeAnalyzer::new().is_lintable("config.yml"));
}

#[test]
fn lintable_json_file() {
    assert!(ChangeAnalyzer::new().is_lintable("package.json"));
}

#[test]
fn lintable_css_file() {
    assert!(ChangeAnalyzer::new().is_lintable("styles.css"));
}

#[test]
fn lintable_md_file() {
    assert!(ChangeAnalyzer::new().is_lintable("README.md"));
}

#[test]
fn lintable_toml_file() {
    assert!(ChangeAnalyzer::new().is_lintable("Cargo.toml"));
}

#[test]
fn not_lintable_other_extension() {
    let analyzer = ChangeAnalyzer::new();
    assert!(!analyzer.is_lintable("image.png"));
    assert!(!analyzer.is_lintable("data.bin"));
    assert!(!analyzer.is_lintable("font.woff2"));
    assert!(!analyzer.is_lintable("document.pdf"));
}

#[test]
fn not_lintable_without_extension() {
    let analyzer = ChangeAnalyzer::new();
    assert!(!analyzer.is_lintable("Makefile"));
    assert!(!analyzer.is_lintable("LICENSE"));
}

#[test]
fn lintable_with_full_path() {
    let analyzer = ChangeAnalyzer::new();
    assert!(analyzer.is_lintable("/home/user/project/src/lib.rs"));
    assert!(analyzer.is_lintable("crates/shared/src/lib.rs"));
}

#[test]
fn lintable_dotfiles_in_hidden_dirs() {
    let analyzer = ChangeAnalyzer::new();
    assert!(analyzer.is_lintable(".config/settings.py"));
    assert!(analyzer.is_lintable(".github/workflows/deploy.rs"));
}

// ─── analyze — deduplication ────────────────────────────────────────────────

#[test]
fn analyze_deduplicates_duplicate_paths() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        WatchEvent::new("src/main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("src/main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("src/lib.rs".to_string(), WatchEventKind::Modified),
    ];
    let result = analyzer.analyze(events);
    assert_eq!(result.len(), 2, "should dedup to 2 unique paths");
}

#[test]
fn analyze_returns_all_unique_events() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        WatchEvent::new("a.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("b.py".to_string(), WatchEventKind::Created),
        WatchEvent::new("c.ts".to_string(), WatchEventKind::Removed),
    ];
    let result = analyzer.analyze(events);
    assert_eq!(result.len(), 3);
}

#[test]
fn analyze_empty_returns_empty() {
    let analyzer = ChangeAnalyzer::new();
    let result = analyzer.analyze(vec![]);
    assert!(result.is_empty());
}

// ─── filter_lintable ───────────────────────────────────────────────────────

#[test]
fn filter_lintable_keeps_source_files() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        WatchEvent::new("src/main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("app.py".to_string(), WatchEventKind::Modified),
    ];
    let result = analyzer.filter_lintable(events);
    assert_eq!(result.len(), 2);
}

#[test]
fn filter_lintable_removes_non_source_files() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        WatchEvent::new("src/main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("image.png".to_string(), WatchEventKind::Modified),
        WatchEvent::new("data.pdf".to_string(), WatchEventKind::Modified),
    ];
    let result = analyzer.filter_lintable(events);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].path, "src/main.rs");
}

#[test]
fn filter_lintable_empty_returns_empty() {
    let analyzer = ChangeAnalyzer::new();
    let result = analyzer.filter_lintable(vec![]);
    assert!(result.is_empty());
}
