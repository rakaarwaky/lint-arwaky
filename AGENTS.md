# AGENTS.md — Lint Arwaky

Read before making any changes to the codebase.
Make sure to read [TEST.md](TEST.md) for pass/fail criteria before committing any changes.

---

## Project Overview

**Lint Arwaky** is an architecture linter for Rust, Python, and TypeScript that enforces the [Agentic Engineering System (AES)](ARCHITECTURE.md) — a 7-layer architecture with 24 rules across 5 groups. The project itself is written in Rust and is self-auditing (it passes its own lint rules).

**Key docs:**

| Document | Purpose |
| --- | --- |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Full 7-layer AES spec, naming conventions, layer rules |
| [PRD.md](PRD.md) | Product requirements, feature map, exit codes |
| [TEST.md](TEST.md) | Test workspaces, pass/fail criteria, expected violation counts |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Setup, code style, PR process |
| [RULES_AES.md](.agents/rules/RULES_AES.md) | All 24 AES rules with severities and descriptions |

---

## Build & dev

```bash
# Build everything
CARGO_INCREMENTAL=0 cargo build --release

# Faster local dev (uses sccache cache)
CARGO_INCREMENTAL=0 cargo check -p <crate>   # type-check only
CARGO_INCREMENTAL=0 cargo clippy -p <crate>  # lint only
cargo nextest run -p <crate>                # tests (3× faster than cargo test)

# Per-crate build/check/test (with sccache)
CARGO_INCREMENTAL=0 cargo build -p import_rules_lint_arwaky
CARGO_INCREMENTAL=0 cargo check -p naming_rules_lint_arwaky
cargo nextest run -p code_analysis_lint_arwaky --lib --tests
```

## Format & lint

```bash
cargo fmt --all
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings
CARGO_INCREMENTAL=0 cargo clippy -p import_rules -- -D warnings  # per crate
```

## Quality gates (run before every commit)

```bash
bash scripts/gates.sh                       # fmt + clippy + self-lint + tests
cargo nextest run --workspace --lib --tests # all tests, 3× faster
```

## Self-lint 

Binary Path
/home/user/.cargo/bin/lint-arwaky-cli

```bash
lint-arwaky-cli scan .   # runs ALL 6 linters on own codebase
```

## Scan test projects

Test workspaces contain intentional violations (`workspaces-bad/`) and clean files (`workspaces-good/`). Language is auto-detected from file extensions — no flag needed.

```bash
# Bad workspaces (should find violations)
lint-arwaky-cli scan workspaces-bad/crates   
lint-arwaky-cli scan workspaces-bad/modules  
lint-arwaky-cli scan workspaces-bad/packages 

# Good workspaces (should find 0 violations)
lint-arwaky-cli scan workspaces-good/crates
lint-arwaky-cli scan workspaces-good/modules
lint-arwaky-cli scan workspaces-good/packages

```

## MCP server & TUI

```bash
lint-arwaky-mcp   # MCP server (stdin/stdout JSON-RPC 2.0)
lint-arwaky-tui   # TUI file browser
```

---

## Architecture: AES 7-Layer System

Every file in the codebase belongs to one of 7 layers. The layer is identified by the filename prefix and must follow strict naming, dependency, and role rules.

See [ARCHITECTURE.md](ARCHITECTURE.md) for full details.
---

## Naming Convention

Every file must follow: `layer_concern_role.<ext>`

Examples: `capabilities_user_checker.rs`, `utility_path_resolver.py`, `contract_scan_protocol.ts`

Full suffix rules per layer are in [RULES_AES.md](.agents/rules/RULES_AES.md) (AES101–AES102).

---

## Workspace Packages Structure

| Directory | Language |
| --- | --- | --- |
| `crates/` | Rust | 
| `packages/` | TypeScript/JS | 
| `modules/` | Python | 

Key crates:

| Crate | Responsibility |
| --- | --- |
| `shared` | Taxonomy VOs, contracts, utilities |
| `config-system` | Config load, merge, workspace detect |
| `filesystem` | File walking, AST parsing (tree-sitter), graph construction |
| `naming-rules` | AES101–102 naming conventions |
| `import-rules` | AES201–205 import boundaries |
| `quality-rules` | AES301–305 quality rules (was `code-analysis`) |
| `role-rules` | AES401–406 layer roles |
| `orphan-rules` | AES501–506 orphan detection (was `orphan-detector`) |
| `auto-fix` | Mechanical fixes: remove + replace + rename |
| `external-lint` | Clippy, Rustfmt, cargo-audit, Ruff, MyPy, Bandit, ESLint, Prettier, TSC adapters |
| `report-formatter` | text, JSON, SARIF 2.1.0, JUnit XML output |
| `cli-commands` | Human CLI surface (`check`, `scan`, `fix`, `ci`) |
| `mcp-server` | AI MCP surface (5 tools, full CLI parity) |
| `git-hooks` | Pre-commit architecture enforcement |
| `file-watch` | Continuous linting on file changes |
| `project-setup` | `init` / `install` / `mcp-config` |
| `maintenance` | `doctor` / `security` / `deps` |
| `tui` | Interactive terminal UI |

---

## Skills & Roles

The `.agents/skills/` directory contains skill definitions for AI-assisted development. Use them via trigger keywords:

**Layer creation skills** (one per language: `rust`, `python`, `typescript`):
- `.agents/skills/create-taxonomy-*` — Create taxonomy layer files
- `.agents/skills/create-contract-*` — Create contract layer files
- `.agents/skills/create-utility-*` — Create utility layer files
- `.agents/skills/create-capabilities-*` — Create capabilities layer files
- `.agents/skills/create-agent-*` — Create agent layer files
- `.agents/skills/create-surface-*` — Create surface layer files
- `.agents/skills/create-root-*` — Create root layer files

**Maintenance skills:**
- `.agents/skills/fix-bypass-*` — Fix bypass comments (`unwrap`, `#[allow]`, `noqa`, etc.)
- `.agents/skills/cleanup-consolidate-*` — Remove dead code, merge duplicates
- `.agents/skills/add-docs-*` — Add docstrings, type hints, crate-level docs
- `.agents/skills/create-test-*` — Generate test suites
- `.agents/skills/lint-arwaky-*` — Run scan and fix violations

**Role workflow pipeline:**
`Architect` → `Business Analyst` → `Tech Lead` → `Fullstack Developer`

1. `Architect` — Reviews layer boundaries, naming, orphans, scalability
2. `Business Analyst` — Reviews requirements, business flow, testability
3. `Tech Lead` — Reviews security, performance, error handling, SOLID
4. `Fullstack Developer` — Executes plans, implements fixes, verifies

Plan files are saved to `.agents/plans/`.

---

## Branch Management

Allowed branch naming: `main`, `develop`

When merging a PR to develop:
- **use `--delete-branch`** — for feature/fix branches after merge
- **do NOT delete `develop`** branch after merge to `main`

**Worktree policy (important):**
- When working on a feature/fix branch, **use a git worktree** under `.worktree/` (e.g. `<repo-root>/.worktree/feature-name`) instead of switching branches in the current checkout with `git checkout`.

---

## Exit Code Contract

| Code | Name | When |
| --- | --- | --- |
| `0` | Ok | Success, clean scan, doctor finished |
| `1` | Policy fail | Violations found, CI threshold failed |
| `2` | Runtime error | Path missing, invalid args, I/O failure |
| `3` | Prerequisite missing | Required external tool not installed |

See [PRD.md](PRD.md#exit-code-contract) for full details.

---

## Pitfalls

- **`CARGO_INCREMENTAL=0`** is required for reproducible builds and in the gates script. Only omit it for quick local edits.
- **Self-lint must pass** — `check .` must produce 0 violations before committing.
- **`workspaces-good/` must produce 0 violations** — any violation is a false positive that must be fixed.
- **tree-sitter** is the only AST parser — no regex fallback. All language parsing goes through `filesystem` crate.
- **No async runtime** — the project uses `std::thread` / `rayon`, not tokio. Do not introduce async.
