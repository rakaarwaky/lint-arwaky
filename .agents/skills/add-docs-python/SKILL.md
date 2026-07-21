---
name: add-docs-python
description: "Add proper docstrings, type hints, and crate-level PRD.md/FRD.md/README.md to Python packages following PEP 257 and project conventions."
metadata:
    tags: [python, docs, docstring, type-hints, prd, frd, readme, pep257]
    triggers:
        - "add docs python"
        - "add docstring python"
        - "add type hints python"
        - "add prd python"
        - "add frd python"
        - "add package readme python"
    dependencies: []
    related:
        - cleanup-files-python
        - consolidate-files-python
---

# add-docs-python

## Purpose

Add documentation at correct locations following project conventions.

## Document Location Matrix

| Document | Location | Audience | Focus |
|----------|----------|----------|-------|
| PRD.md | Root workspace | Stakeholder, PM, Design, Eng | *What* & *Why* |
| README.md | Root workspace | Developer (new/existing) | *How to use/run* |
| FRD.md | Each feature module | Engineer, QA, Tech Lead | *How* (functionally) |

## References

Read these files for detailed rules:

| File | Content |
|------|---------|
| `references/prd-rules.md` | PRD rules, audience, anti-patterns |
| `references/frd-rules.md` | FRD rules, IDs, test scenarios |
| `references/readme-rules.md` | README rules, Quick Start, structure |
| `references/docstring-rules.md` | PEP 257 docstring rules and templates |
| `references/type-hint-rules.md` | Type hint rules and patterns |

## Templates

Use these templates when creating new files:

| File | Purpose |
|------|---------|
| `templates/PRD.md` | New PRD at root workspace |
| `templates/FRD.md` | New FRD in feature module |
| `templates/README.md` | New README at root workspace |

## Definition of Done

1. PRD.md exists at root with Problem Statement, Goals, Personas, Scope, Features.
2. README.md exists at root with Quick Start, Architecture, Scripts, Testing.
3. FRD.md exists in each feature module with Functional Requirements (FR-001 IDs).
4. Documents serve correct audience (PRD=stakeholders, FRD=engineers, README=developers).
5. All modules have one-liner docstrings.
6. All classes have descriptive docstrings.
7. All public functions have parameter/return documentation.
8. All function signatures use type hints.
9. Forward references use string quotes or `__future__.annotations`.
10. Complex types use `typing` module.

## Workflow

### Step 1: Analyze Project

- List feature modules in `modules/`
- Identify public modules, classes, and functions
- Check existing docs (PRD.md / README.md / FRD.md / docstrings / type hints)

### Step 2: Create / Fix PRD.md (root workspace)

Write root-level PRD.md following `templates/PRD.md`. See `references/prd-rules.md` for rules.

### Step 3: Create / Fix FRD.md (each feature module)

For each feature module, write FRD.md following `templates/FRD.md`. See `references/frd-rules.md` for rules.

### Step 4: Create / Update README.md (root workspace)

Write root-level README.md following `templates/README.md`. See `references/readme-rules.md` for rules.

### Step 5: Add Docstrings

See `references/docstring-rules.md` for rules and templates.

### Step 6: Add Type Hints

See `references/type-hint-rules.md` for rules and patterns.

## Quick Commands

```bash
# Check files without docstrings
find modules/ -name "*.py" | while read f; do
    head -1 "$f" | grep -q '^"""' || echo "NO DOCSTRING: $f"
done

# Run mypy for type checking
python -m mypy modules/ --ignore-missing-imports
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
- Using type: ignore without reason → fix the root cause instead of suppressing errors.
