# AGENTS.md — Lint Arwaky

Read before making any changes to the codebase.
Make sure to read [TEST.md](TEST.md) for pass/fail criteria before committing any changes.


## Build & dev

```bash
# Build everything
CARGO_INCREMENTAL=0 cargo build --release

# Faster local dev (uses sccache cache)
CARGO_INCREMENTAL=0 cargo check -p <crate>   # type-check only
CARGO_INCREMENTAL=0 cargo clippy -p <crate>  # lint only
cargo nextest run -p <crate>                # tests (3× faster than cargo test)

# Self-lint (use default incremental for speed on small edits)
cargo run --bin lint-arwaky-cli -- scan .

# Scan
cargo run --bin lint-arwaky-cli -- scan <project-path>

# Run MCP server
cargo run --bin lint-arwaky-mcp

# Run TUI launcher
cargo run --bin lint-arwaky-tui

# Per-crate build/check/test (with sccache)
CARGO_INCREMENTAL=0 cargo build -p import_rules_lint_arwaky
CARGO_INCREMENTAL=0 cargo check -p naming_rules_lint_arwaky
cargo nextest run -p code_analysis_lint_arwaky --lib --tests

# Tests (gates script already sets CARGO_INCREMENTAL=0)
bash scripts/gates.sh                    # all gates
cargo nextest run --workspace --lib --tests  # all tests, 3× faster
cargo test -p import_rules_lint_arwaky          # fallback per crate
cargo test --lib <name_fragment>                # single test by name
```

## Format & lint

```bash
cargo fmt --all
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings
CARGO_INCREMENTAL=0 cargo clippy -p import_rules -- -D warnings  # per crate
```

## Testing with test projects

```bash
cd /home/raka/mcp-arwaky/lint-arwaky
cargo run --bin lint-arwaky-cli -- scan workspaces-bad/crates for rust
cargo run --bin lint-arwaky-cli -- scan workspaces-bad/modules for python
cargo run --bin lint-arwaky-cli -- scan workspaces-bad/packages for typescript
```

## Format & lint

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo clippy -p import_rules -- -D warnings # per crate
```

## Architecture 

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full layer specifications, naming conventions, and concrete examples.

### Workspace Packages Structure

See [README.md](README.md) for the feature crate overview.

## Branch management 

Allowed branch naming:

- `main`
- `develop`

When merging a PR to develop:

- **use `--delete-branch`** — for feature / fix branches after merger
- **do NOT delete `develop` branch** after merge to `main`
