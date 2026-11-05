# Lint Rules (Global)

Shared `lint-arwaky-cli` command surface that applies to every language and every scan session. Language deltas live in the matching `references/*-migration.md`.

**Goal:** Give any migration step a single, checkable home for the CLI invocation, AES101 naming contract, fix-routing table, and triage order, so no step restates these shared facts.

## Invocation

```bash
lint-arwaky-cli scan .
lac fix . --dry-run
lint-arwaky-cli ci . --threshold 90
```

## Shell aliases

| Alias | Target Binary | Description |
| :---- | :------------ | :---------- |
| `lac` | `lint-arwaky-cli` | Primary CLI gatekeeper & scanner |
| `lat` | `lint-arwaky-tui` | Terminal User Interface dashboard |
| `lam` | `lint-arwaky-mcp` | MCP Server (STDIO backend for AI clients) |

## Global CLI options

| Option | Long Flag | Description |
| :----- | :-------- | :---------- |
| `-v` | `--verbose` | Enable debug logging and detailed diagnostic traces. |
| `-q` | `--quiet` | Minimize console output (suppress non-error messages). |
| `-o` | `--output-dir <DIR>` | Directory to save generated reports (overrides active configuration). |
| | `--filter <CODE>` | Filter scan results by specific AES rule code (e.g. `AES101`, `AES301`, `AES401`). |
| `-h` | `--help` | Print help information for the CLI or a specific subcommand. |
| `-V` | `--version` | Print CLI binary version. |

## Commands & subcommands

| Command | Purpose |
| :----- | :----- |
| `scan` / `check [PATH]` | Run all linters; `--format` text / json / sarif / junit, `--member <NAME>`, `--filter <CODE>`, `-o <DIR>` |
| `fix [PATH]` | Apply safe automatic fixes; `--dry-run`, `--filter <CODE>` |
| `ci [PATH]` | Quality gate; `--threshold <SCORE>` (default 80, exit 1 below it). No `--format` flag. |
| `quality`/`import`/`naming`/`role`/`orphan`/`external` | Run one linter family in isolation (same flags as `scan`; `orphan` accepts `--member`) |
| `taxonomy`/`contract`/`capabilities`/`utility`/`agents`/`surface` | Run every linter family, report only one AES layer |
| `security [PATH]` | Code security issues (Bandit / `cargo audit` / ESLint security) |
| `dependencies [PATH]` | Third-party CVE scan of the lockfile/manifest |
| `watch [PATH]` | Re-lint on file save |
| `install-hook` / `uninstall-hook` | Manage the Git pre-commit integration |
| `init` / `install` | Write `lint_arwaky.config.yaml` / install external adapter binaries |
| `config` / `adapters` / `mcp-config` | Print active rules, enabled adapters, MCP client JSON |
| `doctor` / `version` | Environment diagnostics / binary version |

## MCP server tools (`lint-arwaky-mcp`)

Five JSON-RPC 2.0 tools over STDIO for AI clients (Claude Code, Cursor, Windsurf, Hermes):

| Tool Name | Description | Arguments |
| :----- | :----- | :----- |
| `execute_command` | Execute any CLI command action | `action` (required: `"scan"`, `"check"`, `"fix"`, `"security"`, `"doctor"`, …), `args` (optional JSON object) |
| `list_commands` | List available CLI commands catalog | `domain` (optional filter) |
| `read_skill` | Read `SKILL.md` documentation by section | `section` (optional: header name to extract) |
| `health_check` | Check MCP server & adapter health | None |
| `get_config` | Get active architecture config | `path` (optional), `language` (optional: `"rust"`, `"python"`, `"javascript"`) |

## Exit codes

`0` pass · `1` violations (blocks merge) · `2` config/parse error · `3` prerequisite missing (a required external tool is not installed). CI and hooks key off exit code; stdout is not the contract.


