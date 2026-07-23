---
name: create-utility-rust
description: "Create and validate Rust utility layer files following AES rules: stateless standalone functions, no struct, no trait impl, pure functions, domain-agnostic, reusable across modules."
metadata:
  tags:
    [
      rust,
      aes,
      utility,
      stateless,
      pure-functions,
      domain-agnostic,
      reusability,
      taxonomy,
    ]
  triggers:
    - "create utility rust"
    - "add utility rust"
    - "extract to utility rust"
    - "move to utility rust"
    - "check utility rust"
  dependencies: []
  related:
    - create-capabilities-rust
    - cleanup-files-rust
    - consolidate-files-rust
---

# create-utility-rust

## Purpose

Create and validate Rust **utility layer** files following AES rules.

A utility file contains **stateless standalone functions**. It exists so that Capabilities, Agents, and Surfaces can remain clean and expressive by delegating low-level technical mechanics to reusable helpers.

A utility file must:

- contain ONLY free functions (no struct, no `impl` blocks),
- be completely stateless (no `&self`, no field access),
- be pure (input A always produces output B),
- be domain-agnostic (no business rules, no domain knowledge),
- be reusable across multiple modules.

## Role Naming (ARCHITECTURE §7)

Utility role suffixes describe the technical responsibility:

parser
splitter
trimmer
slugifier
sanitizer
normalizer
extractor
replacer
converter
counter
resolver
detector
builder
joiner
serializer
deserializer
encoder
decoder
hasher
generator
formatter
comparator
differ
matcher
checker
calculator
mapper
merger
grouper
sorter
deduplicator
printer

File: `utility_<domain>_<role>.rs`

## Dependencies (ARCHITECTURE §7)

- **May depend on:** Taxonomy only.
- **Must NOT depend on / import:** Capabilities, Agent, Surface, Contract, other Utility (except shared taxonomy utilities).

## Special Rules (ARCHITECTURE §7)

- **Stateless Only:** no struct fields, no `&self`, no instance state.
- **Pure Functions:** input A always produces output B. No randomness, no global state mutation, no I/O side effects (unless domain-agnostic + reusable).
- **No Business Decisions:** utility does not know business rules, domain constraints, or architecture policies.
- **No Contract Implementation:** utility never implements a protocol or aggregate trait.
- **I/O Allowed:** stateless + I/O + domain-agnostic + reusable = valid utility (e.g., `walk_source_files`, `read_file_content`).
- **Standalone Functions Only:** no methods, no impl blocks, no structs. Just `pub fn` declarations.
- **No Magic Constants:** extract reusable constants into `taxonomy_<domain>_constant.rs` in shared.

## Definition of Done

1. NO struct definition — only free functions.
2. NO `impl` blocks — no trait implementations.
3. All functions are stateless (no `&self`, no field access).
4. Functions are pure: input A always produces output B.
5. Functions are domain-agnostic: no business rules, no architecture knowledge.
6. Functions are reusable across multiple modules.
7. No magic constants — use shared taxonomy constants.
8. Only depends on Taxonomy layer.
9. `cargo check -p <crate-name>` passes.

## References

Read these files for detailed rules:

| File                             | Content                                         |
| -------------------------------- | ----------------------------------------------- |
| `references/layer-boundaries.md` | Allowed/Forbidden imports and dependencies      |
| `references/stateless-rules.md`  | Stateless, pure, domain-agnostic decision rules |
| `references/examples.md`         | All BAD/GOOD code examples                      |
| `references/commands.md`         | Quick heuristic check commands                  |

## Templates

Use these templates when creating new files:

| File                        | Purpose                         |
| --------------------------- | ------------------------------- |
| `templates/utility_name.rs` | New utility implementation file |
| `templates/mod.rs`          | Module registration             |

## Workflow

### Step 1: Analyze Code Responsibility

Read the code and ask: **"Is this a stateless, pure, domain-agnostic function?"**

If yes → extract to utility. If no → check if it's business logic (→ capabilities), orchestration (→ agent), or domain data (→ taxonomy).

### Step 2: Check Reusability

Is the function used by multiple modules? Or will it be useful in the future?

- **Single-use + domain-specific** → keep as private helper in Block 3 (capabilities/agent)
- **Reusable + domain-agnostic** → extract to utility

### Step 3: Verify Stateless Purity

Does the function have ANY of these?

- `&self` parameter
- Access to struct fields
- Random number generation
- System clock access
- Global state mutation
- Business rule knowledge

If YES → NOT a utility. Keep as private helper.

### Step 4: Verify Domain Agnosticism

Does the function know about:

- Architecture layer names?
- Business domain rules?
- Specific capability logic?

If YES → NOT a utility. Domain-specific code belongs in capabilities.

### Step 5: Create Utility File

Write the free functions following the template. No struct, no impl blocks.

### Step 6: Update Module Registration

Add `pub mod utility_<name>;` to the appropriate shared domain `mod.rs`.

### Step 7: Verify Compilation

```bash
cargo check -p <crate-name>
```

## Quick Commands

```bash
# Check for forbidden patterns (struct, impl, &self)
rg "pub struct|impl\s+.*\{|fn\s+.*&self" crates/shared/src/<domain>/utility_*.rs

# List all utility functions
rg "^pub fn" crates/shared/src/<domain>/utility_*.rs

# Check imports in utilities (should only use shared::taxonomy)
rg "^\s*use\s+" crates/shared/src/<domain>/utility_*.rs
```

## Common Mistakes

- Adding struct definitions to utility files.
- Implementing trait protocols in utility files.
- Using `&self` or accessing struct fields.
- Including business logic or domain rules.
- Using magic constants instead of shared taxonomy constants.
- Importing Capabilities, Agent, or Surface modules.
- Creating functions that are only used by one module (keep as private helper).
- Mixing pure functions with stateful operations.
- Adding I/O to domain-specific functions (must be domain-agnostic + reusable).
