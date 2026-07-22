// PURPOSE: Acceptance test for FR-006: File Read Error Diagnostics (AES000)
// Emit diagnostic when file cannot be read or exceeds size limit.

use shared::code_analysis::utility_file_reader::{read_lintable_file, MAX_LINT_FILE_BYTES};

/// FR-006: Max file size is 2 MiB
#[test]
fn fr_006_max_file_size_is_2mib() {
    assert_eq!(MAX_LINT_FILE_BYTES, 2 * 1024 * 1024);
}

/// FR-006: Readable file returns Ok(Some(content))
#[test]
fn fr_006_readable_file_returns_content() {
    // Create a small temp file
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    std::fs::write(&file_path, "fn main() {}").unwrap();

    let result = read_lintable_file(file_path.to_str().unwrap());
    assert!(result.is_ok());
    let content = result.unwrap();
    assert!(content.is_some());
    assert_eq!(content.unwrap(), "fn main() {}");
}

/// FR-006: Non-existent file returns Err
#[test]
fn fr_006_nonexistent_file_returns_err() {
    let result = read_lintable_file("/nonexistent/path/file.rs");
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("/nonexistent/path/file.rs"));
}

/// FR-006: Oversized file returns Ok(None) — graceful skip
#[test]
fn fr_006_oversized_file_returns_none() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("large.rs");

    // Create a file > 2 MiB
    let large_content = "x".repeat((2 * 1024 * 1024 + 1) as usize);
    std::fs::write(&file_path, &large_content).unwrap();

    let result = read_lintable_file(file_path.to_str().unwrap());
    assert!(result.is_ok());
    assert!(result.unwrap().is_none()); // Graceful skip, not an error
}

/// FR-006: File at exactly 2 MiB is readable
#[test]
fn fr_006_file_at_exact_limit_readable() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("exact.rs");

    let content = "x".repeat(MAX_LINT_FILE_BYTES as usize);
    std::fs::write(&file_path, &content).unwrap();

    let result = read_lintable_file(file_path.to_str().unwrap());
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}
