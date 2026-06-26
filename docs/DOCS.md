# Documentation Navigation Hub

> Central index for all project documentation. Every developer starts here.

---

## Quick Start by Role

| I am a...             | Read this first                          | Then                               |
| --------------------- | ---------------------------------------- | ---------------------------------- |
| **New contributor** | [CONTRIBUTING.md](../CONTRIBUTING.md)  | [ARCHITECTURE.md](../ARCHITECTURE.md) |
| **AI Agent**        | [SKILL.md](../SKILL.md)                 | [RULES_AES.md](rules/RULES_AES.md)  |
| **CI/DevOps**       | [DEPLOY.md](../DEPLOY.md)              | [TEST.md](../TEST.md)              |
| **Architect**       | [ARCHITECTURE.md](../ARCHITECTURE.md)  | [RULES_AES.md](rules/RULES_AES.md)  |
| **New developer**   | [DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md) | [CRATE_INDEX.md](CRATE_INDEX.md)  |

---

## Project-Level Documents

| Document                            | Purpose                                        | Path                                |
| ----------------------------------- | ---------------------------------------------- | ----------------------------------- |
| [README.md](../README.md)         | Project overview, install, usage, CLI commands | `README.md`                         |
| [ARCHITECTURE.md](../ARCHITECTURE.md) | AES 7-layer architecture specification         | `ARCHITECTURE.md`                   |
| [DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md) | How/why the code works, patterns, adding rules | `docs/DEVELOPER_GUIDE.md`         |
| [DATA_FLOW.md](DATA_FLOW.md)      | End-to-end data flow for all operations        | `docs/DATA_FLOW.md`                 |
| [CRATE_INDEX.md](CRATE_INDEX.md)  | Complete reference for every crate             | `docs/CRATE_INDEX.md`              |
| [PRD.md](../PRD.md)              | Product Requirements Document (24 AES rules)   | `PRD.md`                            |
| [SKILL.md](../SKILL.md)          | AI agent skill context (MCP tools, CLI)        | `SKILL.md`                          |
| [CONTRIBUTING.md](../CONTRIBUTING.md) | Contributor guide, setup, PR process           | `CONTRIBUTING.md`                   |
| [DEPLOY.md](../DEPLOY.md)        | Deployment guide, MCP setup, checklist         | `DEPLOY.md`                         |
| [TEST.md](../TEST.md)            | Test plan, pass/fail criteria, baselines       | `TEST.md`                           |
| [CHANGELOG.md](../CHANGELOG.md)  | Release history                                | `CHANGELOG.md`                      |
| [AGENTS.md](../AGENTS.md)        | AI agent build/test/branching instructions     | `AGENTS.md`                         |
| [LICENSE](../LICENSE)             | MIT License                                    | `LICENSE`                           |

---

## AES Rules Documentation

| Document                                              | Purpose                                      | Path                            |
| ----------------------------------------------------- | -------------------------------------------- | ------------------------------- |
| [RULES_AES.md](rules/RULES_AES.md)                 | Full 24 AES rules catalog (5 groups)         | `docs/rules/RULES_AES.md`     |
| [RULES_RUFF.md](rules/RULES_RUFF.md)               | Python Ruff rule mapping                     | `docs/rules/RULES_RUFF.md`    |
| [RULES_MYPY.md](rules/RULES_MYPY.md)               | Python MyPy rule mapping                     | `docs/rules/RULES_MYPY.md`    |
| [RULES_BANDIT.md](rules/RULES_BANDIT.md)           | Python Bandit security rules                 | `docs/rules/RULES_BANDIT.md`  |
| [RULES_RADON.md](rules/RULES_RADON.md)             | Python Radon complexity rules                | `docs/rules/RULES_RADON.md`   |
| [RULES_CLIPPY.md](rules/RULES_CLIPPY.md)           | Rust Clippy lint mapping                     | `docs/rules/RULES_CLIPPY.md`  |
| [RULES_RUSTFMT.md](rules/RULES_RUSTFMT.md)         | Rust formatting rules                        | `docs/rules/RULES_RUSTFMT.md` |
| [RULES_CARGO_AUDIT.md](rules/RULES_CARGO_AUDIT.md) | Rust dependency audit rules                  | `docs/rules/RULES_CARGO_AUDIT.md` |
| [RULES_ESLINT.md](rules/RULES_ESLINT.md)           | JavaScript/TypeScript ESLint rules           | `docs/rules/RULES_ESLINT.md`  |
| [RULES_TSC.md](rules/RULES_TSC.md)                 | TypeScript compiler rules                    | `docs/rules/RULES_TSC.md`     |

---

## Crate Feature Requirement Documents (FRD)

Each feature crate has its own FRD explaining **why** it exists and **what** it does.

| Crate                       | FRD                                                          | Source Path                                   |
| --------------------------- | ------------------------------------------------------------ | --------------------------------------------- |
| `import-rules`            | [FRD.md](../crates/import-rules/FRD.md)                   | `crates/import-rules/FRD.md`                 |
| `naming-rules`            | [FRD.md](../crates/naming-rules/FRD.md)                   | `crates/naming-rules/FRD.md`                 |
| `code-analysis`           | [FRD.md](../crates/code-analysis/FRD.md)                  | `crates/code-analysis/FRD.md`                |
| `role-rules`              | [FRD.md](../crates/role-rules/FRD.md)                     | `crates/role-rules/FRD.md`                   |
| `orphan-detector`         | [FRD.md](../crates/orphan-detector/FRD.md)                | `crates/orphan-detector/FRD.md`              |
| `auto-fix`                | [FRD.md](../crates/auto-fix/FRD.md)                       | `crates/auto-fix/FRD.md`                     |
| `config-system`           | [FRD.md](../crates/config-system/FRD.md)                  | `crates/config-system/FRD.md`                |
| `external-lint`           | [FRD.md](../crates/external-lint/FRD.md)                  | `crates/external-lint/FRD.md`                |
| `cli-commands`            | [FRD.md](../crates/cli-commands/FRD.md)                   | `crates/cli-commands/FRD.md`                 |
| `mcp-server`              | [FRD.md](../crates/mcp-server/FRD.md)                     | `crates/mcp-server/FRD.md`                   |
| `git-hooks`               | [FRD.md](../crates/git-hooks/FRD.md)                      | `crates/git-hooks/FRD.md`                    |
| `file-watch`              | [FRD.md](../crates/file-watch/FRD.md)                     | `crates/file-watch/FRD.md`                   |
| `project-setup`           | [FRD.md](../crates/project-setup/FRD.md)                  | `crates/project-setup/FRD.md`                |
| `maintenance`             | [FRD.md](../crates/maintenance/FRD.md)                    | `crates/maintenance/FRD.md`                  |
| `tui`                     | [FRD.md](../crates/tui/FRD.md)                            | `crates/tui/FRD.md`                          |

---

## Audit Findings (Per-Crate Reviews)

Completed and in-progress audit findings from the v1.10.14 review cycle.

### Done

| Crate              | Finding                                                        | Path                                                 |
| ------------------ | -------------------------------------------------------------- | ---------------------------------------------------- |
| `shared`         | [shared_v1.10.14.md](finding/shared_v1.10.14.md)            | `docs/finding/shared_v1.10.14.md`                  |
| `cli-commands`   | [cli-commands_v1.10.14.md](finding/cli-commands_v1.10.14.md)| `docs/finding/cli-commands_v1.10.14.md`            |
| `config-system`  | [done/config-system_v1.10.14.md](finding/done/config-system_v1.10.14.md) | `docs/finding/done/config-system_v1.10.14.md` |
| `auto-fix`       | [done/auto-fix_v1.10.14.md](finding/done/auto-fix_v1.10.14.md) | `docs/finding/done/auto-fix_v1.10.14.md`       |
| `git-hooks`      | [done/git-hooks_v1.10.14.md](finding/done/git-hooks_v1.10.14.md) | `docs/finding/done/git-hooks_v1.10.14.md`     |
| `maintenance`    | [done/maintenance_v1.10.14.md](finding/done/maintenance_v1.10.14.md) | `docs/finding/done/maintenance_v1.10.14.md` |
| `orphan-detector`| [done/orphan-detector_v1.10.14.md](finding/done/orphan-detector_v1.10.14.md) | `docs/finding/done/orphan-detector_v1.10.14.md` |
| `role-rules`     | [done/role-rules_v1.10.14.md](finding/done/role-rules_v1.10.14.md) | `docs/finding/done/role-rules_v1.10.14.md`   |
| Source parsing    | [done/source-parsing-decoupling-context.md](finding/done/source-parsing-decoupling-context.md) | `docs/finding/done/source-parsing-decoupling-context.md` |

### Todo

| Crate              | Finding                                                          | Path                                                   |
| ------------------ | ---------------------------------------------------------------- | ------------------------------------------------------ |
| `external-lint`  | [todo/external-lint_v1.10.14.md](finding/todo/external-lint_v1.10.14.md) | `docs/finding/todo/external-lint_v1.10.14.md`    |
| `mcp-server`     | [todo/mcp-server_v1.10.14.md](finding/todo/mcp-server_v1.10.14.md) | `docs/finding/todo/mcp-server_v1.10.14.md`        |
| `project-setup`  | [todo/project-setup_v1.10.14.md](finding/todo/project-setup_v1.10.14.md) | `docs/finding/todo/project-setup_v1.10.14.md` |

---

## Development Plans

### Done

| Plan                                                    | Path                                                  |
| ------------------------------------------------------- | ----------------------------------------------------- |
| [cleanup-import-export-diff.md](plan/done/cleanup-import-export-diff.md) | `docs/plan/done/cleanup-import-export-diff.md`   |
| [cleanup-report.md](plan/done/cleanup-report.md)     | `docs/plan/done/cleanup-report.md`                |
| [cleanup-setup-maintenance.md](plan/done/cleanup-setup-maintenance.md) | `docs/plan/done/cleanup-setup-maintenance.md` |
| [mcp-proper-implementation.md](plan/done/mcp-proper-implementation.md) | `docs/plan/done/mcp-proper-implementation.md` |
| [mcp-rmcp-migration.md](plan/done/mcp-rmcp-migration.md) | `docs/plan/done/mcp-rmcp-migration.md`           |
| [source-parsing-decoupling-finding.md](plan/done/source-parsing-decoupling-finding.md) | `docs/plan/done/source-parsing-decoupling-finding.md` |
| [watch-implementation.md](plan/done/watch-implementation.md) | `docs/plan/done/watch-implementation.md`       |

### Todo

| Plan                                                          | Path                                                |
| ------------------------------------------------------------- | --------------------------------------------------- |
| [bypass-checker-cargo-toml.md](plan/todo/bypass-checker-cargo-toml.md) | `docs/plan/todo/bypass-checker-cargo-toml.md` |
| [ratatui-tui-upgrade.md](plan/todo/ratatui-tui-upgrade.md)   | `docs/plan/todo/ratatui-tui-upgrade.md`         |

### Research (Auto-Repair)

| Document                                        | Path                                       |
| ----------------------------------------------- | ------------------------------------------ |
| [ai-autorepair-burn.md](plan/research/ai-autorepair-burn.md) | `docs/plan/research/ai-autorepair-burn.md` |
| [draf_autorepair.v1.md](plan/research/draf_autorepair.v1.md) — v13 | `docs/plan/research/draf_autorepair.v*.md` |

---

## Related Documents by Topic

### Architecture & Design

- [ARCHITECTURE.md](../ARCHITECTURE.md) — 7-layer AES specification
- [RULES_AES.md](rules/RULES_AES.md) — 24 AES rules with severity levels
- [PRD.md](../PRD.md) — Product requirements, feature specs, AES rule groups
- [AGENTS.md](../AGENTS.md) — Workspace packages, layer table, build commands

### Import & Layer Rules

- [RULES_AES.md](rules/RULES_AES.md) § AES201–AES205 — Import boundary rules
- [crates/import-rules/FRD.md](../crates/import-rules/FRD.md) — Import compliance feature spec
- [ARCHITECTURE.md](../ARCHITECTURE.md) § Layer Hierarchy — Dependency direction diagram

### Naming & Role Rules

- [RULES_AES.md](rules/RULES_AES.md) § AES101–AES102 — Naming convention rules
- [RULES_AES.md](rules/RULES_AES.md) § AES401–AES406 — Role violation rules
- [crates/naming-rules/FRD.md](../crates/naming-rules/FRD.md) — Naming convention feature spec
- [crates/role-rules/FRD.md](../crates/role-rules/FRD.md) — Role violation feature spec

### Quality & Bypass Detection

- [RULES_AES.md](rules/RULES_AES.md) § AES301–AES305 — Quality rules
- [crates/code-analysis/FRD.md](../crates/code-analysis/FRD.md) — Code analysis feature spec

### External Linters (Rust/Python/JS)

- [RULES_CLIPPY.md](rules/RULES_CLIPPY.md) — Rust Clippy mapping
- [RULES_RUSTFMT.md](rules/RULES_RUSTFMT.md) — Rust formatting
- [RULES_CARGO_AUDIT.md](rules/RULES_CARGO_AUDIT.md) — Rust dependency audit
- [RULES_RUFF.md](rules/RULES_RUFF.md) — Python Ruff mapping
- [RULES_MYPY.md](rules/RULES_MYPY.md) — Python type checking
- [RULES_BANDIT.md](rules/RULES_BANDIT.md) — Python security scanning
- [RULES_RADON.md](rules/RULES_RADON.md) — Python complexity analysis
- [RULES_ESLINT.md](rules/RULES_ESLINT.md) — JavaScript/TypeScript linting
- [RULES_TSC.md](rules/RULES_TSC.md) — TypeScript compiler checks
- [crates/external-lint/FRD.md](../crates/external-lint/FRD.md) — External linter adapters feature spec

### MCP & AI Integration

- [SKILL.md](../SKILL.md) — AI agent skill context, MCP tools
- [DEPLOY.md](../DEPLOY.md) § MCP Server Setup — MCP configuration guide
- [crates/mcp-server/FRD.md](../crates/mcp-server/FRD.md) — MCP server feature spec

### CLI & TUI

- [README.md](../README.md) § CLI Commands Reference — All CLI commands
- [crates/cli-commands/FRD.md](../crates/cli-commands/FRD.md) — CLI commands feature spec
- [crates/tui/FRD.md](../crates/tui/FRD.md) — TUI feature spec
- [README.md](../README.md) § TUI — TUI shortcuts and panels

### Testing & CI

- [TEST.md](../TEST.md) — Test plan, pass/fail criteria, baselines
- [DEPLOY.md](../DEPLOY.md) § Production Deployment Checklist — CI/CD checklist
- [AGENTS.md](../AGENTS.md) § Testing with test projects — Test commands

### Git Hooks & Watch

- [crates/git-hooks/FRD.md](../crates/git-hooks/FRD.md) — Git hooks feature spec
- [crates/file-watch/FRD.md](../crates/file-watch/FRD.md) — File watching feature spec
- [README.md](../README.md) § Usage — Watch mode usage

### Orphan & Dead Code

- [RULES_AES.md](rules/RULES_AES.md) § AES501–AES506 — Orphan detection rules
- [crates/orphan-detector/FRD.md](../crates/orphan-detector/FRD.md) — Orphan detection feature spec

### Auto-Fix

- [crates/auto-fix/FRD.md](../crates/auto-fix/FRD.md) — Auto-fix feature spec
- [README.md](../README.md) § Usage — Fix command usage

### Config & Setup

- [crates/config-system/FRD.md](../crates/config-system/FRD.md) — Config loading feature spec
- [crates/project-setup/FRD.md](../crates/project-setup/FRD.md) — Project setup feature spec
- [DEPLOY.md](../DEPLOY.md) § Configuration — Config setup guide

---

## Crate → Layer Mapping

Quick reference: which crates contain which layers.

| Crate             | taxonomy | contract | capabilities | infrastructure | agent | surface | root |
| ----------------- | -------- | -------- | ------------ | -------------- | ----- | ------- | ---- |
| `shared`        | ✅       | ✅       |              |                |       |         |      |
| `import-rules`  |          |          | ✅           | ✅              | ✅    |         | ✅   |
| `naming-rules`  |          |          | ✅           | ✅              | ✅    |         | ✅   |
| `code-analysis` |          |          | ✅           |                | ✅    |         | ✅   |
| `role-rules`    |          |          | ✅           |                | ✅    |         | ✅   |
| `orphan-detector`| ✅      |          | ✅           |                | ✅    |         | ✅   |
| `auto-fix`      |          |          | ✅           |                | ✅    |         | ✅   |
| `config-system` | ✅       | ✅       | ✅           | ✅              | ✅    |         | ✅   |
| `external-lint` |          |          |              | ✅              | ✅    |         | ✅   |
| `cli-commands`  |          |          |              | ✅              |       | ✅      |      |
| `mcp-server`    | ✅       | ✅       |              |                | ✅    | ✅      | ✅   |
| `git-hooks`     | ✅       | ✅       | ✅           | ✅              | ✅    |         | ✅   |
| `file-watch`    | ✅       | ✅       | ✅           | ✅              | ✅    |         | ✅   |
| `project-setup` | ✅       | ✅       | ✅           | ✅              | ✅    |         | ✅   |
| `maintenance`   |          |          | ✅           |                | ✅    |         | ✅   |
| `tui`           | ✅       | ✅       | ✅           | ✅              | ✅    | ✅      | ✅   |
