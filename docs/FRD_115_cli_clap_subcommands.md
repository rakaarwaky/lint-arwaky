# 📄 Feature Requirements Document (FRD)
**Feature Name:** CLI — `clap` 4.6 Derive Subcommands
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
Defines the CLI architecture built with `clap` 4.6 using derive macros for all subcommands, providing type-safe argument parsing and auto-generated help.

### 2.2 Scope
**In-Scope:** `clap` derive macros for all subcommands (`check`, `fix`, `scan`, `watch`, `diff`, `suggest`, `install-hook`, etc.), auto-generated `--help`, argument validation, subcommand groups.
**Out-of-Scope:** Shell completion scripts, interactive mode, TUI.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **clap derive** | Rust crate for declarative CLI argument parsing via `#[derive(Parser)]` |
| **Subcommand group** | `cli::Command` enum with variant per subcommand |

## 3. Feature Overview
### 3.1 Background & Problem
Without a structured CLI framework, argument parsing was ad-hoc and error-prone. Help output was inconsistent.

### 3.2 Business Goals
- Provide consistent `--help` and error messages across all subcommands
- Use Rust's type system for compile-time argument validation
- Support future subcommands via enum extension

### 3.3 Target Users
- Developers running lint operations from terminal

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want `lint-arwaky-cli --help` to show all subcommands with descriptions.
- **US-002:** As a developer, I want `lint-arwaky-cli check --help` to show specific flags for check.

### 4.2 Use Cases & Workflow
```
lint-arwaky-cli
  ├── check <path> [--fix] [--format json|text] [--staged]
  ├── fix <path> [--dry-run]
  ├── scan <project-path>
  ├── watch <path> [--debounce-ms 500]
  ├── diff <path-a> <path-b>
  ├── suggest <path> [--model ollama|openai]
  ├── install-hook [--hook-dir <dir>]
  ├── uninstall-hook [--hook-dir <dir>]
  ├── export [--format yaml|json]
  └── import <file>
```

### 4.3 Business Rules
- All subcommands use `#[derive(Parser)]` with `clap::Subcommand`
- Global flags (e.g., `--verbose`, `--quiet`) defined on root CLI struct
- Each subcommand validates its own arguments

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | CLI startup time | < 50ms |
| NFR-002 | Help generation | < 10ms |

## 6. UI/UX Requirements
```
$ lint-arwaky-cli --help
Lint Arwaky v1.10.2 — AES Architecture Linter

Usage: lint-arwaky-cli <COMMAND>

Commands:
  check         Lint a project path
  fix           Apply safe auto-fixes
  scan         Scan external project
  watch         Watch files for auto-lint
  diff          Compare violation differences
  suggest       AI-powered fix suggestions
  install-hook  Install git pre-commit hook
  uninstall-hook Remove git pre-commit hook
  export        Export configuration
  import        Import configuration
  help          Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  Verbose output
  -q, --quiet    Suppress output
  -h, --help     Print help
  -V, --version  Print version
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Root help | `lint-arwaky-cli --help` | All subcommands listed | Pending Review |
| AC-002 | Subcommand help | `check --help` | Check-specific flags shown | Pending Review |
| AC-003 | Invalid subcommand | `lint-arwaky-cli bogus` | Error with similar command suggestion | Pending Review |
| AC-004 | Missing required arg | `check` (no path) | Error: missing required argument | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Root CLI struct | `cli-commands/surface_root_command.rs` | Pending Review |
| CLI entry point | `cli_main_entry.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-001 (CLI Foundation) | CLI entry point | Root structure must exist | Enum-based extensibility |
| `clap` 4.6 | CLI parsing crate | API changes in future | Pin version in Cargo.toml |

## 10. Appendices
- `src-rust/cli-commands/` — Feature folder
- `src-rust/cli_main_entry.rs` — Entry point
