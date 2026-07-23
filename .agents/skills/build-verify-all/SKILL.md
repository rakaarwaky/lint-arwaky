---
name: build-verify-all
description: "Build workspace and verify clean compile, then optionally run tests. Use after code edits to catch regressions early."
metadata:
  tags: [rust, build, verify, test, cargo]
  triggers:
    - "build workspace"
    - "verify compile"
    - "run tests"
    - "check build"
  dependencies: []
  related: []
---

# Build & Verify

Fast feedback loop after code edits. Run after every meaningful change.

See [README.md](../../../../README.md) for build commands, [TEST.md](../../../../TEST.md) for pass/fail criteria, and [scripts/gates.sh](../../../../scripts/gates.sh) for the full quality gate pipeline.

## Quick Check (30s)

```bash
cargo check --workspace 2>&1 | grep -E "warning|error|Finished"
```

## Full Build + Test (2-3min)

```bash
cargo build --release 2>&1 | tail -3 && cargo test --workspace 2>&1 | tail -10
```

## Per-Crate Check

```bash
cargo check -p <crate-name> 2>&1 | grep -E "error\[|warning\["
```

## After Build Success

Run AES self-lint to catch architecture violations:

```bash
cargo run --bin lint-arwaky-cli -- check . 2>&1 | grep -E "AES[0-9]+" | head -20
```

## Decision Tree

1. After **small edit** (1-5 lines): `cargo check --workspace`
2. After **module refactor**: `cargo check -p <crate>` then `cargo check --workspace`
3. Before **commit**: `cargo build --release && cargo test --workspace`
4. After **dependency change**: `cargo build --release 2>&1 | tail -5`

## Common Errors

| Error                            | Fix                   |
| -------------------------------- | --------------------- |
| `error[E0412]: cannot find type` | Missing `use` import  |
| `error[E0596]: cannot borrow`    | Add `&` or `.clone()` |
| `warning: unused import`         | Remove unused `use`   |
