# 📄 Feature Requirements Document (FRD)
**Feature Name:** File Watcher — `watch` Subcommand
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
Defines the `watch` subcommand that monitors the filesystem for changes and auto-runs lint on save, providing real-time feedback.

### 2.2 Scope
**In-Scope:** `lint-arwaky-cli watch <path>`, filesystem event monitoring via `notify` crate, debounced re-lint on file change, live violation display.
**Out-of-Scope:** In-editor integration, remote file watching, network filesystem support.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Debounce** | Delay re-lint until changes settle (500ms) |
| **notify** | Rust crate for filesystem event monitoring |

## 3. Feature Overview
### 3.1 Background & Problem
Developers had to manually re-run `check .` after every file change. No continuous monitoring existed.

### 3.2 Business Goals
- Provide real-time lint feedback on file save
- Reduce manual command re-runs
- Support configurable watch paths and debounce intervals

### 3.3 Target Users
- Developers wanting instant feedback while coding

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want `watch .` to auto-lint when I save a file, so I see violations immediately.

### 4.2 Use Cases & Workflow
```
lint-arwaky-cli watch . --debounce-ms 500
  │
  ├─► Start filesystem watcher on path
  ├─► On file save event:
  │     ├─► Debounce (wait 500ms for more changes)
  │     ├─► Run lint on changed file(s)
  │     └─► Display violations
  └─► Repeat until Ctrl+C
```

### 4.3 Business Rules
- Default debounce: 500ms
- Only `.rs`, `.py`, `.ts`, `.tsx`, `.js`, `.jsx` files trigger re-lint
- Initial full scan on start, then incremental scans

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Event → lint latency | < 1s (after debounce) |
| NFR-002 | Memory with 10k files | < 100MB |
| NFR-003 | CPU idle (no changes) | < 1% |

## 6. UI/UX Requirements
```
 Watching /project for changes (Ctrl+C to stop)...
 [14:32:01] File changed: src/main.rs
 Running lint on 3 changed files...
 ┌─────────────────────────────────────────────┐
 │ AES003 naming violation in main.rs:12       │
 │       Expected "taxonomy_foo_vo" pattern     │
 └─────────────────────────────────────────────┘
 [14:32:02] Lint complete — 1 violation found
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Watch started on directory | `watch .` | Initial full scan runs | Pending Review |
| AC-002 | File saved with violation | Edit and save a file | Violation displayed within 1.5s | Pending Review |
| AC-003 | Non-watched file changed | Edit a `.md` file | No re-lint triggered | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Watch CLI command | `cli-commands/surface_watch_command.rs` | Pending Review |
| File watcher service | `file-watch/infrastructure_watcher_provider.rs` | Pending Review |
| Watcher orchestrator | `file-watch/agent_watch_orchestrator.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Lint Pipeline) | Re-lint on file change | Frequent changes cause cascading lints | Debounce mechanism |
| `notify` crate | Filesystem event library | Platform-specific event behavior | Test on Linux, macOS, Windows |

## 10. Appendices
- `src-rust/file-watch/` — Feature folder
- `docs/FRD_055_lint_pipeline.md`
