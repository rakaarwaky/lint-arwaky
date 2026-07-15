---
name: find-unused-files
version: 1.0.0
category: validation
tags: [aes, unused, dead-code, orphan, cleanup]
triggers:
  - "find unused files"
  - "find dead code"
  - "find orphan files"
  - "check unused imports"
dependencies: []
related:
  - clean-bloat
  - module_logic_validator
---

# find-unused-files

## Rules

- File with 0 inbound imports = likely unused
- File with only re-exports = likely bloat
- File not referenced by any other file = candidate for deletion

## Purpose

Find files that are not imported by any other file in the module.

## When to Use

- After refactoring a module
- Before committing changes
- When cleaning up bloat

## The Fundamental Question

> **"Does any file import from this file?"**

If no → **Candidate for deletion (verify first)**

## Detection Method

```bash
# Find files not imported by any other file
for f in module/src/*.py; do
  name=$(basename "$f" .py)
  refs=$(grep -rn "$name" module/src/*.py | grep -v "^$f:" | wc -l)
  if [ "$refs" -eq 0 ]; then
    echo "UNUSED: $name"
  fi
done
```

## Exceptions (Keep Even If Unused)

- `__init__.py` — module entry point
- `contract_*.py` — protocol definitions (may be used by external modules)
- Files exported by `__init__.py` — public API

## Workflow

### Step 1: Scan for Unused Files
Run the detection script.

### Step 2: Verify Each Candidate
Check if file is:
- Exported by `__init__.py`
- Used by external modules
- A protocol/contract definition

### Step 3: Report
List confirmed unused files.

### Step 4: Get Approval
Confirm before deletion.

### Step 5: Delete
Remove unused files and update imports.
