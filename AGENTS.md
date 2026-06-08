# AGENTS.md — Lint Arwaky

## Build & dev

```bash
# Build everything
cargo build --release

# Self-lint — project audits itself under AES rules (no path needed)
cargo run --bin lint-arwaky-cli -- check .

# Scan — lint external/test project (runs AES + clippy/ruff/eslint/tsc/...)
cargo run --bin lint-arwaky-cli -- scan <project-path>

# Run MCP server (JSON-RPC 2.0 over stdin/stdout)
cargo run --bin lint-arwaky-mcp

# Tests
cargo test --workspace        # all
cargo test --lib <name_fragment>  # single test by name

# Format & lint
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

## Architecture (6-layer AES + Vertical Slicing)

The codebase uses **6 architectural layers** as file prefixes, organized into **26 feature folders** (vertical slicing):

| Layer (prefix) | Allowed suffixes |
| -------------- | ---------------- |
| `taxonomy_`    | `_vo`, `_entity`, `_event`, `_error`, `_constant` |
| `contract_`    | `_port`, `_protocol`, `_aggregate` |
| `capabilities_` | `_checker`, `_analyzer`, `_processor`, etc. |
| `infrastructure_` | `_adapter`, `_provider`, `_scanner`, etc. |
| `agent_`       | `_container`, `_orchestrator`, `_coordinator`, `_registry`, `_manager` |
| `surface_`     | `_command`, `_handler`, `_controller` |

### Feature folders

```
src-rust/
  layer-rules/       — Import, compliance, cycle, self-lint rules
  role-rules/        — Unused, inheritance, bypass rules
  orphan-detector/   — Orphan code detection
  primitive-checker/ — Primitive obsession (AES006)
  cli-commands/      — CLI command surfaces
  cli-transport/     — CLI execution transport
  config-system/     — Config loading & parsing
  pipeline-jobs/     — Jobs, dispatcher, execution
  naming-rules/      — Naming convention & variants
  semantic-analysis/ — Data flow, scope, tracer
  file-watch/        — File watching
  git-hooks/         — Git hooks management
  multi-project/     — Multi-project governance
  project-setup/     — Project init, doctor, mcp-config
  plugin-system/     — Plugin discovery & management
  output-report/     — Output formatting & report generation
  code-analysis/     — Code analysis (linting, data flow)
  mcp-server/        — MCP server
  source-parsing/    — Source code parsing
  lifecycle-state/   — Agent lifecycle management
  language-adapters/ — Python, JS, Rust adapters
  di-containers/     — DI container aggregates
  file-system/       — File system abstraction
  http-client/       — HTTP client
  metrics-service/   — Metrics provider
  shared-common/     — Shared value objects & errors
```

Import flow: `surface_` → `agent_` → `capabilities_` / `infrastructure_` → `contract_` → `taxonomy_`.
Surfaces must NOT import infrastructure/capabilities directly — they go through `ServiceContainerAggregate` trait (AES023).

AES rules enforced: 31 codes AES001–AES033 (AES028/029 reserved). Config: `lint_arwaky.config.rust.yaml`.

## Key conventions

- Filenames: exactly `[layer]_[concept]_[suffix].rs` (AES003)
- Every logic file must define a struct implementing a contract trait (AES009, AES027)
- `#[allow(...)]`, `unwrap()`, `panic!` are forbidden (AES014)
- `_constant` files may only contain `pub const` / `pub static` (AES033)
- No bypass: `noqa`, `type: ignore` flagged
- Score: starts at 100, deducts per finding. Any CRITICAL = fail regardless.

## Testing with test projects

```bash
cargo run --bin lint-arwaky-cli -- scan test-project-rust/
cargo run --bin lint-arwaky-cli -- scan test-project-python/
cargo run --bin lint-arwaky-cli -- scan test-project-javascript/
```

Each contains intentional violations. See `docs/TEST.md` for pass/fail criteria.

## VCS: jj (Jujutsu) — always use instead of git

This repo uses `jj` (Jujutsu) as its VCS. Do NOT use `git` directly.

```bash
jj status            # working copy state
jj log               # commit history
jj describe          # edit commit message
jj new               # create new change on top
jj squash            # merge changes into parent
jj bookmark list     # list bookmarks (branches)
jj bookmark set <name>  # create/update a bookmark
jj git push          # push bookmarks to remote
jj git fetch         # fetch from remote
```

`jj` interoperates with the git remote — the `.jj/` directory replaces `.git/` for local operations, but `jj git push` pushes to the configured git remote.

## Branch management (CRITICAL — must follow)

Allowed branch naming:
- `main`
- `develop`
- `features/<name>` (plural `features/`, NOT `feature/`)

When merging a PR to develop:
- Use `gh pr merge <num> --squash` ONLY
- **NEVER use `--delete-branch`** — feature branches must NOT be deleted after merge
- Branches that were accidentally deleted must be restored immediately via:
  ```bash
  git branch <branch-name> origin/develop
  jj git import
  jj bookmark set <branch-name>
  ```
