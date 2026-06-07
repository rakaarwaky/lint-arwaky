# FRD — Self-Lint Target (`lint-arwaky-cli check .`)

> **PRD Reference**: [FR-004](PRD.md) — Self-lint target — project audits itself
> **Dependency**: FR-001 (6-layer AES), FR-002 (Config), FR-003 (AST scanning)
> **Status**: ✅ COMPLETE — Full self-lint pipeline with 31 AES rules, CLI commands, multi-language support
> **Self-lint**: `lint-arwaky-cli check .` — project audits `src-rust/` against all AES rules

## 1. Problem Statement

Before self-lint:

| Issue | Description |
|-------|-------------|
| **No dogfooding** | Project could not verify its own architectural compliance |
| **No CI gate** | No automated quality gate for pull requests |
| **Manual architecture review** | AES violations caught only by human code review |
| **No score tracking** | No quantitative measure of codebase health |
| **No violation reporting** | Violations invisible until they caused bugs |

## 2. Solution Overview

Self-lint runs 31 AES rules against `src-rust/` and produces a compliance score.

| Layer | Role |
|-------|------|
| **Taxonomy** | `LintResultVO`, `LintSeverityVO`, `LintScoreVO`, `ArchitectureGovernanceEntity` |
| **Contract** | `IArchLintProtocol`, `ISourceParserPort`, `IConfigOrchestrationProtocol` |
| **Capabilities** | 10 checkers — import, naming, inheritance, cycle, orphan, bypass, etc. |
| **Infrastructure** | Config readers, source parsers, file system |
| **Agent** | `ArchitectureLintOrchestrator`, `LintCheckingCoordinator` |
| **Surfaces** | `cli_check_command.rs`, `cli_core_command.rs` |

### Pipeline

```
User: lint-arwaky-cli check .
  └─► cli_main_entry::handle_check()
        └─► lint_path(root)
              ├─► ArchLintHandler.find_source_dir()
              │     src-rust/ → Rust
              ├─► ConfigLoaderOrchestrator.load_project_config()
              ├─► LintCheckingCoordinator.run_all_checks()
              │     ├─► Per-file: naming, size, primitives, bypass, inheritance
              │     └─► Cross-file: imports, barrels, cycles, orphans
              └─► Score: start 100, deduct per finding, CRITICAL = fail
```

### Score

| Severity | Penalty |
|----------|---------|
| LOW | -1 |
| MEDIUM | -2 |
| HIGH | -3 |
| CRITICAL | -5 |

Any CRITICAL → run fails regardless of score.

## 3. Key Files

| File | Layer | Role |
|------|-------|------|
| `taxonomy/lint_result_vo.rs` | Taxonomy | Single violation result |
| `taxonomy/lint_severity_vo.rs` | Taxonomy | Severity + `score_impact()` |
| `taxonomy/lint_score_vo.rs` | Taxonomy | Score + threshold |
| `taxonomy/architecture_governance_entity.rs` | Taxonomy | Score + violations |
| `contract/architecture_lint_protocol.rs` | Contract | `IArchLintProtocol` |
| `capabilities/architecture_lint_handler.rs` | Capabilities | Top-level handler |
| `capabilities/architecture_compliance_analyzer.rs` | Capabilities | Layer detection |
| `capabilities/architecture_import_checker.rs` | Capabilities | Import validation |
| `capabilities/architecture_naming_checker.rs` | Capabilities | Naming + suffix rules |
| `capabilities/architecture_internal_checker.rs` | Capabilities | Barrel + re-exports |
| `capabilities/architecture_metric_checker.rs` | Capabilities | File size + primitives |
| `capabilities/architecture_cycle_analyzer.rs` | Capabilities | Circular deps |
| `capabilities/architecture_orphan_analyzer.rs` | Capabilities | Unreachable code |
| `capabilities/architecture_inheritance_checker.rs` | Capabilities | Inheritance rules |
| `capabilities/surface_hierarchy_checker.rs` | Capabilities | Surface rules |
| `agent/architecture_lint_orchestrator.rs` | Agent | Orchestration |
| `agent/lint_checking_coordinator.rs` | Agent | Coordinator |
| `surfaces/cli_check_command.rs` | Surfaces | CLI entry |
| `surfaces/cli_core_command.rs` | Surfaces | Clap commands |

## 4. Acceptance Criteria

| # | Kriteria | Status |
|---|----------|--------|
| AC001 | `lint-arwaky-cli check .` runs without error | ✅ |
| AC002 | Detects 153+ violations on own codebase | ✅ |
| AC003 | 31 AES codes (AES001–AES033, AES028/029 reserved) | ✅ |
| AC004 | Score: start 100, deduct by severity, CRITICAL = fail | ✅ |
| AC005 | `scan` runs AES + external adapters | ✅ |
| AC006 | `ci` mode with threshold + exit code | ✅ |
| AC007 | `report` formats: text, JSON, SARIF, JUnit | ✅ |
| AC008 | `cargo check --bin lint-arwaky-cli` passes | ✅ |
| AC009 | `cargo test` passes | ✅ |
