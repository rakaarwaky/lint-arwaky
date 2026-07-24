# FRD — mcp-server

## System Overview

The mcp-server crate implements a Model Context Protocol (MCP) server that exposes the lint-arwaky pipeline as JSON-RPC tools for AI agents and IDEs.

**Product policy (locked):**

- **Five MCP tools** (not four): `execute_command`, `list_commands`, `read_skill`, `health_check`, **`get_config`**.
- **Full CLI parity** for every action under `execute_command` — no silent stubs or placeholder success responses.
- JSON responses include `exit_code` aligned with the workspace Exit Code Contract (`0` / `1` / `2` / `3`) from the root PRD.

## Functional Requirements

### FR-001: Execute Command

- **Description**: Execute any lint-arwaky CLI-equivalent action via MCP with the same business outcome as the CLI.
- **Input**: Action string and optional argument map (keys: `path`, `threshold`, `client`, `dry_run`, `format`, `member`, `base`, …).
- **Output**: JSON with `status`, `action`, `exit_code`, and action-specific fields (e.g. `total_violations`, `results`, `error`).
- **Business Rules**:
  - Supported actions MUST match CLI capability: `check`, `scan`, `fix`, `ci`, `doctor`, `version`, `adapters`, `install-hook`, `uninstall-hook`, `init`, `install`, `mcp-config`, `config-show`, `orphan`, `security`, `duplicates`, `dependencies`, `quality`, `import`, `naming`, `role`, `external`, `watch` (if long-lived, may return explicit `unsupported` with `exit_code: 2` until async watch is designed).
  - Each action **delegates to the same aggregates** used by the CLI (analysis, auto-fix, maintenance, git-hooks, project-setup, etc.).
  - **Forbidden**: placeholder success, empty success without side effects, or “returns action + path only” stubs for actions that perform real work on CLI.
  - `check` / `scan`: default path `"."`; run full pipeline; `exit_code` 0/1/2 per Exit Code Contract.
  - `ci`: default path `"."`, default threshold 80; pass/fail with `exit_code` 0/1/2.
  - `fix`: run auto-fix (remove/replace/rename); honor `dry_run`; report applied/skipped/failed outcomes.
  - `doctor`: toolchain diagnostics; `exit_code` 0 when diagnostic completes (missing tools listed in body); `2` on internal failure.
  - `security`: vulnerability scan; `exit_code` 0 clean, 1 findings, 2 runtime error, **3** tool missing.
  - `install-hook` / `uninstall-hook`: perform real hook install/uninstall via git-hooks aggregate.
  - `init` / `install` / `mcp-config` / `config-show`: perform real setup/config operations via project-setup / config aggregates.
  - `orphan` / `dependencies` / individual linter actions: run real analysis or reports.
  - Unknown action: `{"error": "Unknown action: <action>", "exit_code": 2}`.
- **Edge Cases**:
  - Missing `path`: defaults to `"."`.
  - Missing `threshold`: defaults to 80.
  - Pipeline failure: `exit_code: 2` with error message.
  - Required tool missing (security): `exit_code: 3`.
- **Error Handling**:
  - Errors returned as JSON objects with `error` + `exit_code`; never silent success.

### FR-002: List Commands

- **Description**: List available CLI commands with descriptions and examples, optionally filtered by domain.
- **Input**: Optional domain filter string.
- **Output**: JSON with `commands` array (`name`, `description`, `example`), `total`, `exit_code: 0`.
- **Business Rules**:
  - Non-empty domain filter restricts to commands whose name contains the domain string.
  - Empty/absent domain returns full catalog from taxonomy/command catalog.
- **Edge Cases**:
  - No matches: empty `commands`, `total: 0`.
- **Error Handling**:
  - Serialization failure: `exit_code: 2` with error (prefer error object over empty string).

### FR-003: Read Skill

- **Description**: Read SKILL.md documentation by section from candidate locations.
- **Input**: Optional section filter string.
- **Output**: JSON with `content` or `error`, plus `exit_code`.
- **Business Rules**:
  - Search order: project-relative candidates, then XDG config (`~/.config/lint-arwaky/SKILL.md`).
  - Optional section extracts content between `## <section>` headers.
- **Edge Cases**:
  - Not found: error + searched paths, `exit_code: 2`.
  - Section missing: error, `exit_code: 2`.
- **Error Handling**:
  - File read failure treated as not found.

### FR-004: Health Check

- **Description**: Report adapter availability and server version.
- **Input**: None.
- **Output**: JSON with `version`, `adapters_available`, `adapters_total`, `adapters[]` (`name`, `language`, `status`), `exit_code: 0` when check completes.
- **Business Rules**:
  - Adapters checked include at least: ruff, mypy, bandit, clippy, eslint.
  - Status is `available` or `not_installed`.
  - Completing the check always yields `exit_code: 0` (missing adapters are data, not process failure).
- **Edge Cases**:
  - All adapters missing: `adapters_available: 0`, still `exit_code: 0`.
- **Error Handling**:
  - Spawn/`which` failure for a tool → that adapter `not_installed`.

### FR-005: Get Config (5th tool)

- **Description**: Return the effective architecture configuration for a target path/language so agents can reason about rules, thresholds, and adapters without shelling out.
- **Input**: Optional `path`, optional language hint.
- **Output**: JSON with effective config summary (layers, rules enabled, score threshold, ignored paths, adapter toggles), config source path(s), warnings, `exit_code`.
- **Business Rules**:
  - Loads config via the same config-system path resolution as CLI (`config-show` parity for data).
  - Does not mutate files.
  - Redacts secrets if any env-backed fields appear (none expected for core config).
- **Edge Cases**:
  - No config file: return embedded defaults + warning, `exit_code: 0`.
  - Invalid path: `exit_code: 2`.
- **Error Handling**:
  - Parse failures: surface warnings or `exit_code: 2` when config is unusable.

### FR-006: MCP Protocol Registration

- **Description**: Register all five tools and server metadata with the MCP framework.
- **Input**: None (construction-time).
- **Output**: Server info with protocol version, name `lint-arwaky`, version, tools capability listing five tools.
- **Business Rules**:
  - Tools: `execute_command`, `list_commands`, `read_skill`, `health_check`, `get_config`.
- **Edge Cases**: None (declarative).
- **Error Handling**: Registration failures prevent server start (fail fast).

## API Contract

| Operation | Input | Output | Description |
| --------- | ----- | ------ | ----------- |
| Execute command | action + args | JSON + exit_code | CLI-parity action execution |
| List commands | optional domain | JSON command catalog | Discover actions |
| Read skill | optional section | JSON content/error | Documentation access |
| Health check | none | JSON adapter status | Environment health |
| Get config | optional path/language | JSON effective config | Agent-readable configuration |
| Server info | none | server metadata | MCP handshake |

## Integration Points

- **Internal**:
  - CLI command aggregates / analysis pipeline (same as CLI)
  - auto-fix, maintenance, git-hooks, project-setup, config-system, external-lint
  - shared taxonomy VOs and contracts
- **External**:
  - MCP protocol library (JSON-RPC, tool registration)
  - Host process environment (`which`, cargo, language toolchains)

## Non-functional Requirements (Detailed)

- **Performance**: list/read_skill/get_config/health under 5s typical; execute bounded by underlying pipeline.
- **Parity**: For every non-watch action, MCP and CLI produce equivalent exit semantics and side effects.
- **Concurrency**: Handlers may run concurrently; file mutations (fix) must remain safe (serialize per path if needed).
- **Security**: Unknown actions never invoke arbitrary shell; only allowlisted actions.

## Test Scenarios / QA Checklist

- [ ] FR-001: `check`/`scan` returns violations + `exit_code` 0/1/2 matching CLI on same fixture.
- [ ] FR-001: `fix` applies real fixes (or dry-run report); no placeholder success.
- [ ] FR-001: `install-hook` / `uninstall-hook` change hook state like CLI.
- [ ] FR-001: `security` returns `exit_code: 3` when tool missing.
- [ ] FR-001: unknown action → error + `exit_code: 2`.
- [ ] FR-002: list with/without domain filter.
- [ ] FR-003: read full skill and section; missing skill errors.
- [ ] FR-004: health lists adapters; all missing still `exit_code: 0`.
- [ ] FR-005: get_config returns defaults when no file; errors on bad path.
- [ ] FR-006: MCP tool listing exposes **exactly five** tools.

## Assumptions & Constraints

- Same aggregates as CLI are wired into the MCP composition root.
- Long-lived `watch` may be deferred with explicit unsupported response until async design exists.
- SKILL.md location search is best-effort across project and XDG paths.

## Glossary

- **MCP**: Model Context Protocol — JSON-RPC standard for AI agent tools.
- **Parity**: Same business outcome for an action via CLI or MCP.
- **Exit Code Contract**: 0 ok, 1 policy fail, 2 runtime error, 3 prerequisite missing.
- **get_config**: Fifth MCP tool for effective configuration inspection.

## Reference

- PRD: [PRD.md](../../PRD.md)
