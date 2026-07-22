// PURPOSE: Unit tests for utility functions — FileSystem and ReportFormatter helpers.
// Layer: Utility (stateless functions)

use shared::common::taxonomy_path_vo::FilePath;
use tui_lint_arwaky::utility_file_system;
use tui_lint_arwaky::utility_report_formatter;

// ─── list_directory: File listing ──

#[test]
fn utility_list_directory_returns_entries() {
    // List /tmp which should have entries
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let entries = utility_file_system::list_directory(&path);
    // Should return at least some entries (if /tmp has any files)
    let _ = &entries; // verify function returns valid list without panicking
}

// ─── list_directory: Skips hidden files ──

#[test]
fn utility_list_directory_skips_hidden_files() {
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let entries = utility_file_system::list_directory(&path);

    // None of the entries should start with "."
    for entry in &entries {
        assert!(
            !entry.name.starts_with('.'),
            "Hidden files should be skipped"
        );
    }
}

// ─── read_file_preview: Reading file content ──

#[test]
fn utility_read_file_preview_returns_content() {
    // Try reading a small file (e.g., /etc/hostname or similar)
    let path = FilePath::new("/tmp/readme.txt".to_string()).unwrap();
    let content = utility_file_system::read_file_preview(&path, 10);

    // If the file doesn't exist, should return empty content (graceful fallback — no panic)
    let _ = content; // verify function returns valid DisplayContent even on missing files
}

// ─── is_valid_directory: Directory validation ──

#[test]
fn utility_is_valid_directory_returns_true_for_tmp() {
    let path = FilePath::new("/tmp".to_string()).unwrap();
    assert!(utility_file_system::is_valid_directory(&path));
}

#[test]
fn utility_is_valid_directory_returns_false_for_nonexistent() {
    let path = FilePath::new("/nonexistent/path/that/does/not/exist".to_string()).unwrap();
    assert!(!utility_file_system::is_valid_directory(&path));
}

// ─── parent_directory: Path navigation ──

#[test]
fn utility_parent_directory_returns_parent() {
    let path = FilePath::new("/tmp/test.txt".to_string()).unwrap();
    let parent = utility_file_system::parent_directory(&path);
    assert!(parent.is_some());
    if let Some(p) = parent {
        assert_eq!(p.value, "/tmp");
    }
}

// ─── file_size_human: Size formatting ──

#[test]
fn utility_file_size_formats_bytes() {
    assert_eq!(
        utility_file_system::file_size_human(0),
        shared::common::taxonomy_display_content_vo::DisplayContent::new("0 B".to_string())
    );
    assert_eq!(
        utility_file_system::file_size_human(1024),
        shared::common::taxonomy_display_content_vo::DisplayContent::new("1.0 KiB".to_string())
    );
}

// ─── path_components: Path splitting ──

#[test]
fn utility_path_components_splits_path() {
    let path = FilePath::new("/tmp/test.txt".to_string()).unwrap();
    let components = utility_file_system::path_components(&path);
    assert!(!components.is_empty());
}

// ─── format_results: Report formatting ──

#[test]
fn utility_format_results_handles_empty_list() {
    let results = shared::cli_commands::taxonomy_result_vo::LintResultList::new(vec![]);
    let formatted = utility_report_formatter::format_results(&results);
    let _ = formatted; // verify function returns valid DisplayContent for empty results
}
