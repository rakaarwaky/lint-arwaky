---
name: add-docs-typescript
description: "Add proper JSDoc comments, type annotations, and crate-level PRD.md/FRD.md/README.md to TypeScript packages following project conventions."
metadata:
  tags: [typescript, docs, jsdoc, type-hints, prd, frd, readme]
  triggers:
    - "add docs typescript"
    - "add jsdoc typescript"
    - "add type hints typescript"
    - "add prd typescript"
    - "add frd typescript"
    - "add package readme typescript"
  dependencies: []
  related:
    - cleanup-files-typescript
    - consolidate-files-typescript
---

# add-docs-typescript

## Purpose

Add documentation at correct locations following project conventions.

## Document Location Matrix

| Document  | Location            | Audience                     | Focus                |
| --------- | ------------------- | ---------------------------- | -------------------- |
| PRD.md    | Root workspace      | Stakeholder, PM, Design, Eng | _What_ & _Why_       |
| README.md | Root workspace      | Developer (new/existing)     | _How to use/run_     |
| FRD.md    | Each feature module | Engineer, QA, Tech Lead      | _How_ (functionally) |

## References

Read these files for detailed rules:

| File                                  | Content                              |
| ------------------------------------- | ------------------------------------ |
| `references/prd-rules.md`             | PRD rules, audience, anti-patterns   |
| `references/frd-rules.md`             | FRD rules, IDs, test scenarios       |
| `references/readme-rules.md`          | README rules, Quick Start, structure |
| `references/jsdoc-rules.md`           | JSDoc comment rules and templates    |
| `references/type-annotation-rules.md` | Type annotation rules and patterns   |

## Templates

Use these templates when creating new files:

| File                  | Purpose                      |
| --------------------- | ---------------------------- |
| `templates/PRD.md`    | New PRD at root workspace    |
| `templates/FRD.md`    | New FRD in feature module    |
| `templates/README.md` | New README at root workspace |

## Definition of Done

1. PRD.md exists at root with Problem Statement, Goals, Personas, Scope, Features.
2. README.md exists at root with Quick Start, Architecture, Scripts, Testing.
3. FRD.md exists in each feature module with Functional Requirements (FR-001 IDs).
4. Documents serve correct audience (PRD=stakeholders, FRD=engineers, README=developers).
5. All modules have one-liner JSDoc docstrings.
6. All classes have descriptive JSDoc docstrings.
7. All public methods have parameter/return documentation.
8. All function signatures use type annotations.
9. Complex types use interfaces or type aliases.

## Workflow

### Step 1: Analyze Project

- List feature modules in `packages/`
- Identify public modules, classes, and functions
- Check existing docs (PRD.md / README.md / FRD.md / JSDoc / type annotations)

### Step 2: Create / Fix PRD.md (root workspace)

Write root-level PRD.md following `templates/PRD.md`. See `references/prd-rules.md` for rules.

### Step 3: Create / Fix FRD.md (each feature module)

For each feature module, write FRD.md following `templates/FRD.md`. See `references/frd-rules.md` for rules.

### Step 4: Create / Update README.md (root workspace)

Write root-level README.md following `templates/README.md`. See `references/readme-rules.md` for rules.

### Step 5: Add JSDoc Comments

See `references/jsdoc-rules.md` for rules and templates.

### Step 6: Add Type Annotations

See `references/type-annotation-rules.md` for rules and patterns.

## Quick Commands

```bash
# Check files without docstrings
find packages/ -name "*.ts" | while read f; do
    head -1 "$f" | grep -q '^/\*\*' || echo "NO DOCSTRING: $f"
done

# Run TypeScript type checker
npx tsc --noEmit
```

## Common Mistakes

- PRD contains SQL schema or API details → move to FRD.
- FRD without acceptance criteria → add testable conditions per FR.
- README = essay 10 pages → keep concise, link to other docs.
- One document for all audiences → split by audience.
- Documents "write & forget" → review each sprint/release.
- FRD in root instead of feature module → FRD belongs with the feature code.
- Missing module docstrings → every file needs a one-liner at the top.
- Incomplete parameter documentation → all parameters must be documented.
- Using @ts-ignore without reason → fix the root cause instead of suppressing errors.
