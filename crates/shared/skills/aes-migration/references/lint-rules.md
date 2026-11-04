# Lint Rules (Global)

Shared `lint-arwaky-cli` command surface that applies to every language and every scan session. Language deltas live in the matching `references/*-migration.md`.

**Goal:** Give any migration step a single, checkable home for the CLI invocation, AES101 naming contract, fix-routing table, and triage order, so no step restates these shared facts.

## Invocation

```bash
# Through the agents-arwaky orchestrator (preferred)
aa tool run lint scan .
aa tool run lint fix .
aa tool run lint ci . --threshold 80

# Equivalent direct-binary forms (also reachable as `lac`)
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

## Naming contract (AES101) — shared reference

Every source file must follow `<layer>_<concern>_<role>.<ext>` (lowercase, at least three underscore-separated words; verbatim exceptions such as `main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, `root_cli_main_entry.rs`).

The 7 strictly-ordered layers, **bottom-up** (a layer may only depend on layers below it):

| # | Layer prefix | Purpose | Example |
| - | ------------ | ------- | ------- |
| 1 | `taxonomy_` | value objects, entities, errors, events, constants | `taxonomy_user_vo.py` |
| 2 | `utility_` | pure, stateless helpers | `utility_hash_helper.py` |
| 3 | `contract_` | protocols, aggregates (no implementation) | `contract_payload_protocol.py` |
| 4 | `capabilities_` | domain logic and business rules | `capabilities_task_executor.py` |
| 5 | `agent_` | orchestration across subsystems | `agent_billing_orchestrator.rs` |
| 6 | `surface_` | CLI / MCP / REST / TUI / GUI adapters | `surface_cli_adapter.py` |
| 7 | `root_` | entry points and composition containers | `root_main_entry.py` |

**Core invariant:** upper layers may only depend downward. Lower layers may never import upper layers. **`test` is not a layer.** Test and benchmark files live in `tests/` and `benches/` and follow their own naming convention — see the `aes-testing-suite` skill.

## AES fix routing (language-agnostic)

Every AES code from `lint-arwaky-cli scan` maps to a fix approach and the `aes-*` skill that owns that layer. `{layer}` ∈ `taxonomy`, `utility`, `contract`, `capabilities`, `agent`, `surface`, `root`.

| Rule (name) | What it checks | Fix approach | Skill to use |
| ----------- | -------------- | ------------ | ------------ |
| AES101 (Naming Convention) | Filename is `<layer>_<concern>_<role>.<ext>`, lowercase, ≥3 words | Rename, then update barrel/`mod.rs`/`__init__.py` | `aes-{layer}` |
| AES102 (Suffix/Prefix Rules) | Final suffix is legal for the layer (matrix below) | Change suffix (and therefore the role) to a legal one | `aes-{layer}` |
| AES201 (Forbidden Import) | A layer imports a layer it must not (matrix below) | Remove the cross-layer import; depend on a contract protocol/aggregate injected via DI | `aes-contract` |
| AES202 (Mandatory Import) | A required import is missing (e.g. capabilities must import `contract(*_protocol)`) | Add the required import/type | `aes-{layer}` |
| AES203 (Unused Import) | Imported symbol never used | `lint-arwaky-cli fix <path> --filter AES203` | — |
| AES204 (Dummy Import) | Import kept only to satisfy a linter, with stub usage | Remove the dummy import and its stub; delete the unused symbol | `aes-{layer}` |
| AES205 (Circular Import) | Two files/layers import each other | Extract the shared type or trait downward into `taxonomy` or `contract`; both sides import that | `aes-contract` |
| AES301 (File Maximum Limit) | File exceeds the layer's max lines | Split by responsibility; extract shared logic to `aes-utility` if duplicated | `aes-{layer}`, `aes-utility` |
| AES302 (File Minimum Limit) | File is below the minimum size | Merge into the parent module or delete the file | `aes-{layer}` |
| AES303 (Mandatory Definition) | File has no class/struct/enum/trait of its own (skipped for `utility`/`surface` layers) | Add the definition the filename promises | `aes-{layer}` |
| AES304 (Bypass Comment) | `noqa`, `type: ignore`, `#[allow]`, `@ts-ignore`, `eslint-disable` … | Auto-fixable, or fix the root cause and delete the suppression | `fix --filter AES304` or `aes-{layer}` |
| AES305 (Duplication Code) | Same logic repeated across files | Extract shared logic into a pure utility | `aes-utility` |
| AES401 (Taxonomy Role) | `_constant` purity; primitives in `_entity`/`_error`/`_event` | Replace primitives with taxonomy VOs | `aes-taxonomy` |
| AES402 (Contract Role) | Contract trait/method signatures use primitives instead of taxonomy VO/constant; or a contract file contains a concrete class with real method bodies | Replace primitives with VO/constant types; or move concrete classes out of the contract file into `utility_*` | `aes-contract` |
| AES403 (Capabilities Role) | >3 type declarations; no implementor of the capability protocol | Implement the protocol; split routing across capabilities | `aes-capabilities` |
| AES404 (Utility Role) | Utility must be stateless standalone functions and may import taxonomy only | Move stateful logic to capabilities; drop non-taxonomy imports | `aes-utility` |
| AES405 (Agent Role) | Agent: >3 types, no aggregate implementor, `Any` annotations, direct capabilities import, state outside the constructor | Depend on `contract(*_aggregate)` and delegate to capabilities via protocols | `aes-agent` |
| AES406 (Surface Role) | Surface >15 functions; business logic in a passive surface; role-boundary breach | Move logic down a layer; keep passive surfaces declarative | `aes-surface` |
| AES501–506 (Orphan per layer) | A taxonomy/utility/contract/capabilities/agent/surface file nothing imports | Wire into the root container, or delete the dead file | `aes-{layer}`, `aes-root` |

**Triage order:** 🔴 CRITICAL AES201, AES205, AES304 → 🟡 HIGH AES101–102, AES202, AES301–303, AES401–403, AES406, AES505–506 → 🟢 MEDIUM/LOW AES203–204, AES305, AES404–405, AES501–504.

## Completion

- [`references/python-migration.md`](python-migration.md) cites this reference for shared lint commands, AES101 naming, and fix routing.
- [`references/rust-migration.md`](rust-migration.md) cites this reference for shared lint commands, AES101 naming, and fix routing.
- [`references/typescript-migration.md`](typescript-migration.md) cites this reference for shared lint commands, AES101 naming, and fix routing.
