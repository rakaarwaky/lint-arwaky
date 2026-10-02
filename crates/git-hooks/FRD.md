# FRD — git-hooks

---

## Reference

- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature; this file is specification only.
- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
- Config System FRD: `crates/config-system/FRD.md` (config resolution)
- Project Setup FRD: `crates/project-setup/FRD.md` (config initialization)

## System Overview

The git-hooks crate implements a pre-commit hook system that enforces AES
compliance before code enters the repository. It detects changed files via
git diff, runs linting only on modified files, and blocks commits that
violate AES rules.

The crate follows the AES 7-layer architecture: the diff checker (capability)
implements the diff protocol, the git hook adapter (capability) implements
both the install and uninstall protocols, the hook manager (capability)
implements the config protocol, the git hooks orchestrator (agent) composes
the four protocols, and the git container (root) wires dependencies.

### Architecture & Data Flow

```mermaid
flowchart TD
    A["Surface"] -->|input| B["git hooks orchestrator"]
    B --> C{"action"}

    C -->|"check / git-diff"| D["diff checker"]
    C -->|"install-hook"| E["hook adapter"]
    C -->|"uninstall-hook"| F["hook adapter"]
    C -->|"init-config / ignore-rule"| G["hook manager"]

    D --> H["filesystem\n(run git commands)"]
    H --> I["changed files\n(lintable filter)"]
    I --> J["lint pipeline\n(via linter aggregates)"]
    J --> K["Lint Results"]

    E --> L[".git/hooks/pre-commit"]
    L --> M["Success / Error"]

    F --> M
    G --> N["Config Update"]

    K --> B
    M --> B
    N --> B

    B -->|output| A
```

---

## Functional Requirements

### FR-GitHooks-001: Git Diff Detection

- **Description**: Identify files changed between the current HEAD and the
  default branch using git diff commands, then run the lint pipeline over
  the lintable subset.
- **Input**: `FilePath` (project root directory).
- **Output**: `GitDiffResultVO` (from `get_diff`) or `LintResultList` (from
  `run_git_diff_check`).
- **Business Rules**:

  - Default branch detection: runs
    `git symbolic-ref refs/remotes/origin/HEAD`, falls back to `"main"`.
  - Changed file collection tries multiple diff variants in order:
    1. `origin/<branch>...HEAD`
    2. `HEAD...origin/<branch>`
    3. `<branch>...HEAD`
    4. `master...HEAD`
  - Falls back to `git diff --name-only HEAD` if all variants return empty.
  - Final fallback: `git ls-files --modified --others --exclude-standard`.
  - The fallback chain exists here, and not in the external-lint adapters, by
    the Integration resilience decision in `PRD.md`: every strategy above
    queries the same trusted local repository, so a fallback still yields a
    correct — if broader — file list, whereas a retried external tool run can
    report against a changed tool state.
  - Classification via `--diff-filter`: added (A), modified (M), deleted (D),
    renamed (R with `old => new` parsing).
  - Lintable file filter (source code only):
    `.rs`, `.py`, `.ts`, `.js`, `.jsx`, `.tsx`.
  - Non-source files (`.md`, `.toml`, `.json`, `.yaml`, `.yml`, `.lock`,
    images, binaries) are excluded from lintable list.
- **Edge Cases**:

  - No git repository → diff commands fail silently, returns empty result.
  - No remote configured → `symbolic-ref` fails, defaults to `"main"`.
  - No changes between branches → returns empty lists with
    `total_changed: 0`.
  - Detached HEAD state → diff variants may all fail; falls back to `HEAD`
    diff.
  - Shallow clone → diff may not find base branch; fallback strategies
    handle this.
- **Error Handling**:

  - Git command failure (non-zero exit) → treated as no changes for that
    variant.
  - Invalid `FilePath` from git output → skipped silently.

---

### FR-GitHooks-002: Pre-Commit Hook Installation

- **Description**: Install a pre-commit hook script into `.git/hooks/` that
  runs `lint-arwaky check .` before each commit.
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
  - If executable path is empty, defaults to `"lint-arwaky-cli"`.
  - If not a git repository (no `.git/` dir) → returns
    `SuccessStatus(false)` without error.
- **Edge Cases**:

  - `.git/hooks/` already exists → directory creation is idempotent.
  - Hook file already exists → overwritten.
  - Not a git repository → returns success with `false` (not an error).
  - Windows → permission setting is skipped (Unix-only feature).
- **Error Handling**:

  - Directory creation failure → returns `GitHookError` with message.
  - File write failure → returns `GitHookError` with message.
  - Permission set failure → returns `GitHookError` with message.

---

### FR-GitHooks-003: Pre-Commit Hook Uninstallation

- **Description**: Remove the pre-commit hook script from `.git/hooks/`.
- **Input**: None.
- **Output**: `SuccessStatus` indicating whether the hook was removed.
- **Business Rules**:

  - Removes `.git/hooks/pre-commit` if it exists.
  - If not a git repository → returns `SuccessStatus(false)` without error.
  - If hook file does not exist → returns `SuccessStatus(true)`
    (already clean).
- **Edge Cases**:

  - Hook file does not exist → returns success (idempotent).
  - Not a git repository → returns success with `false`.
- **Error Handling**: File removal failure → returns `GitHookError` with
  message.

---

### FR-GitHooks-004: Project Config Initialization

- **Description**: Create the default lint-arwaky config file and manage
  per-path ignore rules within it.
- **Input**: Path string (for init); `HookIgnoreUpdateVO` (for rule update).
- **Output**: `DescriptionVO` with status message.
- **Business Rules**:

  - Target file: `lint_arwaky.config.yaml` in the given path.
  - Default content includes `ignored_paths: []` structure.
  - If config file already exists → returns "ALREADY_EXISTS" status
    (no-op, idempotent).
  - Ignore rules are path patterns stored in `ignored_paths`.
  - Adding a rule that already exists → no-op, returns "already present".
  - Removing a rule that does not exist → no-op, returns "not found".
  - Config file not found on update → descriptive error suggesting
    `lint-arwaky-cli init`.
- **Edge Cases**:

  - Config file already present → no-op, descriptive status message.
  - File write failure → error description returned.
  - Config parse failure → error description returned.
- **Error Handling**: Write and parse failures return descriptive error
  messages.

---

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
|---|---|---|---|---|---|
| `get_diff` | &FilePath | `GitDiffResultVO` | — | — | Get diff result. |
| `get_changed_files` | &FilePath, &GitBranchName | `FilePathList` | — | — | Get changed files. |
| `get_default_branch` | &FilePath | `GitBranchName` | — | — | Get default branch. |
| `run_git_diff_check` | &FilePath | `LintResultList` | — | — | Run lint on changed files. |
| `install_pre_commit` | &FilePath | `SuccessStatus` | `GitHookError` | — | Install pre-commit hook. |
| `uninstall_pre_commit` | — | `SuccessStatus` | `GitHookError` | — | Uninstall pre-commit hook. |
| `initialize_config` | &str | `DescriptionVO` | — | — | Initialize config. |
| `update_ignore_rule` | HookIgnoreUpdateVO | `DescriptionVO` | — | — | Update ignore rule. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|---|---|---|---|---|---|
| `execute` | GitHooksRequest | `GitHooksResponse` | — | — | Single composite entry point over the feature. |

## Integration Points

| System | Direction | Purpose | Failure mode |
| --- | --- | --- | --- |
| `shared` crate | in | Supply value objects plus the diff, hook, hook-manager, and aggregate contracts | A contract is missing at compile time → the build fails before any hook is installed |
| `filesystem` aggregate | in | Run git subprocesses, perform hook-directory and script writes, and read configuration | A git subprocess cannot be spawned → that diff strategy fails and the next fallback strategy is tried |
| Linter aggregates | in | Run AES analysis over the changed files | Analysis finds violations → the hook exits non-zero and the commit is blocked; analysis itself failing is reported distinctly from violations |
| `git` CLI | in | Report changed paths and the current branch through diff, symbolic-ref, and ls-files | The repository is shallow, detached, or mid-rebase → the primary diff strategy fails and a fallback strategy is used |
| Hook directory | in | Hold the installed hook script | The directory is not writable → installation reports failure and leaves the repository unchanged |
| Standard library file operations | in | Set the executable bit and remove the previous script | Permission setting is unsupported on the platform → the hook is installed without the executable bit and reports which platform it is on |

## Non-functional Requirements

| Metric | Target | Measurement method |
| --- | --- | --- |
| Diff detection | Early termination once changes are found; git subprocess spawn dominates the cost | Time the hook on a workspace with and without staged changes and compare |
| Deduplication | A file changed across several diff strategies is analysed once | Stage a change that two diff strategies both report and assert one analysis run |
| Scan scope | Only changed files are analysed | Compare the analysed file set against the repository's changed-file list |
| Repository-state coverage | Diff detection succeeds in a shallow clone, a detached HEAD, and a mid-rebase state | Run the hook in each of the three states and assert the changed set is correct |
| Memory | The changed-file set is held once and deduplicated | Scale the staged file count and record the retained set size |
| Cross-platform install | The hook installs on Linux, macOS, and Windows, setting the executable bit only where it exists | Install on each platform and assert the script is registered and, where applicable, executable |
| Install atomicity | A failed install leaves the previous hook script in place | Interrupt an install and assert the original script is still registered |
| Block semantics | The hook exits non-zero exactly when violations are found, and zero otherwise | Run the hook against a clean and a violating change set and assert the two exit codes |

## Test Scenarios / QA Checklist

Each scenario is stated below as a table of cases: the input condition and the expected result.

- **SCEN-001 — Git Diff Detection** — e.g. Default branch from `origin/HEAD` → Correct branch detected
- **SCEN-002 — Hook Installation** — e.g. Normal install → Hook script created with correct executable
- **SCEN-003 — Hook Uninstallation** — e.g. Hook exists → Removed, SuccessStatus(true)
- **SCEN-004 — Project Config Initialization** — e.g. Config not present → Default config created, success

### SCEN-001 — Git Diff Detection

FRD Ref: FR-GitHooks-001

| # | Scenario | Expected |
| - | - | - |
| 1 | Default branch from `origin/HEAD` | Correct branch detected |
| 2 | `symbolic-ref` fails | Defaults to "main" |
| 3 | Changed files via `origin/main...HEAD` | Correct file list |
| 4 | All branch variants empty | Fallback to `HEAD` diff |
| 5 | All diff strategies fail | Fallback to `ls-files` |
| 6 | Lintable filter: .rs, .py, .ts, .js, .jsx, .tsx | Included |
| 7 | Non-lintable: .md, .toml, .json, .png, .lock | Excluded |
| 8 | Empty diff | total_changed: 0 |
| 9 | Detached HEAD | Fallback strategies handle |
| 10 | Renamed files classified via `--diff-filter=R` | Old/new paths parsed |
| 11 | Changed files with violations | Lint results returned |
| 12 | No changed files | Empty result list |
| 13 | Changed file with parse failure | Skipped by linters, no warning |
| 14 | All changed files non-lintable | Empty result list |

### SCEN-002 — Hook Installation

FRD Ref: FR-GitHooks-002

| # | Scenario | Expected |
| - | - | - |
| 1 | Normal install | Hook script created with correct executable |
| 2 | `.git/hooks/` missing | Directory created |
| 3 | Hook file already exists | Overwritten |
| 4 | Not a git repo | SuccessStatus(false), no error |
| 5 | Unix permissions | 0o755 set |
| 6 | Windows | Permission setting skipped |
| 7 | Empty executable path | Defaults to "lint-arwaky-cli" |

### SCEN-003 — Hook Uninstallation

FRD Ref: FR-GitHooks-003

| # | Scenario | Expected |
| - | - | - |
| 1 | Hook exists | Removed, SuccessStatus(true) |
| 2 | Hook doesn't exist | SuccessStatus(true), idempotent |
| 3 | Not a git repo | SuccessStatus(false) |

### SCEN-004 — Project Config Initialization

FRD Ref: FR-GitHooks-004

| # | Scenario | Expected |
| - | - | - |
| 1 | Config not present | Default config created, success |
| 2 | Config already exists | Idempotent, "ALREADY_EXISTS" status |
| 3 | Add ignore rule | Rule added to config |
| 4 | Remove ignore rule | Rule removed from config |
| 5 | Config file not found | Error suggesting `lint-arwaky-cli init` |
| 6 | Rule already exists (add) | No-op, "already present" |
| 7 | Write failure | Error description returned |

---

## Assumptions & Constraints

- `git` CLI is installed and available in PATH.
- The project is a git repository (has `.git/` directory) for hook
  operations.
- Git commands execute within a reasonable timeout (subprocess-based,
  invoked via `IFilesystemAggregate::run_git_command`).
- The pre-commit hook runs `lint-arwaky-cli check .` which must be in PATH
  or specified via executable path.
- Config file format (`lint_arwaky.config.yaml`) is stable and
  parseable.
- Lintable files are source code only (.rs, .py, .ts, .js, .jsx, .tsx).
  Non-source files are excluded from linting.
- No async runtime dependency.

---

## Glossary

- **AES**: Agentic Engineering System — the 7-layer coding convention
- **Pre-commit hook**: A git hook that runs before a commit is finalized; can block the commit by exiting non-zero
- **Lintable file**: A source code file that can be analyzed by lint-arwaky (.rs, .py, .ts, .js, .jsx, .tsx)
- **Default branch**: The main development branch (typically `main` or `master`) used as the diff base
- **Diff variant**: A git diff command string tried against the repository to find changed files
- **Hook manager**: Low-level component that handles `.git/hooks/` file operations
- **Diff checker**: Component that runs git commands to identify changed files
- **Parse skip**: Files that fail to parse are skipped by the linter aggregates; no separate warning diagnostic is included in output.

---
