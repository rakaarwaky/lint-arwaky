---
name: cleanup-files-python
description: "Clean up Python files by removing unused imports, fixing formatting, and ensuring PEP 8 compliance."
version: 1.0.0
category: cleanup
tags: [python, cleanup, pep8, formatting, imports, refactoring]
triggers:
  - "cleanup python"
  - "fix formatting python"
  - "remove unused imports python"
dependencies: []
related:
  - add-docs-python
  - consolidate-files-python
---

# cleanup-files-python

## Purpose

Clean up Python files by removing unused imports, fixing formatting issues, and ensuring PEP 8 compliance. Prepares files for production use.

## Rules

### PEP 8 Compliance

- Maximum line length: 88 characters (Black default)
- Use 4 spaces for indentation (no tabs)
- Separate top-level functions/classes with two blank lines
- Separate methods within a class with one blank line

### Import Ordering

1. Standard library imports
2. Third-party imports
3. Local application imports
4. All imports must be alphabetical within each group

### Unused Code

- Remove unused imports (`import X` but never use `X`)
- Remove unused variables and functions
- Remove commented-out code blocks

## When to Use

- After refactoring files
- Before committing changes
- When cleaning up merged branches

## The Fundamental Question

> **"Is this file clean and formatted?"**

If no → **Run cleanup tools**

## Workflow

### Step 1: Remove Unused Imports

```bash
# Using pycln to remove unused imports
pycln modules/ --include ".*" --exclude "venv"
```

### Step 2: Format Code

```bash
# Using Black to format code
black modules/ --line-length 88
```

### Step 3: Check PEP 8 Compliance

```bash
# Using pycodestyle to check PEP 8
pycodestyle modules/ --max-line-length=88
```

### Step 4: Remove Commented Code

Review files for commented-out code blocks and remove them.

## Verification Checklist

- [ ] All imports are sorted alphabetically by group
- [ ] No unused imports remain
- [ ] No unused variables or functions
- [ ] All lines under 88 characters
- [ ] Proper blank line separation between classes/methods
- [ ] No commented-out code blocks

## Quick Commands

```bash
# Remove unused imports
pycln modules/ --include ".*" --exclude "venv"

# Format with Black
black modules/ --line-length 88

# Check PEP 8 compliance
pycodestyle modules/ --max-line-length=88

# Find commented code blocks
grep -rn "^#\s.*def \|^#\s.*class " modules/*/src/
```

## Common Mistakes (AVOID)

- ❌ **Keeping commented-out code**: Remove or commit properly instead of leaving comments
- ❌ **Mixing import groups**: Standard library, third-party, and local imports must be separate
- ❌ **Ignoring line length limits**: Keep lines under 88 characters for Black compatibility
