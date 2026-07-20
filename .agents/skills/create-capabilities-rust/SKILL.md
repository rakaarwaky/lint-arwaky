---
name: create-capabilities-rust
description: "Create and validate Rust capabilities layer files following AES rules: concrete implementation of behavior (business logic + external adaptation), 3-block structure, one impl struct per file, protocol trait contracts, DI for service dependencies, and shared VOs for domain data."
version: 1.3.0
category: refactoring
tags:
  [
    rust,
    aes,
    capability,
    protocol,
    structure,
    aes402,
    aes403,
    aes404,
    3-block-structure,
    di,
    vo,
    error-handling,
    magic-constant,
    imports,
  ]
triggers:
  - "create capability rust"
  - "add capability rust"
  - "fix capability structure rust"
  - "create trait rust"
  - "capability missing trait rust"
  - "check capabilities rust"
  - "audit capabilities rust"
dependencies: []
related:
  - create-agent-rust
  - enforce-1-struct-per-file-rust
  - trait-consolidation-rust
  - module_logic_validator-rust
  - fix-capability-structure-rust
  - create-missing-protocols-rust
---

# create-capabilities-rust

## Purpose

Create and validate Rust **capabilities layer** files following AES rules.

A capabilities file contains the **concrete implementation** of the system's behavior. This layer encapsulates both:

- **Business logic**: computations, validations, transformations, assessments
- **External adaptation**: database access, third-party API calls, file system access, infrastructure mechanics

Capabilities hide these implementations behind Contracts, keeping behavior modular, swappable, and fully isolated from orchestration.

A capabilities file must:

- implement one domain protocol trait,
- follow strict 3-block structure,
- use dependency injection for service collaborators (`Arc<dyn Trait>`),
- use shared VOs for domain data,
- use Utility standalone functions for low-level technical operations.

## Definition of Done

1. ONE implementation struct per file.
2. Struct implements ONE domain protocol trait in Block 2.
3. Block 2 contains ONLY the domain protocol trait implementation.
4. Constructors, std trait impls, private helpers in Block 3.
5. No locally defined domain data structures.
6. Service dependencies use DI via `Arc<dyn Trait>`.
7. Value/configuration fields use shared VOs.
8. No inter-capability dependencies (capabilities must not import other capabilities).
9. Low-level technical operations delegate to Utility standalone functions.
10. `cargo check -p <crate-name>` passes.

## References

Read these files for detailed rules:

| File | Content |
|------|---------|
| `references/layer-boundaries.md` | Allowed/Forbidden imports and dependencies |
| `references/3-block-structure.md` | Block 1/2/3 definitions, method placement rules |
| `references/helper-vs-utility.md` | Helper vs utility decision, I/O Blocker, decision tree |
| `references/primitive-vo-policy.md` | Primitive policy table, VO construction rules |
| `references/error-handling.md` | Error handling rules with examples |
| `references/examples.md` | All BAD/GOOD code examples |
| `references/commands.md` | Quick heuristic check commands |
| `references/checklist.md` | Verification checklist |

## Templates

Use these templates when creating new files:

| File | Purpose |
|------|---------|
| `templates/capabilities_name.rs` | New capabilities implementation file |
| `templates/contract_name_protocol.rs` | New protocol trait definition |
| `templates/mod.rs` | Module registration |

## Workflow

### Step 1: Analyze File Responsibility

Read the file and ask: **"Does this implement protocol behavior?"**

If yes → keep as capabilities. If no → check if it's orchestration (→ agent), domain data (→ taxonomy), or pure technical mechanics (→ utility).

### Step 2: Check Missing Trait (AES403)

Does the capability struct implement a protocol trait? If no → create one.

### Step 3: Create Trait File if Missing

Create `contract_<name>_protocol.rs` in the appropriate shared domain folder.

### Step 4: Enforce 3-Block Structure

Reorganize: struct definition → domain protocol trait impl → constructors/std traits/helpers.

### Step 5: Verify Struct Discipline

One struct, no local data structs, DI via `Arc<dyn Trait>`, shared VOs.

### Step 6: Verify Helper vs Utility Boundary

See `references/helper-vs-utility.md` for the decision tree.

### Step 7: Verify Layer Compliance

No forbidden imports, no inter-capability dependencies, no business logic leakage.

### Step 8: Verify Error Handling, VO, and Constants

See `references/error-handling.md` and `references/primitive-vo-policy.md`.

### Step 9: Verify Compilation

```bash
cargo check -p <crate-name>
```

## Quick Commands

```bash
# Check forbidden imports (no infrastructure, no agent, no other capabilities)
rg "^\s*use\s+.*(infrastructure_|agent_|capabilities_)" crates/<crate>/src/capabilities_*.rs

# List protocol trait implementations
rg -n "impl\s+I[A-Za-z0-9_]+Protocol\s+for" crates/<crate>/src/capabilities_*.rs
```

## Common Mistakes

- Importing other capabilities directly.
- Defining domain data structs in capabilities files.
- Using concrete service types as struct fields.
- Using raw primitives for domain value fields.
- Putting private helpers in the protocol trait.
- Putting constructors in the protocol trait.
- Placing std trait impls before the domain protocol trait.
- Mixing Block 2 and Block 3 responsibilities.
- Silent error swallowing with `unwrap_or_default()`.
- Magic constants in capabilities logic.
- Not delegating low-level technical operations to Utility.
