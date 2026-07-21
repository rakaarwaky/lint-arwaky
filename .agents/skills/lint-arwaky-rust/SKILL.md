---
name: lint-arwaky-rust
description: "Run lint-arwaky CLI scanner and MCP server for Rust projects — validate AES compliance, check layer violations, and fix architecture issues."
metadata:
    tags: [rust, lint, aes, compliance, scanning, mcp, clippy]
    triggers:
        - "lint arwaky rust"
        - "lint code rust"
        - "check compliance rust"
        - "scan rust project"
    dependencies: []
    related:
        - module_logic_validator
        - cleanup-files-rust
---

# lint-arwaky-cli-rust

## Rules

- All Rust code must pass `cargo clippy`
- All Rust code must pass `cargo fmt`
- No security issues
- No architecture violations

## Purpose

Run linters (clippy, rustfmt) and enforce AES compliance rules.

## When to Use

- Before committing changes
- After refactoring code
- When CI/CD checks fail
- When user asks to lint code

## The Fundamental Question

> **"Does the code pass all linters?"**

If no -> **Fix violations**

## Core Commands

### Check & Fix

```bash
# Self-lint (check architecture)
cargo run --bin lint-arwaky-cli -- check .

# Scan project
cargo run --bin lint-arwaky-cli -- scan <project-path>

# Auto-fix formatting
cargo fmt --all

# Check clippy
cargo clippy --all-targets -- -D warnings
```

### Per-Crate Check

```bash
# Check single crate
cargo check -p <crate-name>

# Check single crate with clippy
cargo clippy -p <crate-name> -- -D warnings
```

### Tests

```bash
# Run all tests
cargo test --workspace

# Run single crate tests
cargo test -p <crate-name>

# Run single test by name
cargo test --lib <name_fragment>
```

### Diagnostics

```bash
# Check environment
cargo run --bin lint-arwaky-cli -- maintenance doctor

# Show version
cargo run --bin lint-arwaky-cli -- version
```

## Verification Checklist

- [ ] `cargo fmt --all` clean
- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] `cargo test --workspace` passes
- [ ] `cargo run --bin lint-arwaky-cli -- check .` reports 0 violations
