# AGENTS.md — Lint Arwaky

Read before making any changes to the codebase.
Make sure to read [TEST.md](TEST.md) for pass/fail criteria before committing any changes.
See also [ARCHITECTURE.md](ARCHITECTURE.md) for the full 7-layer specification.

## Build & dev

````bash
_# Build everything
cargo build --release

# Self-lint
cargo run --bin lint-arwaky-cli -- check

# Scan —
cargo run --bin lint-arwaky-cli -- scan <project-path>

# Run MCP server
cargo run --bin lint-arwaky-mcp

# Run TUI launcher
cargo run --bin lint-arwaky-tui

# Per-crate build/check/test
cargo build -p import_rules_lint_arwaky
cargo check -p naming_rules_lint_arwaky
cargo test -p code_analysis_lint_arwaky

# Tests
cargo test --workspace        # all
cargo test -p import_rules_lint_arwaky    # single crate
cargo test --lib <name_fragment>  # single test by name

## Testing with test projects

```bash
cd /home/raka/mcp-arwaky/lint-arwaky
cargo run --bin lint-arwaky-cli -- scan test-workspaces/crates for rust
cargo run --bin lint-arwaky-cli -- scan test-workspaces/modules for python
cargo run --bin lint-arwaky-cli -- scan test-workspaces/packages for typescript
````

# Format & lint

cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo clippy -p import_rules -- -D warnings # per crate

```

## Architecture (7-layer AES + Vertical Slicing + Multi-Crate Workspace)

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full layer specifications, naming conventions, and concrete examples.

### Workspace Packages Structure

See [README.md](README.md) for the feature crate overview.

## Branch management (CRITICAL — must follow)

Allowed branch naming:

- `main`
- `develop`

When merging a PR to develop:

- ** use `--delete-branch`** — for feature / fix branches after merger
- **do NOT be deleted `develop` branch ** after merge to `main`
```
