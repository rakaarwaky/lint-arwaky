---
name: find-unused-files-rust
version: 1.0.0
category: validation
tags: [aes, unused, dead-code, orphan, cleanup, rust]
triggers:
  - "find unused files rust"
  - "find dead code rust"
  - "find orphan files rust"
dependencies: []
related:
  - clean-bloat
  - module_logic_validator
---

# find-unused-files-rust

## Rules

- File with 0 inbound imports = likely unused
- File with only re-exports = likely bloat
- File not referenced by any other file = candidate for deletion

## Purpose

Find files that are not imported by any other file in the crate.

## When to Use

- After refactoring a crate
- Before committing changes
- When cleaning up bloat

## The Fundamental Question

> **"Does any file import from this file?"**

If no → **Candidate for deletion (verify first)**

## Detection Method

```bash
# Find files not imported by any other file
for f in crates/*/src/*.rs; do
  name=$(basename "$f" .rs)
  refs=$(grep -rn "use.*$name" crates/*/src/*.rs | grep -v "^$f:" | wc -l)
  if [ "$refs" -eq 0 ]; then
    echo "UNUSED: $name"
  fi
done
```

## Exceptions (Keep Even If Unused)

- `lib.rs` — crate entry point
- `mod.rs` — module declarations
- `contract_*.rs` — trait definitions (may be used by external crates)
- `main.rs` — binary entry point

## Workflow

### Step 1: Scan for Unused Files

Run the detection script.

### Step 2: Verify Each Candidate

Check if file is:

- Declared in `mod.rs`
- Used by external crates
- A trait/contract definition

### Step 3: Report

List confirmed unused files.

### Step 4: Get Approval

Confirm before deletion.

### Step 5: Delete

Remove unused files and update imports.
