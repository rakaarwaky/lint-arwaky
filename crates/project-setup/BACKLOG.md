# Feature Backlog: Project Setup

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [BACKLOG.md](../../BACKLOG.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p project_setup --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p project_setup --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| PROJ-01 | FR-001 | SCEN-001 - MCP Config — 10 scenarios verified | P0 | Done | `cargo test -p project_setup --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| PROJ-02 | FR-002 | SCEN-002 - Env File — 2 scenarios verified | P0 | Done | `cargo test -p project_setup --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| PROJ-03 | FR-003 | SCEN-003 - Language Detection — 6 scenarios verified | P0 | Done | `cargo test -p project_setup --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| PROJ-04 | FR-004 | SCEN-004 - Adapter Installation — 6 scenarios verified | P0 | Done | `cargo test -p project_setup --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| PROJ-05 | FR-005 | SCEN-005 - Config Template — 5 scenarios verified | P0 | Done | `cargo test -p project_setup --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| PROJ-06 | FR-006 | SCEN-006 - Config Writing — 3 scenarios verified | P0 | Done | `cargo test -p project_setup --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| PROJ-07 | FR-007 | SCEN-007 - Pre-flight Check — 2 scenarios verified | P0 | Done | `cargo test -p project_setup --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Claude config | `mcpServers` wrapper with `lint-arwaky` entry | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Cursor config | `mcpServers` wrapper | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Windsurf config | `mcpServers` wrapper | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Copilot config | `mcpServers` wrapper | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Hermes config | Base config without wrapper | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| VS Code config | `mcp.servers` wrapper | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| `all` client | All client formats in one JSON | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Binary in CARGO_HOME/bin | Resolved path used | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Binary not found anywhere | Bare name`lint-arwaky-mcp` | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| LINT_ARWAKY_MCP_BIN set | Env var path used | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Normal home path | Correct PHANTOM_ROOT value | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Empty home path | PHANTOM_ROOT=/ | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Cargo.toml exists | Rust detected | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| pyproject.toml exists | Python detected | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| package.json exists | JavaScript detected | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Empty directory | Empty list (no default) | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Multi-language project | All detected languages | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| target/, node_modules/ dirs | Skipped | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Python install | pip install --user ruff mypy bandit | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Python PEP 668 retry | --break-system-packages on failure | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| JS install | npm install -g eslint prettier typescript | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| JS install with sudo | sudo npm install -g | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Rust tools | Suggestion message (not installed) | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Empty package list | Ok(()) without spawning | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| "rust" | Rust config template | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| "python" | Python config template | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| "typescript" | TypeScript config template | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Unknown language | Error with supported languages list | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| "Rust" (case mismatch) | Normalized to "rust" | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Write config file | Byte count in description | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Create global config dir | ~/.config/lint-arwaky/ created | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| Dir already exists | Idempotent | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| pip available | status "ok" | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |
| npm not found | status "not_found" | Automated | `tests/project-setup/` | cargo test -p project_setup | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p project_setup --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 34 scenarios mapped; all Automated via `cargo test -p project_setup` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
