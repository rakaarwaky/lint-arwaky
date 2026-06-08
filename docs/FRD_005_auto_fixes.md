# 📄 Feature Requirements Document (FRD)
**Feature Name:** Apply Safe Auto-Fixes (Rust + Python + JS/TS)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 08/06/2026  
**Version:** v1.0  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the auto-fix feature that automatically applies safe fixes to detected violations. It covers fix orchestration, linter adapter `apply_fix()` implementations, dry-run mode, and the naming renamer for AES003 violations.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli fix <path>` — apply safe fixes
- `lint-arwaky-cli fix <path> --dry-run` — preview fixes
- `NamingRenamerProcessor` for AES003 (naming) fixes
- `apply_fix()` on linter adapters (clippy, ruff, prettier, eslint, rustfmt)
- `FixAppliedEvent` tracking for audit trail

**Out-of-Scope:**
- Non-auto-fixable rules (AES004 size, AES005 small, AES006 primitive, AES001 import)
- Running linters without fixing

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **LintFixOrchestrator** | Agent that coordinates the fix pipeline |
| **NamingRenamerProcessor** | Capability that renames symbols project-wide |
| **FixResult** | Struct with output description and optional error |
| **FixAppliedEvent** | Event recorded after a fix is applied |
| **apply_fix()** | Method on ILinterAdapterPort to fix violations |
| **Dry-run** | Preview fixes without modifying files |

## 3. Feature Overview
### 3.1 Background & Problem
Violations were only reported, never fixed automatically. Developers had to manually fix every violation, including repetitive tasks like renaming symbols or removing bypass comments. There was no way to preview fixes before applying them, and no audit trail of what was fixed.

### 3.2 Business Goals
- Reduce manual fix effort for repetitive violations
- Provide safe, automated fixes for low-risk violations
- Enable preview before applying changes
- Maintain audit trail of all applied fixes

### 3.3 Target Users
- **Developers**: Run `fix .` to auto-fix naming, bypass, and unused import violations
- **AI Agents**: Use MCP to trigger fix pipeline autonomously

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to run `lint-arwaky-cli fix .` to auto-fix naming convention violations, so I don't have to rename files manually.
- **US-002:** As a developer, I want to run `lint-arwaky-cli fix . --dry-run` to see what would be changed, so I can review before applying.
- **US-003:** As a developer, I want non-fixable violations to be listed as manual steps, so I know what still needs attention.

### 4.2 Use Cases & Workflow
**Fix Pipeline (Implemented):**
```
lint-arwaky-cli fix .
  │
  ├─► 1. Self-lint → collect violations
  │
  ├─► 2. Classify violations:
  │     ├── Auto-fixable: AES003 (naming), AES014 (bypass), AES015 (unused)
  │     └── Manual: AES001 (import), AES004 (size), AES006 (primitive)
  │
  ├─► 3. Apply fixes:
  │     ├── AES003 → NamingRenamerProcessor.rename_symbol()
  │     ├── AES014 → adapter.apply_fix() (remove #[allow], noqa)
  │     └── AES015 → adapter.apply_fix() (remove unused imports)
  │
  ├─► 4. Record FixAppliedEvent for audit
  │
  └─► 5. Report:
        "3 violations fixed automatically"
        "5 violations require manual fix"
```

**Naming Renamer Workflow:**
```
NamingRenamerProcessor.rename_symbol(root, old_name, new_name)
  │
  ├─► Walk all files in project
  ├─► For each file:
  │     ├── Skip: comments, string literals, template literals
  │     └── Replace old_name → new_name (word boundary regex)
  └─► Return count of modified files
```

**Adapter apply_fix() (Real implementations):**
| Adapter | Command | Scope |
|---------|---------|-------|
| clippy | `cargo clippy --fix --allow-dirty` | Rust |
| ruff | `ruff check <path> --fix` | Python |
| prettier | `prettier --write <path>` | JS/TS formatting |
| eslint | `eslint <path> --fix` | JS/TS lint |
| rustfmt | `cargo fmt` | Rust formatting |

### 4.3 Business Rules
- Only safe, automated fixes are applied without human review
- Dry-run mode: analyze but do NOT modify files
- Non-auto-fixable violations reported as manual steps
- Each fix generates a `FixAppliedEvent` with timestamp

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Fix application (50 files) | < 3s |
| NFR-002 | Dry-run preview (50 files) | < 2s |
| NFR-003 | Zero data loss on fix | Guaranteed (dry-run first) |

## 6. UI/UX Requirements
CLI output:
```
 Applying safe fixes to /project...
 ✅ architecture_import_checker.rs → renamed symbol (AES003)
 ✅ my_file.rs → removed unused import (AES015)
 ⚠️  architecture_lint_handler.rs → requires manual fix (AES001 - import violation)
 ℹ️  Dry-run mode: 2 of 5 violations can be auto-fixed
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File with AES003 naming violation | `fix .` runs | File renamed to correct 3-word pattern | ✅ `LintFixOrchestrator` calls `SymbolRenamerProcessor` for AES003 fixes |
| AC-002 | File with `#[allow(...)]` bypass | `fix .` runs | `#[allow(...)]` line removed | ✅ `fix_bypass_comments()` implemented with dry-run support |
| AC-003 | File with unused import | `fix .` runs | Unused import line removed | ✅ `fix_unused_import()` removes import line by line number |
| AC-004 | `--dry-run` flag set | `fix . --dry-run` runs | Files inspected but NOT modified | ✅ `--dry-run` flag defined in CLI, wired to `LintFixOrchestrator::with_dry_run()` |
| AC-005 | Non-fixable violations present | `fix .` runs | Listed as manual steps | ✅ `report_non_fixable()` classifies by AES code, lists manual steps |
| AC-006 | Naming violation with AES003 | `NamingRenamerProcessor` runs | Symbol renamed project-wide | ✅ Working |
| AC-007 | Clippy fixable issue in Rust file | `rust_linter_adapter.apply_fix()` | `cargo clippy --fix` executes | ✅ Real |

## 8. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Parser identifies violations to fix | Regex inaccuracy affects fix targeting | Use external tool's own fix mechanism |
| External tools | clippy, ruff, eslint, prettier must be installed | Tool not found → fix fails | Check tool availability before fix |
| `NamingRenamerProcessor` | Regex-based rename | False positives in strings/comments | Skip non-code contexts |

## 9. Appendices
- `src-rust/surfaces/cli_fix_command.rs` — CLI command (stub)
- `src-rust/agent/lint_fix_orchestrator.rs` — Orchestrator (stub)
- `src-rust/capabilities/naming_renamer_processor.rs` — Naming renamer (working)
- `src-rust/taxonomy/fix_result_vo.rs` — FixResult VO
- `src-rust/taxonomy/fix_applied_event.rs` — FixAppliedEvent
- `src-rust/contract/lint_fix_aggregate.rs` — Fix orchestrator trait
- `src-rust/contract/linter_adapter_port.rs` — `apply_fix()` trait method
