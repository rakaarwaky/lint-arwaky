// Unit tests — ChangeFilter edge cases and deduplication logic.
use file_watch_lint_arwaky::capabilities_change_filter::ChangeFilter;
use shared_file_watch::contract_watch_protocol::IChangeFilterProtocol;
use shared_file_watch::taxonomy_file_watch_vo::{WatchEvent, WatchEventKind};

// ─── is_lintable edge cases (FR-002) ──────────────────────

#[test]
fn is_lintable_rust_source() {
    let filter = ChangeFilter::new();
    assert!(filter.is_lintable("main.rs"));
    assert!(filter.is_lintable("src/lib.rs"));
}

#[test]
fn is_lintable_python_source() {
    let filter = ChangeFilter::new();
    assert!(filter.is_lintable("app.py"));
    assert!(filter.is_lintable("tests/test_main.py"));
}

#[test]
fn is_lintable_typescript_sources() {
    let filter = ChangeFilter::new();
    assert!(filter.is_lintable("index.ts"));
    assert!(filter.is_lintable("component.tsx"));
    assert!(filter.is_lintable("util.jsx"));
    assert!(filter.is_lintable("helper.mjs"));
    assert!(filter.is_lintable("helper.cjs"));
}

#[test]
fn is_lintable_config_files() {
    let filter = ChangeFilter::new();
    assert!(filter.is_lintable("Cargo.toml"));
    assert!(filter.is_lintable("config.yaml"));
    assert!(filter.is_lintable("data.yml"));
    assert!(filter.is_lintable("package.json"));
    assert!(filter.is_lintable("styles.css"));
    assert!(filter.is_lintable("README.md"));
}

#[test]
fn is_lintable_non_lintable_extensions() {
    let filter = ChangeFilter::new();
    assert!(!filter.is_lintable("image.png"));
    assert!(!filter.is_lintable("data.bin"));
    assert!(!filter.is_lintable("archive.tar.gz"));
    assert!(!filter.is_lintable("video.mp4"));
    assert!(!filter.is_lintable("sound.wav"));
}

#[test]
fn is_lintable_no_extension() {
    let filter = ChangeFilter::new();
    assert!(!filter.is_lintable("Makefile"));
    assert!(!filter.is_lintable("Dockerfile"));
    assert!(!filter.is_lintable("LICENSE"));
}

#[test]
fn is_lintable_hidden_files() {
    let filter = ChangeFilter::new();
    // Hidden files like .gitignore have no matching extension
    assert!(!filter.is_lintable(".gitignore"));
    assert!(!filter.is_lintable(".env"));
    assert!(!filter.is_lintable(".dockerignore"));
}

#[test]
fn is_lintable_multiple_dots() {
    let filter = ChangeFilter::new();
    // FR-002: matches on the final extension
    assert!(filter.is_lintable("file.test.ts"));
    assert!(filter.is_lintable("spec.unit.py"));
    assert!(filter.is_lintable("config.prod.yaml"));
}

#[test]
fn is_lintable_case_sensitive() {
    let filter = ChangeFilter::new();
    // FR-002: extension matching is case-sensitive (no normalization)
    assert!(!filter.is_lintable("FILE.RS"));
    assert!(!filter.is_lintable("App.PY"));
    assert!(!filter.is_lintable("Index.TS"));
}

#[test]
fn is_lintable_empty_string() {
    let filter = ChangeFilter::new();
    assert!(!filter.is_lintable(""));
}

#[test]
fn is_lintable_path_with_directories() {
    let filter = ChangeFilter::new();
    assert!(filter.is_lintable("/home/user/project/src/main.rs"));
    assert!(filter.is_lintable("./src/lib.py"));
    assert!(!filter.is_lintable("/home/user/project/image.png"));
}

// ─── dedup_events deduplication (FR-002) ───────────────────────

#[test]
fn dedup_empty_input() {
    let filter = ChangeFilter::new();
    let result = filter.dedup_events(vec![]);
    assert!(result.is_empty());
}

#[test]
fn dedup_no_duplicates() {
    let filter = ChangeFilter::new();
    let events = vec![
        WatchEvent::new("a.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("b.py".to_string(), WatchEventKind::Modified),
        WatchEvent::new("c.ts".to_string(), WatchEventKind::Modified),
    ];
    let result = filter.dedup_events(events);
    assert_eq!(result.len(), 3);
}

#[test]
fn dedup_all_same_path() {
    let filter = ChangeFilter::new();
    let events = vec![
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Modified),
    ];
    let result = filter.dedup_events(events);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].path, "main.rs");
}

#[test]
fn dedup_mixed_duplicates() {
    let filter = ChangeFilter::new();
    let events = vec![
        WatchEvent::new("a.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("b.py".to_string(), WatchEventKind::Modified),
        WatchEvent::new("a.rs".to_string(), WatchEventKind::Created),
        WatchEvent::new("c.ts".to_string(), WatchEventKind::Modified),
        WatchEvent::new("b.py".to_string(), WatchEventKind::Removed),
    ];
    let result = filter.dedup_events(events);
    assert_eq!(result.len(), 3);
    let paths: Vec<&str> = result.iter().map(|e| e.path.as_str()).collect();
    assert!(paths.contains(&"a.rs"));
    assert!(paths.contains(&"b.py"));
    assert!(paths.contains(&"c.ts"));
}

// ─── filter_events (FR-002) ─────────────────────────────

#[test]
fn filter_events_keeps_lintable() {
    let filter = ChangeFilter::new();
    let events = vec![
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("app.py".to_string(), WatchEventKind::Modified),
        WatchEvent::new("index.ts".to_string(), WatchEventKind::Modified),
    ];
    let result = filter.filter_events(events);
    assert_eq!(result.len(), 3);
}

#[test]
fn filter_events_removes_non_lintable() {
    let filter = ChangeFilter::new();
    let events = vec![
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("image.png".to_string(), WatchEventKind::Modified),
        WatchEvent::new("data.bin".to_string(), WatchEventKind::Modified),
    ];
    let result = filter.filter_events(events);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].path, "main.rs");
}

#[test]
fn filter_events_empty_input() {
    let filter = ChangeFilter::new();
    let result = filter.filter_events(vec![]);
    assert!(result.is_empty());
}

#[test]
fn filter_events_all_non_lintable() {
    let filter = ChangeFilter::new();
    let events = vec![
        WatchEvent::new("image.png".to_string(), WatchEventKind::Modified),
        WatchEvent::new("video.mp4".to_string(), WatchEventKind::Modified),
    ];
    let result = filter.filter_events(events);
    assert!(result.is_empty());
}

// ─── Combined: dedup_events + filter_events pipeline ──────

#[test]
fn pipeline_dedup_then_filter() {
    let filter = ChangeFilter::new();
    let events = vec![
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Created),
        WatchEvent::new("image.png".to_string(), WatchEventKind::Modified),
        WatchEvent::new("app.py".to_string(), WatchEventKind::Modified),
    ];
    let filtered = filter.filter_events(events);
    assert_eq!(filtered.len(), 2);
    let paths: Vec<&str> = filtered.iter().map(|e| e.path.as_str()).collect();
    assert!(paths.contains(&"main.rs"));
    assert!(paths.contains(&"app.py"));
}
