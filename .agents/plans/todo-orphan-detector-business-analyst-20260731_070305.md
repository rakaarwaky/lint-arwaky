# Review Plan: orphan-detector -- Business Analyst

## Summary

The orphan-detector FRD (v1.12.0) is well-structured, comprehensive, and closely aligned with the implementation. 13 FRs cover the full AES501-AES506 orphan detection lifecycle. The AST migration from v1.11 (regex) to v1.12 (syn + structured parsing) is largely complete. **3 critical mismatches** exist between the FRD specification and the code: (1) FR-005 Check 2 incorrectly requires protocol contracts to be "called by" layers when the FRD explicitly allows "implementation exists = not orphan," causing false positives on the entire workspace; (2) FR-002 `root_*` entry point prefix is never matched during orchestrated scans; (3) FR-009 surface chain validation is acknowledged as incomplete in the code (TODO). Several secondary issues include missing barrel exceptions, a phantom AST parser reference, and an unused FRD-mentioned parameter.

---

## Findings by Category

### Requirements Clarity

| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| RC-1 | RED CRITICAL | FR-005 Edge Case contradicts Check 2: FRD says "Protocol with an implementation but zero callers is still reachable (implementation exists)" but Check 2 flags protocols as orphan when not "called by" agent/capabilities/surface/container -- even though they ARE implemented. This causes false positives across the entire workspace for every protocol contract. | `capabilities_orphan_contract_analyzer.rs:95-113` | **Code change:** Remove Check 2 entirely. Per FR-005 edge case, if a protocol has an implementation, it is reachable. The caller check is redundant and violates the spec. Alternatively, if callers are still desired, update FR-005 to remove the edge case and specify "Protocol must be implemented AND called" explicitly. |
| RC-2 | YELLOW WARNING | FR-005 does not clearly specify the difference between "called by" (Check 2/3) and "implemented by" (Check 1). "Called by" semantics are ambiguous -- does "called by" mean any file-level reference, or must it be an actual invocation in code? The code uses substring/whole-word matching against file contents, which may match comments or strings. | FRD FR-005 business rules | Add explicit definition of "called by" to the FRD: "A contract is considered 'called by' a layer file when the contract's trait/interface name appears in a `use`/`import` statement or `impl` block of that layer file." |
| RC-3 | YELLOW WARNING | FR-009 specifies a dependency chain `Entry -> Smart -> Utility -> Passive` but does not specify how the chain is validated when a file is BFS-reachable but imported by the wrong category (e.g., passive imported by passive). The FRD says "A passive surface imported only by another passive surface is orphan" but the code trusts BFS reachability. | FRD FR-009 edge cases; `capabilities_orphan_surfaces_analyzer.rs:42-56` | Either update FR-009 to acknowledge BFS reachability is an approximation (and add the TODO as a known limitation), or implement full chain validation by passing `inbound_links` to the surface analyzer. |

### Business Flow

| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| BF-1 | RED CRITICAL | FR-002 specifies "Files starting with `root_` are also treated as entry points" but `get_orphan_entry_points()` in the orchestrator never includes a `root_` prefix pattern. The graph resolver's default branch (no configured patterns) handles `starts_with("root_")`, but the orchestrator always provides configured patterns -- so this branch is unreachable in practice. `root_*` files (e.g., `root_cli_main_entry.py`) will not be recognized as entry points during orchestrated scans. | `agent_orphan_orchestrator.rs:515-533`; `capabilities_orphan_graph_resolver.rs:65-80` | Add `root_` to the orchestrator's `get_orphan_entry_points()` return list AND update `identify_entry_points` to support prefix matching for patterns starting with `root_`. |
| BF-2 | YELLOW WARNING | FR-010 specifies barrel exceptions include `index.tsx` and `index.jsx` but the orchestrator's barrel check (`_evaluate_layer`) only checks `index.ts` and `index.js`. Both `.tsx` and `.jsx` variants are missing. | `agent_orphan_orchestrator.rs:418-426` | Add `index.tsx` and `index.jsx` to the barrel file check in `_evaluate_layer`. |

### Logic Implementation

| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| LI-1 | YELLOW WARNING | The `regex` workspace dependency is still declared in `Cargo.toml` but no source file in `orphan-detector/src/` uses regex. The deprecated `utility_orphan_regex_patterns` module is still re-exported from `lib.rs` with `#[allow(deprecated)]`. This creates confusion about the migration status and adds dead dependency weight. | `Cargo.toml:12`; `lib.rs:18` | Remove `regex` from `Cargo.toml` `[dependencies]`. Remove the `#[allow(deprecated)] pub use ... utility_orphan_regex_patterns` re-export from `lib.rs`. The deprecated module lives in the shared crate and can remain there until v2.0 cleanup. |
| LI-2 | GREEN INFO | `FileDefinitionMap` parameter in `ContractOrphanAnalyzer::is_contract_orphan` is declared in the protocol but never used in the implementation (`_file_definitions`). The FRD lists it as an input for FR-005 but the code relies solely on AST parsing for trait extraction. | `capabilities_orphan_contract_analyzer.rs:38` | Either use the `FileDefinitionMap` to populate the contract analyzer's checks (as the FRD intends), or remove it from the protocol signature and FRD input list if AST parsing fully supersedes it. |
| LI-3 | GREEN INFO | The FRD Integration Points section references `utility_orphan_ast_parser.rs` as a centralized AST parser. This file does not exist -- the Rust parser is `utility_orphan_rust_parser.rs` (in orphan-detector crate), and Python/TS parsers are in the shared crate. | FRD Integration Points; FRD Reference section | Update FRD references to reflect actual file names: `utility_orphan_rust_parser.rs` (orphan-detector) and `utility_orphan_python_parser.rs` / `utility_orphan_ts_parser.rs` (shared). |

### Testability & Acceptance Criteria

| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| TA-1 | YELLOW WARNING | FR-005 "Protocol with an implementation but zero callers is still reachable" is listed as a test scenario but the current code will fail this test (Check 2 flags it). This means the acceptance test for this scenario is either not present or would fail. | FRD QA: Layer-Specific Detection bullet 1 | Verify `acceptance_AES502.rs` includes a test for "protocol with implementation, zero callers -> NOT flagged." If not present, add it. |
| TA-2 | YELLOW WARNING | FR-009 "Surface dependency chain" test scenario requires per-category validation but the code acknowledges TODO -- chain validation is incomplete. | FRD QA: Layer-Specific Detection bullet 5; `capabilities_orphan_surfaces_analyzer.rs:42-56` | Either implement the chain validation (pass `inbound_links` to surface analyzer) or mark this test as a known limitation. |
| TA-3 | GREEN INFO | The QA Checklist has ~50 test scenarios but many are manual checkbox items. No dedicated test file matches the AST parsing scenarios (multi-line impl, glob imports, etc.). These may be covered indirectly but are not isolated. | FRD QA Checklist; `tests/` directory | Consider adding `acceptance_FR_011_ast_parsing.rs` per PRD naming standard (`acceptance_FR_00N.rs`). |

### Traceability (FRD -> Code)

| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| TR-1 | YELLOW WARNING | FR-002 -> Code: The `root_` prefix entry point rule is specified in FRD but not enforced in the orchestrator code path. Traceability gap. | FRD FR-002; `agent_orphan_orchestrator.rs:515-533` | Fix in orchestrator (see BF-1). |
| TR-2 | YELLOW WARNING | FR-005 -> Code: The edge case "implementation exists = not orphan" is specified in FRD but Check 2 in the code violates it. Traceability gap. | FRD FR-005 Edge Case; `capabilities_orphan_contract_analyzer.rs:95-113` | Fix in code (see RC-1). |
| TR-3 | GREEN INFO | FR-006 -> Code: "Container files are identified by suffix: `*_entry.*`" is in the FRD but the capabilities analyzer (`check_wired_in_container`) searches for struct/trait name presence in container files rather than checking suffix-based entry point membership. The behavior is correct (DI wiring check) but the FRD wording could be clearer. | FRD FR-006; `capabilities_orphan_capabilities_analyzer.rs:42-65` | Clarify FR-006 to distinguish between "entry point identification" (suffix-based) and "wiring check" (identifier-based DI). |

---

## Violations

| # | Severity | Violation | Location | Recommendation |
|---|----------|-----------|----------|----------------|
| V-1 | RED | **FR-005 Check 2 violates FRD Edge Case** -- Protocol contracts with implementations but no callers are incorrectly flagged as orphans, contradicting the FRD's explicit edge case rule. | `capabilities_orphan_contract_analyzer.rs:95-113` | Remove Check 2 from the contract analyzer. The edge case is the authoritative rule. |
| V-2 | RED | **FR-002 `root_` prefix not in configured patterns** -- Entry point detection for `root_*` files is unreachable during orchestrated scans. | `agent_orphan_orchestrator.rs:515-533` | Add `root_` prefix to orchestrator's entry point patterns or add prefix-aware matching. |
| V-3 | YELLOW | **FR-009 Surface chain validation incomplete** -- The FRD specifies per-category importer validation but the code relies solely on BFS reachability. | `capabilities_orphan_surfaces_analyzer.rs:42-56` | Implement chain validation or update FRD to document the approximation. |
| V-4 | YELLOW | **FR-010 Barrel exceptions incomplete** -- `index.tsx` and `index.jsx` are listed in FRD but not in the code's barrel check. | `agent_orphan_orchestrator.rs:418-426` | Add `.tsx` and `.jsx` to barrel file exceptions. |

---

## Action Items

- [ ] RED P0 -- FR-005 Check 2 false positive -- Remove the protocol "called by" check (Check 2) from `ContractOrphanAnalyzer::is_contract_orphan`. Per FR-005 edge case, a protocol with an implementation is reachable regardless of callers.
- [ ] RED P0 -- FR-002 root_ entry point -- Add `"root_"` to `get_orphan_entry_points()` in the orchestrator AND update `identify_entry_points` to support prefix matching for `root_` patterns.
- [ ] YELLOW P1 -- FR-009 surface chain -- Decide: (A) implement full chain validation with `inbound_links` in `SurfacesOrphanAnalyzer`, or (B) update FR-009 to document BFS-reachability as the authoritative check.
- [ ] YELLOW P1 -- FR-010 barrel exceptions -- Add `index.tsx` and `index.jsx` to the barrel file check in `_evaluate_layer`.
- [ ] YELLOW P2 -- Deprecated regex cleanup -- Remove `regex` from `Cargo.toml` and the `#[allow(deprecated)]` re-export from `lib.rs`.
- [ ] YELLOW P2 -- FRD reference section -- Update FRD Integration Points and Reference to use actual file names.
- [ ] GREEN P3 -- FileDefinitionMap usage -- Either use `FileDefinitionMap` in the contract analyzer or remove from protocol/FRD.
- [ ] GREEN P3 -- Test coverage -- Add `acceptance_FR_011_ast_parsing.rs` for AST-specific test scenarios per PRD naming standard.

---

## Fixed Code

### Fix 1: Remove FR-005 Check 2 (Protocol caller validation)

**File:** `crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs`

```rust
// REMOVE Check 2 entirely -- FR-005 edge case says implementation exists = not orphan.
// Delete the following block (lines ~95-113):
//
// if suffix == "protocol"
//     && !Self::is_referenced_by_layers(
//         &trait_names,
//         &search_files,
//         &["agent_", "capabilities_", "surface_"],
//         &["_container.rs", "_container.py", "_container.ts", "_container.js"],
//     )
// {
//     return OrphanIndicatorResult::new(true, ..., Severity::MEDIUM);
// }
```

After this change, contract orphan detection becomes:
1. Extract trait names via AST -- empty = not orphan
2. Barrel re-export check -- re-exported = not orphan
3. Check 1: Implementation check via AST -- not implemented = orphan (Check 2 removed)
4. Check 3: Aggregate must be called by surface or container (remains unchanged)

### Fix 2: Add root_ prefix to orchestrator entry points

**File:** `crates/orphan-detector/src/agent_orphan_orchestrator.rs`

```rust
fn get_orphan_entry_points(&self) -> Vec<String> {
    let mut entry_points = vec![
        "_container.rs".into(),
        "_container.py".into(),
        "_container.ts".into(),
        "_container.js".into(),
        "_entry.rs".into(),
        "_entry.py".into(),
        "_entry.ts".into(),
        "_entry.js".into(),
        "root_".into(),  // ADD: FR-002 root_* prefix
        "main.rs".into(),
        "lib.rs".into(),
        "main.py".into(),
        "__main__.py".into(),
        "main.ts".into(),
        "main.js".into(),
        "index.ts".into(),
        "index.js".into(),
    ];
    // ... rest unchanged
}
```

**File:** `crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs`

In `identify_entry_points`, add prefix matching for `root_`:

```rust
configured_strs.iter().any(|pattern| {
    basename == pattern
        || stem == *pattern
        || (pattern.starts_with('_') && stem.ends_with(pattern.as_str()))
        || (pattern.starts_with('.') && basename.ends_with(pattern.as_str()))
        || (pattern == "root_" && basename.starts_with("root_"))  // ADD
        || ((pattern.ends_with(".rs") || pattern.ends_with(".py")
            || pattern.ends_with(".ts") || pattern.ends_with(".js"))
            && basename.ends_with(pattern.as_str()))
})
```

### Fix 3: Add index.tsx / index.jsx barrel exceptions

**File:** `crates/orphan-detector/src/agent_orphan_orchestrator.rs`

In `_evaluate_layer`, update barrel file check:

```rust
if f.ends_with("__init__.py")
    || f.ends_with("/mod.rs")
    || f.ends_with("\\mod.rs")
    || f.ends_with("/index.ts")
    || f.ends_with("\\index.ts")
    || f.ends_with("/index.js")
    || f.ends_with("\\index.js")
    || f.ends_with("/index.tsx")   // ADD
    || f.ends_with("\\index.tsx")  // ADD
    || f.ends_with("/index.jsx")   // ADD
    || f.ends_with("\\index.jsx")  // ADD
{
    return OrphanIndicatorResult::new(false, String::new(), Severity::HIGH);
}
```
