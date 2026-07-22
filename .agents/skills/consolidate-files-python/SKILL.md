---
name: consolidate-files-python
description: "Consolidate multiple Python files into single cohesive modules following single responsibility principle."
metadata:
  tags: [python, consolidation, single-responsibility, refactoring, structure]
  triggers:
    - "consolidate python"
    - "merge files python"
    - "combine modules python"
  dependencies: []
  related:
    - add-docs-python
    - cleanup-files-python
    - create-capabilities-python
---

# consolidate-files-python

## Purpose

Consolidate multiple Python files into single cohesive modules following single responsibility principle. Ensures each module has one clear purpose and all related code lives together.

## Rules

### Single Responsibility

- Each file should have ONE clear purpose
- Related classes/functions belong in the same file
- Unrelated code must be split into separate files

### File Organization

- Place related functionality together in modules
- Use `__init__.py` for module exports and re-exports
- Keep public API clear through `__all__`

## When to Use

- Files with scattered responsibilities
- Multiple small files that belong together
- After refactoring that split code across files

## The Fundamental Question

> **"Do these files serve the same purpose?"**

If yes → **Consolidate into single module**

## Workflow

### Step 1: Analyze File Responsibilities

Read files and identify related functionality:

```bash
# List classes/functions in files
grep -rn "^class \|^def " modules/*/src/ | sort
```

### Step 2: Identify Consolidation Candidates

Files that should be merged:

- Multiple files with related classes (e.g., `parser.py`, `parser_utils.py`)
- Files that only import from each other
- Split functionality across multiple small files

### Step 3: Merge Related Code

Move classes/functions to target file:

```python
# Before: parser.py and parser_utils.py
# After: Single parser.py with all related code
```

### Step 4: Update Imports

Fix all imports across the codebase:

```bash
# Find files importing from removed modules
grep -rn "from parser_utils import" modules/
```

### Step 5: Verify

Run syntax check and tests:

```bash
python -c "import <module>"
pytest modules/ -v
```

## Verification Checklist

- [ ] Consolidated file has single clear purpose
- [ ] All related classes/functions are in same file
- [ ] No scattered functionality across multiple files
- [ ] All imports updated to reflect new structure
- [ ] `__init__.py` exports consolidated module correctly
- [ ] Tests pass after consolidation

## Quick Commands

```bash
# Find files with related functionality
grep -rn "^class " modules/*/src/ | sort | uniq -f1

# Check for files that only import from each other
grep -rn "^from \. import \|^import \." modules/*/src/__init__.py

# Verify imports after consolidation
python -c "import <module>"
```

## Common Mistakes (AVOID)

- ❌ **Merging unrelated files**: Only consolidate files with clear shared purpose
- ❌ **Forgetting to update imports**: All references must be updated after consolidation
- ❌ **Breaking module exports**: Ensure `__init__.py` exports are maintained
