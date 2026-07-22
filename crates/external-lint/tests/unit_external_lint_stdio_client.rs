// PURPOSE: Unit tests for StdioClient — subprocess execution via tokio::process.

use external_lint_lint_arwaky::StdioClient;
use shared::common::contract_executor_protocol::ICommandExecutorProtocol;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_path_vo::FilePath;

fn sut() -> StdioClient {
    StdioClient::new(Timeout::new(10.0))
}

// ─── Happy Path ───────────────────────────────────────────

#[tokio::test]
async fn execute_echo_command_returns_stdout() {
    let client = sut();
    let result = client
        .execute_command(
            PatternList::new(vec!["echo".to_string(), "hello".to_string()]),
            FilePath::new("/tmp".to_string()).unwrap(),
            Some(Timeout::new(5.0)),
        )
        .await;

    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.stdout.trim(), "hello");
    assert_eq!(resp.returncode, 0);
}

#[tokio::test]
async fn execute_command_captures_stderr() {
    let client = sut();
    let result = client
        .execute_command(
            PatternList::new(vec![
                "sh".to_string(),
                "-c".to_string(),
                "echo error >&2".to_string(),
            ]),
            FilePath::new("/tmp".to_string()).unwrap(),
            Some(Timeout::new(5.0)),
        )
        .await;

    assert!(result.is_ok());
    let resp = result.unwrap();
    assert!(resp.stderr.contains("error"));
}

#[tokio::test]
async fn execute_command_returns_nonzero_exit_code() {
    let client = sut();
    let result = client
        .execute_command(
            PatternList::new(vec![
                "sh".to_string(),
                "-c".to_string(),
                "exit 42".to_string(),
            ]),
            FilePath::new("/tmp".to_string()).unwrap(),
            Some(Timeout::new(5.0)),
        )
        .await;

    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.returncode, 42);
}

#[tokio::test]
async fn metadata_contains_stdio_protocol() {
    let client = sut();
    let result = client
        .execute_command(
            PatternList::new(vec!["echo".to_string(), "test".to_string()]),
            FilePath::new("/tmp".to_string()).unwrap(),
            None,
        )
        .await;

    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(
        resp.metadata.get("protocol").and_then(|v| v.as_str()),
        Some("Stdio")
    );
}

// ─── Error Paths ──────────────────────────────────────────

#[tokio::test]
async fn empty_command_returns_error() {
    let client = sut();
    let result = client
        .execute_command(
            PatternList::default(),
            FilePath::new("/tmp".to_string()).unwrap(),
            Some(Timeout::new(5.0)),
        )
        .await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Empty command"));
}

#[tokio::test]
async fn nonexistent_binary_returns_error() {
    let client = sut();
    let result = client
        .execute_command(
            PatternList::new(vec!["nonexistent_binary_xyz_12345".to_string()]),
            FilePath::new("/tmp".to_string()).unwrap(),
            Some(Timeout::new(5.0)),
        )
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn timeout_kills_long_running_command() {
    let client = StdioClient::new(Timeout::new(0.001));
    let result = client
        .execute_command(
            PatternList::new(vec!["sleep".to_string(), "60".to_string()]),
            FilePath::new("/tmp".to_string()).unwrap(),
            Some(Timeout::new(0.001)),
        )
        .await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("timed out"));
}

// ─── Health Check ─────────────────────────────────────────

#[tokio::test]
async fn health_check_returns_ok() {
    let client = sut();
    let result = client.health_check().await;
    assert!(result.is_ok());
}

// ─── Working Directory ────────────────────────────────────

#[tokio::test]
async fn respects_working_directory() {
    let client = sut();
    let result = client
        .execute_command(
            PatternList::new(vec!["pwd".to_string()]),
            FilePath::new("/tmp".to_string()).unwrap(),
            Some(Timeout::new(5.0)),
        )
        .await;

    assert!(result.is_ok());
    let resp = result.unwrap();
    // /tmp may resolve to /private/tmp on macOS
    assert!(resp.stdout.trim().ends_with("/tmp") || resp.stdout.trim().ends_with("/private/tmp"));
}
