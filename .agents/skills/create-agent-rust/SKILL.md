---
name: create-agent-rust
description: "Create and validate Rust agent layer files following AES rules: orchestration-only, zero I/O, zero business logic, zero domain computation, 3-block structure, one impl struct per file, aggregate contracts, DI for service dependencies, and shared VOs for domain data."
version: 1.3.0
category: refactoring
tags:
  [
    rust,
    aes,
    agent,
    aggregate,
    structure,
    3-block-structure,
    di,
    orchestration,
    vo,
    error-handling,
    magic-constant,
    imports,
  ]
triggers:
  - "create agent rust"
  - "add agent rust"
  - "fix agent structure rust"
  - "create aggregate rust"
  - "agent missing aggregate rust"
  - "validate agent logic rust"
  - "check agent rust"
  - "audit agent rust"
dependencies: []
related:
  - create-capabilities-rust
  - create-infrastructure-rust
  - create-taxonomy-rust
  - enforce-1-struct-per-file-rust
  - trait-consolidation-rust
  - module_logic_validator-rust
  - fix-agent-di
---

# create-agent-rust

## Purpose

Create and validate Rust **agent layer** files following clean architecture / AES rules.

An agent file must contain **orchestration / pipeline execution only**.

Agents coordinate capabilities, infrastructure ports, and shared taxonomy types, but agents MUST NOT contain I/O, business logic, domain rules, domain computation, or domain data definitions.

## Definition of Done

1. ONE implementation struct per file.
2. Struct implements ONE domain aggregate trait in Block 2.
3. Block 2 contains ONLY the aggregate trait implementation.
4. Constructors, std trait impls, private helpers in Block 3.
5. Zero I/O, zero business logic, zero domain computation.
6. No locally defined domain data structures.
7. Service dependencies use DI via `Arc<dyn Trait>`.
8. Value/configuration fields use shared VOs.
9. Aggregate signatures use shared VOs for domain data.
10. `cargo check -p <crate-name>` passes.

## References

| File | Content |
|------|---------|
| `references/layer-boundaries.md` | Allowed/Forbidden imports and dependencies |
| `references/3-block-structure.md` | Block 1/2/3 definitions, trait placement rules |
| `references/helper-vs-utility.md` | Helper vs utility decision, I/O Blocker, decision tree |
| `references/computation-detection.md` | Computation detection rules, forbidden/allowed patterns |
| `references/error-handling.md` | 4 error handling rules |
| `references/primitive-vo-policy.md` | Primitive policy table, VO rules |
| `references/examples.md` | All BAD/GOOD code examples |
| `references/commands.md` | Quick heuristic check commands |
| `references/checklist.md` | 27-item verification checklist |

## Templates

| File | Purpose |
|------|---------|
| `templates/agent_name.rs` | New agent implementation file |
| `templates/contract_name_aggregate.rs` | New aggregate trait definition |
| `templates/mod.rs` | Module registration |

## Workflow

### Step 1: Analyze File

Read the file and ask: **"Is this orchestration only?"**

If yes → keep as agent. If it contains computation → capabilities, I/O → infrastructure, domain data → taxonomy.

### Step 2: Check for Missing Aggregate

Does the agent struct implement an aggregate trait? If no → create one.

### Step 3: Create Aggregate File if Missing

Create `contract_<name>_aggregate.rs` in the appropriate shared domain folder.

### Step 4: Enforce 3-Block Structure

Reorganize: struct definition → aggregate trait impl → constructors/std traits/helpers.

### Step 5: Verify Struct Discipline

One struct, no local data structs, DI via `Arc<dyn Trait>`, shared VOs.

### Step 6: Verify Helper vs Utility Boundary

See `references/helper-vs-utility.md` for the decision tree.

### Step 7: Verify Layer Compliance

No forbidden imports, no I/O, no business logic, no domain computation.

### Step 8: Verify Error Handling, VO, and Constants

See `references/error-handling.md` and `references/primitive-vo-policy.md`.

### Step 9: Verify Compilation

```bash
cargo check -p <crate-name>
```

## Quick Commands

```bash
# List aggregate trait implementations
rg -n "impl\s+I[A-Za-z0-9_]+Aggregate\s+for" crates/<crate>/src/agent_*.rs

# Check computation patterns
rg "\.sum\(\)|\.len\(\)|\.map\(|\.fold\(" crates/<crate>/src/agent_*.rs

# Check forbidden imports
rg "^\s*use\s+.*(capabilities_|infrastructure_|surface_)" crates/<crate>/src/agent_*.rs
```

## Common Mistakes

- Putting domain computation in agents.
- Putting business logic in agents.
- Putting I/O in agents.
- Defining domain data structs in agent files.
- Using concrete service types as struct fields.
- Putting private helpers in the aggregate trait.
- Placing std trait impls before the aggregate trait.
- Mixing Block 2 and Block 3 responsibilities.
- Silent error swallowing with `unwrap_or_default()`.
- Magic constants in agent logic.
