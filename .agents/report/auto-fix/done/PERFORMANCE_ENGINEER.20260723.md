# Review Report: auto-fix-lint-arwaky (Crate 13) — Performance Engineer

## Summary

**Crate:** auto-fix-lint-arwaky
**Files audited:** 5
**Performance issues found:** 4 significant, 2 moderate

---

## Critical Issues

### 1. Double File I/O in execute() — HIGH IMPACT
**Location:** `capabilities_fix_processor.rs` (execute)

**Problem:** The execute() method reads the file content twice when fixes are applied:
```rust
let results = self.linter.run_code_analysis(path).values; // first read (via linter)
// ... apply fixes (each fix reads + writes individual files) ...
if fixed_count > 0 {
    let after_results = self.linter.run_code_analysis(path).values; // second read
}
```

Each fix operation (rename_symbol, fix_bypass_comments_impl, fix_unused_import_impl) also reads the full file content separately. For a file with 3 naming violations + 2 bypass comments + 1 unused import: 6 file reads for fixes alone, plus 2 linter reads = 8 total reads.

**Fix:** Batch all file operations into a single read pass. Cache file contents in a HashMap<String, String> at the start of execute(), then apply all fixes to cached content before writing.

### 2. String::replace() Without Scope Limiting — MODERATE IMPACT
**Location:** `capabilities_fix_processor.rs` (rename_symbol)

**Problem:** `content.replace(old_name, new_name)` replaces ALL occurrences of old_name in the entire file content, including:
- Comments containing the name
- String literals
- Part of longer identifiers (e.g., renaming "foo" would replace "foobar")

This causes incorrect renames and potential compilation errors. Additionally, the full content string is allocated twice (original + replaced).

**Fix:** Use line-by-line replacement with word-boundary matching (\bold_name\b for Rust, \bold_name\b for Python). Only apply to code lines, skip comments and strings.

### 3. String Concatenation in Loop — MODERATE IMPACT
**Location:** `capabilities_fix_processor.rs` (fix_bypass_comments_impl, fix_unused_import_impl)

**Problem:** Both methods build result strings using push_str in a loop:
```rust
let mut result = String::new();
for (i, l) in lines.iter().enumerate() {
    if i != target_idx {
        result.push_str(l);
        result.push('\n');
    }
}
```

This creates O(n) allocations for n lines. For a 500-line file, that's 500 reallocations.

**Fix:** Pre-calculate result length (content.len() - target_line.len()), use String::with_capacity(), or use a Vec<&str> + join() pattern which allocates once.

---

## Moderate Issues

### 4. Box::leak for Static Array — MODERATE IMPACT
**Location:** `capabilities_fix_processor.rs` (fixable_codes)

**Problem:** `Box::leak(Box::new([...]))` leaks memory permanently. The array is never freed for the lifetime of the program. While small (~48 bytes), this pattern is bad practice and should use &'static directly.

**Fix:** Store as `const FIXABLE_CODES: &[ErrorCode] = &[...];` and return `&FIXABLE_CODES`.

### 5. Redundant String Formatting in Pattern Matching — LOW IMPACT
**Location:** `capabilities_fix_processor.rs` (fix_bypass_comments_impl)

**Problem:** Multiple format!() calls for static patterns:
```rust
let allow_attr = format!("#[{}", "allow("); // redundant, just use "#[allow("
let unwrap_call = format!("unw{}", "rap()"); // redundant
let nq_pat = format!("n{}", "oqa");
```

These create temporary String allocations for static strings. Use &'static str directly.

**Fix:** Replace with `const ALLOW_ATTR: &str = "#[allow(";` etc.

### 6. Unused Event Emission — LOW IMPACT
**Location:** `capabilities_fix_processor.rs` (emit_fix_event_impl)

**Problem:** Creates FixApplied event objects that are immediately dropped:
```rust
let event = FixApplied::new(...);
let _ = event; // never used
```

Event creation involves serde serialization overhead if the event system is active.

**Fix:** Either dispatch events properly or remove dead code to avoid allocation overhead.

---

## Positive Findings

- Orchestrator layer is clean — pure delegation with no performance concerns
- File adapter uses existing utility functions without unnecessary abstractions
- Auto-fix container wires dependencies efficiently with Arc<dyn>
- Early returns for non-existent files prevent wasted I/O
- Fix operations check target_line.contains(pattern) before attempting fixes — avoids unnecessary writes

---

## Estimated Impact

**Worst-case scenario (large file with many violations):** A 1000-line file with 20 naming violations, 10 bypass comments, and 5 unused imports would result in ~35 file reads and 35 file writes. Total fix operation time: ~3-8 seconds depending on disk I/O.

**Priority fix:** Cache all file contents at the start of execute(), batch all modifications, then write once. This single change would reduce file I/O by 80-90%.
