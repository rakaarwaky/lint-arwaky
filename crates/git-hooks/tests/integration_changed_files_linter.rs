// Integration test — FR-GitHooks-001 lint half (issue #582).
//
// The diff checker used to stub `run_git_diff_check` to zero violations.
// This test builds the real per-file aggregates, stages a violating Rust file
// in a temporary git repository, and asserts the pre-commit check path
// actually reports it.

use git_hooks_lint_arwaky::capabilities_changed_files_linter::ChangedFilesLinter;
use git_hooks_lint_arwaky::capabilities_diff_checker::DiffChecker;
use shared_common::FilePath;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_git_hooks::contract_git_hooks_protocol::IDiffDetectionProtocol;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use tempfile::TempDir;

fn git(dir: &Path, args: &[&str]) {
    let status = Command::new("git")
        .args(args)
        .current_dir(dir)
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "test@example.com")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "test@example.com")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .expect("git subprocess");
    assert!(
        status.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&status.stderr)
    );
}

/// Build the real lint pipeline over the changed-file seam, mirroring the
/// root container wiring.
fn real_linter() -> Arc<ChangedFilesLinter> {
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fc.orchestrator();
    let io = fc.io();
    let workspace = fc.workspace();
    let parser = fc.parser();

    let config_container = config_system::root_config_system_container::ConfigContainer::new(
        filesystem.clone(),
        io.clone(),
    );
    let config_orchestrator = config_container.orchestrator();

    let quality =
        quality_rules::root_quality_rules_container::CodeAnalysisContainer::from_orchestrator(
            &config_orchestrator,
            ".",
        )
        .code_analysis_linter();

    let import = import_rules::root_import_rules_container::ImportContainer::from_orchestrator(
        &config_orchestrator,
        ".",
        filesystem.clone(),
        io.clone(),
        workspace.clone(),
        parser.clone(),
    )
    .orchestrator();

    let naming = naming_rules::root_naming_rules_container::NamingContainer::new(
        Arc::new(
            config_orchestrator
                .execute(shared_config_system::ConfigRequest::load_sync(
                    &FilePath::new(".".to_string()).unwrap_or_default(),
                ))
                .into_sync_config(),
        ),
        Arc::new(shared_common::LayerMapVO::new(
            config_orchestrator
                .execute(shared_config_system::ConfigRequest::load_sync(
                    &FilePath::new(".".to_string()).unwrap_or_default(),
                ))
                .into_sync_config()
                .layers
                .clone(),
        )),
    )
    .orchestrator();

    let role = role_rules::root_role_rules_container::RoleContainer::new_with_config(
        config_orchestrator
            .execute(shared_config_system::ConfigRequest::load_sync(
                &FilePath::new(".".to_string()).unwrap_or_default(),
            ))
            .into_sync_config(),
    )
    .orchestrator();

    Arc::new(ChangedFilesLinter::new(
        io, workspace, parser, filesystem, quality, role, import, naming,
    ))
}

#[test]
fn run_git_diff_check_reports_staged_violation() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();

    git(dir, &["init", "--initial-branch=main"]);
    git(dir, &["add", "."]);
    // An empty tree commit so `diff HEAD` has a base.
    git(dir, &["commit", "--allow-empty", "-m", "init"]);

    // A staged Rust file with an unused import — a deterministic per-file
    // AES203 violation for the import rule group.
    let bad = dir.join("staged_violation.rs");
    std::fs::write(
        &bad,
        "use std::collections::HashMap;\n\npub fn nothing() {}\n",
    )
    .unwrap();
    git(dir, &["add", "staged_violation.rs"]);

    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let io: Arc<dyn IFileSystemIOProtocol> = fc.io();
    let checker = DiffChecker::new(io, real_linter());

    let target = FilePath::new(dir.to_string_lossy().to_string()).unwrap();
    let results = checker.run_git_diff_check(&target);

    assert!(
        !results.values.is_empty(),
        "run_git_diff_check must lint changed files, not stub the result away (issue #582)"
    );
    assert!(
        results
            .values
            .iter()
            .any(|r| r.code.code() == "AES203"),
        "expected an unused-import AES203 finding, got: {:?}",
        results
            .values
            .iter()
            .map(|r| r.code.code().to_string())
            .collect::<Vec<_>>()
    );
}

#[test]
fn clean_tree_produces_no_violations() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();

    git(dir, &["init", "--initial-branch=main"]);
    git(dir, &["commit", "--allow-empty", "-m", "init"]);

    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let io: Arc<dyn IFileSystemIOProtocol> = fc.io();
    let checker = DiffChecker::new(io, real_linter());

    let target = FilePath::new(dir.to_string_lossy().to_string()).unwrap();
    let results = checker.run_git_diff_check(&target);
    assert!(results.values.is_empty());
}
