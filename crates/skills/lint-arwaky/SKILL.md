---
name: lint-arwaky
description: AES architecture linter for Python Rust TS. Use when scanning, fixing AES101-AES506 violations.
metadata:
  tags: [python, rust, typescript, lint, aes, compliance, scanning, architecture, mcp, ci, fix]
  triggers:
    - "lint arwaky"
    - "lint arwaky python"
    - "lint arwaky rust"
    - "lint arwaky typescript"
    - "lint code"
    - "scan project"
    - "scan python project"
    - "scan rust project"
    - "scan typescript project"
    - "verify aes compliance"
    - "check compliance"
    - "aes violations"
    - "fix architecture violations"
    - "fix violations"
    - "scan and fix"
    - "audit codebase"
    - "architecture health check"
    - "ci quality gate"
  dependencies: []
  related:
    - create-taxonomy
    - create-utility
    - create-contract
    - create-capabilities
    - create-agent
    - create-surface
    - create-root
    - testing-suite
    - cleanup-consolidate
    - fix-bypass
---
# Lint Arwaky (AES Architecture Linter)

Lint Arwaky is a high-speed Rust-based architecture enforcement engine. Structured under the
Agentic Engineering System (AES) specification, it enforces layer boundaries, naming
conventions, import directions, and role boundaries across Rust, Python, and TypeScript
codebases.

## When to Use This Skill

Activate this skill when:

- Creating, modifying, or refactoring files in AES-governed repositories.
- Verifying layer boundaries and dependency rules before submitting pull requests.
- Diagnosing architecture violations, orphan files, or illegal upward/circular imports.
- Running automated CI quality gates or architecture health checks.

## AES 7-Layer Hierarchy & Naming Contract

Every source file must follow `<layer>_<concern>_<role>.<ext>` (AES101: lowercase, at least
three underscore-separated words; verbatim exceptions such as `main.rs`, `lib.rs`, `mod.rs`,
`__init__.py`, `index.ts`, `root_cli_main_entry.rs`).

The 7 strictly-ordered layers, **bottom-up** (a layer may only depend on layers below it):

| # | Layer prefix   | Purpose                                             | Example                    |
| - | ---------------- | ----------------------------------------------------- | ---------------------------- |
| 1 | `taxonomy_`    | value objects, entities, errors, events, constants  | `taxonomy_status_type.py`  |
| 2 | `utility_`     | pure, stateless helpers                             | `utility_hash_helper.py`   |
| 3 | `contract_`    | protocols, aggregates, schemas, DTOs                | `contract_payload_model.py`|
| 4 | `capabilities_`| domain logic and business rules                     | `capabilities_task_executor.py` |
| 5 | `agent_`       | orchestration across subsystems                       | `agent_billing_orchestrator.rs` |
| 6 | `surface_`     | CLI / MCP / REST / TUI / GUI adapters                 | `surface_cli_adapter.py`   |
| 7 | `root_`        | entry points and composition containers             | `root_main_entry.py`       |

*Core invariant:* upper layers may only depend downward. Lower layers may never import upper
layers.

**`test` is not a layer.** Test and benchmark files live in `tests/` and `benches/` and follow
their own naming convention — see the `testing-suite` skill.

## Invoking the Linter

```bash
# Repo convention: through the agents-arwaky orchestrator
aa tool run lint scan .
aa tool run lint check src/modules/
aa tool run lint fix .
aa tool run lint ci . --threshold 80

# Equivalent direct-binary forms (also reachable as `lac`)
lint-arwaky-cli scan .
lac fix . --dry-run
```

### Shell aliases

| Alias | Target Binary     | Description                               | Example Usage                       |
| :---- | :---------------- | :---------------------------------------- | :------------------------------------ |
| `lac` | `lint-arwaky-cli` | Primary CLI gatekeeper & scanner          | `lac scan .`, `lac fix`, `lac doctor` |
| `lat` | `lint-arwaky-tui` | Terminal User Interface (TUI) dashboard   | `lat`                               |
| `lam` | `lint-arwaky-mcp` | MCP Server (STDIO backend for AI clients) | Configured in Claude / Cursor / Windsurf |

### Global CLI Options

| Option | Long Flag              | Description                                                                             |
| :----- | :--------------------- | :-------------------------------------------------------------------------------------- |
| `-v` | `--verbose`          | Enable debug logging and detailed diagnostic traces.                                    |
| `-q` | `--quiet`            | Minimize console output (suppress non-error messages).                                  |
| `-o` | `--output-dir <DIR>` | Directory to save generated reports (overrides active configuration).                   |
|      | `--filter <CODE>`    | Filter scan results by specific AES rule code (e.g. `AES101`, `AES301`, `AES401`). |
| `-h` | `--help`             | Print help information for the CLI or a specific subcommand.                            |
| `-V` | `--version`          | Print CLI binary version.                                                               |

## Commands & Subcommands

| Command                              | Purpose                                                                     |
| -------------------------------------- | ------------------------------------------------------------------------------ |
| `scan` / `check [PATH]`            | Run all linters; `--format` text / json / sarif / junit, `--member <NAME>`, `--filter <CODE>`, `-o <DIR>` |
| `fix [PATH]`                       | Apply safe automatic fixes; `--dry-run`, `--filter <CODE>`                |
| `ci [PATH]`                        | Quality gate; `--threshold <SCORE>` (default 80, exit 1 below it), `--format` |
| `quality`/`import`/`naming`/`role`/`orphan`/`external` | Run one linter family in isolation (same flags as `scan`; `orphan` accepts `--member`) |
| `security [PATH]`                  | Code security issues (Bandit / `cargo audit` / ESLint security)             |
| `dependencies [PATH]`              | Third-party CVE scan of the lockfile/manifest                              |
| `watch [PATH]`                     | Re-lint on file save                                                       |
| `install-hook` / `uninstall-hook`  | Manage the Git pre-commit integration                                    |
| `init` / `install`                 | Write `lint_arwaky.config.yaml` / install external adapter binaries       |
| `config-show` / `adapters` / `mcp-config` | Print active rules, enabled adapters, MCP client JSON                     |
| `doctor` / `version`               | Environment diagnostics / binary version                                 |

```bash
# Per-rule targeting and reporting
lint-arwaky-cli scan . --format json --filter AES201
lint-arwaky-cli orphan crates/ --member shared_common --format json
lint-arwaky-cli fix modules/ --dry-run --filter AES101

# Reports go to the XDG data dir
lint-arwaky-cli scan crates/ --format sarif \
  > ~/.local/share/lint-arwaky/reports/scan_rust.sarif
```

See `references/python.md`, `references/rust.md`, `references/typescript.md` for the
per-language flags, adapters, native tooling commands, and verify pipelines.

## MCP Server Tools Reference (`lint-arwaky-mcp`)

`lint-arwaky-mcp` exposes 5 JSON-RPC 2.0 tools over STDIO for AI clients (Claude Code, Cursor,
Windsurf, Hermes):

| Tool Name           | Description                               | Arguments / Parameters                                                                                                       |
| :------------------ | :---------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| `execute_command` | Execute any CLI command action            | `action` (required: `"scan"`, `"check"`, `"fix"`, `"security"`, `"doctor"`, …), `args` (optional JSON object, e.g. `{"path": "/abs/path"}`) |
| `list_commands`   | List available CLI commands catalog       | `domain` (optional filter, e.g. `"setup"`, `"check"`)                                                                     |
| `read_skill`      | Read `SKILL.md` documentation by section | `section` (optional: header name to extract)                                                                              |
| `health_check`    | Check MCP server & adapter health         | None (0 parameters)                                                                                                          |
| `get_config`      | Get active architecture config            | `path` (optional), `language` (optional: `"rust"`, `"python"`, `"javascript"`)                                          |

```json
{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"execute_command","arguments":{"action":"scan","args":{"path":"/abs/path/to/crates"}}}}
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"health_check","arguments":{}}}
```

## Scan → Diagnose → Fix → Verify Workflow

1. **Pre-flight** — make sure the codebase builds in its own language (see the language
   reference). Violations reported on broken code are unreliable.
2. **Scan** — `lint-arwaky-cli scan <target-path> --format json > /tmp/arwaky-scan.json`, then
   triage CRITICAL first: 🔴 AES201, AES205, AES304 → 🟡 AES101–102, AES202, AES301–303,
   AES401–403, AES406, AES505–506 → 🟢 AES203–204, AES305, AES404–405, AES501–504.
3. **Diagnose** — for each violation: which rule, which layer (file prefix), what the message
   says, whether it is auto-fixable (AES101 rename, AES203 unused import, AES304 bypass → yes;
   AES201 wrong dependency → manual), and the root cause (naming, wrong import, missing
   implementation, dead code).
4. **Fix** — auto: `lint-arwaky-cli fix <target-path>` (add `--dry-run` first, `--filter AES101`
   to scope). Manual: follow `references/rule-routing.md`, which maps every AES code to the
   remediation and the `create-*` skill that owns that layer.
5. **Verify** — re-scan to 0 violations, then run the language build/import/lint checks in the
   matching reference file.
6. **Commit** — `git add -A && git commit -m "fix: resolve <N> AES violations (<rules>)"`.

## Exit Codes

- `0`: All architecture rules and layer constraints passed.
- `1`: Architectural violations detected (blocks merge/quality gates).
- `2`: Configuration or parse errors.
