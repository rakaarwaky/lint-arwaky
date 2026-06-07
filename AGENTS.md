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

## Layer-gated compilation (unique Cargo features)

`src-rust/lib.rs` conditionally compiles modules by feature. Each `scripts/check_*.sh` invokes one layer:

```
check_taxonomy.sh     →  cargo check --lib --no-default-features --features check_taxonomy
check_contract.sh     →  cargo check --lib --no-default-features --features check_contract
check_infrastructure.sh  →  cargo check --lib --no-default-features --features check_infrastructure
check_capabilities.sh    →  cargo check --lib --no-default-features --features check_capabilities
check_agent.sh        →  cargo check --lib --no-default-features --features check_agent
check_surfaces.sh     →  cargo check --lib --no-default-features --features check_surfaces
```

Feature chain: `check_taxonomy` → `check_contract` → `check_infrastructure` / `check_capabilities` → `check_agent` → `check_surfaces` (default).

## Architecture (6-layer AES)

Layer ordering (bottom→top) and allowed filename suffixes:

| Layer | Dir | Allowed suffixes |
|---|---|---|
| taxonomy | `src-rust/taxonomy/` | `_vo`, `_entity`, `_event`, `_error`, `_constant` |
| contract | `src-rust/contract/` | `_port`, `_protocol`, `_aggregate` |
| capabilities | `src-rust/capabilities/` | `_checker`, `_analyzer`, `_processor`, etc. |
| infrastructure | `src-rust/infrastructure/` | `_adapter`, `_provider`, `_scanner`, etc. |
| agent | `src-rust/agent/` | `_container`, `_orchestrator`, `_coordinator`, `_registry`, `_manager` |
| surfaces | `src-rust/surfaces/` | `_command`, `_handler`, `_controller` |

Import flow: surfaces → agent → capabilities/infrastructure → contract → taxonomy.
Surfaces must NOT import infrastructure/capabilities directly — they go through `ServiceContainerAggregate` trait (AES023).

AES rules enforced: 31 codes AES001–AES033 (AES028/029 reserved). Config: `lint_arwaky.config.rust.yaml`.

## Key conventions

- Filenames: exactly 3-word snake_case with layer suffix (AES003)
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

## Stale files (do not trust)

- `.github/workflows/ci.yml` and `publish.yml` — leftover from Python v1.9.x, not active
- `.vscode/settings.json` — references Python/JS configs from old project structure
