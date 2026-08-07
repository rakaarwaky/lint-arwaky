# Plan: auto-fix — Tech-Lead

## Summary

The auto-fix crate (14 files, 5 source + 8 tests + 1 bench) implements mechanical corrections for AES101, AES203, and AES304 violations. The architecture is clean — correct layer separation (agent → capabilities → root), contracts in `shared`, zero async, proper DI. The core concern is a **UTF-8 correctness bug** in word-boundary replacement that will corrupt non-ASCII files. Secondary issues: the `execute()` method exceeds reasonable complexity (~150 lines with nested loops), fix events are collected but never published, and the acceptance test file is misnamed.

## Findings

### Security

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🔴 CRITICAL | `word_boundary_replace` casts `bytes[i] as char` — only valid for ASCII. Multi-byte UTF-8 (accented chars, CJK, emoji) produces garbled output or panics. | `capabilities_fix_processor.rs:310-325` `word_boundary_replace()` | Replace byte-level iteration with `.char_indices()` iteration, match against char-level boundaries. |
| 2 | 🔴 CRITICAL | `word_boundary_count` uses byte-slice comparison (`&bytes[i..i+target_len] == target_bytes`) which fails for UTF-8 target strings. | `capabilities_fix_processor.rs:295-309` `word_boundary_count()` | Use `.match_indices()` or char-level iteration. |
| 3 | 🟡 WARNING | `strip_inline_comment` finds `//` byte position and slices — splits mid-character for non-ASCII before the `//`. | `capabilities_fix_processor.rs:284` `strip_inline_comment()` | Use `.find("//")` on `&str` (char-aware) instead of byte position. Note: `str::find` returns byte offset so the slice is still char-safe, but verify with Unicode content. |

### Performance

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 4 | 🟡 WARNING | `execute()` runs `self.linter.run_code_analysis(path)` twice when fixes are applied — once for detection, once for post-fix count. Linting is the bottleneck per FRD §NFR. | `capabilities_fix_processor.rs:69-72` (post-fix re-lint) | Already partially addressed by BF-4. Consider: if `total_fixable` is known, `remaining = results.len() - fixed_count` is a valid approximation for non-dry-run without re-linting. Only re-lint when exact count is required by the caller. |
| 5 | 🟢 INFO | Each fix method (`fix_bypass_comments_impl`, `fix_unused_import_impl`, `rename_symbol_impl`) reads the file independently via `file_adapter`. In `execute()`, if multiple violations target the same file, the file is read N times. | `capabilities_fix_processor.rs:execute()` inner loops | Acceptable for current single-file-per-call design. Document as known limitation for future multi-file support. |

### Error Handling

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 6 | 🟡 WARNING | `FixResult.error` is set to "All fix attempts failed" when `!dry_run && fixed_count == 0 && total_fixable > 0`. This conflates "linter ran fine but no fixable patterns matched" with "actual I/O errors". | `capabilities_fix_processor.rs:168-173` | Track `io_failures: bool` separately. Only set error when at least one `FixOutcome::Failed` occurred. If all outcomes are `Skipped`, that's success (no action needed), not an error. |
| 7 | 🟢 INFO | `fix_bypass_comments_impl` constructs `allow_attr = format!("#[{}", "allow(")` at runtime to avoid AES304 self-detection. Clever, but the same trick is not applied to other bypass patterns (`noqa`, etc.) which are constructed as plain `let` bindings. | `capabilities_fix_processor.rs:213-216` | All bypass pattern strings use this pattern already (some via `format!`, some via `let`). Document why this is needed in a comment for maintainers. |

### SOLID

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 8 | 🟡 WARNING | `LintFixProcessor::execute()` is ~150 lines with 3 nested violation-type loops + formatting + error logic. Violates SRP — it's simultaneously linter-runner, violation-classifier, fix-dispatcher, post-linter, report-builder. | `capabilities_fix_processor.rs:69-180` | Extract `classify_violations()` and `build_output()` as private helpers on `LintFixProcessor`. Keep `execute()` as the orchestrating flow only. |
| 9 | 🟡 WARNING | Adding a new fixable error code (e.g., AES204) requires modifying: (a) `FIXABLE_CODES` static, (b) a new filter+loop in `execute()`, (c) possibly a new `_impl` method. Three touch points for one new code. | `capabilities_fix_processor.rs:55-63` (FIXABLE_CODES), `execute()` | Consider a `FixDispatcher` pattern: register `(error_code, fixer_fn)` pairs. Not urgent — current 3 codes are manageable. Flag for future if codes expand. |

### Code Quality

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 10 | 🟡 WARNING | `events: Vec<FixApplied>` is collected in `execute()` but never published, returned, or stored. Dead code that creates allocations for nothing. | `capabilities_fix_processor.rs:112` (declaration), `133, 146, 159` (push) | Either: (a) return `events` in `FixResult` (requires contract change — do both), or (b) remove the `events` collection entirely if event publishing is out of scope. |
| 11 | 🟡 WARNING | Acceptance test file `acceptance_AES201_fix.rs` tests AES203 (unused import removal), not AES201 (forbidden import). AES201 is not in `FIXABLE_CODES` — the auto-fix crate doesn't handle AES201 at all. | `tests/acceptance_AES201_fix.rs` | Rename to `acceptance_AES203_fix.rs` and update test names/descriptions to reflect AES203. |
| 12 | 🟢 INFO | Unused import `use shared::common::taxonomy_lint_result_vo::LintResult;` at top of `agent_fix_orchestrator.rs` — the type is only used via the `&[LintResult]` parameter in `manual_report`, which is imported via `shared::auto_fix::*` re-export. | `agent_fix_orchestrator.rs:10` | Verify if this import is actually needed or redundant with re-exports. If unused, remove. |
| 13 | 🟢 INFO | `LintFixProcessor` has `#[deprecated]` on `with_dry_run` but not on the struct itself. Since this is a v1.11.0 crate and `with_dry_run` was explicitly deprecated for per-request dry_run, the migration is clean. | `capabilities_fix_processor.rs:191-198` | No action needed — deprecated method is correctly marked. Consider removing entirely in next major version. |

### Maintainability

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 14 | 🟢 INFO | Comment `// BF-4: No double linting` in `execute()` is slightly misleading — there IS a re-lint call when fixes are applied. The BF-4 improvement was removing the *unconditional* re-lint. | `capabilities_fix_processor.rs:161-166` | Update comment to: `// BF-4: Only re-lint when fixes were actually applied (not unconditional)` |
| 15 | 🟢 INFO | `word_boundary_count` and `word_boundary_replace` are duplicated implementations — both scan for the same boundaries but one counts and one replaces. | `capabilities_fix_processor.rs:295-325` | Extract a shared `find_word_boundaries(text, target) -> Vec<usize>` helper. Both functions then consume the positions. Low priority — code is small. |
| 16 | 🟢 INFO | Benchmark file covers dry-run only. No benchmarks for actual fix application (which exercises file I/O and string manipulation). | `benches/bench_auto_fix.rs` | Add a `bench_apply_fixes` group that writes files with known violations and measures non-dry-run throughput. |

## Action Items

- [ ] 🔴 CRITICAL Fix #1+#2: Replace byte-level word-boundary functions with char-aware iteration in `capabilities_fix_processor.rs`
- [ ] 🟡 WARNING Fix #6: Track `io_failures` separately in `execute()`, only set error on actual `Failed` outcomes
- [ ] 🟡 WARNING Fix #8: Extract `classify_violations()` and `build_output()` private helpers from `execute()`
- [ ] 🟡 WARNING Fix #10: Remove dead `events` collection OR add to `FixResult` (scope-dependent)
- [ ] 🟡 WARNING Fix #11: Rename `acceptance_AES201_fix.rs` → `acceptance_AES203_fix.rs` + fix test names

## Fixed Code

### Fix #1+#2: UTF-8-safe word-boundary replacement

```rust
// capabilities_fix_processor.rs — replace word_boundary_count, word_boundary_replace, is_word_boundary

/// Count occurrences of `target` that match word boundaries.
fn word_boundary_count(text: &str, target: &str) -> usize {
    text.match_indices(target)
        .filter(|(pos, _)| is_word_boundary_text(text, *pos, target.len()))
        .count()
}

/// Replace occurrences of `target` with `replacement` only at word boundaries.
fn word_boundary_replace(text: &str, target: &str, replacement: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut last_end = 0;

    for (pos, _) in text.match_indices(target) {
        if is_word_boundary_text(text, pos, target.len()) {
            result.push_str(&text[last_end..pos]);
            result.push_str(replacement);
            last_end = pos + target.len();
        }
    }
    result.push_str(&text[last_end..]);
    result
}

/// Check if a match at byte `pos` of byte-length `len` is at a word boundary.
/// Uses `str::char_indices`-safe slicing — `text[..pos]` and `text[pos+len..]`
/// are valid even for multi-byte UTF-8 because `str::match_indices` guarantees
/// alignment to char boundaries.
fn is_word_boundary_text(text: &str, pos: usize, len: usize) -> bool {
    let before_ok = pos == 0 || {
        let ch = text[..pos].chars().next_back().unwrap();
        !ch.is_alphanumeric() && ch != '_'
    };
    let after_ok = pos + len >= text.len() || {
        let ch = text[pos + len..].chars().next().unwrap();
        !ch.is_alphanumeric() && ch != '_'
    };
    before_ok && after_ok
}
```

**Why:** The original `bytes[i] as char` cast produces invalid chars for multi-byte sequences. The new version uses `str::match_indices` (which guarantees char-boundary alignment) and `chars().next_back()`/`chars().next()` for boundary checks. `is_word_boundary` (the byte-based version) is no longer needed and should be removed.

### Fix #6: Correct error semantics in execute()

```rust
// capabilities_fix_processor.rs — in execute(), replace the error-setting block:

    // Track actual I/O failures vs. policy skips
    let had_io_failure = {
        let outcomes_all: Vec<&FixOutcome> = naming_violations.iter()
            .filter_map(|v| {
                let msg = v.message.value();
                msg.split_whitespace()
                    .find(|w| w.contains('_') && w.len() > 3)
                    .and_then(|old_name| {
                        if RUST_KEYWORDS.contains(&old_name) { return None; }
                        let parts: Vec<&str> = old_name.split('_').collect();
                        let new_name = if parts.len() >= 3 { old_name.to_string() } else { format!("renamed_{}", old_name) };
                        if old_name != new_name { Some(()) } else { None }
                    })
                    .map(|_| self.rename_symbol_impl(path.value(), "", "", dry_run)) // placeholder — track inline
            })
            .collect();
        false // Simplified: track per-outcome in actual loops
    };

    let error = if !dry_run && fixed_count == 0 && total_fixable > 0 {
        // Only error when there were actual fix attempts that ALL failed (not all skipped)
        Some(shared::common::taxonomy_common_error::ErrorMessage::new(
            "All fix attempts failed".to_string(),
        ))
    } else {
        None
    };
```

**Better approach** — add a simple bool tracker in the existing loops:

```rust
    let mut io_failures = false;

    // In each fix loop, after getting outcome:
    match &outcome {
        FixOutcome::Failed(_) => io_failures = true,
        _ => {}
    }

    // At the end of execute():
    let error = if !dry_run && io_failures && fixed_count == 0 {
        Some(shared::common::taxonomy_common_error::ErrorMessage::new(
            "All fix attempts failed".to_string(),
        ))
    } else {
        None
    };
```

### Fix #8: Extract helpers from execute()

```rust
// capabilities_fix_processor.rs — add private helpers before Block 3

/// Classify violations by error code into typed buckets.
fn classify_violations(results: &[LintResult]) -> (Vec<&LintResult>, Vec<&LintResult>, Vec<&LintResult>) {
    let naming: Vec<_> = results.iter()
        .filter(|r| r.code == ErrorCode::raw("AES101"))
        .collect();
    let bypass: Vec<_> = results.iter()
        .filter(|r| r.code == ErrorCode::raw("AES304"))
        .collect();
    let unused_import: Vec<_> = results.iter()
        .filter(|r| r.code == ErrorCode::raw("AES203"))
        .collect();
    (naming, bypass, unused_import)
}

/// Build the output string for dry-run or post-fix results.
fn build_output(
    dry_run: bool,
    fixed_count: usize,
    total_fixable: usize,
    naming_len: usize,
    bypass_len: usize,
    unused_import_len: usize,
    remaining: usize,
    manual_steps: &[LintMessage],
) -> String {
    let manual_str = manual_steps.iter().map(|m| m.to_string()).collect::<Vec<_>>().join("\n");
    if dry_run {
        format!(
            "Dry-run: would fix {} violations ({} AES101 naming, {} AES304 bypass, {} AES203 unused import)\nManual violations remaining:\n{}",
            total_fixable, naming_len, bypass_len, unused_import_len, manual_str
        )
    } else if fixed_count > 0 {
        format!(
            "Fixed {} violations automatically ({} remaining)\nManual violations requiring attention:\n{}",
            fixed_count, remaining, manual_str
        )
    } else {
        format!(
            "No automatic fixes applied\nManual violations requiring attention:\n{}",
            manual_str
        )
    }
}
```

Then `execute()` becomes:

```rust
    fn execute(&self, path: &FilePath, dry_run: bool) -> FixResult {
        let analysis = self.linter.run_code_analysis(path);
        let results = &analysis.values;
        let (naming_violations, bypass_violations, unused_import_violations) = Self::classify_violations(results);

        let mut fixed_count = 0usize;
        let mut total_fixable = naming_violations.len() + bypass_violations.len() + unused_import_violations.len();
        let mut manual_skipped: Vec<LintMessage> = Vec::new();
        let mut io_failures = false;

        // ... existing per-violation loops (unchanged) ...

        let mut manual_steps = self.report_non_fixable(results);
        manual_steps.extend(manual_skipped);

        let remaining = if !dry_run && fixed_count > 0 {
            let after_results = self.linter.run_code_analysis(path).values;
            after_results.len()
        } else {
            results.len()
        };

        let output = Self::build_output(dry_run, fixed_count, total_fixable,
            naming_violations.len(), bypass_violations.len(), unused_import_violations.len(),
            remaining, &manual_steps);

        let error = if !dry_run && io_failures && fixed_count == 0 {
            Some(shared::common::taxonomy_common_error::ErrorMessage::new(
                "All fix attempts failed".to_string(),
            ))
        } else {
            None
        };

        FixResult { output: DescriptionVO::new(output), error }
    }
```

## Checklist

- [x] Prerequisites read (RULES_AES.md, ARCHITECTURE.md, PRD.md)
- [x] Feature identified (auto-fix, LOCKED, no prior history)
- [x] All 6 dimensions analyzed
- [x] Severity categorized (2 CRITICAL, 7 WARNING, 7 INFO)
- [x] History checked (features.json — first iteration)
- [x] Plan written (findings + fixed code)
- [x] Saved to correct path (`.agents/graph-loop/results/tech-lead-auto-fix.md`)
