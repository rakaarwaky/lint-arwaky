# Lint Arwaky — Rust

The shared command reference, global options, MCP tools, workflow, exit codes and the full
AES101–AES506 fix table live in `../SKILL.md` and `rule-routing.md`. This file keeps only what
differs for Rust.

## Adapters & native tooling

`lint-arwaky-cli install` installs the external adapters; `lint-arwaky-cli adapters` lists the
active ones (Clippy, Rustfmt, etc.); `lint-arwaky-cli external crates/` runs only clippy.
`get_config` accepts `"language": "rust"`.

```bash
# Auto-format Rust code
cargo fmt --all

# Check Clippy lints
cargo clippy --all-targets -- -D warnings

# Per-crate build/check/test
cargo check -p <crate-name>
cargo test -p <crate-name>
cargo test --workspace
```

## Workspace layout & `--member`

Rust workspaces are laid out as `workspaces-bad/crates/<crate>/src/…`, so path examples use
`crates/`, and `--member` targets a single **workspace member / package** name. `scan`, `ci`,
`orphan`, `quality`, `import`, `naming`, `role`, `external` all accept paths; `--member` is
accepted by `scan` and by `orphan`.

```bash
lint-arwaky-cli scan workspaces-bad/crates                       # basic scan (text)
lint-arwaky-cli scan workspaces-bad/crates --format json
lint-arwaky-cli scan workspaces-bad/crates --member shared       # single workspace member
lint-arwaky-cli scan workspaces-bad/crates --filter AES401       # by rule ID
lint-arwaky-cli ci crates/ --threshold 80 --format junit
lint-arwaky-cli naming crates/
lint-arwaky-cli orphan crates/ --member shared_common --format json
lint-arwaky-cli import crates/code_analysis                      # one path only
lint-arwaky-cli role crates/
lint-arwaky-cli security crates/                                 # cargo audit style CVEs
lint-arwaky-cli dependencies crates/
lint-arwaky-cli watch crates/
lint-arwaky-cli scan crates/ --format json > ~/.local/share/lint-arwaky/reports/scan_rust.json
lint-arwaky-cli scan crates/ --format sarif > ~/.local/share/lint-arwaky/reports/scan_rust.sarif
lint-arwaky-cli scan .    # self-lint for the lint-arwaky project itself
```

## Rust-specific fix routing

- AES101 rename → after `git mv`, update the `mod.rs` references (a module not declared in
  `mod.rs` is unreachable and reports as an orphan, AES501–506).
- AES205 circular import → extract the shared trait/type downward and have both sides import it
  (see recipe below); route the new interface through `create-contract`.
- AES303 mandatory definition → add the missing `struct` / `enum` / `trait`.
- AES304 bypass → remove `#[allow(...)]`, and replace `unwrap()` / `expect()` / `panic!()` with
  real error handling.
- AES403/405 protocol enforcement: capability `impl`s must target the `_protocol` trait; agent
  structs hold `Arc<dyn Trait>` aggregates and never `use` capabilities directly.
- AES404 (Utility Role): utility modules are free functions only — no `impl` blocks for
  contracts, no state, and no `use` of any layer but `taxonomy`.

## Role boundaries

| Layer        | Can Contain                  | Cannot Contain             |
| :----------- | :--------------------------- | :------------------------- |
| capabilities | Pure computation, validation | I/O, network, database     |
| utility      | Stateless `pub fn` helpers   | State, trait impls, non-taxonomy imports |
| agent        | Orchestration flow           | Computation, I/O, business |

## Pre-flight (Rust)

```bash
# Ensure project builds
CARGO_INCREMENTAL=0 cargo build -p <crate> 2>&1 | tail -20
```

If build fails, fix compilation errors first. Violations on broken code are unreliable.

## Verify pipeline (Rust)

```bash
# 1. Re-scan — should show 0 violations
lint-arwaky-cli scan <target-path>

# 2. Build check
CARGO_INCREMENTAL=0 cargo check -p <crate>

# 3. Tests pass
cargo nextest run -p <crate> --lib --tests

# 4. Format + clippy
cargo fmt --all
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings
```

## Quick fix recipes (Rust)

### Rename file (AES101/102)

```bash
# Before: capabilities_scanner.rs (wrong — no underscore prefix for capabilities)
# After:  capabilities_file_scanner.rs
git mv src/capabilities_scanner.rs src/capabilities_file_scanner.rs
# Update mod.rs references
```

### Remove unused import (AES203)

```bash
# Auto-fixable
lint-arwaky-cli fix src/file.rs --filter AES203
```

### Fix bypass comments (AES304)

```bash
# Find all bypass patterns
grep -rn 'unwrap\|expect\|panic!\|#\[allow' src/

# Fix each one:
# unwrap() → use ? or match
# expect("msg") → use ? with context
# panic!("msg") → return Result::Err
# #[allow(...)] → fix the underlying warning
```

### Remove dead code (AES501–506)

```bash
# Find orphan files
lint-arwaky-cli orphan crates/ --format json

# If orphan is truly dead → delete it
# If orphan should be wired → add to container or import chain
```

### Break circular dependency (AES205)

```bash
# Identify the cycle
lint-arwaky-cli import <target-path> --filter AES205

# Solution: extract shared types/traits to a lower layer (taxonomy or contract)
# Move the shared interface to contract_<concern>_protocol.rs
# Both sides import from contract instead of importing each other
```

## Verification Checklist (Rust)

- [ ] `cargo fmt --all` clean
- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] `cargo test --workspace` passes
- [ ] `lint-arwaky-cli scan .` reports 0 violations
