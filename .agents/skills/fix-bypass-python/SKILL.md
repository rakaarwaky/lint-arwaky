---
name: fix-bypass-python
description: "Fix Python bypass comments (type: ignore, noqa) by addressing root causes instead of suppressing errors."
metadata:
    tags: [python, bypass, comments, type-hints, refactoring, noqa]
    triggers:
        - "fix bypass python"
        - "remove noqa python"
        - "remove type ignore python"
    dependencies: []
    related:
        - cleanup-files-python
---

# fix-bypass-python

## Rules

- NO `# type: ignore` allowed without justification
- NO `# noqa` allowed without justification
- Fix the root cause instead of suppressing errors

## Purpose

Remove `type: ignore`, `noqa` comments and fix the underlying type/error issues.

## When to Use

- File has bypass comments
- Type checker reports errors that are suppressed
- Linter violations hidden by noqa

## The Fundamental Question

> **"Why is there a bypass comment?"**

If yes → **Fix root cause and remove comment**

## Workflow

### Step 1: Find Bypass Comments

Read code and find bypass comments:

```bash
# Find type ignore comments
grep -rn "type: ignore" modules/*/src/

# Find noqa comments
grep -rn "noqa" modules/*/src/
```

### Step 2: Fix Root Cause

Fix underlying type error or lint violation:

- For `type: ignore` → Add proper type annotations
- For `noqa` → Fix the linting issue (formatting, naming, etc.)

### Step 3: Remove Comment

Remove the bypass comment once root cause is fixed.

## Detection Patterns

### Type Ignore Comments

```python
# BAD: Suppressing type errors
from typing import Any

def process(data: Any) -> None:  # type: ignore
    # Fix: Add proper type annotations
    pass
```

### Noqa Comments

```python
# BAD: Suppressing linting errors
import os, sys  # noqa: F401

# Fix: Remove unused imports or address the violation
```

## Verification Checklist

- [ ] All `type: ignore` comments removed (or justified)
- [ ] All `noqa` comments removed (or justified)
- [ ] Type checker passes without errors
- [ ] Linter passes without violations

## Quick Commands

```bash
# Find type ignore comments
grep -rn "type: ignore" modules/*/src/

# Find noqa comments
grep -rn "noqa" modules/*/src/

# Run mypy to check types
python -m mypy modules/ --ignore-missing-imports

# Run pycodestyle to check linting
pycodestyle modules/ --max-line-length=88
```

## Common Mistakes (AVOID)

- ❌ **Keeping bypass comments without fixing**: Always fix the root cause
- ❌ **Adding type: ignore for wrong reasons**: Only use when type system can't express the truth
- ❌ **Suppressing legitimate errors**: Fix formatting/naming issues instead of hiding them
