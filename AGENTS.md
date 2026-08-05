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
just gates                                  # same via justfile
cargo nextest run --workspace --lib --tests # all tests, 3× faster
just test-fast                              # same via justfile
```

## Self-lint (must be clean — 0 violations)

```bash
cargo run --bin lint-arwaky-cli -- scan .   # runs ALL 6 linters on own codebase
just self-lint                              # same via justfile
```

## Scan test projects

Test workspaces contain intentional violations (`workspaces-bad/`) and clean files (`workspaces-good/`). Language is auto-detected from file extensions — no flag needed.

```bash
# Bad workspaces (should find violations)
cargo run --bin lint-arwaky-cli -- scan workspaces-bad/crates   # Rust
cargo run --bin lint-arwaky-cli -- scan workspaces-bad/modules  # Python
cargo run --bin lint-arwaky-cli -- scan workspaces-bad/packages # JS/TS

# Good workspaces (should find 0 violations — false positive test)
cargo run --bin lint-arwaky-cli -- scan workspaces-good/crates
cargo run --bin lint-arwaky-cli -- scan workspaces-good/modules
cargo run --bin lint-arwaky-cli -- scan workspaces-good/packages

# Individual rule surfaces
cargo run --bin lint-arwaky-cli -- naming  workspaces-bad/crates
cargo run --bin lint-arwaky-cli -- import  workspaces-bad/crates
cargo run --bin lint-arwaky-cli -- orphan  workspaces-bad/crates
```

## MCP server & TUI

```bash
cargo run --bin lint-arwaky-mcp   # MCP server (stdin/stdout JSON-RPC 2.0)
cargo run --bin lint-arwaky-tui   # TUI file browser
```

---

## Architecture: AES 7-Layer System

Every file in the codebase belongs to one of 7 layers. The layer is identified by the filename prefix and must follow strict naming, dependency, and role rules.

| Layer | Prefix | Purpose | Dependencies |
| --- | --- | --- | --- |
| **Taxonomy** | `taxonomy_` | Domain foundation: VOs, entities, errors, events, constants | Nothing |
| **Contract** | `contract_` | Public interfaces (protocols) and facades (aggregates) | Taxonomy only |
| **Utility** | `utility_` | Stateless technical functions (parsers, resolvers, builders) | Taxonomy only |
| **Capabilities** | `capabilities_` | Business logic + external adaptation (implements protocols) | Taxonomy, Contract, Utility |
| **Agent** | `agent_` | Orchestration: sequences capabilities into flows | Taxonomy, Contract, Utility |
| **Surface** | `surface_` | Entry points: commands, controllers, pages, views | Contract only |
| **Root** | `root_` | Composition: wires capabilities to contracts, starts app | All layers |

**Dependency rule:** Unidirectional bottom-up. Higher layers import lower; never the reverse.

---

## Naming Convention

Every file must follow: `layer_concern_role.<ext>`

Examples: `capabilities_user_checker.rs`, `utility_path_resolver.py`, `contract_scan_protocol.ts`

Full suffix rules per layer are in [RULES_AES.md](.agents/rules/RULES_AES.md) (AES101–AES102).

---

## Workspace Packages Structure

| Directory | Language | Count |
| --- | --- | --- |
| `crates/` | Rust | 18 workspace members |
| `packages/` | TypeScript/JS | (see `packages/src/`) |
| `modules/` | Python | (see `workspaces-*/modules/` for tests) |

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
- `create-taxonomy-*` — Create taxonomy layer files
- `create-contract-*` — Create contract layer files
- `create-utility-*` — Create utility layer files
- `create-capabilities-*` — Create capabilities layer files
- `create-agent-*` — Create agent layer files
- `create-surface-*` — Create surface layer files
- `create-root-*` — Create root layer files

**Maintenance skills:**
- `fix-bypass-*` — Fix bypass comments (`unwrap`, `#[allow]`, `noqa`, etc.)
- `cleanup-consolidate-*` — Remove dead code, merge duplicates
- `add-docs-*` — Add docstrings, type hints, crate-level docs
- `create-test-*` — Generate test suites
- `lint-arwaky-*` — Run scan and fix violations

**Role workflow pipeline:**
`Architect` → `Business Analyst` → `Tech Lead` → `Fullstack Developer`

1. `role-architect` — Reviews layer boundaries, naming, orphans, scalability
2. `role-business-analyst` — Reviews requirements, business flow, testability
3. `role-tech-lead` — Reviews security, performance, error handling, SOLID
4. `role-fullstack-developer` — Executes plans, implements fixes, verifies

Plan files are saved to `.agents/plans/`.

---

## Branch Management

Allowed branch naming: `main`, `develop`

When merging a PR to develop:
- **use `--delete-branch`** — for feature/fix branches after merge
- **do NOT delete `develop`** branch after merge to `main`

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
- **Capabilities never import other Capabilities** — compose via Agent orchestration only.
