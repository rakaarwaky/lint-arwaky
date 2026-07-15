---
name: enforce-1-struct-per-file-rust
version: 1.0.0
category: refactoring
tags: [aes, struct, structure, single-responsibility, rust]
triggers:
  - "enforce 1 struct per file rust"
  - "merge structs rust"
  - "one struct per file rust"
dependencies: []
related:
  - trait-consolidation
  - fix-capability-structure
---

# enforce-1-struct-per-file-rust

## Rules

- 1 file = 1 main struct
- Enums, constants → move to shared taxonomy
- Helper structs → merge into main struct as methods
- Trait definitions stay in shared contract

## Purpose

Ensure each capability/infrastructure/agent file contains exactly ONE main struct.

## When to Use

- File has multiple structs
- Enum or constant defined locally in infrastructure
- Helper struct that supports the main struct

## The Fundamental Question

> **"Does this file have more than 1 struct?"**

If yes → **Merge into 1 struct or move to taxonomy**

## Detection Pattern

```rust
// BAD: 2 structs in 1 file
struct CoordinateTransforms;  // → MOVE to CoordinateMapper as methods
struct CoordinateMapper;

// GOOD: 1 struct per file
struct CoordinateMapper;

impl CoordinateMapper {
    pub fn world_to_camera(...) {  // was in CoordinateTransforms
        ...
    }
}
```

## Types to Move to Taxonomy

| Type | Move To |
|------|---------|
| `struct` with data | `taxonomy_*_vo.rs` |
| `enum` | `taxonomy_*_vo.rs` |
| `const` | `taxonomy_*_constant.rs` |

## Types to Merge into Main Struct

| Type | Merge Into |
|------|-----------|
| Helper struct | Main struct as methods |
| Factory struct | Main struct methods |
| Builder struct | Main struct methods |

## Workflow

### Step 1: Count Structs

Find files with >1 struct.

### Step 2: Classify Each Struct

- Data/enum → move to taxonomy
- Helper → merge into main struct
- Trait → move to shared contract

### Step 3: Execute

Move or merge each struct.

### Step 4: Update Imports

Fix all references.

### Step 5: Verify

Run `cargo check`.
