---
name: cleanup-files-rust
description: "Find and remove dead code, unused files, stubs, thin wrappers, and duplicates across Rust crates to reduce bloat and improve signal-to-noise ratio."
version: 2.0.0
category: validation
tags:
  [
    rust,
    aes,
    cleanup,
    bloat,
    stubs,
    thin-wrappers,
    dead-code,
    orphan,
    unused-files,
    mvp,
    boilerplate,
  ]
triggers:
  - "clean bloat rust"
  - "remove stubs rust"
  - "remove thin wrappers rust"
  - "clean capabilities rust"
  - "find unused files rust"
  - "find dead code rust"
  - "find orphan files rust"
dependencies: []
related:
  - module_logic_validator-rust
  - consolidate-files-rust
---

# cleanup-rust

## Purpose

Find and remove dead code across Rust crates. This skill combines **file-level cleanup** (unused files, orphaned modules) and **function-level cleanup** (stubs, thin wrappers, duplicates, overengineered patterns not in MVP scope). The goal is to maximize signal-to-noise ratio by eliminating anything NOT required by the current MVP scope.

**CRITICAL: Never Remove Real Logic** — Only remove code that serves no purpose in the current MVP scope. If a function is called by another method that's required by MVP, keep it. Always update traits when removing methods. Always run lint after changes.

## Rules

- **Never remove real logic** — only remove code not relevant to MVP scope
- **Always update trait** — when removing methods from impl, remove from trait too
- **Always run lint after changes** — verify no compilation errors or regressions
- **File with 0 inbound imports** = likely unused (verify first)
- **File with only re-exports** = likely bloat (consider consolidation)
- **File not referenced by any other file** = candidate for deletion

## When to Use

- After refactoring capability modules
- Before committing capability changes
- When user asks to clean bloat from a module
- After refactoring a crate (find orphaned files)
- When cleaning up accumulated dead code

## The Fundamental Question

Before keeping any function or file, ask:

> **"Why does this function/file need to exist?"**

If the answer is:

- "Because it was always there" → **REMOVE**
- "Because it might be useful someday" → **REMOVE**
- "Because it handles edge cases we don't have" → **REMOVE**
- "Because it's required by MVP" → **KEEP**
- "Because it's called by a method that's required by MVP" → **KEEP**

---

## Detection Patterns: Function-Level Bloat

### Thin Wrappers (Remove)

```rust
// Simple attribute return
fn get_something(&self, obj: &Obj) -> f64 {
    obj.attribute
}
// WHY: Direct attribute access is simpler. No logic added.

// Simple enum comparison
fn should_force_x(&self, hint: &ActionHint) -> bool {
    *hint == ActionHint::X
}
// WHY: Comparison is already simple. Wrapper adds no value.
```

### Stubs (Remove)

```rust
fn method(&self) -> Option<()> { None }
fn method(&self) -> String { String::new() }
// WHY: Empty implementations provide no value.
```

### Duplicate Functions (Remove)

Same function in multiple capability files — keep in the file that owns the logic.
WHY: Duplicates create maintenance burden. Single source of truth.

### Overengineered Patterns (Remove)

```rust
// Temporal enforcer, circular dependency detection, etc.
// if NOT in MVP → REMOVE
// WHY: Complexity without clear MVP requirement is waste.
```

---

## Detection Patterns: File-Level Orphans

### Unused Files

Files not imported by any other file in the crate:

```rust
// Example: capabilities_orphan_feature.rs never imported
crates/my-crate/src/capabilities_orphan_feature.rs  // 0 inbound refs
```

### Re-Export Only Files

Files that only re-export from another module — bloat if the re-export adds no value:

```rust
// capabilities_reexport.rs
pub use super::capabilities_real_impl::MyStruct;
// WHY: Just a passthrough. Consolidate into the real impl file.
```

---

## Exceptions (Keep Even If Unused)

- `lib.rs` — crate entry point
- `mod.rs` — module declarations
- `contract_*.rs` — trait definitions (may be used by external crates)
- `main.rs` — binary entry point
- Taxonomy utility files (referenced by any layer)

---

## Workflow

### Step 1: Read Requirements

Read the requirements to understand MVP scope.

### Step 2: Scan for Unused Files

Run detection script to find files not imported by any other file:

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

### Step 3: List Capability Files and Analyze Each

List all capability/trait files. For each file, ask "Why does each function need to exist?"

### Step 4: Mark for Removal

If answer is not "required by MVP" → mark for removal. Check both:

- **Function-level**: stubs, thin wrappers, duplicates, overengineered patterns
- **File-level**: unused files, re-export-only files

### Step 5: Report

Report per file — show what to keep/remove. Group findings into:

| Category           | What It Is                                  | Action                           |
| ------------------ | ------------------------------------------- | -------------------------------- |
| **Stubs**          | Empty or near-empty methods                 | Remove                           |
| **Thin Wrappers**  | Direct attribute access, simple comparisons | Remove                           |
| **Duplicates**     | Same function in multiple files             | Keep in owning file, remove rest |
| **Overengineered** | Patterns not in MVP scope                   | Remove                           |
| **Unused Files**   | 0 inbound imports                           | Delete (after verification)      |
| **Re-export Only** | Files with only re-exports                  | Consolidate into real impl       |

### Step 6: Get Approval

Get approval per file before making changes.

### Step 7: Execute Cleanup

Remove bloat, update traits, delete unused files:

```bash
# Remove unused file(s)
rm crates/<crate>/src/capabilities_orphan_feature.rs

# Update trait definitions (remove removed methods)
# Update mod.rs (remove removed modules)
```

---

## Verification Checklist

- [ ] Requirements read and MVP scope understood
- [ ] Unused files scanned with detection script
- [ ] Each function evaluated against MVP scope
- [ ] Report generated showing keep/remove per file
- [ ] Approval received before making changes
- [ ] Traits updated when methods removed
- [ ] mod.rs updated when modules deleted
- [ ] `cargo check -p <crate-name>` passes without errors
- [ ] Lint runs clean after changes

## Quick Commands

```bash
# Find unused files (0 inbound imports)
for f in crates/*/src/*.rs; do
  name=$(basename "$f" .rs)
  refs=$(grep -rn "use.*$name" crates/*/src/*.rs | grep -v "^$f:" | wc -l)
  if [ "$refs" -eq 0 ] && [[ ! "$name" =~ ^(lib|mod|main)$ ]]; then
    echo "UNUSED: $name in $f"
  fi
done

# Find stubs (methods returning None or empty strings)
grep -rn "None }\|String::new()\|vec!\[\]" crates/*/src/ | grep -E "fn [a-z_]+\(&self\)" | head -30

# Find thin wrappers (single-expression methods)
grep -rn "^    fn .*->.*{ [a-z.]* }" crates/*/src/ | head -20

# Find duplicate functions across files
grep -rn "^    pub fn " crates/*/src/ | cut -d: -f2-3 | sort | uniq -wD | head -20

# Verify compilation after cleanup
cargo check -p <crate-name> 2>&1 | grep -E "error|cannot find"
```

## Common Mistakes (AVOID)

- ❌ **Removing real MVP logic**: If a function is called by another method required by MVP, keep it.
- ❌ **Forgetting to update traits**: When removing methods from impl blocks, also remove them from trait definitions.
- ❌ **Deleting files without checking mod.rs**: After deleting a file, update `mod.rs` to remove the module declaration.
- ❌ **Removing contract/trait files**: Contract layer files are intentionally unused by direct imports — they're interfaces.
- ❌ **Skipping lint verification**: Always run `cargo check` and lint after cleanup changes.
