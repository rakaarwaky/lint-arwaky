---
name: add-docs-rust
version: 1.0.0
category: documentation
tags: [aes, docs, readme, doc-comments, rust]
triggers:
  - "add docs rust"
  - "add crate readme rust"
  - "add doc comments rust"
  - "document public api rust"
dependencies: []
related:
  - lint-arwaky-cli
  - fix-naming
---

# add-docs-rust

## Rules

- Crate-level `README.md` MUST exist in every crate directory
- All public structs and methods MUST have `///` doc comments (visible in `cargo doc`)
- Doc comments MUST explain "what" and "why", not "how" (code shows how)
- Example code in doc comments MUST be valid Rust

## Purpose

Add crate-level documentation: `README.md` for entry points and `///` doc comments on all public items for `cargo doc` visibility.

## When to Use

- New crate has no `README.md`
- Public structs/methods lack `///` doc comments
- `cargo doc` output is incomplete or missing
- User asks to document the crate or add docs

## The Fundamental Question

> **"Can a newcomer understand this crate's purpose in 30 seconds?"**

If no -> **Add README.md**

> **"Will this struct appear in `cargo doc`?"**

If no (only `//` comments) -> **Convert to `///` doc comments**

## Detection Patterns

### Missing README (Create)

```
crates/import-rules/
├── src/
│   ├── lib.rs        # has module declarations, no README nearby
│   └── ...
└── tests/            # no README.md in crate directory
```

### Missing Doc Comments (Add)

```rust
// [FORBIDDEN] Only // comment — won't appear in cargo doc
pub struct ImportOrchestrator {
    mandatory: Arc<dyn IImportMandatoryProtocol>,
}

// [OK] /// doc comment — appears in cargo doc
/// Orchestrates AES201-AES205 import rule checks.
/// 
/// Execution order:
/// 1. Mandatory imports (concurrent)
/// 2. Forbidden imports (concurrent)
/// 3. Sequential checks
/// 4. Cycle detection (last, requires full graph)
pub struct ImportOrchestrator {
    mandatory: Arc<dyn IImportMandatoryProtocol>,
}
```

## README.md Template

```markdown
# <Crate Name>

## Purpose

Brief description of what this crate does and its role in the larger system.

## Architecture

Follows the 7-layer AES architecture:

| Layer | Files | Responsibility |
|-------|-------|----------------|
| Agent | `agent_*.rs` | Orchestration, no computation |
| Capabilities | `capabilities_*.rs` | Business logic, no I/O |
| Infrastructure | `infrastructure_*.rs` | I/O abstractions |

## AES Rules Implemented

| Rule | Description | Checker |
|------|-------------|---------|
| AES201 | Forbidden imports | `CapabilitiesImportForbiddenChecker` |
| AES202 | Mandatory imports | `CapabilitiesImportMandatoryChecker` |
| ... | ... | ... |

## Public API

```rust
use import_rules::*;

let container = ImportContainer::new(/* args */);
let orchestrator = container.orchestrator();
```

## Testing

```bash
cargo test -p import_rules
```
```

## Doc Comment Template

```rust
/// Short summary (one line).
///
/// Longer explanation if needed. Explain:
/// - What this struct/type does
/// - Why it exists
/// - Key invariants
///
/// # Example
/// ```
/// let instance = MyType::new();
/// ```
pub struct MyType { ... }

/// Does X with Y.
///
/// # Arguments
/// - `param`: description
///
/// # Returns
/// - `Ok(T)` on success
/// - `Err(E)` on failure (describe when)
///
/// # Panics
/// - Describe if any
pub fn my_method(&self, param: &Type) -> Result<T, E> { ... }
```

## Workflow

### Step 1: Analyze Crate

- List files in `crates/<name>/src/`
- Identify public structs and methods
- Check existing docs

### Step 2: Create README.md

Write crate-level README.md following the template above. Include:
- Purpose (one sentence)
- Architecture overview (layer table)
- AES rules implemented
- Public API example
- Test commands

### Step 3: Add Doc Comments

For each public struct and method:
1. Convert `//` comments to `///` doc comments
2. Add summary line
3. Add explanation if >10 lines of logic
4. Add `# Example` block if applicable

### Step 4: Verify

```bash
# Generate docs locally
cargo doc --no-deps -p <crate-name> --open

# Check no public items lack docs
cargo doc --no-deps -p <crate-name> 2>&1 | grep -i "missing"
```

## Quick Commands

```bash
# View generated docs
cargo doc --no-deps -p import_rules --open

# Find public items without doc comments
rg "^pub (struct|fn|trait|enum)" crates/import-rules/src/ | while read line; do
  file=$(echo "$line" | cut -d: -f1)
  linenum=$(echo "$line" | cut -d: -f2)
  prev_line=$((linenum - 1))
  head -n "$prev_line" "$file" | tail -1 | grep -q "^///" || echo "$file:$linenum NEEDS DOC"
done

# Check README exists
ls crates/import-rules/README.md
```
