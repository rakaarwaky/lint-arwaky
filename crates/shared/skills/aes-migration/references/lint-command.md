# Lint Rules (Global)

Shared `lint-arwaky-cli` command surface that applies to every language and every scan session. Language deltas live in the matching `references/*-migration.md`.

**Goal:** Give any migration step a single, checkable home for the CLI invocation, AES101 naming contract, fix-routing table, and triage order, so no step restates these shared facts.

## Invocation

```bash
lint-arwaky-cli scan .
lac fix . --dry-run
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

| Option | Long Flag            | Description                                                              |
| :------ | :-------------------- | :------------------------------------------------------------------------ |
| `-v`   | `--verbose`          | Increase log verbosity (count: `-v` info, `-vv` debug).                  |
| `-q`   | `--quiet`            | Minimize console output (suppress non-error messages).                   |
| `-o`   | `--output-dir <DIR>` | Directory to save generated reports (overrides active configuration).    |
|        | `--layer <LAYER>`    | Filter scan output by AES layer: taxonomy, contract, agent, surface, utility, capabilities, logical. Keeps violations whose file basename starts with `<LAYER>_`. |
| `-h`   | `--help`             | Print help information for the CLI or a specific subcommand.             |
| `-V`   | `--version`          | Print CLI binary version.                                                |

## Commands &amp; subcommands

| Command                 | Purpose                                                                                                                                                                                               |
| :----------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `scan` / `check [PATH]` | Run all linters: naming, import, quality, role, orphan, external, , doc, structure. `--format text/json/sarif/junit` (default text), `--rules <CODE>`, `--layer <LAYER>`, `--member <NAME>`, `--group <GROUP>`. |
| `report [PATH]`         | Progress report: counts per member and what changed since the last run.                                                                                                                               |
| `ci [PATH]`             | Quality gate; `--threshold <SCORE>` (default 80, exit 1 below it). No `--format` flag.                                                                                                                |
| `config`                | Show effective architecture config.                                                                                                                                                                   |
| `fix [PATH]`            | Apply safe automatic fixes (AES301-305 only); `--dry-run`.                                                                                                                                            |
| `git [PATH]`            | Scan files changed since git base; `--base <BRANCH>` (default `develop`), `--layer <LAYER>`.                                                                                                               |
| `doctor`                | Environment diagnostics.                                                                                                                                                                              |
| `security [PATH]`       | Security vulnerability scan.                                                                                                                                                                          |
| `dependencies [PATH]`   | Dependency report.                                                                                                                                                                                    |
| `adapters`              | List external lint adapters.                                                                                                                                                                          |
| `init`                  | Create config files + docs in project.                                                                                                                                                                |
| `install`               | Install adapter dependencies; `--sudo`.                                                                                                                                                               |
| `mcp-config <CLIENT>`   | Print MCP client configuration snippet.                                                                                                                                                               |
| `watch [PATH]`          | Watch files and auto-lint on change.                                                                                                                                                                  |
| `install-hook`          | Install git pre-commit hook.                                                                                                                                                                          |
| `uninstall-hook`        | Uninstall git pre-commit hook.                                                                                                                                                                        |
| `version`               | Print version.                                                                                                                                                                                        |
| `update` (alias `la`)   | Self-update: install latest release binary from GitHub; `--check-only`.                                                                                                                               |
| `skill read <NAME>`     | Print the SKILL.md (and optionally references) for the named skill; `--with-references`.                                                                                                              |
| `skill list`            | List all embedded skills.                                                                                                                                                                             |

`--group` values for `scan`/`check`: `all` (default), `quality`, `role`, `import`, `naming`, `orphan`, `external`, `doc`, `structure`.

## Output formats

`text` | `json` | `sarif` | `junit` — applies to `scan`/`check`/group scans via `--format`.
