# HOW TO USE LINT COMMANDS

> **Purpose**: Run `lint-arwaky-cli` (and its TUI/MCP/alias surface) with the shared command
> set that applies to every language and every scan session.
>
> **Audience**: Agents and engineers invoking the AES linter from a shell, orchestrator, or MCP
> client.
>
> **Scope**: Invocation forms, shell aliases, global flags, subcommands, report paths, MCP tools,
> exit codes. Language deltas live in `HOW-TO-USE-LINT-PYTHON.md` / `-RUST.md` / `-TYPESCRIPT.md`;
> AES-code fix routing lives in `HOW-TO-USE-LINT-ROUTING.md`.
>
> **Location**: Repo root (or target root); default reports under
> `${XDG_DATA_HOME:-~/.local/share}/aes-lint-arwaky/reports/`.
>
> **Length**: One session = pick invocation → scan with format/filter → fix → verify → exit-code
> check. Detail stays in the command and MCP tables below.

---

## Rules

Six rules. Each one prevents a specific failure mode.

1. **Prefer the orchestrator in this repo.** `aa tool run lint <sub> [PATH]` resolves the binary
   the same way other tools do; direct `lint-arwaky-cli` / `lac` are equivalent fallbacks.
2. **Shared flags are defined only here.** Global options, aliases, subcommands, MCP tools, and
   exit codes are not restated in the language HOW-TOs — those only add language deltas.
3. **Always choose an explicit `--format` when the output is consumed.** `json` for triage
   pipelines, `sarif`/`junit` for CI, `text` for humans. Default text is not machine-stable.
4. **Scope before you auto-fix.** `fix` supports `--dry-run` and `--filter <CODE>`; run dry-run
   (and a narrow filter when only one family is in play) before writing to disk.
5. **`--filter` is an AES code, not a substring.** e.g. `AES101`, `AES201`. Family subcommands
   (`import`, `naming`, `role`, `orphan`, …) already narrow the linter; use `--filter` for a
   single rule within a family.
6. **Exit code is the gate.** `0` pass · `1` violations (blocks merge) · `2` config/parse error.
   CI and hooks key off this; do not re-parse stdout to decide success.

---

## Template

Copy, fill, delete nothing.

### Invocation

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
| :---- | :---------------- | :---------------------------------------- | :---------------------------------- |
| `lac` | `lint-arwaky-cli` | Primary CLI gatekeeper & scanner          | `lac scan .`, `lac fix`, `lac doctor` |
| `lat` | `lint-arwaky-tui` | Terminal User Interface (TUI) dashboard   | `lat`                               |
| `lam` | `lint-arwaky-mcp` | MCP Server (STDIO backend for AI clients) | Configured in Claude / Cursor / Windsurf |

### Global CLI options

| Option | Long Flag              | Description                                                                             |
| :----- | :--------------------- | :-------------------------------------------------------------------------------------- |
| `-v` | `--verbose`          | Enable debug logging and detailed diagnostic traces.                                    |
| `-q` | `--quiet`            | Minimize console output (suppress non-error messages).                                  |
| `-o` | `--output-dir <DIR>` | Directory to save generated reports (overrides active configuration).                   |
|      | `--filter <CODE>`    | Filter scan results by specific AES rule code (e.g. `AES101`, `AES301`, `AES401`). |
| `-h` | `--help`             | Print help information for the CLI or a specific subcommand.                            |
| `-V` | `--version`          | Print CLI binary version.                                                               |

### Commands & subcommands

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
  > ~/.local/share/aes-lint-arwaky/reports/scan_rust.sarif
```

### MCP server tools (`lint-arwaky-mcp`)

Five JSON-RPC 2.0 tools over STDIO for AI clients (Claude Code, Cursor, Windsurf, Hermes):

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

### Naming contract (AES101) — shared reference

Every source file must follow `<layer>_<concern>_<role>.<ext>` (lowercase, at least three
underscore-separated words; verbatim exceptions such as `main.rs`, `lib.rs`, `mod.rs`,
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
their own naming convention — see the `aes-testing-suite` skill.

---

## Section Contract

Every shared-command session is required to carry the rows that apply. Each exists for one reason.

| Section | Why it belongs here |
| ------- | ------------------- |
| Invocation form | Orchestrator vs direct binary must be deliberate and reproducible. |
| Alias + global flags | `lac`/`lat`/`lam` and `-o`/`--filter` are the only stable shared surface. |
| Subcommand choice | `scan` vs family commands vs `ci` changes what runs and what exits non-zero. |
| Explicit `--format` | Machine consumers must not parse ad-hoc text. |
| MCP tool table | STDIO clients call `execute_command`/`get_config` by name, not by guesswork. |
| Naming + layer matrix | AES101/AES102 decisions before any fix, shared by all languages. |
| Exit code | CI gates on `0`/`1`/`2`; stdout is not the contract. |

---

## Verify

```bash
# 1. Binary healthy
lint-arwaky-cli doctor
lint-arwaky-cli version

# 2. Scan consumes the shared surface
lint-arwaky-cli scan <target-path> --format json --filter AES201

# 3. Exit code is the gate
echo "exit=$?"   # 0 pass · 1 violations · 2 config/parse error
```

Manual:

- [ ] Invocation matches the orchestrator-or-direct rule (and alias if used).
- [ ] `--format` explicit whenever output is piped or written to a report path.
- [ ] `fix` used `--dry-run` (and `--filter` if scoped) before a real write.
- [ ] Exit code interpreted as the gate (`1` means violations remain — not "done").
- [ ] Language deltas and AES fix routing consulted from the matching HOW-TO, not re-guessed.
