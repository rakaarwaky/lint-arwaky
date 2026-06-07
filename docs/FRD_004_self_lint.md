# FRD — Self-Lint Target (`lint-arwaky-cli check .`)

> **PRD Reference**: [FR-004](PRD.md) — Self-lint target — project audits itself
> **Dependency**: FR-001 (6-layer AES), FR-002 (Config), FR-003 (Source parsing)
> **Status**: ✅ **PRODUCTION-READY** — 31 rules real, 153+ violations detected. NOTE: ~500 lines duplicated between coordinator and compliance_analyzer; 170+ `unwrap()` causes AES014 self-violations.
> **Self-lint**: `lint-arwaky-cli check .` — project audits `src-rust/` against all AES rules

## 1. Problem Statement

Before self-lint:

| Issue | Description |
|-------|-------------|
| **No dogfooding** | Project cannot check its own architecture compliance |
| **No CI gate** | PRs can be merged with architecture violations |
| **Manual review** | AES violations only detected during code review |
| **No score** | No quantitative measure of codebase health |
| **No reporting** | Violations are not reported in a structured way |

## 2. Core Concept

Self-lint = run all 31 AES rules against `src-rust/` (the project's own code). The result:
1. **Score**: Starts at 100, deducted per violation
2. **Violations**: Complete list with file, line, rule, severity
3. **CRITICAL auto-fail**: If there is a CRITICAL → exit non-zero

```
The project lints itself — dogfooding.
If there is a bug in rule AES001, self-lint will detect it itself.
```

## 3. Working Mechanism — Step by Step

### 3.1 Entry Point

```
User: lint-arwaky-cli check .
    │
    ▼
cli_main_entry.rs: main()
    │
    ├─► Clap parse args → Commands::Check { path: Some("."), git_diff: false }
    │
    ├─► handle_check(".", false)
    │     │
    │     ├─► lint_path(".")
    │     │
    │     └─► Print results:
    │           ├─► "Score: 87.5 / 100"
    │           ├─► "CRITICAL: 0 | HIGH: 3 | MEDIUM: 5 | LOW: 2"
    │           └─► List of violations per file
    │
    └─► ExitCode::Success (0) or Failure (1)
```

### 3.2 Lint Pipeline

```
lint_path(project_root)
    │
    ├─► Step 1: Find source directory
    │     ArchLintHandler.find_source_dir(project_root)
    │     ├─► Check: src-rust/ → Rust ✅
    │     ├─► Check: src-python/ → Python
    │     ├─► Check: src-javascript/ → JavaScript
    │     └─► Check: src/ (generic)
    │
    ├─► Step 2: Load config
    │     ConfigLoaderOrchestrator.load_project_config(project_root)
    │     ├─► detect_language(".") → Rust
    │     ├─► read_config(".", "rust") → lint_arwaky.config.rust.yaml
    │     └─► parse → ArchitectureConfig
    │
    ├─► Step 3: Run all checks
    │     LintCheckingCoordinator.run_all_checks(ArchitectureConfig, source_dir)
    │     │
    │     │  ╔══════════════════════════════════════╗
    │     │  ║  3a. File discovery                  ║
    │     │  ╚══════════════════════════════════════╝
    │     │  Walk source_dir → collect all *.rs / *.py / *.js *.ts
    │     │
    │     │  ╔══════════════════════════════════════╗
    │     │  ║  3b. Per-file checks (22 rules)      ║
    │     │  ╚══════════════════════════════════════╝
    │     │  For EACH file:
    │     │  ├── Layer detection
    │     │  │     detect_layer() → find which layer the file is in
    │     │  │
    │     │  ├── AES003: Naming convention
    │     │  │     Regex: ^word_word_word\.rs$ ?
    │     │  │     → "architecture_compliance_analyzer.rs" ✅
    │     │  │     → "my_file.rs" ❌ (only 2 words)
    │     │  │
    │     │  ├── AES004: File too large (>500 lines)
    │     │  │     get_line_count() > 500 ? → FLAG
    │     │  │
    │     │  ├── AES005: File too short (<10 lines)
    │     │  │     get_line_count() < 10 ? → FLAG
    │     │  │
    │     │  ├── AES006: Primitive usage
    │     │  │     find_primitive_violations() → look for String/i32 in domain
    │     │  │
    │     │  ├── AES008: Contract suffix
    │     │  │     Check: if in contract/ → suffix _port/_protocol/_aggregate?
    │     │  │
    │     │  ├── AES009: Mandatory struct/trait
    │     │  │     get_raw_symbols() → is there a struct/trait/enum?
    │     │  │
    │     │  ├── AES011: Suffix mismatch
    │     │  │     Check: is file suffix in layer's allowed list?
    │     │  │
    │     │  ├── AES014: Bypass comment
    │     │  │     find_bypass_comments() → #[allow, unwrap(), panic!
    │     │  │
    │     │  ├── AES015: Unused import
    │     │  │     find_unused_imports() → symbol exists but not used
    │     │  │
    │     │  ├── AES016: Dead inheritance
    │     │  │     Empty Struct {} or trait {}? → FLAG
    │     │  │
    │     │  ├── AES021: Agent role
    │     │  │     If _container → only wiring?
    │     │  │     If _orchestrator → stateless?
    │     │  │
    │     │  ├── AES022: Surface role
    │     │  │     Smart surface → must delegate via container
    │     │  │     Passive surface → only taxonomy import
    │     │  │
    │     │  ├── AES023: Surface direct import
    │     │  │     extract_imports() → is there "use crate::infrastructure::" ?
    │     │  │
    │     │  ├── AES024: Agent any-bypass
    │     │  │     Check: is there `dyn Any` or `Box<Any>` in agent?
    │     │  │
    │     │  ├── AES025: MCP schema
    │     │  │     MCP files → is there docstring + JSON Schema?
    │     │  │
    │     │  ├── AES026: Forbidden inheritance
    │     │  │     Contract Aggregate must not `impl PortTrait for ...`
    │     │  │
    │     │  ├── AES027: Mandatory inheritance
    │     │  │     Every file → implements contract trait?
    │     │  │
    │     │  ├── AES030: Capability method exists
    │     │  │     Dispatch catalog → method exists in class?
    │     │  │
    │     │  ├── AES031: Single bottleneck
    │     │  │     All dispatch to 1 class? → FLAG
    │     │  │
    │     │  ├── AES032: Missing VO
    │     │  │     Capability call → is there a VO parameter?
    │     │  │
    │     │  └── AES033: Constant purity
    │     │        File _constant → only pub const/pub static?
    │     │
    │     │  ╔══════════════════════════════════════╗
    │     │  ║  3c. Cross-file checks (9 rules)     ║
    │     │  ╚══════════════════════════════════════╝
    │     │  ├── AES001: Import layer violation
    │     │  │     Match each import against per-layer import rules
    │     │  │
    │     │  ├── AES002: Mandatory import missing
    │     │  │     Layer must import taxonomy? → check extract_imports
    │     │  │
    │     │  ├── AES007: Layer import alias
    │     │  │     Contract imports must go through barrel (mod.rs)
    │     │  │
    │     │  ├── AES010: Root layer
    │     │  │     Root files → only entry point?
    │     │  │
    │     │  ├── AES012: Barrel completeness
    │     │  │     mod.rs → exports all files in the layer?
    │     │  │
    │     │  ├── AES013: Internal all forbidden
    │     │  │     Non-mod.rs → pub mod / pub use? → FLAG
    │     │  │
    │     │  ├── AES018: Surface hierarchy
    │     │  │     Utility surface imports Smart surface? → FLAG
    │     │  │
    │     │  ├── AES019: Passive surface violation
    │     │  │     Passive surface imports agent/contract? → FLAG
    │     │  │
    │     │  └── AES020: Circular dependency
    │     │        Graph analysis → is there a cycle in imports?
    │     │
    │     └── 3d. Collect all violations
    │
    ├─► Step 4: Compute score
    │     ArchitectureGovernanceEntity
    │     ├─► Score = 100.0
    │     ├─► For each violation:
    │     │     LOW     → score -= 1
    │     │     MEDIUM  → score -= 2
    │     │     HIGH    → score -= 3
    │     │     CRITICAL → score -= 5
    │     ├─► score = max(0, score)
    │     ├─► If there is CRITICAL → is_passing = false
    │     └─► If score < threshold (ci mode) → exit non-zero
    │
    └─► Step 5: Print report
          ├─► Format: text (default), JSON (--format json), SARIF, JUnit
          ├─► Group by severity
          └─► Display per file with line number
```

### 3.3 CI Mode

```
lint-arwaky-cli ci . --threshold 80
    │
    └─► Same as check, but:
          ├─► If score < 80 → exit code 1 (CI FAIL)
          ├─► Output JSON (machine-readable)
          └─► Exit code:
                0 = PASS (score >= threshold, no CRITICAL)
                1 = FAIL (score < threshold OR has CRITICAL)
```

### 3.4 Git Diff Mode

```
lint-arwaky-cli check . --git-diff
    │
    └─► Only check files changed in git working tree
          ├─► git diff --name-only → list of modified files
          └─► Run checks only for those files
```

## 4. Key Files

### Surface
| File | Function |
|------|----------|
| `cli_core_command.rs` | Define all CLI subcommands via Clap |
| `cli_check_command.rs` | `CheckCommandsSurface` — entry point check/scan |
| `cli_main_entry.rs` | `handle_check()`, `handle_scan()`, `handle_ci()`, `lint_path()` |

### Agent
| File | Function |
|------|----------|
| `architecture_lint_orchestrator.rs` | `run_self_lint(project_root)` — find source + load config + run checks |
| `lint_checking_coordinator.rs` | `run_all_checks(config, src)` — orchestrate 31 AES rules |
| `dependency_injection_container.rs` | Wire all dependencies |

### Capabilities (10 checkers)
| File | AES Rules |
|------|-----------|
| `architecture_compliance_analyzer.rs` | Layer detection |
| `architecture_import_checker.rs` | AES001, AES002, AES023 |
| `architecture_naming_checker.rs` | AES003, AES008, AES011 |
| `architecture_internal_checker.rs` | AES012, AES013 |
| `architecture_metric_checker.rs` | AES004, AES005, AES006 |
| `architecture_cycle_analyzer.rs` | AES020 |
| `architecture_orphan_analyzer.rs` | AES017 |
| `architecture_inheritance_checker.rs` | AES026, AES027 |
| `surface_hierarchy_checker.rs` | AES018, AES019, AES022 |
| `architecture_lint_handler.rs` | `ArchLintHandler` — implement `IArchLintProtocol` |

## 5. Score Computation Detail

```
Raw score = 100.0

Violation: AES001 | HIGH | infrastructure/foo.rs:42
  Score -= 3 → 97.0

Violation: AES014 | CRITICAL | capabilities/bar.rs:15
  Score -= 5 → 92.0
  is_passing = false ← CRITICAL auto-fail!

Violation: AES003 | LOW | agent/my_file.rs
  Score -= 1 → 91.0

Final score = max(0, 91.0) = 91.0
Status: FAIL (CRITICAL found)
```

## 6. Report Formats

| Format | Output | Use Case |
|--------|--------|----------|
| Text | Human readable table | Local dev |
| JSON | `{"score": 91.0, "violations": [...]}` | Machine parsing |
| SARIF | SARIF 2.1.0 JSON | GitHub Code Scanning |
| JUnit | JUnit XML | Jenkins/CI pipeline |

## 7. Acceptance Criteria

| # | Criteria | Status |
|---|----------|--------|
| AC001 | `lint-arwaky-cli check .` runs without error | ✅ |
| AC002 | Detects 153+ violations in own codebase | ✅ |
| AC003 | 31 AES codes (AES001–AES033, 028/029 reserved) | ✅ |
| AC004 | Score: start 100, deduct per severity, CRITICAL = fail | ✅ |
| AC005 | `scan` command = AES + external adapters (clippy, ruff, eslint) | ✅ |
| AC006 | `ci` mode with threshold + exit code | ✅ |
| AC007 | Report formats: text, JSON, SARIF, JUnit | ✅ |
| AC008 | `git-diff` — only check changed files | ✅ |
| AC009 | `cargo check --bin lint-arwaky-cli` passes | ✅ |
| AC010 | `cargo test` — all tests pass | ✅ |
