---
name: create-infrastructure-rust
description: "Create and validate Rust infrastructure layer files following AES rules: I/O and external integration only, zero business logic, 3-block structure, one impl struct per file, port trait contracts, DI for service dependencies, and shared VOs for domain data."
version: 1.3.0
category: refactoring
tags:
  [
    rust,
    aes,
    infrastructure,
    port,
    structure,
    aes404,
    3-block-structure,
    di,
    vo,
    error-handling,
    magic-constant,
    imports,
  ]
triggers:
  - "create infrastructure rust"
  - "add infrastructure rust"
  - "fix infrastructure structure rust"
  - "create port rust"
  - "infrastructure missing port rust"
  - "check infrastructure rust"
  - "audit infrastructure rust"
dependencies: []
related:
  - create-capabilities-rust
  - create-agent-rust
  - enforce-1-struct-per-file-rust
  - trait-consolidation-rust
  - module_logic_validator-rust
  - fix-infrastructure-structure-rust
  - create-missing-ports-rust
---

# create-infrastructure-rust

## Purpose

Create and validate Rust **infrastructure layer** files following clean architecture / AES rules.

An infrastructure file must contain **I/O and external system integration only**:

- file system access, network calls, database access, external API calls,
- environment/system integration, technical mapping, serialization/deserialization,
- error mapping, adapter implementation for port traits.

Infrastructure MUST NOT contain business logic.

## Definition of Done

1. ONE implementation struct per file.
2. Struct implements ONE domain port trait in Block 2.
3. Block 2 contains ONLY the port trait implementation.
4. Constructors, std trait impls, private helpers in Block 3.
5. Zero business logic.
6. No locally defined domain data structures.
7. Service dependencies use DI via `Arc<dyn Trait>`.
8. Value/configuration fields use shared VOs.
9. I/O errors are propagated explicitly.
10. `cargo check -p <crate-name>` passes.

## References

| File | Content |
|------|---------|
| `references/layer-boundaries.md` | Allowed/Forbidden imports and dependencies |
| `references/3-block-structure.md` | Block 1/2/3 definitions, trait placement rules |
| `references/helper-vs-utility.md` | Helper vs utility decision, I/O Blocker, decision tree |
| `references/primitive-vo-policy.md` | Primitive policy table, VO rules |
| `references/error-handling.md` | 4 error handling rules |
| `references/examples.md` | All BAD/GOOD code examples |
| `references/commands.md` | Quick heuristic check commands |
| `references/checklist.md` | 23-item verification checklist |

## Templates

| File | Purpose |
|------|---------|
| `templates/infrastructure.rs` | New infrastructure implementation file |
| `templates/port.rs` | New port trait definition |

## Workflow

### Step 1: Analyze File Responsibility

Read the file and ask: **"Is this pure I/O or external system integration?"**

If yes → keep as infrastructure. If it contains business logic → move to capabilities.

### Step 2: Check for Missing Port

Does the infrastructure struct implement a port trait? If no → create one.

### Step 3: Create Port File if Missing

Create `contract_<name>_port.rs` in the appropriate shared domain folder.

### Step 4: Enforce 3-Block Structure

Reorganize: struct definition → port trait impl → constructors/std traits/helpers.

### Step 5: Verify Struct Discipline

One struct, no local data structs, DI via `Arc<dyn Trait>`, shared VOs.

### Step 6: Verify Helper vs Utility Boundary

See `references/helper-vs-utility.md` for the decision tree.

### Step 7: Verify Layer Compliance

No forbidden imports, no business logic, no local domain data definitions.

### Step 8: Verify Error Handling, VO, and Constants

See `references/error-handling.md` and `references/primitive-vo-policy.md`.

### Step 9: Verify Compilation

```bash
cargo check -p <crate-name>
```

## Quick Commands

```bash
# List port trait implementations
rg -n "impl\s+I[A-Za-z0-9_]+Port\s+for" crates/<crate>/src/infrastructure_*.rs

# Check business logic keywords
rg "is_orphan|analyze|validate|calculate|compute|business" crates/<crate>/src/infrastructure_*.rs

# Check forbidden imports
rg "^\s*use\s+.*(capabilities_|agent_)" crates/<crate>/src/infrastructure_*.rs
```

## Common Mistakes

- Putting business logic in infrastructure.
- Defining domain data structs in infrastructure files.
- Using concrete service types as struct fields.
- Using raw primitives for domain value fields.
- Putting private helpers in the port trait.
- Putting constructors in the port trait.
- Placing std trait impls before the port trait.
- Mixing Block 2 and Block 3 responsibilities.
- Keeping reusable, domain-agnostic utility functions inside Block 3.
- Silent error swallowing with `unwrap_or_default()`.
- Magic constants in infrastructure logic.
- Infrastructure returning lint results directly.
