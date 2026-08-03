// Unit tests — ChangeAnalyzer edge cases and deduplication logic.
use file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer;
use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
use shared::file_watch::taxonomy_watch_event_vo::{WatchEvent, WatchEventKind};

// ─── is_lintable edge cases (FR-003) ──────────────────────

#[test]
fn is_lintable_rust_source() {
    let analyzer = ChangeAnalyzer::new();
    assert!(analyzer.is_lintable("main.rs"));
    assert!(analyzer.is_lintable("src/lib.rs"));
}

#[test]
fn is_lintable_python_source() {
    let analyzer = ChangeAnalyzer::new();
    assert!(analyzer.is_lintable("app.py"));
    assert!(analyzer.is_lintable("tests/test_main.py"));
}

#[test]
fn is_lintable_typescript_sources() {
    let analyzer = ChangeAnalyzer::new();
    assert!(analyzer.is_lintable("index.ts"));
    assert!(analyzer.is_lintable("component.tsx"));
    assert!(analyzer.is_lintable("util.jsx"));
    assert!(analyzer.is_lintable("helper.mjs"));
    assert!(analyzer.is_lintable("helper.cjs"));
}

#[test]
fn is_lintable_config_files() {
    let analyzer = ChangeAnalyzer::new();
    assert!(analyzer.is_lintable("Cargo.toml"));
    assert!(analyzer.is_lintable("config.yaml"));
    assert!(analyzer.is_lintable("data.yml"));
    assert!(analyzer.is_lintable("package.json"));
    assert!(analyzer.is_lintable("styles.css"));
    assert!(analyzer.is_lintable("README.md"));
}

#[test]
fn is_lintable_non_lintable_extensions() {
    let analyzer = ChangeAnalyzer::new();
    assert!(!analyzer.is_lintable("image.png"));
    assert!(!analyzer.is_lintable("data.bin"));
    assert!(!analyzer.is_lintable("archive.tar.gz"));
    assert!(!analyzer.is_lintable("video.mp4"));
    assert!(!analyzer.is_lintable("sound.wav"));
}

#[test]
fn is_lintable_no_extension() {
    let analyzer = ChangeAnalyzer::new();
    assert!(!analyzer.is_lintable("Makefile"));
    assert!(!analyzer.is_lintable("Dockerfile"));
    assert!(!analyzer.is_lintable("LICENSE"));
}

#[test]
fn is_lintable_hidden_files() {
    let analyzer = ChangeAnalyzer::new();
    // Hidden files like .gitignore have no matching extension
    assert!(!analyzer.is_lintable(".gitignore"));
    assert!(!analyzer.is_lintable(".env"));
    assert!(!analyzer.is_lintable(".dockerignore"));
}

#[test]
fn is_lintable_multiple_dots() {
    let analyzer = ChangeAnalyzer::new();
    // FR-003: matches on the final extension
    assert!(analyzer.is_lintable("file.test.ts"));
    assert!(analyzer.is_lintable("spec.unit.py"));
    assert!(analyzer.is_lintable("config.prod.yaml"));
}

#[test]
fn is_lintable_case_sensitive() {
    let analyzer = ChangeAnalyzer::new();
    // FR-003: extension matching is case-sensitive (no normalization)
    assert!(!analyzer.is_lintable("FILE.RS"));
    assert!(!analyzer.is_lintable("App.PY"));
    assert!(!analyzer.is_lintable("Index.TS"));
}

#[test]
fn is_lintable_empty_string() {
    let analyzer = ChangeAnalyzer::new();
    assert!(!analyzer.is_lintable(""));
}

#[test]
fn is_lintable_path_with_directories() {
    let analyzer = ChangeAnalyzer::new();
    assert!(analyzer.is_lintable("/home/user/project/src/main.rs"));
    assert!(analyzer.is_lintable("./src/lib.py"));
    assert!(!analyzer.is_lintable("/home/user/project/image.png"));
}

// ─── analyze deduplication (FR-004) ───────────────────────

#[test]
fn analyze_empty_input() {
    let analyzer = ChangeAnalyzer::new();
    let result = analyzer.analyze(vec![]);
    assert!(result.is_empty());
}

#[test]
fn analyze_no_duplicates() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        WatchEvent::new("a.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("b.py".to_string(), WatchEventKind::Modified),
        WatchEvent::new("c.ts".to_string(), WatchEventKind::Modified),
    ];
    let result = analyzer.analyze(events);
    assert_eq!(result.len(), 3);
}

#[test]
fn analyze_all_same_path() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Modified),
    ];
    let result = analyzer.analyze(events);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].path, "main.rs");
}

#[test]
fn analyze_mixed_duplicates() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        WatchEvent::new("a.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("b.py".to_string(), WatchEventKind::Modified),
        WatchEvent::new("a.rs".to_string(), WatchEventKind::Created),
        WatchEvent::new("c.ts".to_string(), WatchEventKind::Modified),
        WatchEvent::new("b.py".to_string(), WatchEventKind::Removed),
    ];
    let result = analyzer.analyze(events);
    assert_eq!(result.len(), 3);
    let paths: Vec<&str> = result.iter().map(|e| e.path.as_str()).collect();
    assert!(paths.contains(&"a.rs"));
    assert!(paths.contains(&"b.py"));
    assert!(paths.contains(&"c.ts"));
}

// ─── filter_lintable (FR-003) ─────────────────────────────

#[test]
fn filter_lintable_keeps_lintable() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("app.py".to_string(), WatchEventKind::Modified),
        WatchEvent::new("index.ts".to_string(), WatchEventKind::Modified),
    ];
    let result = analyzer.filter_lintable(events);
    assert_eq!(result.len(), 3);
}

#[test]
fn filter_lintable_removes_non_lintable() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("image.png".to_string(), WatchEventKind::Modified),
        WatchEvent::new("data.bin".to_string(), WatchEventKind::Modified),
    ];
    let result = analyzer.filter_lintable(events);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].path, "main.rs");
}

#[test]
fn filter_lintable_empty_input() {
    let analyzer = ChangeAnalyzer::new();
    let result = analyzer.filter_lintable(vec![]);
    assert!(result.is_empty());
}

#[test]
fn filter_lintable_all_non_lintable() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        WatchEvent::new("image.png".to_string(), WatchEventKind::Modified),
        WatchEvent::new("video.mp4".to_string(), WatchEventKind::Modified),
    ];
    let result = analyzer.filter_lintable(events);
    assert!(result.is_empty());
}

// ─── Combined: analyze + filter_lintable pipeline ──────────

#[test]
fn pipeline_dedup_then_filter() {
    let analyzer = ChangeAnalyzer::new();
    let events = vec![
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Created),
        WatchEvent::new("image.png".to_string(), WatchEventKind::Modified),
        WatchEvent::new("app.py".to_string(), WatchEventKind::Modified),
    ];
    let deduped = analyzer.analyze(events);
    let filtered = analyzer.filter_lintable(deduped);
    assert_eq!(filtered.len(), 2);
    let paths: Vec<&str> = filtered.iter().map(|e| e.path.as_str()).collect();
    assert!(paths.contains(&"main.rs"));
    assert!(paths.contains(&"app.py"));
}
