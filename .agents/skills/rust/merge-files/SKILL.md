---
name: merge-files-rust
version: 1.0.0
category: refactoring
tags: [aes, merge, rust, consolidation, single-file, single-class]
triggers:
  - "merge two files into one"
  - "combine two impl files"
  - "consolidate files"
  - "merge capabilities files"
  - "merge infrastructure files"
  - "merge agent files"
dependencies: []
related:
  - trait-consolidation-rust
  - fix-cross-import
  - enforce-1-class-per-file
---

# merge-files-rust

## Purpose

Merge TWO Rust implementation files into ONE file with ONE class/struct. This skill handles the mechanical merging process — combining structs, impl blocks, imports, and free functions into a single coherent file.

## Rules

- Merge two impl files → single file with single struct/class
- Combine fields from both old structs into one struct
- Merge all methods into ONE impl block (follows trait-consolidation if traits exist)
- Remove duplicate imports
- Delete the old file after merge
- Update ALL references across the codebase

## When to Use

- Two impl files share the same domain and can be unified
- You want to reduce file count while keeping all functionality
- Both files implement related functionality that belongs together
- Refactoring to follow single-class-per-domain principle

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

### Step 1: Analyze Both Files

Read both files to understand:
- What structs/classes exist
- What traits they implement
- What fields each struct has
- What methods each impl block has
- What free functions exist
- What imports are used

```bash
# Read both files
cat crates/<crate>/src/file1.rs
cat crates/<crate>/src/file2.rs

# Count structs, methods, imports
grep -c "^pub struct" crates/<crate>/src/file1.rs
grep -c "^    fn \|^    pub fn " crates/<crate>/src/file1.rs
```

### Step 2: Merge Imports

Combine imports from both files, remove duplicates:

```rust
// From file1 + file2 — deduplicated
use async_trait::async_trait;
use shared::common::...;
use shared::import_rules::...;
use std::collections::{HashMap, HashSet};
```

### Step 3: Merge Structs

Combine fields from both old structs into one struct:

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

### Step 4: Merge Impl Blocks

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

### Step 5: Merge Free Functions

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

### Step 6: Update All References

Find and update ALL references across the codebase:

```bash
# Find all references to old names
grep -r "OldStructA\|OldStructB\|TraitA\|TraitB" crates/

# Update lib.rs exports
# Update root container wiring
# Update test files
```

### Step 7: Delete Old File

Remove the file whose functionality was merged:

```bash
rm crates/<crate>/src/file2.rs
```

### Step 8: Verify Compilation

```bash
cargo check -p <crate-name> 2>&1 | grep -E "error|cannot find"
```

## Verification Checklist

- [ ] Both files read and analyzed
- [ ] Imports merged and deduplicated
- [ ] Structs combined into one struct with all fields
- [ ] All methods moved to impl blocks
- [ ] Free functions kept as standalone or converted to methods
- [ ] Old file deleted
- [ ] All references updated (lib.rs, root container, tests)
- [ ] `cargo check` passes

## Quick Commands

```bash
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
    fn scan(...) -> Vec<LintResult> { self.do_scan(...) }
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
