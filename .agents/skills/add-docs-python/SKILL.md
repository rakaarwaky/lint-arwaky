---
name: add-docs-python
description: "Add proper docstrings, type hints, and crate-level FRD.md/README.md to Python packages following PEP 257 and project conventions."
version: 1.1.0
category: documentation
tags: [python, docs, docstring, type-hints, frd, readme, pep257]
triggers:
  - "add docs python"
  - "add docstring python"
  - "add type hints python"
  - "add frd python"
  - "add package readme python"
dependencies: []
related:
  - cleanup-files-python
  - consolidate-files-python
---

# add-docs-python

## Rules

- Every Python package directory (e.g. `modules/<name>/`) MUST contain TWO crate-level docs: `FRD.md` and `README.md`.
- `FRD.md` is STATELESS — it describes the IDEAL TARGET only. It MUST NOT record progress, status, current-state notes, or "what's done so far". If reality diverges, fix `README.md`, never pollute `FRD.md` with state.
- `README.md` describes the REAL CURRENT STATE — what actually exists today. It is allowed (and expected) to diverge from the ideal target in `FRD.md`.
- Relationship: **FRD = ideal target, README = current reality.** README should call out gaps vs FRD; FRD must stay clean of any "as-built" noise.
- All public modules, classes, and functions MUST have docstrings (PEP 257) and type hints.
- Docstrings MUST explain "what" and "why", not "how" (code shows how).

## Purpose

Add package-level documentation and docstrings/type hints:
- `FRD.md` — stateless ideal target (Feature Goal / Requirements & Scope / Success Indicators).
- `README.md` — real current state (what exists, public API surface, known gaps vs FRD).
- PEP 257 docstrings + type hints on all public items.

## When to Use

- New package has no `FRD.md` or no `README.md`.
- `FRD.md` contains state/progress notes (violates stateless rule) — clean it.
- README and FRD are conflated (state leaking into FRD) — split them.
- Public modules/classes/functions lack docstrings or type hints.
- User asks to document the package or add docs.

## The Fundamental Question

> **"Can a newcomer understand this package's purpose in 30 seconds?"**

If no -> **Add FRD.md (ideal target) + README.md (reality).**

> **"Is this code documented and typed?"**

If no -> **Add docstring and type hints.**

## Detection Patterns

### Missing FRD.md / README.md (Create)

```
modules/<name-folder>/
├── src/
│   ├── __init__.py
│   └── ...
├── tests/
├── FRD.md        # stateless ideal target
└── README.md     # real current state
```

### Missing Docstrings / Type Hints (Add)

```python
# PURPOSE explain file in one sentence
class ImportRuleVO:
    ...

# [OK] docstring + type hints
class ImportRuleVO:
    """Value object representing an import rule with pattern and message."""
```

## FRD.md Template (STATELESS — ideal target only)

```markdown
# FRD — <package-name>

> Stateless document. Describes the IDEAL TARGET only. Never record progress,
> status, or current-state notes. If reality diverges from this, update
> README.md — do NOT add state to this file.

## Feature Goal
<One paragraph: what this package is supposed to accomplish when complete.>

## Requirements & Scope
- In scope: <...>
- Out of scope: <...>

## Success Indicators
- [ ] <measurable ideal outcome>
- [ ] <measurable ideal outcome>
```

## README.md Template (REAL current state)

```markdown
# <package-name>

> Current real state — what actually exists today. May diverge from FRD.md
> (the ideal target). Keep this honest; gaps belong here, not in FRD.

## What exists now
- <real modules / features implemented>
- <real behavior>

## Public API surface
- `<Class>` — <one-line reality of what it does>
- `<function>` — <...>

## Known gaps vs FRD
- <deviation from ideal target — what's missing or different>
```

## Workflow

### Step 1: Analyze Package

- List files in `modules/<name>/src/`
- Identify public modules, classes, and functions
- Check existing docs (README.md / FRD.md / docstrings / type hints)

### Step 2: Create / Fix FRD.md (ideal target, stateless)

Write package-level FRD.md following the FRD template. It MUST contain only:

1. Feature Goal
2. Requirements & Scope
3. Success Indicators

Strip any state, progress, or "as-built" notes. FRD is the ideal target — it never changes because code isn't done yet.

### Step 3: Create / Update README.md (reality)

Write README.md reflecting the ACTUAL current state:

1. What exists now (real modules, real behavior)
2. Public API surface (real items)
3. Known gaps vs FRD (where reality diverges from the ideal target)

README is the current reality — it changes as the package evolves.

### Step 4: Add Docstrings (PEP 257)

- **Module docstrings**: One-liner at top of file describing module purpose
- **Class docstrings**: One-liner describing class purpose and behavior
- **Function/method docstrings**: Describe purpose, parameters, return values, and exceptions

```python
"""Taxonomy value objects for import rules."""


class ImportRuleVO:
    """Value object representing an import rule with pattern and message."""


def check(self, path: str) -> bool:
    """Check if path matches the import rule.

    Args:
        path: File path to check

    Returns:
        True if path matches the rule

    Raises:
        ValueError: If path is empty
    """
```

### Step 5: Add Type Hints

- Use Python 3.5+ type hint syntax (`def foo(x: int) -> str:`)
- Import `from __future__ import annotations` for forward references
- Use `typing` module for complex types (`List`, `Dict`, `Optional`, `Union`)

```python
def validate(self, data: dict[str, Any]) -> tuple[bool, str]:
    """Validate data against the import rule."""
```

## Verification Checklist

- [ ] FRD.md exists and is stateless (no progress/state notes)
- [ ] README.md exists and reflects real current state with gaps vs FRD
- [ ] All modules have one-liner docstrings
- [ ] All classes have descriptive docstrings
- [ ] All public functions have parameter/return documentation
- [ ] All function signatures use type hints
- [ ] Forward references use string quotes or `__future__.annotations`
- [ ] Complex types use `typing` module

## Quick Commands

```bash
# Check files without docstrings
find modules/ -name "*.py" | while read f; do
    head -1 "$f" | grep -q '^"""' || echo "NO DOCSTRING: $f"
done

# Run mypy for type checking
python -m mypy modules/ --ignore-missing-imports
```

## Common Mistakes (AVOID)

- ❌ **Missing module docstrings**: Every file needs a one-liner at the top
- ❌ **Incomplete parameter documentation**: All parameters must be documented
- ❌ **Using type: ignore without reason**: Fix the root cause instead of suppressing errors
- ❌ **State leaking into FRD.md**: FRD is stateless — put reality/gaps in README
- ❌ **Over-documenting obvious code**: Keep docstrings concise and meaningful
