# 📄 Feature Requirements Document (FRD)
**Feature Name:** Apply Safe Fixes (`fix [path]`)  
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
This document defines the apply safe fixes CLI command `fix [path]`. It automatically applies safe auto-fixes for detected AES violations, including naming convention fixes (AES003), bypass comment removal (AES014), unused import removal (AES015), and dead inheritance cleanup (AES016). Supports dry-run mode for preview.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli fix <path>` — apply all safe auto-fixes
- `lint-arwaky-cli fix <path> --dry-run` — preview without applying
- `LintFixOrchestrator` coordinating fix pipeline
- `NamingRenamerProcessor` for AES003 symbol renaming
- Bypass comment removal (`#[allow(...)]`, `noqa`, `type: ignore`)
- Unused import removal
- Dead inheritance cleanup (empty `impl` blocks)
- `FixAppliedEvent` tracking for audit trail

**Out-of-Scope:**
- Non-auto-fixable rules (AES001 import, AES004 size, AES005 small, AES006 primitive)
- External tool fixes (clippy --fix, ruff --fix — planned for future)
- Config file modification

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **LintFixOrchestrator** | Agent that coordinates the fix pipeline |
| **NamingRenamerProcessor** | Capability that renames symbols project-wide |
| **FixResult** | Struct with output description and optional error |
| **FixAppliedEvent** | Event recorded after a fix is applied |
| **Dry-run** | Preview fixes without modifying files |
| **Auto-fixable** | Violation type that can be safely repaired automatically |

## 3. Feature Overview
### 3.1 Background & Problem
Violations were only reported, never fixed automatically. Developers had to manually fix every violation, including repetitive tasks like renaming symbols, removing bypass comments, and deleting unused imports. There was no preview mode to review changes before applying them.

### 3.2 Business Goals
- Reduce manual fix effort by 80% for auto-fixable violations
- Provide safe, predictable automated fixes
- Enable dry-run preview for developer confidence
- Maintain a complete audit trail of all applied fixes

### 3.3 Target Users
- **Developers**: Run `fix .` to auto-repair naming, bypass, and import violations
- **AI Agents**: Use MCP to trigger fix pipeline autonomously
- **CI/CD Pipelines**: Run `fix . --dry-run` to check if fixes are needed without modifying code

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to run `fix .` to automatically fix naming convention violations (AES003) by renaming files and symbols.
- **US-002:** As a developer, I want `fix . --dry-run` to preview all changes before they are applied, so I can review them.
- **US-003:** As a developer, I want non-fixable violations listed as manual steps after `fix` runs, so I know what still needs attention.
- **US-004:** As a developer, I want dead inheritance blocks (empty `impl` blocks) removed automatically.

### 4.2 Use Cases & Workflow
**Fix Pipeline:**
```
lint-arwaky-cli fix /project
  │
  ├─► 1. Run check → collect all violations
  │
  ├─► 2. Classify by auto-fixability:
  │     ├── Auto-fixable:
  │     │   ├── AES003 (naming) → NamingRenamerProcessor
  │     │   ├── AES014 (bypass) → remove #[allow], noqa, type: ignore
  │     │   ├── AES015 (unused) → remove import line
  │     │   └── AES016 (dead inh.) → remove empty impl block
  │     └── Manual only:
  │         ├── AES001 (import violation)
  │         ├── AES004 (file too large)
  │         ├── AES005 (file too small)
  │         └── AES006 (primitive obsession)
  │
  ├─► 3. Apply fixes (skip in --dry-run)
  │     ├── For each auto-fixable violation:
  │     │     ├── Compute fix operation
  │     │     ├── In dry-run: log intended operation
  │     │     └── In normal mode: apply + record FixAppliedEvent
  │
  └─► 4. Report:
        "3 violations fixed automatically"
        "5 violations require manual fix"
        "Use --dry-run to preview before applying"
```

**Naming Renamer Detail:**
```
NamingRenamerProcessor.rename_symbol(root, old_name, new_name)
  ├─► Walk all files in project
  ├─► For each file:
  │     ├── Skip: comments, string literals, template literals
  │     ├── Match: old_name → new_name (word boundary regex)
  │     └── Replace in-place
  └─► Return count of modified files
```

### 4.3 Business Rules
- Only violations marked as "safe" are auto-fixed
- Dry-run mode: analyze but never write to disk
- Each fix operation generates a `FixAppliedEvent` with timestamp, file, and AES code
- Non-fixable violations are collected and reported as manual steps
- If the check step finds CRITICAL violations, fix still proceeds for auto-fixable violations

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Fix application (50 violations) | < 3s |
| NFR-002 | Dry-run preview (50 violations) | < 2s |
| NFR-003 | Zero data loss on fix | Guaranteed (file backup + dry-run first) |

## 6. UI/UX Requirements
CLI output:
```
$ lint-arwaky-cli fix /project
🔧 Applying safe fixes to /project
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Fixed (3):
  AES003 - layer-rules/taxonomy_import_vo.rs → renamed prefix
  AES014 - cli-commands/surface_check.rs → removed #[allow(dead_code)]
  AES015 - naming-rules/capabilities_renamer.rs → removed unused import

📋 Manual steps required (2):
  AES001 - pipeline-jobs/agent_orchestrator.rs:12 - Import layer violation
  AES006 - primitive-checker/capabilities_checker.rs:33 - Primitive obsession

ℹ️  Dry-run: Use --dry-run to preview before applying
```

Dry-run output:
```
$ lint-arwaky-cli fix /project --dry-run
🔍 Dry-run: previewing fixes for /project

Would fix (3):
  AES003 - layer-rules/taxonomy_import_vo.rs → rename prefix to taxonomy_
  AES014 - cli-commands/surface_check.rs → remove #[allow(dead_code)]
  AES015 - naming-rules/capabilities_renamer.rs → remove unused import

Would NOT fix (2):
  AES001, AES006 — require manual intervention

ℹ️  Run without --dry-run to apply these 3 fixes.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File with AES003 naming violation | `fix /project` runs | File renamed to correct prefix-suffix pattern | Pending Review |
| AC-002 | File with `#[allow(dead_code)]` | `fix /project` runs | `#[allow(dead_code)]` line removed | Pending Review |
| AC-003 | File with `# noqa` bypass | `fix /project` runs | `noqa` comment removed | Pending Review |
| AC-004 | File with unused import | `fix /project` runs | Unused import line removed | Pending Review |
| AC-005 | Empty `impl Foo for Bar {}` block | `fix /project` runs | Empty impl block removed | Pending Review |
| AC-006 | `--dry-run` flag set | `fix --dry-run` runs | Files inspected, NOT modified | Pending Review |
| AC-007 | Non-fixable violations present | `fix /project` runs | Listed as manual steps | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| CLI fix command | `cli-commands/surface_fix_command.rs` | 86 | **FULLY IMPLEMENTED** — fix and fix --dry-run |
| Fix orchestrator | `code-analysis/agent_fix_orchestrator.rs` | — | **FULLY IMPLEMENTED** — bypass removal, unused import, dry-run |
| Naming renamer | `naming-rules/capabilities_renamer_processor.rs` | — | **FULLY IMPLEMENTED** — symbol rename |
| Dead inheritance fix | `role-rules/capabilities_inheritance_fixer.rs` | — | **FULLY IMPLEMENTED** — empty impl block removal |
| Fix aggregate trait | `code-analysis/contract_fix_aggregate.rs` | — | **STUB** |
| FixResult VO | `shared-common/taxonomy_fix_result_vo.rs` | — | **FULLY IMPLEMENTED** |
| FixAppliedEvent | `shared-common/taxonomy_fix_event.rs` | — | **FULLY IMPLEMENTED** |

### 8.2 Bugs Found

1. **Line-number-based removal is fragile** (`fix_bypass_comments()`, `fix_unused_import()`)
   - Fixes use line numbers from lint phase to target edits
   - If file changes between lint and fix (e.g., another process), wrong line may be edited
   - **Impact**: Incorrect file modification in concurrent scenarios
   - **Fix**: Use content-aware matching (match line content, not line number)

2. **NamingRenamerProcessor uses global search/replace** — may rename symbols in vendored/third-party code
   - **Impact**: Accidental modification of library code
   - **Fix**: Skip `vendor/`, `node_modules/`, `.git/`, `target/` directories

3. **Fix pipeline runs even if check fails** — if the check step crashes, fix still attempts to start
   - **Impact**: Operations on undefined state
   - **Fix**: Abort fix pipeline if check step produces errors (not violations, actual errors)

### 8.3 What Needs to Be Added

- **Content-aware matching**: Match by file content + line hash instead of line number
- **Third-party skip**: Add exclude patterns for vendor/third-party code
- **Check-step validation**: Abort fix pipeline if check returns errors
- **External tool fixes**: Wire `cargo clippy --fix`, `ruff check --fix` into fix pipeline

### 8.4 What to Keep

- **Dry-run mode** ✅ — preview without applying
- **Bypass comment removal** ✅ — correctly handles all three bypass formats
- **Symbol renamer** ✅ — word-boundary matching, skips strings/comments
- **Dead inheritance fixer** ✅ — correctly identifies and removes empty impl blocks

### 8.5 Empirical Evidence from Test Projects

- `lint-arwaky-cli fix test-project-rust/ --dry-run` correctly previews fixes
- `NamingRenamerProcessor` tested on test-project-rust/ — renames symbols across files
- Bypass comment removal verified on files with `#[allow(dead_code)]` and `# noqa`
- Pending Review: Content-aware matching, third-party exclusion

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-005 (Auto-Fixes) | Base auto-fix framework | Existing bugs affect new fixes | Fix line-number fragility first |
| FR-055 (Check) | Violation detection is prerequisite | Check fails = no violations to fix | Separate error handling |
| NamingRenamerProcessor | Regex-based rename | False positives in strings | Skip non-code contexts |
| File system write | Fix involves file modification | Write failures | Atomic writes, error reporting |

## 10. Appendices
- `src-rust/cli-commands/surface_fix_command.rs` — CLI fix command (86 lines)
- `src-rust/code-analysis/agent_fix_orchestrator.rs` — Fix orchestrator
- `src-rust/naming-rules/capabilities_renamer_processor.rs` — Symbol renamer
- `src-rust/role-rules/capabilities_inheritance_fixer.rs` — Dead inheritance fixer
- `src-rust/shared-common/taxonomy_fix_event.rs` — FixAppliedEvent
