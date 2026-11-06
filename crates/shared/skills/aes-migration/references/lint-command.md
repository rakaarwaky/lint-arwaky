# Lint Rules (Global)

Shared `lint-arwaky-cli` command surface that applies to every language and every scan session. Language deltas live in the matching `references/*-migration.md`.

**Goal:** Give any migration step a single, checkable home for the CLI invocation, AES101 naming contract, fix-routing table, and triage order, so no step restates these shared facts.

## Invocation

```bash
lint-arwaky-cli scan .
lint-arwaky-cli fix . --dry-run
lint-arwaky-cli ci . --threshold 90
lint-arwaky-cli check . --format json
lint-arwaky-cli ci . --threshold 80
```

## Shell aliases

| Alias | Target Binary     | Description                               |
| :----- | :----------------- | :----------------------------------------- |
| `lac` | `lint-arwaky-cli` | Primary CLI gatekeeper &amp; scanner      |
| `lat` | `lint-arwaky-tui` | Terminal User Interface dashboard         |
| `lam` | `lint-arwaky-mcp` | MCP Server (STDIO backend for AI clients) |

## Global CLI options

| Option | Long Flag   | Description                                                              |
| :------ | :----------- | :------------------------------------------------------------------------ |
| `-v`   | `--verbose` | Increase log verbosity (`-v` → info, `-vv` → debug; scan-phase tracing emits real-time events on `lint_arwaky::audit`). |
| `-h`   | `--help`    | Print help information for the CLI or a specific subcommand.             |
| `-V`   | `--version` | Print CLI binary version.                                                |

## Commands &amp; subcommands

| Command                  | Subcommands / Flags                                                        | Purpose                                                       | Example                                                       |
| :----------------------- | :--------------------------------------------------------------------------- | :-------------------------------------------------------------- | :-------------------------------------------------------------- |
| `scan`                   | `[PATH]`, `--format <text\|json>`, `--rules <RULES>`, `--layer <LAYER>`, `--member <MEMBER>`, `--group <all\|quality\|role\|import\|naming\|orphan\|external\|doc\|structure>` | Run all 8 linter groups; `--group` restricts to one. `PATH` defaults to `.`. | `lint-arwaky-cli scan . --group structure`                     |
| `check`                  | Same flags as `scan` (alias of `scan`)                                      | Run all 8 linter groups (alias of `scan`).                     | `lint-arwaky-cli check . --format json`                         |
| `report`                 | `[PATH]`                                                                     | Progress report: counts per member and what changed since the last run. | `lint-arwaky-cli report .`                                    |
| `ci`                     | `[PATH]`, `--threshold <N>` (default `80`)                                  | CI threshold validation.                                       | `lint-arwaky-cli ci . --threshold 90`                           |
| `config`                 | —                                                                            | Show effective architecture config.                             | `lint-arwaky-cli config`                                        |
| `fix`                    | `[PATH]`, `--dry-run`                                                       | Auto-fix AES301–AES305 violations.                              | `lint-arwaky-cli fix . --dry-run`                               |
| `git`                    | `[PATH]`, `--base <BASE>` (default `develop`), `--filter <FILTER>`          | Scan files changed since git base.                              | `lint-arwaky-cli git . --base develop`                          |
| `doctor`                 | —                                                                            | Environment diagnostics.                                        | `lint-arwaky-cli doctor`                                        |
| `security`               | `[PATH]`                                                                     | Security vulnerability scan.                                     | `lint-arwaky-cli security .`                                    |
| `dependencies`           | `[PATH]`                                                                     | Dependency report.                                              | `lint-arwaky-cli dependencies .`                                |
| `adapters`               | —                                                                            | List external lint adapters.                                     | `lint-arwaky-cli adapters`                                      |
| `init`                   | —                                                                            | Create config files + docs in project.                           | `lint-arwaky-cli init`                                          |
| `install`                | `--sudo`                                                                     | Install adapter dependencies.                                     | `lint-arwaky-cli install`                                       |
| `mcp-config`             | `<CLIENT>`                                                                   | Print MCP client configuration snippet.                          | `lint-arwaky-cli mcp-config claude`                              |
| `watch`                  | `[PATH]`                                                                     | Watch files and auto-lint on change.                             | `lint-arwaky-cli watch .`                                       |
| `install-hook`           | —                                                                            | Install git pre-commit hook.                                     | `lint-arwaky-cli install-hook`                                  |
| `uninstall-hook`         | —                                                                            | Uninstall git pre-commit hook.                                    | `lint-arwaky-cli uninstall-hook`                                 |
| `version`                | —                                                                            | Print version.                                                   | `lint-arwaky-cli version`                                       |
| `update`                 | —                                                                            | Self-update: install latest release binary from GitHub.          | `lint-arwaky-cli update`                                          |
| `skill`                  | `read <NAME>` (print the SKILL.md and optionally references for the named skill), `list` (list all embedded skills) | Read or list embedded AES skill documentation.                  | `lint-arwaky-cli skill list`                                    |
| `help`                   | —                                                                            | Print this message or the help of the given subcommand(s).        | `lint-arwaky-cli help`                                            |

## AES101 — File Naming

AES101 requires every non-entry Rust source file's basename to start with the exact layer prefix, followed by `_`. A file whose first identifier is `_` does not satisfy AES101.

**Canonical prefix → path mapping** (the first identifier of the file must equal the first word of the target path):

| Layer prefix | Valid target paths                                        |
| :----------- | :--------------------------------------------------------- |
| `agent_`     | `crates/auto-fix`, `crates/dispatcher`, `crates/mcp-server`, `crates/project-setup`, `crates/report-formatter`, `crates/tui` |
| `contract_`  | `crates/shared/src/contract`                                |
| `root_`      | crate root files: `lib.rs`, `main.rs`, `cli.rs`             |
| `surface_`   | `crates/cli-commands`, `crates/dispatcher`, `crates/mcp-server`, `crates/project-setup`, `crates/report-formatter`, `crates/tui` |
| `taxonomy_`  | `crates/shared/src/taxonomy`                                |
| `utility_`   | `crates/auto-fix`, `crates/config-system`, `crates/file-watch`, `crates/filesystem`, `crates/git-hooks`, `crates/logging` |
| `capabilities_` | `crates/doc-rules`, `crates/import-rules`, `crates/maintenance`, `crates/naming-rules`, `crates/orphan-rules`, `crates/quality-rules`, `crates/role-rules`, `crates/structure-rules` |

### AES101 naming examples

| Status | File name            | Why                                                                                          |
| :----- | :------------------- | :------------------------------------------------------------------------------------------- |
| Valid  | `agent_dispatch.rs`  | Starts with `agent_`; path is `crates/dispatcher/`                                            |
| Invalid| `utils_parser.rs`    | First identifier is `utils` (no underscore boundary, no layer) — must start with an exact layer prefix |
| Invalid| `agent_parser.rs` in `crates/report-formatter/` | Starts with `agent_` but path is report-formatter, which is not an agent target path |
| Valid  | `lib.rs`             | Crate root entry point; exempt from the prefix rule                                          |

## Fix route table

| AES code | Group  | Fix target layer / path                                   | Typical fix                                                                                                   |
| :------- | :----- | :---------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| `AES101` | naming | `crates/` module file names                                 | Rename the file so the basename starts with the exact layer prefix; update every import path that references the old name |
| `AES2xx` | import | `crates/shared/src/contract`, `crates/shared/src/taxonomy` | Move the shared symbol to its contract or taxonomy module; update imports to the new path                     |
| `AES3xx` | role   | `crates/dispatcher`, `crates/tui`, `crates/mcp-server`, `crates/cli-commands`, `crates/report-formatter` | Route the logic through the owning layer (root → surface → agent → capabilities); keep the public surface stable |
| `AES4xx` | import | `crates/` module `mod` declarations                         | Add the missing `mod` declaration, or move the module to the correct crate                                     |
| `AES5xx` | quality| `crates/` source files                                      | Fix dead code, unused imports, TODO/FIXME, and naming violations                                               |
| `AES6xx` | doc    | `docs/`, `ROADMAP.md`, `PRD.md`                            | Restore the required section heading, or remove the stale section                                             |
| `AES7xx` | struct | `docs/` folder layout                                       | Move the file to the required folder, or add the missing folder                                                |

Fix every row before finishing the step. After a row is marked Done, re-run the scan and confirm that count is 0.

## Triage order

1. **AES6xx + AES7xx first.** Docs and structure failures are cheap, high-confidence, and unblock the doc-rules gate before code is touched.
2. **AES1xx–AES4xx second.** Rename and rewire naming, import, and role failures while the module graph is still small.
3. **AES5xx last.** Quality findings (dead code, TODOs, style) tend to be noise; clean them after the module graph is stable so the count does not inflate during renames.

## Scan &amp; fix contract

1. **Locate the violation.** Use `fix target` in the table above, or run `lint-arwaky-cli scan . --format json` and read each object's `fix_target` and `fix_hint` fields.
2. **Route to the fix skill.** Read `crates/shared/skills/aes-fix/references/<CODE>.md`. Run its step files in order.
3. **Apply the fix.** Edit the target file; keep public API stable.
4. **Verify.** Re-run `lac fix . --dry-run` and `lac ci . --threshold 90`. The code's count must reach 0 before the next step starts.

## Scan &amp; fix loop

```bash
while true; do
  OUT=$(lac scan . --format json 2>&1)
  CODE=$(lac extract-code "$OUT")   # first AESxxxx code, or "none"
  if [ "$CODE" = "none" ]; then break; fi
  sed -i "s/^| $CODE |.*$/| $CODE | Done (2026-03-29, auto) |" ROADMAP.md
  # Read and apply the fix-skill step files for $CODE, re-scan, and stop on zero findings.
done
```
