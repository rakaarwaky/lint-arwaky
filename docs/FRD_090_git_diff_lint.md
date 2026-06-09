# 📄 Feature Requirements Document (FRD)
**Feature Name:** Git Diff Lint (`git-diff` subcommand)
**Product:** Lint Arwaky v1.10.2
**Author:** Raka
**Date:** 09/06/2026
**Version:** v1.0

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 09/06/2026 | Raka | Initial document creation | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the git-diff lint feature that runs lint rules only on files that have changed in the git working tree (unstaged + staged changes). This enables fast, focused linting during development and pre-commit scenarios.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli git-diff [path]` — lint only changed files
- Detects unstaged and staged changes via `git diff` / `git diff --cached`
- Runs all applicable lint rules on changed files only
- Default comparison: working tree vs HEAD
- Optional `--cached` flag: staged changes only
- Optional `--base <ref>` flag: diff against arbitrary ref (e.g., `main`)

**Out-of-Scope:**
- Pre-commit hook installation (separate feature FR-XXX)
- Auto-stashing of unstaged changes
- Lint of deleted files (no file to lint)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Working tree** | Current state of files on disk (unstaged changes) |
| **Staged** | Changes added to git index via `git add` |
| **HEAD** | Current commit |
| **`--base <ref>`** | Compare against an arbitrary git ref (branch, tag, commit) |

## 3. Feature Overview
### 3.1 Background & Problem
Running the full lint suite on every change is slow, especially for large codebases. Developers often skip linting because it takes too long. By linting only changed files, feedback becomes near-instant, encouraging developers to lint before committing.

### 3.2 Business Goals
- Reduce lint feedback time from minutes to seconds
- Encourage pre-commit linting by making it fast
- Enable CI to lint only changed files in a PR (diff against base branch)
- Integrate with git workflows seamlessly

### 3.3 Target Users
- **Developers**: Fast pre-commit linting
- **CI/CD Pipelines**: Lint only PR changes
- **Code Reviewers**: Verify only new code meets quality standards

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to run `lint-arwaky-cli git-diff` to lint only files I've changed, so I get fast feedback before committing.
- **US-002:** As a CI pipeline, I want to run `lint-arwaky-cli git-diff --base origin/main` to lint only files changed in a PR, so I catch issues without re-linting unchanged code.
- **US-003:** As a developer, I want `--cached` to lint only staged files, so I can check what I'm about to commit.

### 4.2 Use Cases & Workflow
**Git Diff Lint Pipeline:**
```
lint-arwaky-cli git-diff --base origin/main
  │
  ├─► 1. Run git diff --name-only <base>
  │     └── Returns list of changed files
  │
  ├─► 2. Filter to supported file types (.rs, .py, .js, .ts, etc.)
  │
  ├─► 3. Run full lint pipeline on each changed file
  │     ├── AES architecture rules
  │     ├── Naming conventions
  │     ├── Code quality checks
  │     └── Language-specific rules
  │
  └─► 4. Report violations only for changed files
        "src/main.rs:42  AES001  CRITICAL  Layer violation"
        "3 files changed, 2 violations found"
```

**Example Output:**
```
$ lint-arwaky-cli git-diff
🔎 Git-Diff Lint — /home/user/project
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Base: HEAD (working tree changes)
  Files changed: 4 (3 Rust, 1 Python)
  ─────────────────────────────────────────────────
  src/main.rs
    ├── AES001  CRITICAL  Layer 'capabilities' cannot import from 'infrastructure'
    └── AES014  CRITICAL  Unwrap detected: line 88

  src/utils.rs
    └── ✅ No violations

  src/parser.py
    └── ✅ No violations
  ─────────────────────────────────────────────────
  2 violations across 4 changed files
  Score: 98.5 / 100 (changed files only)
```

### 4.3 Business Rules
- Default comparison: HEAD (working tree + staged changes)
- `--cached`: staged changes only (vs HEAD)
- `--base <ref>`: changed files between ref and HEAD
- Deleted files are excluded (no file to lint)
- Non-tracked files are excluded unless added to index
- Violations only reported for changed files (unchanged file violations suppressed)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Git diff command latency | < 100ms |
| NFR-002 | File filter for supported types | < 10ms |
| NFR-003 | Lint time per changed file (small file) | < 200ms |
| NFR-004 | Total lint time for 20 changed files | < 5s |

## 6. UI/UX Requirements
```
$ lint-arwaky-cli git-diff --base origin/develop
🔎 Git-Diff Lint — comparing HEAD with origin/develop
  Files changed: 7
  ─────────────────────────────────────────────────
  ✗ src/api/handler.rs       2 violations
  ✗ src/db/models.rs         1 violation
  ✓ src/config.rs            clean
  ✓ src/main.rs              clean
  ✓ src/utils/helper.rs      clean
  ✓ tests/test_api.rs        clean
  ✓ src/lib.rs               clean
  ─────────────────────────────────────────────────
  3 violations in 2 of 7 changed files
  Score: 97.0 / 100
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | 3 files changed in working tree | `git-diff` runs | Only the 3 changed files are linted | Pending Review |
| AC-002 | 2 files changed, 1 has violations | `git-diff` runs | Violations from the changed file shown, clean file not listed | Pending Review |
| AC-003 | `--base origin/main` flag used | `git-diff` runs | Diff computed against origin/main | Pending Review |
| AC-004 | `--cached` flag used with staged changes | `git-diff --cached` runs | Only staged files linted | Pending Review |
| AC-005 | No files changed | `git-diff` runs | "No changes detected" message | Pending Review |

## 8. Empirical Findings (Code Audit)
### 8.1 Current Implementation
| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| Git diff command | `cli-commands/` | — | **NOT IMPLEMENTED** |
| Git diff executor | `infrastructure_/` | — | **NOT IMPLEMENTED** |
| Changed file filter | `code-analysis/` | — | **NOT IMPLEMENTED** |

### 8.2 What Needs to Be Added
- `handle_git_diff()` handler in CLI commands
- `git diff --name-only` executor with ref parsing
- File type filter (supported extensions)
- Changed-only lint runner (wraps existing lint engine)
- `--base`, `--cached` flags

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Lint Core) | Core lint engine for per-file linting | Must support single-file linting | Ensure engine accepts file list |
| Git CLI | `git diff` command must be available | Git not installed | Friendly error with hint |
| Git repository | Project must be a git repo | Non-git project fails | Error: "Not a git repository" |

## 10. Appendices
- `cli-commands/surface_git_diff_command.rs` — Git diff CLI handler
- `infrastructure_/infrastructure_git_diff_provider.rs` — Git command executor
- Config keys: `git_diff.base_ref`, `git_diff.include_untracked`
