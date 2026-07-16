---
name: add-docs-python
description: "Add proper docstrings and type hints to Python files following PEP 257 and project conventions."
version: 1.0.0
category: documentation
tags:
  [
    python,
    docs,
    docstring,
    type-hints,
    pep257,
    refactoring,
  ]
triggers:
  - "add docs python"
  - "add docstring python"
  - "add type hints python"
dependencies: []
related:
  - cleanup-files-python
  - consolidate-files-python
---

# add-docs-python

## Purpose

Add proper docstrings and type hints to Python files following PEP 257 and project conventions. Ensures all modules, classes, and functions have clear documentation.

## Rules

### Docstring Style (PEP 257)

- **Module docstrings**: One-liner at top of file describing module purpose
- **Class docstrings**: One-liner describing class purpose and behavior
- **Method docstrings**: Describe purpose, parameters, return values, and exceptions

### Type Hints

- Use Python 3.5+ type hint syntax (`def foo(x: int) -> str:`)
- Import `from __future__ import annotations` for forward references
- Use `typing` module for complex types (`List`, `Dict`, `Optional`, `Union`)

## When to Use

- New files without docstrings
- Files with incomplete type hints
- Before committing changes to shared/taxonomy or contract layers

## The Fundamental Question

> **"Is this code documented and typed?"**

If no → **Add docstring and type hints**

## Workflow

### Step 1: Analyze File

Read file and identify undocumented modules, classes, and functions.

### Step 2: Add Module Docstring

Add one-liner at top of file:

```python
"""Taxonomy value objects for import rules."""
```

### Step 3: Add Class Docstrings

Add class-level documentation:

```python
class ImportRuleVO:
    """Value object representing an import rule with pattern and message."""
```

### Step 4: Add Method Docstrings

Add method documentation with parameters, returns, and raises:

```python
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

Add type annotations to all function signatures:

```python
def validate(self, data: dict[str, Any]) -> tuple[bool, str]:
    """Validate data against the import rule."""
```

## Verification Checklist

- [ ] All modules have one-liner docstrings
- [ ] All classes have descriptive docstrings
- [ ] All public methods have parameter/return documentation
- [ ] All function signatures use type hints
- [ ] Forward references use string quotes or `__future__.annotations`
- [ ] Complex types use `typing` module (`List`, `Dict`, `Optional`)

## Quick Commands

```bash
# Check files without docstrings
find modules/ -name "*.py" | while read f; do
    head -1 "$f" | grep -q "^\"\"\"" || echo "NO DOCSTRING: $f"
done

# Check for missing type hints
grep -rn "def " modules/*/src/ | grep -v ": " | head -20

# Run pyright/mypy for type checking
python -m mypy modules/ --ignore-missing-imports
```

## Common Mistakes (AVOID)

- ❌ **Missing module docstrings**: Every file needs a one-liner at the top
- ❌ **Incomplete parameter documentation**: All parameters must be documented
- ❌ **Using type: ignore without reason**: Fix the root cause instead of suppressing errors
- ❌ **Over-documenting obvious code**: Keep docstrings concise and meaningful
