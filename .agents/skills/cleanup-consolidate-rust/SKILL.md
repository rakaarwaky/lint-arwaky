---
name: cleanup-consolidate-rust
description: "Find and remove dead code, unused files, stubs, thin wrappers, and duplicates across Rust crates, then merge overlapping files into single cohesive modules."
metadata:
  tags:
    [
      rust,
      cleanup,
      consolidation,
      bloat,
      stubs,
      thin-wrappers,
      dead-code,
      orphan,
      unused-files,
      merge,
      deduplication,
      single-file,
      single-struct,
      aes,
    ]
  triggers:
    - "cleanup rust"
    - "clean bloat rust"
    - "remove stubs rust"
    - "remove thin wrappers rust"
    - "find unused files rust"
    - "find dead code rust"
    - "remove dead code rust"
    - "cleanup crate rust"
    - "merge two files into one"
    - "combine two impl files"
    - "consolidate files"
    - "merge capabilities files"
    - "merge agent files"
    - "merge overlap rust"
    - "deduplicate modules rust"
  dependencies: []
  related:
    - add-docs-rust
    - create-capabilities-rust
    - create-agent-rust
---

# cleanup-consolidate-rust

## Purpose

Unified Rust codebase cleanup skill combining **dead code removal** and **file consolidation**. First find and remove dead code, unused files, stubs, thin wrappers, and duplicates. Then detect overlapping files that share the same domain and merge them into single cohesive modules. The result is a cleaner codebase with fewer files, less bloat, and maximum signal-to-noise ratio.

**CRITICAL: Two-Phase Approach** — Phase 1 removes dead code. Phase 2 merges overlapping files. Never skip Phase 1 — consolidating files with dead code wastes effort.

---

## Rules

- **Never remove real logic** — only remove code not relevant to FRD scope
- **Always update trait** — when removing methods from impl, remove from trait too
- **Always run lint after changes** — verify no compilation errors or regressions
- **Always snapshot before cleanup** — git commit or stash before any deletion
- **File with 0 inbound references** = likely unused (verify with multi-pattern check)
- **File with only re-exports** = likely bloat (consider consolidation)
- **File not referenced by any other file, test, or build script** = candidate for deletion
- **Respect `#[allow(dead_code)]`** — investigate intent before removing
- **Respect `#[cfg(...)]` gates** — code behind feature flags or test cfg is NOT dead
- **One Struct Per File** (consolidation): merge two impl files into single file with single struct
- **Target Selection**: keep file with most logic as target; move unique functions from source files into target

---

## When to Use

- After refactoring capability modules
- Before committing capability changes
- When user asks to clean bloat from a module
- After refactoring a crate (find orphaned files)
- When cleaning up accumulated dead code
- Before release (final bloat pass)
- Two impl files share the same domain and can be unified
- Multiple files implement the same concept (e.g., 7 coordinate transform files)
- Multiple files handle the same feature (e.g., cursor drawer + cursor renderer)
- Multiple adapter files for the same technology (e.g., 3 FFmpeg adapters)

---

## The Fundamental Questions

### For Cleanup (Phase 1)

Before keeping any function or file, ask:

> **"Why does this function/file need to exist?"**

| Answer | Verdict |
| ------------------------------------------------------------- | ---------- |
| "Because it was always there" | **REMOVE** |
| "Because it might be useful someday" | **REMOVE** |
| "Because it handles edge cases we don't have" | **REMOVE** |
| "Because it's required by FRD" | **KEEP** |
| "Because it's called by a method required by FRD" | **KEEP** |
| "Because it's behind a feature flag we still ship" | **KEEP** |
| "Because it's used by tests that validate FRD behavior" | **KEEP** |
| "Because a proc macro / derive generates code referencing it" | **KEEP** |
| "Because `build.rs` or integration tests reference it" | **KEEP** |

### For Consolidation (Phase 2)

> **"Do these files do the same thing or share the same domain?"**

If yes → **Merge them into 1 file**

---

## Phase 1: Dead Code Cleanup

### Detection Patterns: Function-Level Bloat

#### Thin Wrappers (Remove)

```rust
// ❌ Simple attribute return — direct access is simpler
fn get_something(&self, obj: &Obj) -> f64 {
    obj.attribute
}

// ❌ Simple enum comparison — comparison is already trivial
fn should_force_x(&self, hint: &ActionHint) -> bool {
    *hint == ActionHint::X
}

// ❌ Single-field delegation — no logic added
fn name(&self) -> &str {
    &self.inner.name
}
```

**Exception — KEEP thin wrappers when:**

- They are part of a public trait implementation (removing breaks the trait contract)
- They add documentation value (`/// Converts meters to kilometers`)
- They are the sole implementation of a trait method used polymorphically

#### Stubs (Remove)

```rust
// ❌ Empty implementations providing no value
fn method(&self) -> Option<()> { None }
fn method(&self) -> String { String::new() }
fn method(&self) -> Vec<Item> { vec![] }
fn method(&self) -> Result<(), Error> { Ok(()) }
fn method(&self) -> bool { false }
fn method(&self) -> i32 { 0 }
```

**Exception — KEEP stubs when:**

- They are required by a trait definition that external crates implement
- They are placeholder for a confirmed next-sprint FRD item (add `// TODO(FRD-XXX): implement` comment)

#### Duplicate Functions (Remove)

Same function logic in multiple capability files — keep in the file that **owns the domain logic**.

```rust
// ❌ In capabilities_movement.rs AND capabilities_physics.rs:
fn clamp_velocity(v: f64, max: f64) -> f64 {
    v.clamp(-max, max)
}
// KEEP in the file that owns velocity logic. Remove from the other.
```

**Detection:** Match on function body similarity, not just name. Two functions with different names but identical bodies are also duplicates.

#### Overengineered Patterns (Remove)

```rust
// ❌ Temporal enforcer, circular dependency detection, plugin registries, etc.
// if NOT in MVP → REMOVE
```

**3-Point Decision Test — ALL must be true to remove:**

1. ✅ The pattern is **NOT referenced** in any FRD requirement document
2. ✅ Removing it does **NOT break** any existing test (`cargo test` passes)
3. ✅ The pattern adds **>20 lines** of code for **<3 lines** of actual consumed logic

If **any** check fails → **KEEP** and add comment: `// REVIEW: candidate for removal post-MVP`

### Detection Patterns: File-Level Orphans

#### Unused Files

Files not imported, declared, or referenced by any other file in the crate:

```
crates/my-crate/src/capabilities_orphan_feature.rs  // 0 inbound refs
```

#### Re-Export Only Files

Files that only re-export from another module — bloat if the re-export adds no value:

```rust
// ❌ capabilities_reexport.rs — just a passthrough
pub use super::capabilities_real_impl::MyStruct;
pub use super::capabilities_real_impl::MyTrait;
// WHY: Consolidate into the real impl file or into mod.rs directly.
```

**Exception — KEEP re-export files when:**

- They form a deliberate public API surface (`pub use` in `lib.rs` pattern)
- Multiple downstream crates import from the re-export path (changing would be a breaking change)

### Exceptions (NEVER Remove Without Explicit Approval)

| File/Pattern | Reason |
| ---------------------------------------------------- | ------------------------------------------------------------ |
| `lib.rs` | Crate entry point |
| `mod.rs` | Module declarations |
| `main.rs` | Binary entry point |
| `contract_*.rs` / `traits.rs` | Trait definitions (may be used by external crates) |
| `build.rs` | Build script |
| Files behind `#[cfg(feature = "...")]` | Conditionally compiled — verify feature is truly deprecated |
| `#[cfg(test)]` modules / `tests/` directory | Test code — check `cargo test` not just `cargo check` |
| Files referenced by `build.rs` | Build-time code generation |
| Files referenced by integration tests (`tests/*.rs`) | Not visible from `src/` imports |
| Files referenced by proc macros / derive macros | Invisible to grep — referenced via macro expansion |
| Items with `#[allow(dead_code)]` | Developer explicitly marked as intentional — investigate WHY |
| Taxonomy / utility files referenced by any layer | Cross-cutting concerns |

### Phase 1 Workflow

#### Step 1.1: Safety Snapshot

```bash
# ALWAYS do this first — non-negotiable
git add -A && git commit -m "pre-cleanup snapshot: <crate-name>" --allow-empty
git checkout -b cleanup/<crate-name>-$(date +%Y%m%d)
```

If anything goes wrong:

```bash
git checkout main
git branch -D cleanup/<crate-name>-$(date +%Y%m%d)
# Or restore specific files:
git checkout HEAD~1 -- crates/<crate>/src/<file>.rs
```

#### Step 1.2: Read Requirements

Read the FRD / requirements document to understand MVP scope. List all required capabilities, traits, and behaviors.

#### Step 1.3: Run Primary Detection (Tooling)

Use Rust-native tooling FIRST — it understands cfg, macros, and the module system:

```bash
# Primary: cargo clippy dead code detection
cargo clippy -p <crate-name> --all-features -- -W dead_code -W unused_imports -W unused_variables 2>&1 | tee /tmp/clippy_report.txt

# Secondary: cargo-udeps (finds unused dependencies and unreachable modules)
cargo udeps -p <crate-name> --all-features 2>&1 | tee /tmp/udeps_report.txt

# Tertiary: cargo check with all features (catches cfg-gated code)
cargo check -p <crate-name> --all-features 2>&1 | tee /tmp/check_report.txt

# Test compilation (catches test-only references)
cargo test -p <crate-name> --no-run --all-features 2>&1 | tee /tmp/test_report.txt
```

#### Step 1.4: Run Secondary Detection (File-Level Scan)

Multi-pattern scan for files not referenced anywhere:

```bash
#!/usr/bin/env bash
# find_unused_files.sh — comprehensive orphan detection
CRATE_DIR="crates/<crate-name>/src"

for f in "$CRATE_DIR"/*.rs "$CRATE_DIR"/**/*.rs; do
  [ -f "$f" ] || continue
  name=$(basename "$f" .rs)

  # Skip protected files
  [[ "$name" =~ ^(lib|mod|main|build)$ ]] && continue
  [[ "$name" =~ ^contract_ ]] && continue

  # Check ALL reference patterns:
  refs=0
  refs=$((refs + $(grep -rnE "(mod|pub mod)\s+${name}\s*;" "$CRATE_DIR" | grep -v "^$f:" | wc -l)))
  refs=$((refs + $(grep -rnE "use\s+.*\b${name}\b" "$CRATE_DIR" | grep -v "^$f:" | wc -l)))
  refs=$((refs + $(grep -rnE "(crate|super|self)::${name}\b" "$CRATE_DIR" | grep -v "^$f:" | wc -l)))
  refs=$((refs + $(grep -rnE "\b${name}\b" crates/<crate-name>/build.rs 2>/dev/null | wc -l)))
  refs=$((refs + $(grep -rnE "\b${name}\b" crates/<crate-name>/tests/ 2>/dev/null | wc -l)))

  parent_dir=$(dirname "$f")
  glob_refs=$(grep -rnE "use\s+(super|self)::\*" "$parent_dir" 2>/dev/null | grep -v "^$f:" | wc -l)

  if [ "$refs" -eq 0 ] && [ "$glob_refs" -eq 0 ]; then
    echo "UNUSED: $f (0 references, 0 glob imports in parent)"
  elif [ "$refs" -eq 0 ] && [ "$glob_refs" -gt 0 ]; then
    echo "MAYBE_UNUSED: $f (0 direct refs, but $glob_refs glob import(s) in parent — verify manually)"
  fi
done
```

#### Step 1.5: Detect Function-Level Bloat

```bash
# Find stubs (methods returning trivial values)
grep -rnP "fn\s+\w+\s*\([^)]*\)\s*(->\s*\S+)?\s*\{\s*(None|Some\(\(\)\)|String::new\(\)|vec!\[\]|Ok\(\(\)\)|false|0|Default::default\(\))\s*\}" \
  "$CRATE_DIR" | head -40

# Find thin wrappers (single-expression bodies, multi-line aware)
rg -U "fn\s+\w+\s*\([^)]*\)[^{]*\{\s*\n\s*(self\.\w+|&self\.\w+|\*\w+\s*==\s*\S+)\s*\n\s*\}" \
  "$CRATE_DIR" | head -30

# Find duplicate function names across files
grep -rn "^\s*pub fn \|^\s*fn " "$CRATE_DIR" | \
  sed 's/.*fn \([a-z_0-9]*\).*/\1/' | sort | uniq -d | while read dup; do
    echo "DUPLICATE: $dup"
    grep -rn "fn ${dup}" "$CRATE_DIR"
    echo "---"
  done

# Find #[allow(dead_code)] items (investigate, don't auto-remove)
grep -rn "#\[allow(dead_code)\]" "$CRATE_DIR" | head -20

# Find cfg-gated code (DO NOT remove without verifying feature status)
grep -rn "#\[cfg(feature" "$CRATE_DIR" | head -20
grep -rn "#\[cfg(test)\]" "$CRATE_DIR" | head -20
```

#### Step 1.6: Analyze and Categorize

For each flagged item, apply **The Fundamental Question**. Categorize findings:

| Category | What It Is | Action | Confidence |
| -------------------- | ------------------------------------------- | -------------------------------- | --------------- |
| **Stubs** | Empty or trivial-return methods | Remove | High |
| **Thin Wrappers** | Direct attribute access, simple comparisons | Remove (unless trait impl) | High |
| **Duplicates** | Same logic in multiple files | Keep in owning file, remove rest | High |
| **Overengineered** | Patterns failing 3-point test | Remove | Medium — verify |
| **Unused Files** | 0 inbound refs (all patterns checked) | Delete | High |
| **Re-export Only** | Files with only `pub use` passthrough | Consolidate | Medium |
| **Maybe Unused** | 0 direct refs but glob import in parent | Manual review | Low — verify |
| **cfg-gated** | Behind `#[cfg(feature/test)]` | KEEP unless feature deprecated | N/A |
| **allow(dead_code)** | Explicitly marked by developer | Investigate intent | Low — ask |

#### Step 1.7: Report Phase 1

Generate a per-file report:

```markdown
## Cleanup Report: <crate-name>

### Summary

- Files scanned: X
- Functions analyzed: Y
- Items flagged for removal: Z
- Estimated lines removed: N

### Per-File Findings

#### `capabilities_movement.rs`

| Item               | Type         | Lines | Verdict | Reason                             |
| ------------------ | ------------ | ----- | ------- | ---------------------------------- |
| `get_velocity()`   | Thin wrapper | 3     | REMOVE  | Direct `self.velocity` access      |
| `clamp_velocity()` | Duplicate    | 5     | REMOVE  | Owned by `capabilities_physics.rs` |
| `apply_force()`    | Real logic   | 22    | KEEP    | Required by FRD-012                |

#### `capabilities_orphan_feature.rs`

| Item        | Type        | Lines | Verdict | Reason                                        |
| ----------- | ----------- | ----- | ------- | --------------------------------------------- |
| Entire file | Unused file | 87    | DELETE  | 0 inbound refs, no glob imports, not in tests |

### Items Requiring Manual Review

- `utils_temporal.rs` — `#[allow(dead_code)]` on 3 items. Developer intent unclear.
- `capabilities_experimental.rs` — Behind `#[cfg(feature = "experimental")]`. Is feature deprecated?
```

#### Step 1.8: Get Approval for Phase 1

Present report to user. Get **explicit per-file approval** before making changes.

For "Maybe Unused" and "cfg-gated" items, require **explicit confirmation** — do not batch-remove.

#### Step 1.9: Execute Phase 1 Cleanup

```bash
# Remove unused file(s)
rm crates/<crate>/src/capabilities_orphan_feature.rs

# Update mod.rs — remove module declaration
sed -i '/mod capabilities_orphan_feature;/d' crates/<crate>/src/mod.rs

# Update trait definitions — remove removed methods
# (Manual: open trait file, delete method signatures matching removed impls)

# Remove thin wrappers / stubs from impl blocks
# (Manual: edit file, remove function, update trait if applicable)
```

#### Step 1.10: Verify Phase 1

```bash
# Compilation check (all features to catch cfg-gated breakage)
cargo check -p <crate-name> --all-features 2>&1 | grep -E "^error"

# Test compilation
cargo test -p <crate-name> --no-run --all-features 2>&1 | grep -E "^error"

# Full test run (if fast enough)
cargo test -p <crate-name> --all-features 2>&1 | tail -5

# Clippy clean
cargo clippy -p <crate-name> --all-features -- -D warnings 2>&1 | grep -E "^error|^warning"

# Check downstream crates that depend on this one
cargo check --workspace --all-features 2>&1 | grep -E "^error"
```

---

## Phase 2: File Consolidation

### Detection Patterns: Same-Concept Files (Merge)

```rust
capabilities_world_to_camera.rs
capabilities_camera_to_world.rs
capabilities_camera_to_viewport.rs
// All do coordinate transforms → merge into capabilities_coordinate_mapper.rs
```

### Detection Patterns: Same-Feature Files (Merge)

```rust
capabilities_brush_cursor_drawer.rs
capabilities_drag_cursor_drawer.rs
capabilities_cursor_data_renderer.rs
// All render cursors → merge into capabilities_cursor_renderer.rs
```

### Detection Patterns: Same-Technology Adapters (Merge)

```rust
utility_ffmpeg_adapter.rs
utility_video_ffmpeg_adapter.rs
// Both use FFmpeg → merge into 1 adapter
```

### The Consolidation Pattern

#### Before Merge (Two Files)

```
crates/<crate>/src/capabilities_<name1>.rs
  - StructA implements TraitA
  - Fields: field_a, field_b
  - Methods: method_a, helper_a

crates/<crate>/src/capabilities_<name2>.rs
  - StructB implements TraitB
  - Fields: field_c, field_d
  - Methods: method_b, helper_b
```

#### After Merge (One File)

```rust
use async_trait::async_trait;
use shared::...;

/// Unified struct combining StructA and StructB for [domain description].
pub struct UnifiedStruct {
    // Fields from BOTH old structs (merge all fields)
    field_a: TypeA,
    field_b: TypeB,
    field_c: TypeC,
    field_d: TypeD,
}

#[async_trait]
impl TraitA for UnifiedStruct {
    fn method_a(&self, ...) -> ... {
        self.do_method_a(...)  // wrapper calls do_* method
    }

    fn do_method_a(&self, ...) -> ... {
        // merged logic from old StructA
    }
}

#[async_trait]
impl TraitB for UnifiedStruct {
    fn method_b(&self, ...) -> ... {
        self.do_method_b(...)  // wrapper calls do_* method
    }

    fn do_method_b(&self, ...) -> ... {
        // merged logic from old StructB
    }
}

// Free functions — keep as standalone or make methods
fn helper_a(...) -> ... { ... }
fn helper_b(...) -> ... { ... }
```

### Phase 2 Workflow

#### Step 2.1: Detect Overlaps and Analyze Files

Group files by concept/feature/technology. Read each file to understand:

- What structs/classes exist
- What traits they implement
- What fields each struct has
- What methods each impl block has
- What free functions exist
- What imports are used

```bash
# Group files by capability name pattern
ls crates/<crate>/src/capabilities_*.rs

# Analyze both files
wc -l crates/<crate>/src/file1.rs crates/<crate>/src/file2.rs
grep -c "^pub struct" crates/<crate>/src/file1.rs
grep -c "^    fn \|^    pub fn " crates/<crate>/src/file1.rs
```

#### Step 2.2: Pick Target File

Select the file with the most logic (most lines, most methods, most fields) as the merge target.

#### Step 2.3: Merge Imports

Combine imports from all files, remove duplicates:

```rust
// From file1 + file2 — deduplicated
use async_trait::async_trait;
use shared::common::...;
use shared::import_rules::...;
use std::collections::{HashMap, HashSet};
```

#### Step 2.4: Merge Structs

Combine fields from all old structs into one struct:

```rust
pub struct UnifiedStruct {
    // Fields from StructA
    field_a: TypeA,
    field_b: TypeB,

    // Fields from StructB
    field_c: TypeC,
    field_d: TypeD,
}
```

**Merge carefully**: If both structs have the same field (e.g., `_config`), keep only one.

#### Step 2.5: Merge Impl Blocks

Put ALL methods into impl blocks. If multiple traits exist, create separate impl blocks for each trait.

**For each trait:**

- Trait method (public) → wrapper calling `do_*` method
- Internal implementation → `do_*` prefix

```rust
impl TraitA for UnifiedStruct {
    fn public_method(&self, ...) -> ... {
        self.do_public_method(...)  // calls internal method
    }

    fn do_public_method(&self, ...) -> ... {
        // actual logic from old StructA
    }
}

impl TraitB for UnifiedStruct {
    fn public_method(&self, ...) -> ... {
        self.do_public_method(...)  // calls internal method
    }

    fn do_public_method(&self, ...) -> ... {
        // actual logic from old StructB
    }
}
```

#### Step 2.6: Merge Free Functions

Keep free functions as standalone (outside impl block) or convert to methods:

```rust
// Option A: Keep as standalone free functions
fn helper_a(...) -> ... { ... }
fn helper_b(...) -> ... { ... }

// Option B: Convert to methods (if they need self)
impl UnifiedStruct {
    fn do_helper_a(&self, ...) -> ... { ... }
    fn do_helper_b(&self, ...) -> ... { ... }
}
```

#### Step 2.7: Update All References

Find and update ALL references across the codebase:

```bash
# Find all references to old names
grep -r "OldStructA\|OldStructB\|TraitA\|TraitB" crates/

# Update lib.rs exports
# Update root container wiring
# Update test files
```

#### Step 2.8: Delete Source File(s)

Remove the file(s) whose functionality was merged:

```bash
rm crates/<crate>/src/file2.rs
```

#### Step 2.9: Verify Phase 2

```bash
cargo check -p <crate-name> 2>&1 | grep -E "error|cannot find"
```

---

## Final Verification (Both Phases)

```bash
# Compilation check
cargo check -p <crate-name> --all-features 2>&1 | grep -E "^error"

# Test compilation
cargo test -p <crate-name> --no-run --all-features 2>&1 | grep -E "^error"

# Full test run
cargo test -p <crate-name> --all-features 2>&1 | tail -5

# Clippy clean
cargo clippy -p <crate-name> --all-features -- -D warnings 2>&1 | grep -E "^error|^warning"

# Check downstream crates
cargo check --workspace --all-features 2>&1 | grep -E "^error"
```

---

## Commit

```bash
git add -A
git commit -m "cleanup(<crate-name>): remove N dead items + merge M files (K lines)

Removed:
- X stubs
- Y thin wrappers
- Z duplicate functions
- W unused files

Consolidated:
- A files merged into B files

All cargo check/test/clippy passing with --all-features."
```

---

## Verification Checklist

### Phase 1: Dead Code Cleanup

- [ ] Git snapshot created before any changes
- [ ] Working on dedicated cleanup branch
- [ ] FRD / requirements read and MVP scope understood
- [ ] `cargo clippy --all-features` run as primary detection
- [ ] File-level scan uses multi-pattern detection (mod, use, path, glob, build.rs, tests)
- [ ] Each function evaluated against Fundamental Question
- [ ] `#[cfg(feature)]` and `#[cfg(test)]` items NOT auto-removed
- [ ] `#[allow(dead_code)]` items investigated, not auto-removed
- [ ] Proc macro / derive macro references checked
- [ ] Integration tests (`tests/`) checked for references
- [ ] Report generated showing keep/remove per file with reasons
- [ ] Approval received before making changes
- [ ] Traits updated when methods removed from impl
- [ ] `mod.rs` updated when modules deleted
- [ ] `cargo check -p <crate> --all-features` passes
- [ ] `cargo test -p <crate> --all-features` passes
- [ ] `cargo clippy -p <crate> --all-features -- -D warnings` passes
- [ ] `cargo check --workspace --all-features` passes (downstream crates)

### Phase 2: File Consolidation

- [ ] Files analyzed and overlaps confirmed
- [ ] Target file selected (most logic)
- [ ] Imports merged and deduplicated
- [ ] Structs combined into one struct with all fields
- [ ] All methods moved to impl blocks (trait impl + inherent impl)
- [ ] Free functions kept as standalone or converted to methods
- [ ] Source file(s) deleted
- [ ] All references updated (lib.rs, root container, tests)
- [ ] `cargo check -p <crate-name>` passes without warnings or errors

### Final

- [ ] Committed with descriptive message

---

## Quick Reference Commands

```bash
# === PHASE 1: PRIMARY DETECTION ===
cargo clippy -p <crate> --all-features -- -W dead_code -W unused_imports 2>&1
cargo udeps -p <crate> --all-features 2>&1

# === PHASE 1: FILE-LEVEL ORPHAN SCAN ===
# (Use the full script from Step 1.4 above)

# === PHASE 1: FUNCTION-LEVEL BLOAT ===
# Stubs:
rg "fn\s+\w+\([^)]*\)\s*(->\s*\S+)?\s*\{\s*(None|String::new|vec!\[\]|Ok\(\(\)\)|false|0)\s*\}" crates/<crate>/src/

# Thin wrappers (multiline):
rg -U "fn\s+\w+\([^)]*\)[^{]*\{\s*\n\s*(self\.\w+|&self\.\w+)\s*\n\s*\}" crates/<crate>/src/

# Duplicates:
grep -rn "fn " crates/<crate>/src/ | sed 's/.*fn \([a-z_0-9]*\).*/\1/' | sort | uniq -d

# cfg-gated code (DO NOT REMOVE):
rg "#\[cfg\(" crates/<crate>/src/

# allow(dead_code) (INVESTIGATE):
rg "#\[allow\(dead_code\)\]" crates/<crate>/src/

# === PHASE 2: OVERLAP DETECTION ===
ls crates/<crate>/src/capabilities_*.rs | xargs -n1 basename | sort
wc -l crates/<crate>/src/file1.rs crates/<crate>/src/file2.rs
grep -c "^pub struct" crates/<crate>/src/file1.rs
grep -c "^    fn \|^    pub fn " crates/<crate>/src/file1.rs

# === VERIFICATION ===
cargo check -p <crate> --all-features 2>&1 | grep "^error"
cargo test -p <crate> --all-features 2>&1 | tail -3
cargo clippy -p <crate> --all-features -- -D warnings 2>&1 | grep "^error"
cargo check --workspace --all-features 2>&1 | grep "^error"

# === ROLLBACK ===
git checkout HEAD~1 -- crates/<crate>/src/<file>.rs   # restore one file
git reset --hard HEAD~1                                  # nuclear option
```

---

## Common Mistakes (AVOID)

| Mistake | Why It's Dangerous | Prevention |
| -------------------------------------------------- | --------------------------------------------------------- | ----------------------------------------------- |
| Removing real MVP logic | Breaks required functionality | Fundamental Question + FRD cross-reference |
| Forgetting to update traits | Compilation errors in downstream crates | Always edit trait file when editing impl |
| Deleting files without updating `mod.rs` | Compilation error: "file not found for module" | Checklist item; grep for `mod <name>;` |
| Removing `contract_*.rs` / trait files | Breaks external crate consumers | Exception list; check `Cargo.toml` dependents |
| Skipping `--all-features` in verification | Misses breakage in cfg-gated code | Always use `--all-features` in check/test/clippy |
| Removing `#[cfg(test)]` code | Breaks `cargo test` | Run `cargo test --no-run` as verification step |
| Removing code behind `#[cfg(feature)]` | Breaks feature-gated builds | Check `Cargo.toml` `[features]` section first |
| Ignoring glob imports (`use super::*`) | File appears unused but is imported via glob | Check parent module for `*` imports |
| Ignoring proc macro / derive references | File is referenced via macro expansion, invisible to grep | Check `#[derive(...)]` and proc macro crates |
| Skipping git snapshot | Cannot rollback if cleanup breaks something | Step 1.1 is non-negotiable |
| Batch-removing "Maybe Unused" items | Glob imports or macros may reference them | Require manual review + explicit approval |
| Removing `#[allow(dead_code)]` items without asking | Developer had a reason to mark it | Investigate git blame / ask author |
| Consolidating files with dead code | Wastes effort merging code that should be deleted | Always run Phase 1 before Phase 2 |
| Forgetting to update lib.rs exports after merge | Compilation error: "unresolved import" | Grep for old module names after merge |
| Leaving orphan references after merge | Runtime errors from stale imports | Grep for old struct/trait names after merge |

---

## Decision Flowchart

```
START
│
├─ PHASE 1: DEAD CODE CLEANUP
│  │
│  ├─ Item flagged for removal
│  │  │
│  │  ├─ Is it in the Exceptions list?
│  │  │  └─ YES → KEEP (stop)
│  │  │
│  │  ├─ Is it behind #[cfg(feature/test)]?
│  │  │  └─ YES → KEEP unless feature is confirmed deprecated (stop)
│  │  │
│  │  ├─ Does it have #[allow(dead_code)]?
│  │  │  └─ YES → Investigate intent. Ask author. Do NOT auto-remove. (stop)
│  │  │
│  │  ├─ Is it referenced by proc macro / derive / build.rs / integration test?
│  │  │  └─ YES → KEEP (stop)
│  │  │
│  │  ├─ Apply Fundamental Question:
│  │  │  ├─ "Required by FRD?" → KEEP
│  │  │  ├─ "Called by FRD-required method?" → KEEP
│  │  │  ├─ "Always there / might be useful / edge case?" → REMOVE
│  │  │  └─ Unclear? → Flag for manual review (do NOT auto-remove)
│  │  │
│  │  ├─ If Overengineered pattern:
│  │  │  └─ Pass 3-point test? → REMOVE. Fail any point? → KEEP + comment.
│  │  │
│  │  └─ Execute removal → Update trait → Update mod.rs → Verify
│  │
│  └─ Phase 1 Complete → Proceed to Phase 2
│
├─ PHASE 2: FILE CONSOLIDATION
│  │
│  ├─ Do files share the same domain/concept/feature?
│  │  └─ NO → Skip consolidation for these files
│  │
│  ├─ YES → Merge into single file:
│  │  ├─ Pick target (most logic)
│  │  ├─ Merge imports (deduplicate)
│  │  ├─ Merge structs (combine fields)
│  │  ├─ Merge impl blocks (one per trait)
│  │  ├─ Merge free functions
│  │  ├─ Update all references
│  │  ├─ Delete source file(s)
│  │  └─ Verify compilation
│  │
│  └─ Phase 2 Complete → Final Verification
│
└─ FINAL VERIFICATION
   ├─ cargo check --all-features
   ├─ cargo test --all-features
   ├─ cargo clippy --all-features -- -D warnings
   ├─ cargo check --workspace --all-features
   └─ Commit with descriptive message
```

---

## Dry-Run Mode

When user requests `--dry-run` or says "just show me what you'd remove":

1. Run Phase 1 Steps 1.1–1.6 (detection + analysis)
2. Run Phase 2 Step 2.1 (overlap detection)
3. Generate the full report (Phase 1 Step 1.7 + Phase 2 findings)
4. **Do NOT execute any deletions or edits**
5. Present report and wait for explicit approval to proceed

This is the **default mode** for first-time runs on a crate.
