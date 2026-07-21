---
name: consolidate-files-rust
description: "Merge multiple Rust implementation files with overlapping concerns into a single file with one struct, combining structs, impl blocks, imports, and free functions."
metadata:
    tags: [rust, merge, consolidation, deduplication, single-file, single-struct, aes]
    triggers:
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
        - cleanup-files-rust
        - create-capabilities-rust
---

# consolidate-files-rust

## Purpose

Merge Rust implementation files with overlapping concerns into a single file. This skill handles both **two-file merging** (combine two impl files into one) and **overlap detection** (identify files doing the same thing and consolidate them). The result is always a single coherent file with one struct, combined impl blocks, deduplicated imports, and merged free functions.

**CRITICAL: The Consolidation Rule** — After merging, there MUST be exactly ONE struct in the target file. All fields from old structs are combined, all methods are placed into appropriate impl blocks (following trait-consolidation if traits exist), and the source file(s) are deleted.

## Rules

- **One Struct Per File**: Merge two impl files → single file with single struct. Combine fields from both old structs into one.
- **Target Selection**: Keep the file with the most logic as the target; move unique functions from source files into target.
- **Merge All Methods**: Put ALL methods into impl blocks. If multiple traits exist, create separate `impl Trait` blocks for each trait (follows trait-consolidation).
- **Deduplicate Imports**: Combine imports from both files, remove duplicates.
- **Delete Source Files**: Remove the merged source file(s) after confirming compilation passes.
- **Update All References**: Fix all references across the codebase — lib.rs exports, root container wiring, test files.

## When to Use

- Two impl files share the same domain and can be unified.
- Multiple files implement the same concept (e.g., 7 coordinate transform files).
- Multiple files handle the same feature (e.g., cursor drawer + cursor renderer).
- Multiple adapter files for the same technology (e.g., 3 FFmpeg adapters).
- You want to reduce file count while keeping all functionality.

## The Fundamental Question

> **"Do these files do the same thing or share the same domain?"**

If yes → **Merge them into 1 file**

## Detection Patterns

### Same-Concept Files (Merge)

```rust
capabilities_world_to_camera.rs
capabilities_camera_to_world.rs
capabilities_camera_to_viewport.rs
// All do coordinate transforms → merge into capabilities_coordinate_mapper.rs
```

### Same-Feature Files (Merge)

```rust
capabilities_brush_cursor_drawer.rs
capabilities_drag_cursor_drawer.rs
capabilities_cursor_data_renderer.rs
// All render cursors → merge into capabilities_cursor_renderer.rs
```

### Same-Technology Adapters (Merge)

```rust
infrastructure_ffmpeg_adapter.rs
infrastructure_video_ffmpeg_adapter.rs
// Both use FFmpeg → merge into 1 adapter
```

## The Pattern

### Before Merge (Two Files)

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

### After Merge (One File)

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

## Step-by-Step Process

### Step 1: Detect Overlaps and Analyze Files

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

### Step 2: Pick Target File

Select the file with the most logic (most lines, most methods, most fields) as the merge target.

### Step 3: Merge Imports

Combine imports from all files, remove duplicates:

```rust
// From file1 + file2 — deduplicated
use async_trait::async_trait;
use shared::common::...;
use shared::import_rules::...;
use std::collections::{HashMap, HashSet};
```

### Step 4: Merge Structs

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

### Step 5: Merge Impl Blocks

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

### Step 6: Merge Free Functions

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

### Step 7: Update All References

Find and update ALL references across the codebase:

```bash
# Find all references to old names
grep -r "OldStructA\|OldStructB\|TraitA\|TraitB" crates/

# Update lib.rs exports
# Update root container wiring
# Update test files
```

### Step 8: Delete Source File(s)

Remove the file(s) whose functionality was merged:

```bash
rm crates/<crate>/src/file2.rs
```

### Step 9: Verify Compilation

```bash
cargo check -p <crate-name> 2>&1 | grep -E "error|cannot find"
```

## Verification Checklist

- [ ] Files analyzed and overlaps confirmed
- [ ] Target file selected (most logic)
- [ ] Imports merged and deduplicated
- [ ] Structs combined into one struct with all fields
- [ ] All methods moved to impl blocks (trait impl + inherent impl)
- [ ] Free functions kept as standalone or converted to methods
- [ ] Source file(s) deleted
- [ ] All references updated (lib.rs, root container, tests)
- [ ] `cargo check -p <crate-name>` passes without warnings or errors

## Quick Commands

```bash
# Detect potential overlaps by concept/feature
ls crates/<crate>/src/capabilities_*.rs | xargs -n1 basename | sort

# Analyze files before merge
wc -l crates/<crate>/src/file1.rs crates/<crate>/src/file2.rs
grep -c "^pub struct" crates/<crate>/src/file1.rs
grep -c "^    fn \|^    pub fn " crates/<crate>/src/file1.rs

# Find references after merge
grep -r "OldStructName" crates/

# Verify compilation
cargo check -p <crate-name> 2>&1 | grep -E "error|cannot find"
```

## Example: Cycle Import Analyzer Merge

### Before (Two Files)

```
crates/import-rules/src/capabilities_cycle_import_analyzer.rs
  - DependencyCycleAnalyzer implements ICycleAnalysisProtocol
  - Fields: _config, parser
  - Methods: scan, check_cycles, do_scan

crates/import-rules/src/capabilities_cycle_analyzer.rs
  - CycleAnalyzer implements ICycleAnalyzerPort
  - Fields: (none)
  - Methods: detect_cycle_edges, normalize_to_layer
```

### After Merge (One File)

```rust
pub struct CycleImportAnalyzer {
    _config: ArchitectureConfig,
    parser: Arc<dyn IImportParserPort>,
}

#[async_trait]
impl ICycleImportProtocol for CycleImportAnalyzer {
    fn scan(...) -> Vec<<ResultVO>> { self.do_scan(...) }
    async fn check_cycles(...) -> ... { ... }
    fn pure_detect_cycle_edges(...) -> Vec<SymbolName> { self.do_detect_cycle_edges(...) }
    fn pure_normalize_to_layer(...) -> String { self.do_normalize_to_layer(...) }

    fn do_scan(...) { ... }
    fn do_detect_cycle_edges(...) { ... }
    fn do_normalize_to_layer(...) { ... }
}
```

## File Locations

```
# Before merge (two files)
crates/<crate>/src/capabilities_<name1>.rs  # Old impl 1
crates/<crate>/src/capabilities_<name2>.rs  # Old impl 2

# After merge (one file)
crates/<crate>/src/capabilities_<name>.rs   # Unified impl (single struct)
```

## Common Pitfalls

- **Don't forget to update lib.rs** — exports must change from old names to new
- **Don't forget test files** — imports and usages must update
- **Don't leave orphan references** — grep for old names after merge
- **Keep enums/structs outside impl block** — only functions go inside impl
- **Update root container wiring** — DI must use new struct/trait names
- **Merge fields carefully** — if both structs have `_config`, keep only one
