# FRD — git-hooks

## System Overview

The git-hooks crate implements a pre-commit hook system that enforces AES compliance before code enters the repository. It detects changed files via git diff, runs linting only on modified files, and blocks commits that violate AES rules. The crate follows the AES 7-layer architecture: `DiffChecker` and `HookManager` (capabilities) implement `IDiffProtocol` and `IHookProtocol`, `GitHookAdapter` (capabilities) implements `IHookManagerProtocol` for low-level hook file operations, `GitHooksOrchestrator` (agent) composes the three protocols, and `GitContainer` (root) wires dependencies.

## Functional Requirements

### FR-001: Git Diff Detection

- **Description**: Identify files changed between the current HEAD and the default branch using git diff commands.
- **Input**: `FilePath` (project root directory).
- **Output**: `GitDiffResultVO` containing lists of added, modified, deleted, renamed files; a filtered `lintable_files` list; and total change count.
- **Business Rules**:
  - Default branch detection: runs `git symbolic-ref refs/remotes/origin/HEAD`, falls back to `"main"`.
  - Changed file collection tries multiple diff variants in order:
    1. `origin/<branch>...HEAD`
    2. `HEAD...origin/<branch>`
    3. `<branch>...HEAD`
    4. `master...HEAD`
  - Falls back to `git diff --name-only HEAD` if all variants return empty.
  - Final fallback: `git ls-files --modified --others --exclude-standard`.
  - Lintable file filter: `.rs`, `.py`, `.ts`, `.js`, `.jsx`, `.tsx`, `.md`, `.toml`, `.json`, `.yaml`, `.yml`.
- **Edge Cases**:
  - No git repository: diff commands fail silently, returns empty result.
  - No remote configured: `symbolic-ref` fails, defaults to `"main"`.
  - No changes between branches: returns empty lists with `total_changed: 0`.
  - Detached HEAD state: diff variants may all fail; falls back to `HEAD` diff.
  - Shallow clone: diff may not find base branch; fallback strategies handle this.
- **Error Handling**:
  - Git command failure (non-zero exit): treated as no changes for that variant.
  - Invalid `FilePath` from git output: skipped silently.

### FR-002: Pre-Commit Hook Installation

- **Description**: Install a pre-commit hook script into `.git/hooks/` that runs `lint-arwaky check .` before each commit.
- **Input**: `FilePath` (path to the `lint-arwaky` executable).
- **Output**: `SuccessStatus` indicating whether the hook was installed.
- **Business Rules**:
  - Hook script content:
    ```bash
    #!/bin/bash
    # Lint Arwaky Pre-Commit Hook
    echo "Running Lint Arwaky check..."
    <executable> check .
    if [ $? -ne 0 ]; then
      echo "Linting failed. Please fix issues before committing."
      exit 1
    fi
    echo "Linting passed."
    exit 0
    ```
  - Creates `.git/hooks/` directory if it does not exist.
  - Sets hook file permissions to `0o755` on Unix systems.
  - If executable path is empty, defaults to `"lint-arwaky"`.
  - If not a git repository (no `.git/` dir): returns `SuccessStatus(false)` without error.
- **Edge Cases**:
  - `.git/hooks/` already exists: `create_dir_all` is idempotent.
  - Hook file already exists: overwritten.
  - Not a git repository: returns success with `false` (not an error).
  - Windows: permission setting is skipped (no `#[cfg(unix)]` block).
- **Error Handling**:
  - Directory creation failure: returns `GitHookError` with message.
  - File write failure: returns `GitHookError` with message.
  - Permission set failure: returns `GitHookError` with message.

### FR-003: Pre-Commit Hook Uninstallation

- **Description**: Remove the pre-commit hook script from `.git/hooks/`.
- **Input**: None.
- **Output**: `SuccessStatus` indicating whether the hook was removed.
- **Business Rules**:
  - Removes `.git/hooks/pre-commit` if it exists.
  - If not a git repository: returns `SuccessStatus(false)` without error.
  - If hook file does not exist: returns `SuccessStatus(true)` (already clean).
- **Edge Cases**:
  - Hook file does not exist: returns success (idempotent).
  - Not a git repository: returns success with `false`.
- **Error Handling**:
  - File removal failure: returns `GitHookError` with message.

### FR-004: Git Hooks Check Execution

- **Description**: Run the git diff check and lint pipeline on changed files.
- **Input**: `FilePath` (project root).
- **Output**: `LintResultList` containing lint results for changed files.
- **Business Rules**:
  - Delegates to `IDiffProtocol::run_git_diff_check` which collects changed files and returns lint results.
  - Only lintable file types (per FR-001 filter) are included in the diff result.
- **Edge Cases**:
  - No changed files: returns empty `LintResultList`.
  - All changed files are non-lintable: returns empty list.
- **Error Handling**:
  - Git command failure: treated as no changes.

### FR-005: Diff Data Comparison

- **Description**: Compare two file paths to determine their diff status.
- **Input**: Two file path strings.
- **Output**: `GitDiffDataVO` with version info, difference score, and status.
- **Business Rules**:
  - Status is determined by file existence:
    - Both missing: `MissingFirst` or `MissingSecond` (based on which is missing).
    - First missing: `MissingFirst`.
    - Second missing: `MissingSecond`.
    - Both exist but not files: `NotAFile`.
    - Both exist and are files: `Unchanged`.
  - Difference score is `0.0` (stub implementation).
- **Edge Cases**:
  - Both paths are the same file: status is `Unchanged`.
  - Paths are directories, not files: `NotAFile`.
- **Error Handling**:
  - None (pure file existence checks).

### FR-006: Config Initialization and Ignore Rule Management

- **Description**: Initialize lint-arwaky config and manage ignore rules in the config file.
- **Input**: Path string (for init), `HookIgnoreUpdateVO` (for ignore rules).
- **Output**: `DescriptionVO` with status message.
- **Business Rules**:
  - Init: checks if `lint_arwaky.config.yaml` exists at the given path; returns `ALREADY_EXISTS:<path>` if present, otherwise returns `Initialized <path>`.
  - Update ignore rule: adds or removes a rule from the ignore list in the config file.
  - If config file not found: returns error message.
- **Edge Cases**:
  - Config already exists: returns `ALREADY_EXISTS` status.
  - Config file not found during ignore update: returns descriptive error.
- **Error Handling**:
  - Config file not found: returns error description (not a `GitHookError`).

## Data Model / Entity Relationship

```
GitDiffResultVO (output from get_diff)
  ├── added: FilePathList
  ├── modified: FilePathList
  ├── deleted: FilePathList
  ├── renamed: RenamedFileList
  ├── lintable_files: FilePathList
  ├── all_files: FilePathList
  └── total_changed: Count

GitDiffDataVO (output from get_diff_data)
  ├── version1: GitDiffSideVO { path, score }
  ├── version2: GitDiffSideVO { path, score }
  ├── difference: f64
  └── status: GitDiffStatus (MissingFirst | MissingSecond | NotAFile | Unchanged)

GitHookError (error)
  └── message: LintMessage

HookIgnoreUpdateVO (input)
  ├── config_path: String
  ├── rule: String
  └── remove: bool

SuccessStatus (output)
  └── success: bool
```

## API Contract

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| `DiffChecker::run_git_diff_check(path)` | `&FilePath` | `LintResultList` | Run diff and collect changed files |
| `DiffChecker::get_diff(path)` | `&FilePath` | `GitDiffResultVO` | Get full diff result with lintable filter |
| `DiffChecker::get_changed_files(path, base)` | `&FilePath, &GitBranchName` | `FilePathList` | Get files changed vs base branch |
| `DiffChecker::get_default_branch(path)` | `&FilePath` | `GitBranchName` | Detect default branch name |
| `GitHookAdapter::install_pre_commit(exe_path)` | `&FilePath` | `Result<SuccessStatus, GitHookError>` | Write hook script to `.git/hooks/pre-commit` |
| `GitHookAdapter::uninstall_pre_commit()` | None | `Result<SuccessStatus, GitHookError>` | Remove hook script |
| `HookManager::install_pre_commit(exe_path)` | `&FilePath` | `Result<SuccessStatus, GitHookError>` | Delegate to hook adapter |
| `HookManager::uninstall_pre_commit()` | None | `Result<SuccessStatus, GitHookError>` | Delegate to hook adapter |
| `HookManager::initialize_config(path)` | `&str` | `DescriptionVO` | Check/create config file |
| `HookManager::update_ignore_rule(request)` | `HookIgnoreUpdateVO` | `DescriptionVO` | Add/remove ignore rule |
| `HookManager::get_diff_data(path1, path2)` | `&str, &str` | `GitDiffDataVO` | Compare two file paths |
| `GitHooksOrchestrator::run_git_hooks_check(path)` | `&FilePath` | `LintResultList` | Run full git hooks check |
| `GitHooksOrchestrator::install_hook(exe_path)` | `&FilePath` | `Result<SuccessStatus, GitHookError>` | Install pre-commit hook |
| `GitHooksOrchestrator::uninstall_hook()` | None | `Result<SuccessStatus, GitHookError>` | Uninstall pre-commit hook |
| `GitContainer::new(hook_adapter)` | `Arc<dyn IHookManagerProtocol>` | `GitContainer` | Wire and return container |
| `GitContainer::new_default()` | None | `GitContainer` | Wire with default adapter |

## Integration Points

- **Internal**:
  - `cli-commands` crate: `LintResultList`, `LintResult` for lint output.
  - `shared` crate: VOs (`FilePath`, `FilePathList`, `GitDiffResultVO`, `SuccessStatus`, `GitHookError`), contracts (`IDiffProtocol`, `IHookProtocol`, `IHookManagerProtocol`, `GitHooksAggregate`), utilities (`utility_git_io` for git command execution, `utility_file_handler` for file operations).
- **External**:
  - `git` CLI: `diff --name-only`, `symbolic-ref`, `ls-files` for change detection.
  - Filesystem: `.git/hooks/` directory operations, config file read/write.
  - `std::fs`: permissions, file removal (via `utility_git_io`).

## Non-functional Requirements (Detailed)

- **Performance**: Diff detection uses multiple fallback strategies; early termination when changes are found. Git command execution is the bottleneck (subprocess spawn).
- **Memory**: Changed files are collected into a `HashSet<FilePath>` to deduplicate across diff variants. Memory scales with number of changed files.
- **Accuracy**: Only actually changed files are scanned. Multiple diff strategies ensure compatibility with different git states (shallow clone, detached HEAD, etc.).
- **Cross-platform**: Hook installation supports Linux, macOS (Unix permissions), and Windows (no permission setting). Git commands are platform-agnostic.
- **Reliability**: Multiple fallback strategies for diff detection ensure the system works even when the primary diff command fails.

## Test Scenarios / QA Checklist

- [ ] Default branch detected from `origin/HEAD` symbolic ref.
- [ ] Default branch defaults to `"main"` when `symbolic-ref` fails.
- [ ] Changed files collected via `origin/main...HEAD` variant.
- [ ] Fallback to `HEAD` diff when branch variants return empty.
- [ ] Fallback to `ls-files` when all diff strategies fail.
- [ ] Lintable filter includes `.rs`, `.py`, `.ts`, `.js`, `.json`, `.yaml`.
- [ ] Lintable filter excludes `.bin`, `.exe`, `.png`, `.lock`.
- [ ] Empty diff returns `total_changed: 0`.
- [ ] Hook installation creates `.git/hooks/` if missing.
- [ ] Hook script contains correct executable path.
- [ ] Hook permissions set to `0o755` on Unix.
- [ ] Hook installation skips gracefully when not a git repo.
- [ ] Hook uninstallation removes `.git/hooks/pre-commit`.
- [ ] Hook uninstallation is idempotent (no error if already removed).
- [ ] `get_diff_data` returns `MissingFirst` when first path missing.
- [ ] `get_diff_data` returns `Unchanged` when both paths exist as files.
- [ ] `initialize_config` returns `ALREADY_EXISTS` when config present.
- [ ] `update_ignore_rule` returns error when config file missing.
- [ ] Full `run_git_hooks_check` returns lint results for changed files.
- [ ] `install_hook` + `uninstall_hook` round-trip works correctly.

## Assumptions & Constraints

- `git` CLI is installed and available in PATH.
- The project is a git repository (has `.git/` directory) for hook operations.
- Git commands execute within a reasonable timeout (subprocess-based).
- The pre-commit hook runs `lint-arwaky check .` which must be in PATH or specified via executable path.
- Config file format (`lint_arwaky.config.yaml`) is stable and parseable.

## Glossary

- **Pre-commit hook**: A git hook that runs before a commit is finalized; can block the commit by exiting non-zero.
- **Lintable file**: A source file that can be analyzed by lint-arwaky (filtered by extension).
- **Default branch**: The main development branch (typically `main` or `master`) used as the diff base.
- **Diff variant**: A git diff command string tried against the repository to find changed files.
- **AES**: Arwaky Engineering Standards — the architecture rules enforced by lint-arwaky.
- **Hook manager**: Low-level component that handles `.git/hooks/` file operations.
- **Diff checker**: Component that runs git commands to identify changed files.

## Reference

- PRD: [PRD.md](../../PRD.md)
