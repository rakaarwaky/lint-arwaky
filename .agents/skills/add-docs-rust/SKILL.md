---
name: add-docs-rust
description: "Add proper doc comments, type annotations, and crate-level PRD.md/FRD.md/README.md to Rust crates following project conventions."
version: 1.4.0
category: documentation
tags: [rust, docs, doc-comments, prd, frd, readme]
triggers:
  - "add docs rust"
  - "add crate readme rust"
  - "add prd rust"
  - "add frd rust"
  - "add doc comments rust"
  - "document public api rust"
dependencies: []
related:
  - lint-arwaky-cli
  - fix-naming
---

# add-docs-rust

## Purpose

Add documentation at correct locations following project conventions.

## Document Location Matrix

| Document | Location | Audience | Focus |
|----------|----------|----------|-------|
| PRD.md | Root workspace | Stakeholder, PM, Design, Eng | *What* & *Why* |
| README.md | Root workspace | Developer (new/existing) | *How to use/run* |
| FRD.md | Each feature crate | Engineer, QA, Tech Lead | *How* (functionally) |

## References

Read these files for detailed rules:

| File | Content |
|------|---------|
| `references/prd-rules.md` | PRD rules, audience, anti-patterns |
| `references/frd-rules.md` | FRD rules, IDs, test scenarios |
| `references/readme-rules.md` | README rules, Quick Start, structure |
| `references/doc-comment-rules.md` | `///` doc comment rules and templates |
| `references/type-annotation-rules.md` | Type annotation rules and patterns |

## Templates

Use these templates when creating new files:

| File | Purpose |
|------|---------|
| `templates/PRD.md` | New PRD at root workspace |
| `templates/FRD.md` | New FRD in feature crate |
| `templates/README.md` | New README at root workspace |

## Definition of Done

1. PRD.md exists at root with Problem Statement, Goals, Personas, Scope, Features.
2. README.md exists at root with Quick Start, Architecture, Commands, Testing.
3. FRD.md exists in each feature crate with Functional Requirements (FR-001 IDs).
4. Documents serve correct audience (PRD=stakeholders, FRD=engineers, README=developers).
5. All public structs have `///` doc comments.
6. All public methods have `///` doc comments with Args/Returns/Errors.
7. All function signatures use type annotations.
8. Example code in doc comments is valid Rust.

## Workflow

### Step 1: Analyze Project

- List feature crates in `crates/`
- Identify public structs and methods
- Check existing docs (PRD.md / README.md / FRD.md / `///` comments)

### Step 2: Create / Fix PRD.md (root workspace)

Write root-level PRD.md following `templates/PRD.md`. See `references/prd-rules.md` for rules.

### Step 3: Create / Fix FRD.md (each feature crate)

For each feature crate, write FRD.md following `templates/FRD.md`. See `references/frd-rules.md` for rules.

### Step 4: Create / Update README.md (root workspace)

Write root-level README.md following `templates/README.md`. See `references/readme-rules.md` for rules.

### Step 5: Add Doc Comments

See `references/doc-comment-rules.md` for rules and templates.

### Step 6: Add Type Annotations

See `references/type-annotation-rules.md` for rules and patterns.

## Quick Commands

```bash
# Check files without doc comments
find crates/ -name "*.rs" | while read f; do
    head -1 "$f" | grep -q '^///' || echo "NO DOC COMMENT: $f"
done

# Run cargo doc
cargo doc --open
```

## Common Mistakes

- PRD contains SQL schema or API details → move to FRD.
- FRD without acceptance criteria → add testable conditions per FR.
- README = essay 10 pages → keep concise, link to other docs.
- One document for all audiences → split by audience.
- Documents "write & forget" → review each sprint/release.
- FRD in root instead of feature crate → FRD belongs with the feature code.
- Missing doc comments → every public item needs `///` doc comment.
- Using `//` instead of `///` → use `///` for cargo doc visibility.
- Incomplete parameter documentation → all parameters must be documented.
