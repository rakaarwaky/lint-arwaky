---
name: create-root-rust
description: "Create and validate Rust root layer files: composition root that wires Capabilities to Contract traits/aggregates and bootstraps the application. Container connects implementations, Entry starts the system."
metadata:
  tags: [rust, aes, root, container, entry, composition, di, wiring]
  triggers:
    - "create root rust"
    - "add root rust"
    - "create container rust"
    - "create entry rust"
    - "wire dependencies rust"
    - "check root rust"
    - "audit root rust"
  dependencies: []
  related:
    - create-capabilities-rust
    - create-agent-rust
    - create-contract-rust
    - create-taxonomy-rust
---

# create-root-rust

## Purpose

Create and validate Rust **root layer** files — the composition layer that assembles the system.

Root connects concrete implementations to contracts and starts the application. It may depend on all layers.

Two root roles exist:

- `container` → Wires one feature by connecting Capabilities to Contract traits and aggregates
- `entry` → Bootstraps the application and composes feature containers

## Definition of Done

1. Root file uses correct suffix: `_container` or `_entry`.
2. Container wires Capabilities to Contract traits/aggregates.
3. Entry bootstraps the application and composes feature containers.
4. Root may instantiate and wire components.
5. Root must not contain business logic.
6. Root must not contain orchestration policy.
7. Root must not contain technical parsing or UI behavior.
8. `cargo check -p <crate-name>` passes.

## Workflow

### Step 1: Determine Root Role

Ask: **"What does this file do?"**

| Role      | Suffix       | Responsibility                                    |
| --------- | ------------ | ------------------------------------------------- |
| Container | `_container` | Wire one feature's Capabilities to Contracts      |
| Entry     | `_entry`     | Bootstrap application, compose feature containers |

### Step 2: Create Root File

Create `root_<concept>_<suffix>.rs` in the appropriate location.

### Step 3: Wire Dependencies

Connect Capabilities implementations to their Contract traits/aggregates.

### Step 4: Verify

```bash
cargo check -p <crate-name>
```

## Quick Commands

```bash
# Find root files
rg "root_|container|entry" crates/*/src/

# Check for business logic in root
rg "fn calculate|fn validate|fn analyze" crates/*/src/root_*.rs
```

## Common Mistakes

- Putting business logic in root files.
- Putting orchestration policy in root files.
- Root containing UI behavior.
- Forgetting to wire Capabilities to Contracts.
- Creating circular dependencies between containers.
