# Crate Index

> **Complete reference for every crate in the workspace.**
> Each section explains **what** the crate does, **why** it exists, and **how** to work with it.

---

## Table of Contents

1. [shared/](#1-shared) — Foundation
2. [import-rules/](#2-import-rules) — Import Compliance
3. [naming-rules/](#3-naming-rules) — Naming Conventions
4. [code-analysis/](#4-code-analysis) — Code Quality
5. [role-rules/](#5-role-rules) — Layer-Role Violations
6. [orphan-detector/](#6-orphan-detector) — Dead Code Detection
7. [auto-fix/](#7-auto-fix) — Automatic Fixes
8. [config-system/](#8-config-system) — Configuration
9. [external-lint/](#9-external-lint) — External Linters
10. [cli-commands/](#10-cli-commands) — CLI Surface
11. [mcp-server/](#11-mcp-server) — MCP Server
12. [tui/](#12-tui) — Terminal UI
13. [file-watch/](#13-file-watch) — File Watching
14. [git-hooks/](#14-git-hooks) — Git Integration
15. [project-setup/](#15-project-setup) — Project Init
16. [maintenance/](#16-maintenance) — Maintenance Operations

---

## 1. shared/

**Package:** `shared-lint-arwaky`
**Source files:** 168
**Dependencies:** None (foundation crate)

### What it does

Contains **all shared types** used across the entire workspace:
- **Taxonomy layer:** Value Objects (VOs), Entities, Events, Errors, Constants
- **Contract layer:** Port traits, Protocol traits, Aggregate traits

### Why it exists

Every other crate depends on shared types. Without a shared crate, you'd have circular dependencies. The `shared` crate is the **single source of truth** for domain models and interfaces.

### Key files

| File | Purpose |
|------|---------|
| `taxonomy_file_path_vo.rs` | `FilePath` VO — validated, normalized file paths |
| `taxonomy_severity_vo.rs` | `Severity` enum — Error, Warning, Info |
| `taxonomy_language_vo.rs` | `Language` enum — Rust, Python, JavaScript |
| `taxonomy_layer_vo.rs` | `Layer` enum — the 7 architectural layers |
| `taxonomy_lint_result_vo.rs` | `LintResult` — violation record |
| `contract_import_runner_aggregate.rs` | `IImportRunnerAggregate` trait |
| `contract_code_analysis_aggregate.rs` | `ICodeAnalysisAggregate` trait |
| `contract_naming_runner_aggregate.rs` | `INamingRunnerAggregate` trait |

### How to add new shared types

1. Create `taxonomy_{name}_vo.rs` in `shared/src/common/` (or feature subfolder)
2. Add `pub mod` in the parent `mod.rs`
3. The type is now available to all crates via `use shared_lint_arwaky::common::taxonomy_{name}_vo::*`

---

## 2. import-rules/

**Package:** `import_rules-lint-arwaky`
**Source files:** 11
**AES rules:** AES201–AES205

### What it does

Enforces import compliance across Rust, Python, and JavaScript/TypeScript:
- **AES201:** Forbidden imports (e.g., importing from wrong layer)
- **AES202:** Mandatory imports (e.g., every file must import its VO)
- **AES203:** Unused imports
- **AES204:** Dummy/meaningless imports
- **AES205:** Circular dependency detection

### Why it exists

Import boundaries are the **first line of defense** against architectural violations. If a `capabilities_*` file imports from another `capabilities_*` file in a different crate, the architecture is broken.

### File structure

```
import-rules/src/
├── taxonomy_import_rule_vo.rs              ← Rule configuration VO
├── contract_import_runner_aggregate.rs     ← IImportRunnerAggregate trait
├── contract_import_protocol.rs             ← Checker protocol traits
├── capabilities_import_mandatory_checker.rs ← AES202 implementation
├── capabilities_import_forbidden_checker.rs ← AES201 implementation
├── capabilities_import_dummy_checker.rs    ← AES204 implementation
├── capabilities_unused_import_checker.rs   ← AES203 implementation
├── infrastructure_import_cycle_analyzer.rs ← AES205 implementation
├── infrastructure_import_parser_adapter.rs ← Source file parser
├── agent_import_orchestrator.rs            ← Wires checkers together
└── root_import_rules_container.rs          ← DI container
```

### How it works

1. `ImportOrchestrator` receives `FilePath` target
2. Walks directory, collects `.rs/.py/.js/.ts` files
3. For each file: runs all 5 checkers sequentially
4. Aggregates `LintResult` violations
5. Returns to caller (CLI, MCP, or TUI)

---

## 3. naming-rules/

**Package:** `naming_rules-lint-arwaky`
**Source files:** 6
**AES rules:** AES101–AES102

### What it does

Enforces naming conventions:
- **AES101:** File suffix convention (e.g., `*_vo.rs` for value objects)
- **AES102:** Filename pattern matching (e.g., `snake_case` for Rust)

### Why it exists

Naming is **documentation**. When every file follows `{layer}_{feature}_{type}.rs`, developers can instantly understand a file's role from its name.

### File structure

```
naming-rules/src/
├── taxonomy_naming_rule_vo.rs              ← Rule configuration
├── contract_naming_runner_aggregate.rs     ← INamingRunnerAggregate trait
├── contract_naming_protocol.rs             ← Checker protocol traits
├── capabilities_naming_convention_checker.rs ← AES102 implementation
├── capabilities_suffix_prefix_checker.rs   ← AES101 implementation
├── agent_naming_orchestrator.rs            ← Wires checkers
└── root_naming_rules_container.rs          ← DI container
```

---

## 4. code-analysis/

**Package:** `code_analysis-lint-arwaky`
**Source files:** 7
**AES rules:** AES301–AES305

### What it does

Checks code quality metrics:
- **AES301:** File line limits (max 500 lines)
- **AES302:** Function line limits (max 50 lines)
- **AES303:** Bypass suppression detection (`noqa`, `type: ignore`, `#[allow(...)]`)
- **AES305:** Mandatory definitions (every file must define something)
- **Todo/unimplemented detection:** Flags `TODO!()`, `unimplemented!()`, `todo!()`

### Why it exists

Large files and functions are **hard to maintain**. Bypass suppressions **hide real problems**. This crate catches both.

### File structure

```
code-analysis/src/
├── contract_code_analysis_aggregate.rs     ← ICodeAnalysisAggregate trait
├── contract_line_protocol.rs               ← Line counting protocol
├── contract_bypass_checker_protocol.rs     ← Bypass detection protocol
├── capabilities_file_line_checker.rs       ← AES301 implementation
├── capabilities_bypass_checker.rs          ← AES304 implementation
├── capabilities_mandatory_def_checker.rs   ← AES305 implementation
├── agent_code_analysis_orchestrator.rs     ← Wires checkers
└── root_code_analysis_container.rs         ← DI container
```

---

## 5. role-rules/

**Package:** `role_rules-lint-arwaky`
**Source files:** 9
**AES rules:** AES401–AES406

### What it does

Enforces layer-role boundaries:
- **AES401:** Layer-role suffix mismatch (e.g., `taxonomy_*` file can't define a struct named `Checker`)
- **AES402:** Contract uses primitive types (must use VOs)
- **AES403:** Capability complexity (too many dependencies)
- **AES404:** Agent line limits (orchestrators should be thin)
- **AES405:** Surface function limits (commands should be short)
- **AES406:** Infrastructure imports capabilities (wrong direction)

### Why it exists

Each layer has a **specific job**. When code crosses layer boundaries, the architecture degrades. This crate catches violations at the code level.

### File structure

```
role-rules/src/
├── taxonomy_role_rule_vo.rs                ← Rule configuration
├── contract_role_runner_aggregate.rs       ← IRoleRunnerAggregate trait
├── contract_role_protocol.rs               ← Per-layer protocol traits
├── capabilities_taxonomy_role_checker.rs   ← Taxonomy layer auditor
├── capabilities_contract_role_checker.rs   ← Contract layer auditor
├── capabilities_capabilities_role_checker.rs ← Capabilities layer auditor
├── capabilities_infrastructure_role_checker.rs ← Infrastructure layer auditor
├── capabilities_surface_role_checker.rs    ← Surface layer auditor
├── capabilities_agent_role_checker.rs      ← Agent layer auditor
├── agent_role_orchestrator.rs              ← Wires all checkers
└── root_role_rules_container.rs            ← DI container
```

---

## 6. orphan-detector/

**Package:** `orphan_detector-lint-arwaky`
**Source files:** 11
**AES rules:** AES501–AES506

### What it does

Detects **dead/unreachable code**:
- **AES501:** Unused taxonomy files (VOs never imported)
- **AES502:** Unused contracts (traits never implemented)
- **AES503:** Unused capabilities (checkers never called)
- **AES504:** Unused infrastructure (adapters never used)
- **AES505:** Unused agents (orchestrators never instantiated)
- **AES506:** Unused surfaces (commands never dispatched)

### Why it exists

Dead code **confuses developers** and **bloats the codebase**. Orphan detection finds code that exists but is never used.

### File structure

```
orphan-detector/src/
├── taxonomy_orphan_vo.rs                   ← Orphan result VO
├── contract_orphan_aggregate.rs            ← IOrphanAggregate trait
├── contract_graph_resolver_protocol.rs     ← Import graph resolver
├── capabilities_taxonomy_orphan_analyzer.rs ← AES501 implementation
├── capabilities_contract_orphan_analyzer.rs ← AES502 implementation
├── capabilities_capabilities_orphan_analyzer.rs ← AES503 implementation
├── capabilities_infrastructure_orphan_analyzer.rs ← AES504 implementation
├── capabilities_agent_orphan_analyzer.rs   ← AES505 implementation
├── capabilities_surface_orphan_analyzer.rs ← AES506 implementation
├── infrastructure_graph_resolver.rs        ← Builds import dependency graph
├── agent_orphan_orchestrator.rs            ← Wires analyzers
└── root_orphan_detector_container.rs       ← DI container
```

---

## 7. auto-fix/

**Package:** `auto_fix-lint-arwaky`
**Source files:** 4

### What it does

Applies **safe automatic fixes** for certain violations:
- Removes unused imports (AES203)
- Removes forbidden imports (AES201)
- Removes dummy imports (AES204)

### Why it exists

Manual fixes for trivial violations waste developer time. Auto-fix handles the safe ones automatically.

### Safety rules

Only **removal** operations are automated. No code is **added** or **modified** — only deleted lines.

---

## 8. config-system/

**Package:** `config_system-lint-arwaky`
**Source files:** 8

### What it does

Loads and parses configuration:
- Reads `lint_arwaky.config.{rust,python,javascript}.yaml`
- Detects workspace root (walks up directory tree)
- Discovers workspace members (Cargo.toml, pyproject.toml, package.json)
- Validates configuration against expected schema

### Why it exists

Every project has different rules and thresholds. The config system allows **per-project customization** without code changes.

### Config file location

```
project-root/
├── lint_arwaky.config.rust.yaml        ← Rust rules
├── lint_arwaky.config.python.yaml      ← Python rules
└── lint_arwaky.config.javascript.yaml  ← JS/TS rules
```

---

## 9. external-lint/

**Package:** `external_lint-lint-arwaky`
**Source files:** 15

### What it does

Wraps **external linter tools**:
- **Rust:** Clippy, rustfmt, Cargo Audit
- **Python:** Ruff, MyPy, Bandit
- **JavaScript:** ESLint, Prettier, TypeScript Compiler

### Why it exists

External tools are **best-in-class** for their languages. Lint Arwaky **delegates** to them rather than reimplementing their checks.

### Adapter pattern

Each adapter:
1. Executes the external tool as a subprocess
2. Parses the tool's output (stdout/stderr)
3. Converts to `LintResult` format
4. Returns unified results

```rust
// Example: Clippy adapter
impl IExternalLintProtocol for ClippyAdapter {
    fn run(&self, target: &str) -> Vec<LintResult> {
        let output = Command::new("cargo")
            .args(["clippy", "--message-format=json"])
            .output()?;
        parse_clippy_json(output.stdout)
    }
}
```

---

## 10. cli-commands/

**Package:** `cli_commands-lint-arwaky`
**Source files:** 12

### What it does

Implements all CLI subcommands:
- `check` — Single project lint
- `scan` — Multi-project lint
- `fix` — Auto-fix violations
- `ci` — CI-optimized lint with exit codes
- `orphan` — Dead code detection
- `security` — Dependency audit
- `duplicates` — Duplicate code detection
- `dependencies` — Dependency analysis
- `watch` — Real-time file watching
- `install-hook` / `uninstall-hook` — Git hooks
- `doctor` — Environment diagnostics
- `version` — Version info

### Why it exists

Each command is a **surface** that dispatches to the appropriate orchestrator. The CLI is the primary developer interface.

### File structure

```
cli-commands/src/
├── surface_check_command.rs    ← check/scan/ci handlers
├── surface_fix_command.rs      ← fix handler
├── surface_orphan_command.rs   ← orphan handler
├── surface_security_command.rs ← security handler
├── surface_watch_command.rs    ← watch handler
├── surface_setup_command.rs    ← init/install/mcp-config
├── surface_config_command.rs   ← config-show
└── surface_adapter_command.rs  ← adapters list
```

---

## 11. mcp-server/

**Package:** `mcp_server-lint-arwaky`
**Source files:** 6

### What it does

Implements a **Model Context Protocol (MCP) server** for AI agents:
- JSON-RPC 2.0 over stdio
- 5 tools: `execute_command`, `list_commands`, `command_schema`, `read_skill`, `health_check`
- Async via tokio

### Why it exists

AI agents (Claude, GPT, etc.) need a **standardized interface** to interact with Lint Arwaky. MCP provides this.

### MCP tools

| Tool | Purpose |
|------|---------|
| `execute_command` | Run a lint command (check, scan, fix, ci, etc.) |
| `list_commands` | List available commands and their parameters |
| `command_schema` | Get JSON schema for a command's parameters |
| `read_skill` | Read SKILL.md content (AI agent context) |
| `health_check` | Verify server is running |

---

## 12. tui/

**Package:** `tui-lint-arwaky`
**Source files:** 14

### What it does

Terminal-based file browser with **real-time lint results**:
- 3-panel layout (Ranger-style)
- Keyboard navigation
- File content preview
- Lint result display

### Why it exists

Some developers prefer **keyboard-driven interfaces**. The TUI provides a visual way to browse and lint files without leaving the terminal.

### Key bindings

| Key | Action |
|-----|--------|
| `↑/↓` | Navigate files |
| `Enter` | Open file / expand directory |
| `q` | Quit |
| `r` | Refresh |
| `f` | Toggle file filter |
| `l` | Show/hide lint results |

---

## 13. file-watch/

**Package:** `file_watch-lint-arwaky`
**Source files:** 5

### What it does

Real-time file system monitoring:
- Uses `inotify` on Linux (via `notify` crate)
- Debounces rapid changes (500ms)
- Filters to source files only
- Triggers lint pipeline on change

### Why it exists

Developers want **instant feedback** when they save a file. File watching provides this without manual re-runs.

---

## 14. git-hooks/

**Package:** `git_hooks-lint-arwaky`
**Source files:** 6

### What it does

Manages Git pre-commit hooks:
- `install-hook` — Creates `.git/hooks/pre-commit`
- `uninstall-hook` — Removes the hook
- Hook runs `lint-arwaky-cli check` on staged files

### Why it exists

Git hooks **enforce quality at commit time**. Developers can't commit code that violates AES rules.

---

## 15. project-setup/

**Package:** `project_setup-lint-arwaky`
**Source files:** 5

### What it does

Project initialization and diagnostics:
- `init` — Create config files for a new project
- `doctor` — Check environment (Rust, Python, Node.js versions)
- `mcp-config` — Generate MCP config for Claude Desktop

### Why it exists

New projects need **quick setup**. The setup crate automates config generation and environment validation.

---

## 16. maintenance/

**Package:** `maintenance-lint-arwaky`
**Source files:** 4

### What it does

Ongoing maintenance operations:
- Environment diagnostics
- Security vulnerability scans
- Dependency analysis and updates

### Why it exists

Codebases need **regular maintenance**. This crate provides tools for ongoing health checks.

---

## Crate Dependency Graph

```
                    ┌─────────────┐
                    │    shared    │
                    │  (168 files) │
                    └──────┬──────┘
                           │
          ┌────────────────┼────────────────┐
          │                │                │
          ▼                ▼                ▼
   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
   │ import-rules│ │ naming-rules│ │code-analysis│
   └──────┬──────┘ └──────┬──────┘ └──────┬──────┘
          │                │                │
          ▼                ▼                ▼
   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
   │  role-rules │ │   orphan-   │ │  auto-fix   │
   └──────┬──────┘ │  detector   │ └──────┬──────┘
          │        └──────┬──────┘        │
          │               │               │
          ▼               ▼               ▼
   ┌─────────────────────────────────────────────┐
   │              cli-commands                    │
   │              mcp-server                      │
   │              tui                             │
   └─────────────────────────────────────────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │ root_*_entry │
                    │  (3 binaries)│
                    └─────────────┘
```

---

## Quick Reference: Which Crate for What?

| I want to... | Edit crate |
|--------------|-----------|
| Add a new value type | `shared/` |
| Add a new trait interface | `shared/` |
| Add a new import rule | `import-rules/` |
| Add a new naming rule | `naming-rules/` |
| Add a new quality check | `code-analysis/` |
| Add a new role check | `role-rules/` |
| Add a new orphan check | `orphan-detector/` |
| Add a new auto-fix | `auto-fix/` |
| Add a new config option | `config-system/` |
| Add a new external tool | `external-lint/` |
| Add a new CLI command | `cli-commands/` |
| Add a new MCP tool | `mcp-server/` |
| Add a new TUI view | `tui/` |
| Modify file watching | `file-watch/` |
| Modify git hooks | `git-hooks/` |
| Modify project setup | `project-setup/` |

---

## Further Reading

| Topic | Document |
|-------|----------|
| Navigation hub (start here) | [DOCS.md](DOCS.md) |
| Developer guide (patterns, how-to) | [DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md) |
| End-to-end data flow | [DATA_FLOW.md](DATA_FLOW.md) |
| AES 7-layer architecture | [ARCHITECTURE.md](../ARCHITECTURE.md) |
| 24 AES rules catalog | [rules/RULES_AES.md](rules/RULES_AES.md) |
