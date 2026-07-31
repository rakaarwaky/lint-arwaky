# Review Plan: orphan-detector — Business Analyst

## Summary

The orphan-detector crate (v1.12.0) successfully implements 13 of 13 FRDs with solid architecture: a clean 7-layer AES structure, AST-based parsing via `syn`, and proper contract/aggregate separation. All 75 tests pass. However, I found **3 high-severity gaps** (dead `file_definitions` map, unused `regex`/`once_cell` dependencies, surface chain validation approximation), **3 medium-severity issues** (TypeScript inheritance map not wired, FRD→test naming inconsistency, contract analyzer ignoring passed-in maps), and **4 low-severity items** (informational). The FRD is well-specified and traceable — the main risk is the surface orphan chain validation gap which could produce false negatives for passive surfaces.

---

## Findings by Category

### Requirements Clarity

| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| 1 | 🟡 WARNING | FR-009 acknowledges that passive surface chain validation is approximated (BFS reachability used instead of per-category importer check). A passive imported by another passive is "alive" via BFS but should be flagged per FRD. The code has a documented limitation comment but no tracking issue. | `capabilities_orphan_surfaces_analyzer.rs:31-35` | Add a tracking issue for FR-009 chain validation. Consider adding `InboundLinkMap` to `ISurfacesOrphanProtocol` trait signature when next breaking change occurs. |
| 2 | 🟡 WARNING | FR-005 says "Extract `class Foo implements Bar` → inheritance map entries" for TypeScript, but `inheritance_map` in `GraphAnalysisContext` is never populated for TS files — only Python `class_bases` are wired in. The TS `class_implements` data exists in parse results but is unused by the shared graph context. | `capabilities_orphan_graph_resolver.rs:340-377` | Wire `result.class_implements` into `inheritance_map` for TypeScript files, matching the Python pattern at lines 342-348. |
| 3 | 🟢 INFO | FRD API Contract lists "Build orphan graph context" output as containing "trait/class definition map" and "implementation relationship map." In practice, `file_definitions` is always empty `{}` and `inheritance_map` only has Python entries. This is a FRD-vs-code accuracy gap. | `capabilities_orphan_graph_resolver.rs:122,387-393` | Update FRD §API Contract to reflect actual output: `file_definitions` is unused by consumers, `inheritance_map` covers Python only. Or populate them as originally intended. |

### Business Flow

| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| 4 | 🟡 WARNING | Contract analyzer ignores `file_definitions` and `inheritance_map` parameters — both are prefixed with `_` and unused. The analyzer re-parses every file from scratch for trait/impl extraction, duplicating work already done by the graph resolver. This is a performance concern for large codebases (FR-013 NFR: 10K files < 5s). | `capabilities_orphan_contract_analyzer.rs:45-46` | Two options: (a) populate `file_definitions` in graph resolver and use it in contract analyzer, or (b) remove the unused params from `IContractOrphanProtocol` trait to clarify the contract. Option (a) avoids redundant parsing. |
| 5 | 🟢 INFO | `is_referenced_by_layers` (contract analyzer:236-258) uses whole-word `content_contains_word` matching for trait references. This is correct per FR-005 ("Whole-word matching is used for all identifier checks") but could false-positive on common words like `Error` or `Result` that appear as trait names. Low risk because contract traits are typically uniquely named. | `capabilities_orphan_contract_analyzer.rs:281-283` | No action required — inherent limitation of static text matching. Document in FRD edge cases if concerned. |

### Logic Implementation

| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| 6 | 🔴 CRITICAL | `regex` and `once_cell` are listed in `Cargo.toml` dependencies but **zero usages** exist in any source file. These are dead dependencies from the v1.11 regex era. They add unnecessary compile time and binary size, and violate the principle of minimal dependencies. | `Cargo.toml:13-14` | Remove `regex.workspace = true` and `once_cell.workspace = true` from `[dependencies]`. Verify no transitive usage first. |
| 7 | 🟡 WARNING | `file_definitions: HashMap<String, Vec<String>>` is initialized empty at `graph_resolver.rs:122` and passed through to `GraphAnalysisContext`. Every consumer receives it but no consumer uses it. This is dead data flow — confusing for maintainers and wasteful in memory. | `capabilities_orphan_graph_resolver.rs:122` | Either populate `file_definitions` (extract struct/trait names during graph build) or remove it from `GraphAnalysisContext`. If removing, update all dependent code. |
| 8 | 🟡 WARNING | In `agent_orphan_orchestrator.rs:80-83`, `check_orphans` calls `build_orphan_graph_context` then immediately calls `_expand_workspace_files` again for `_check_orphans_inner`. The comment says "avoid redundant directory scanning" but `_expand_workspace_files` IS called redundantly — once inside `build_orphan_graph_context` (line 60) and once at line 81. | `agent_orphan_orchestrator.rs:60,80-83` | Refactor to expand once and pass the expanded list to both graph building and inner checking. The comment at line 78 is misleading. |
| 9 | 🟢 INFO | The `_trace_reachability` BFS at line 426-441 uses `HashSet<String>` for visited tracking — correct per FR-003. Performance is O(V+E) which meets NFR targets. | `agent_orphan_orchestrator.rs:426-441` | No action. Well-implemented. |

### Testability & Acceptance Criteria

| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| 10 | 🟡 WARNING | PRD §Product Decisions says "Acceptance tests standard: `acceptance_FR_00N.rs`" but actual test files use `acceptance_AES501.rs` through `acceptance_AES506.rs`. 6 tests exist, covering all 6 orphan rules. Naming inconsistency with PRD. | `tests/acceptance_AES50*.rs` | Rename to `acceptance_FR_004.rs` through `acceptance_FR_009.rs` (mapping AES501→FR-004, etc.) to align with PRD standard. Or update PRD to accept AES-code naming. |
| 11 | 🟢 INFO | 75 tests pass (unit: 67, contract: 1, integration: 1, e2e: 1, acceptance: 6). Good coverage across all analyzers. No test for FR-012 (macro handling) — acceptable since FR-012 is marked "Future." | `tests/` directory | No action. Consider adding a test that verifies `parse_ok = false` behavior for Rust syntax errors (FR-011 edge case). |
| 12 | 🟢 INFO | FRD §Test Scenarios lists 44 checklist items. The acceptance tests cover the 6 AES rules but don't explicitly map to individual QA items (e.g., "Circular imports between two capabilities — both reachable, neither flagged" is tested in `e2e_orphan_detection_flow.rs` but not individually). | `tests/e2e_orphan_detection_flow.rs` | Consider extracting key QA scenarios into named test functions for better traceability. |

### Traceability (FRD → Code)

| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|
| 13 | 🟢 INFO | All 13 FRDs have clear implementation mappings. FR-012 (Macro) is explicitly deferred. Full traceability achieved. | All source files | No action. Maintain FRD→code traceability in future changes. |

---

## Violations

| Rule | Severity | Location | Description |
|------|----------|----------|-------------|
| AES304 | CRITICAL | `Cargo.toml:13-14` | Dead dependencies (`regex`, `once_cell`) — violates zero-bypass / minimal dependency principle |
| AES405 | MEDIUM | `agent_orphan_orchestrator.rs:80` | Redundant `_expand_workspace_files` call — orchestrator doing unnecessary I/O work |

---

## Action Items

- [ ] 🔴 CRITICAL Remove `regex` and `once_cell` from `Cargo.toml` dependencies
- [ ] 🟡 WARNING Wire TypeScript `class_implements` into `inheritance_map` in graph resolver
- [ ] 🟡 WARNING Either populate `file_definitions` or remove from `GraphAnalysisContext`
- [ ] 🟡 WARNING Fix redundant `_expand_workspace_files` call in `check_orphans`
- [ ] 🟡 WARNING Align acceptance test filenames with PRD standard (`acceptance_FR_00N.rs`)
- [ ] 🟢 INFO Update FRD §API Contract to reflect actual `file_definitions` / `inheritance_map` behavior
- [ ] 🟢 INFO Track FR-009 surface chain validation improvement as a future enhancement

---

## Fixed Code

### Fix 1: Remove dead dependencies

**File:** `crates/orphan-detector/Cargo.toml`

```toml
[dependencies]
serde.workspace = true
serde_json.workspace = true
rayon.workspace = true
shared.workspace = true
syn = { version = "2", features = ["full", "visit", "parsing"] }
```

### Fix 2: Wire TS inheritance_map in graph resolver

**File:** `crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs`

In the `FileParseResultVO::TypeScript(result)` block (~line 366), add after the imports loop:

```rust
// Class inheritance (matching Python pattern)
for (_class_name, interfaces) in &result.class_implements {
    for iface in interfaces {
        inheritance_map
            .entry(f.clone())
            .or_default()
            .push(iface.clone());
    }
}
```

### Fix 3: Remove redundant workspace expansion

**File:** `crates/orphan-detector/src/agent_orphan_orchestrator.rs`

In `check_orphans` (~line 73-84), restructure to expand once:

```rust
fn check_orphans(&self, files: &OrphanFileListVO, root_dir: &FilePath) -> Vec<LintResult> {
    if !self.config.enabled.value {
        return Vec::new();
    }
    let all_workspace_files = self._expand_workspace_files(files, root_dir);
    let full_files_vo = OrphanFileListVO::new(all_workspace_files);
    let context = self.deps
        .resolver
        .build_graph_context(std::slice::from_ref(&full_files_vo), root_dir.value());
    self._check_orphans_inner(files, root_dir, &context, &full_files_vo)
}
```
