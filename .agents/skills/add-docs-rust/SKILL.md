---
name: add-docs-rust
version: 1.1.0
category: documentation
tags: [aes, docs, readme, frd, doc-comments, rust]
triggers:
  - "add docs rust"
  - "add crate readme rust"
  - "add frd rust"
  - "add doc comments rust"
  - "document public api rust"
dependencies: []
related:
  - lint-arwaky-cli
  - fix-naming
---

# add-docs-rust

## Rules

- Every crate directory MUST contain TWO crate-level docs: `FRD.md` and `README.md`.
- `FRD.md` is STATELESS — it describes the IDEAL TARGET only. It MUST NOT record progress, status, current-state notes, or "what's done so far". If reality diverges, fix `README.md`, never pollute `FRD.md` with state.
- `README.md` describes the REAL CURRENT STATE — what actually exists today. It is allowed (and expected) to diverge from the ideal target in `FRD.md`.
- Relationship: **FRD = harapan (ideal), README = kenyataan (reality).** README should call out gaps vs FRD; FRD must stay clean of any "as-built" noise.
- All public structs and methods MUST have `///` doc comments (visible in `cargo doc`).
- Doc comments MUST explain "what" and "why", not "how" (code shows how).
- Example code in doc comments MUST be valid Rust.

## Purpose

Add crate-level documentation and `///` doc comments:
- `FRD.md` — stateless ideal target (Feature Goal / Requirements & Scope / Success Indicators).
- `README.md` — real current state (what exists, public API surface, known gaps vs FRD).
- `///` doc comments on all public items for `cargo doc` visibility.

## When to Use

- New crate has no `FRD.md` or no `README.md`.
- `FRD.md` contains state/progress notes (violates stateless rule) — clean it.
- README and FRD are conflated (state leaking into FRD) — split them.
- Public structs/methods lack `///` doc comments.
- `cargo doc` output is incomplete or missing.
- User asks to document the crate or add docs.

## The Fundamental Question

> **"Can a newcomer understand this crate's purpose in 30 seconds?"**

If no -> **Add FRD.md (ideal target) + README.md (reality).**

> **"Will this struct appear in `cargo doc`?"**

If no (only `//` comments) -> **Convert to `///` doc comments.**

## Detection Patterns

### Missing FRD.md / README.md (Create)

```
crates/<name-folder>/
├── src/
│   ├── lib.rs
│   └── ...
├── tests/
├── FRD.md        # stateless ideal target
└── README.md     # real current state
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

## FRD.md Template (STATELESS — ideal target only)

```markdown
# FRD — <crate-name>

> Stateless document. Describes the IDEAL TARGET only. Never record progress,
> status, or current-state notes. If reality diverges from this, update
> README.md — do NOT add state to this file.

## Feature Goal
<One paragraph: what this crate is supposed to accomplish when complete.>

## Requirements & Scope
- In scope: <...>
- Out of scope: <...>

## Success Indicators
- [ ] <measurable ideal outcome>
- [ ] <measurable ideal outcome>
```

## README.md Template (REAL current state)

```markdown
# <crate-name>

> Current real state — what actually exists today. May diverge from FRD.md
> (the ideal target). Keep this honest; gaps belong here, not in FRD.

## What exists now
- <real modules / features implemented>
- <real behavior>

## Public API surface
- `<Type>` — <one-line reality of what it does>
- `<fn>` — <...>

## Known gaps vs FRD
- <deviation from ideal target — what's missing or different>
```

## Workflow

### Step 1: Analyze Crate

- List files in `crates/<name>/src/`
- Identify public structs and methods
- Check existing docs (README.md / FRD.md / `///` comments)

### Step 2: Create / Fix FRD.md (ideal target, stateless)

Write crate-level FRD.md following the FRD template. It MUST contain only:

1. Feature Goal
2. Requirements & Scope
3. Success Indicators

Strip any state, progress, or "as-built" notes. FRD is the harapan — it never changes because code isn't done yet.

### Step 3: Create / Update README.md (reality)

Write README.md reflecting the ACTUAL current state:

1. What exists now (real modules, real behavior)
2. Public API surface (real items)
3. Known gaps vs FRD (where reality diverges from the ideal target)

README is the kenyataan — it changes as the crate evolves.

### Step 4: Add Doc Comments

For each public struct and method:

1. Convert `//` comments to `///` doc comments
2. Add summary line
3. Add explanation if >10 lines of logic
4. Add `# Example` block if applicable
