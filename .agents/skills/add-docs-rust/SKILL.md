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

- Crate-level `FRD.md` MUST exist in every crate directory
- All public structs and methods MUST have `///` doc comments (visible in `cargo doc`)
- Doc comments MUST explain "what" and "why", not "how" (code shows how)
- Example code in doc comments MUST be valid Rust

## Purpose

Add crate-level documentation: `README.md` for entry points and `///` doc comments on all public items for `cargo doc` visibility.

## When to Use

- New crate has no `FRD.md`
- Public structs/methods lack `///` doc comments
- `cargo doc` output is incomplete or missing
- User asks to document the crate or add docs

## The Fundamental Question

> **"Can a newcomer understand this crate's purpose in 30 seconds?"**

If no -> **Add FRD.md**

> **"Will this struct appear in `cargo doc`?"**

If no (only `//` comments) -> **Convert to `///` doc comments**

## Detection Patterns

### Missing FRD.md (Create)

```
crates/<name-folder>/
├── src/
│   ├── lib.rs      
│   └── ...
├── tests/          
└── FRD.md
```

### Missing Doc Comments (Add)

```rust
// PURPOSE expalin file in one sentence
pub struct ImportOrchestrator {
    mandatory: Arc<dyn IImportMandatoryProtocol>,
}

// [OK] /// doc comment — appears in cargo doc
/// Orchestrates <name-feature>.
///
/// Execution order:
/// 1. 
/// 2. 
/// 3. 
/// 4. 
pub struct ImportOrchestrator {
    mandatory: Arc<dyn IImportMandatoryProtocol>,
}
```

## Workflow

### Step 1: Analyze Crate

- List files in `crates/<name>/src/`
- Identify public structs and methods
- Check existing docs

### Step 2: Create FRD.md

Write crate-level FRD.md following the template above. Include:

1. Feature Goal
2. Requirements & Scope
3. Success Indicators

### Step 3: Add Doc Comments

For each public struct and method:

1. Convert `//` comments to `///` doc comments
2. Add summary line
3. Add explanation if >10 lines of logic
4. Add `# Example` block if applicable
