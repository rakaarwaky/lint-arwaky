# Plan: quality-rules — Architect (Merged Plan)

## Summary

The `quality-rules` crate is a well-structured, mature feature implementing AES301–AES305 quality rules. The architecture cleanly follows AES conventions: agent layer is pure delegation, capabilities implement protocol traits, utilities are stateless free functions, and the root container wires everything via DI. Dependencies are correctly restricted to `shared` + `rayon` at library level. The crate has 13 test files covering contract, unit, integration, E2E, acceptance, and smoke levels. No CRITICAL issues found. Two warnings identified: an inline message format in the orchestrator (VO empty container pattern inconsistency) and a fragile character-array string construction pattern in `BypassChecker`.

## Merged Findings

### Layer Boundaries

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| — | — | No layer boundary violations found | — | All imports are correct. `Cargo.toml` has only `shared` + `rayon` as lib deps. Capabilities import from `shared` contracts and `crate::utility_*` only. Agent delegates to capabilities via protocol traits. Root wires aggregates via DI. |

### Naming

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| — | — | All 10 source files follow `layer_concern_role` convention | — | `agent_quality_orchestrator.rs`, `root_quality_rules_container.rs`, `capabilities_*_checker.rs`/`analyzer.rs`, `utility_*.rs` — all correct. |

### Orphan

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| — | — | All modules are re-exported via `lib.rs` and wired in `root_quality_rules_container.rs` | — | No orphan files. All 4 capabilities, 4 utilities, agent, and root are connected. |

### Scalability

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 🟡 WARNING | 1 | `classify_token` constructs comparison strings character-by-character (`mk(&['u','n','w','r','a','p'])`) — fragile if token names change, hard to read | `capabilities_check_bypass_checker.rs:508–526` | Use shared taxonomy constant strings or simple string literals. The `WORD_PATTERN_TOKENS` constant already exists in shared — use it for classification too. |
| 🟢 INFO | 2 | `capabilities_check_bypass_checker.rs` is 656 lines — largest file in crate | `capabilities_check_bypass_checker.rs` | Monitor. Currently under 1000-line limit but approaching midpoint. If more bypass patterns are added, consider extracting language-specific checks into per-language modules. |
| 🟢 INFO | 3 | `default_forbidden_bypass()` also uses character-array construction for pattern strings | `capabilities_check_bypass_checker.rs:531–570` | Same fix as #1 — use shared constants or string literals for readability. |

### Data Flow

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| — | — | Data flow is unidirectional: Surface → filesystem → orchestrator → capabilities → violations | — | Correct. No cycles. Orchestrator receives pre-fetched `FileEntry[]` and passes to capabilities. Zero I/O enforced by `legacy_entry_disabled` on legacy paths. |

## Additional Architectural Observations

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 🟡 WARNING | 4 | AES305 violation message is formatted inline in the orchestrator (`format!("AES305 CODE_DUPLICATION:...")`) instead of using a shared `format_*` function | `agent_quality_orchestrator.rs:134–142` | Per VO empty container pattern, `AesCodeAnalysisViolation::CodeDuplication` should have its full message produced by a shared `format_code_analysis_violation()` function in `shared::`, not assembled inline in the orchestrator. Currently only `format!("AES305...")` and `format!("{:?}", other)` are used — the latter is a debug fallback. |
| 🟢 INFO | 5 | `has_critical` is a free function in agent layer file — correct pattern (no domain logic, just result predicate) | `agent_quality_orchestrator.rs:158–160` | No action needed — this is a pure utility predicate, not business logic. |

## Validation

- [x] FRD compliance checked — All 5 FRs (FR-001 through FR-005) are implemented by corresponding capabilities. 1:1 mapping confirmed.
- [x] AES compliance checked — File naming (AES101/102), layer dependencies (AES201), capability protocol impl (AES303), zero-I/O agent (AES405) all pass.
- [x] Skip Report validated — BA and TL reports were both empty (no output files). Independent analysis performed as fallback.
- [x] Assumptions validated — Pre-fetched FileEntry model confirmed in `run_analysis_with_entries`. Zero I/O enforced. Config-driven thresholds confirmed.
- [x] Timestamp + Correlation ID signed — `2026-08-07T14:20:00+07:00` / `corr-20260807-quality-rules-140`

## Action Items

- [ ] 🟡 **P2** Extract AES305 inline message format into shared `format_code_analysis_violation()` function (Issue #4)
- [ ] 🟡 **P3** Replace `classify_token` character-array strings with shared constants or string literals (Issue #1)
- [ ] 🟢 **P4** Monitor `capabilities_check_bypass_checker.rs` file size as patterns grow (Issue #2)

## Fixed Code

### Issue #4: Extract AES305 inline format to shared function

The orchestrator currently formats AES305 violations inline:

```rust
// agent_quality_orchestrator.rs:134–142 (CURRENT)
let msg = match &aes_violation {
    shared::quality_rules::AesCodeAnalysisViolation::CodeDuplication { reason } => {
        format!(
            "AES305 CODE_DUPLICATION: Duplicate code block detected.\nWHY? {}\nFIX: Extract the duplicated logic into a shared function.",
            reason.as_ref().map(|r| r.to_string()).unwrap_or_default()
        )
    }
    other => format!("{:?}", other),
};
```

**Recommended fix** — Add a `format_code_analysis_violation()` function in `shared::quality_rules` that owns the full message:

```rust
// In shared crate — new function
pub fn format_code_analysis_violation(v: &AesCodeAnalysisViolation) -> String {
    match v {
        AesCodeAnalysisViolation::CodeDuplication { reason } => {
            format!(
                "AES305 CODE_DUPLICATION: Duplicate code block detected.\nWHY? {}\nFIX: Extract the duplicated logic into a shared function.",
                reason.as_ref().map(|r| r.to_string()).unwrap_or_default()
            )
        }
        // ... other variants
    }
}
```

Then the orchestrator calls:

```rust
let msg = shared::quality_rules::format_code_analysis_violation(&aes_violation);
```

### Issue #1: Replace classify_token character arrays

**Current** (fragile):
```rust
let mk = |c: &[char]| c.iter().collect::<String>();
let unwrap = mk(&['u', 'n', 'w', 'r', 'a', 'p']);
```

**Recommended** (readable):
```rust
fn classify_token(token: &str) -> ViolationKind {
    match token {
        "unwrap" | "expect" => ViolationKind::UnwrapExpect,
        "panic" => ViolationKind::Panic,
        "todo" => ViolationKind::Todo,
        "unimplemented" | "unreachable" => ViolationKind::Unimplemented,
        _ => ViolationKind::BypassComment,
    }
}
```

Same pattern applies to `default_forbidden_bypass()` — use string literals instead of char arrays.
