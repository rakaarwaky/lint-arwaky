# 📄 Feature Requirements Document (FRD)
**Feature Name:** Git Pre-Commit Hook — `install-hook`/`uninstall-hook`
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
Defines the `install-hook` and `uninstall-hook` subcommands for managing Git pre-commit hooks that auto-lint staged files before commits.

### 2.2 Scope
**In-Scope:** `lint-arwaky-cli install-hook [--hook-dir .git/hooks]`, `uninstall-hook`, pre-commit hook script generation, staged-files-only linting.
**Out-of-Scope:** Husky integration, server-side hooks, post-commit hooks.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Pre-commit hook** | Git hook script that runs before commit |
| **Staged files** | Files added to Git index via `git add` |

## 3. Feature Overview
### 3.1 Background & Problem
Teams committed code with violations because lint was not enforced pre-commit. Manual linting was easy to skip.

### 3.2 Business Goals
- Prevent commits with lint violations
- Only lint staged files for speed
- Simple install/uninstall workflow

### 3.3 Target Users
- Development teams enforcing code quality gates

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a team lead, I want `install-hook` to create a pre-commit hook that lints staged files.
- **US-002:** As a developer, I want `uninstall-hook` to remove the hook when switching projects.

### 4.2 Use Cases & Workflow
```
lint-arwaky-cli install-hook --hook-dir .git/hooks
  │
  ├─► Generate pre-commit script
  ├─► Script content: run "lint-arwaky-cli check --staged"
  │     on staged files; exit 1 if violations found
  └─► Write to .git/hooks/pre-commit (executable)

Developer commits:
  git commit -m "fix: ..."
    │
    ├─► Hook runs: lint-arwaky-cli check --staged
    ├─► Violations found? → Commit ABORTED
    └─► Clean? → Commit proceeds
```

### 4.3 Business Rules
- Hook only lints staged files (not full project)
- Exit code 1 if violations found (aborts commit)
- `--staged` flag on check command filters to staged files only

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Hook execution (100 staged files) | < 3s |
| NFR-002 | Hook installation | < 100ms |

## 6. UI/UX Requirements
```
$ lint-arwaky-cli install-hook
 ✅ Pre-commit hook installed at .git/hooks/pre-commit

$ git commit -m "wip"
 🔍 Linting staged files...
 ⛔ Commit blocked: 2 violations found
   AES003 - naming violation in src/main.rs:12
   AES014 - #[allow(dead_code)] in src/lib.rs:5
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Install hook | `install-hook` | Script created at `.git/hooks/pre-commit` | Pending Review |
| AC-002 | Uninstall hook | `uninstall-hook` | Hook script removed | Pending Review |
| AC-003 | Commit with violations | `git commit` with lint errors | Commit aborted with violation report | Pending Review |
| AC-004 | Commit clean | `git commit` with no violations | Commit proceeds normally | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Install hook command | `git-hooks/surface_install_hook_command.rs` | Pending Review |
| Uninstall hook command | `git-hooks/surface_uninstall_hook_command.rs` | Pending Review |
| Hook script generator | `git-hooks/capabilities_hook_generator.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Lint Pipeline) | Hook invokes lint on staged files | Hook adds latency to commit | Only lint staged files |
| `.git/hooks` directory | Must exist for install | No `.git` → install fails | Validate `.git` exists before install |

## 10. Appendices
- `src-rust/git-hooks/` — Feature folder
