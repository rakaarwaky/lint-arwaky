# 📄 Feature Requirements Document (FRD)
**Feature Name:** Root Layer Detection / Strict Suffix Policy (AES010)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 09/06/2026  
**Version:** v1.1  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |
| v1.1 | 09/06/2026 | Raka | Updated to prefix-based architecture: layers are filename prefixes, not directories; updated file paths to reflect 26 feature folders | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the AES010 rule that enforces strict suffix policy. Layers with `suffix_policy: "strict"` require every file to use an allowed suffix from the layer's definition. Files without a valid suffix are flagged.

### 2.2 Scope
**In-Scope:**
- Extracting suffix from filename stem
- Validating against `allowed_suffix.values` per layer
- Skipping barrel files and entry points
- HIGH severity violations

**Out-of-Scope:**
- Forbidden suffix rules (AES011 — separate FRD)
- Contract-specific suffix rules (AES008 — separate FRD)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES010** | Rule code for root layer suffix policy violation |
| **check_domain_suffixes()** | Main detection method |
| **get_stem()** | Removes file extension |
| **get_suffix()** | Extracts last underscore-delimited word |
| **suffix_policy** | Config field: "strict" or "flexible" |

## 3. Feature Overview
### 3.1 Background & Problem
Files in capabilities, infrastructure, and other layers could have arbitrary names — `helpers.rs`, `utils.rs`, `types.rs` — with no indication of their architectural role. Without enforced suffixes, a file's layer identity was not visible from its name.

### 3.2 Business Goals
- Every filename communicates its architectural role via suffix
- Enforce layer-specific allowed suffix lists
- Provide clear guidance on which suffix to use

### 3.3 Target Users
- **Developers**: Guided to use correct suffix for their layer
- **Architects**: Standardize naming across the codebase

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer adding a capabilities_-prefixed file, I want to be told which suffix to use, so my file follows the naming convention.

### 4.2 Use Cases & Workflow
**Suffix Extraction:**
```
"architecture_import_checker.rs"
  → stem: "architecture_import_checker"
  → suffix: "checker"

"project_helpers.rs"
  → stem: "project_helpers"
  → suffix: "helpers"
```

**Validation:**
```
File: capabilities_project_helpers.rs
  filename starts with "capabilities_" → layer = "capabilities"
  suffix = "helpers"

Look up capabilities layer definition:
  allowed_suffix = ["analyzer", "checker", "processor", ...]
  suffix_policy = "strict"

"helpers" NOT in allowed list
  → AES010 HIGH violation
```

### 4.3 Business Rules
- Severity: HIGH
- Skip barrel files (`mod.rs`), entry points, and exception files
- If layer is "contract" → emit AES008 instead of AES010
- Configurable via YAML `suffix_policy` and `allowed_suffix`

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |

## 6. UI/UX Requirements
```
AES010 HIGH - src-rust/layer-rules/capabilities_project_helpers.rs
  AES010 SUFFIX_MISMATCH: File is missing a required strict suffix.
  WHY? Strict suffixes ensure every component has a clear role.
  FIX: Add one of: analyzer, checker, processor, evaluator, ...
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File prefixed capabilities_ has no valid suffix | `check_domain_suffixes()` runs | AES010 HIGH flagged | Pending Review |
| AC-002 | Barrel file (mod.rs) | `check_domain_suffixes()` runs | Skipped | Pending Review |
| AC-003 | Entry point file (main.rs / index.ts / main.py) | `check_domain_suffixes()` runs | Skipped | Pending Review Recognizes Rust (main.rs, lib.rs), Python (main.py, app.py, __init__.py), JS/TS (index.js, index.ts, index.jsx, index.tsx, main.ts) |
| AC-004 | File has valid suffix for its layer | `check_domain_suffixes()` runs | No violation | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| `check_domain_suffixes()` | `naming-rules/capabilities_naming_checker.rs:148` | 70 lines | Active — called at coordinator line 154 per-file |
| Contract protocol trait | `layer-rules/contract_rule_protocol.rs:39,188` | — | Trait + default impl (delegating) |
| Multi-project trait | `multi-project/contract_governance_protocol.rs:18` | — | Trait declaration only |
| Test fixture | `test-project-rust/src-rust/root_violation.rs` | 4 lines | Present — root file missing suffix |

The AES010 check shares the `check_domain_suffixes()` method with AES011 (forbidden suffix). The method:
1. Skips barrel files (`mod.rs`, `__init__.py`, `index.ts`, etc.) — line 156
2. Skips entry points (`main.rs`, `lib.rs`, `app.py`, etc.) — line 156
3. Checks exceptions list — line 165
4. Extracts stem via `get_stem()` — line 169
5. Extracts last underscore-delimited word via `get_suffix()` — line 174
6. First checks forbidden suffixes (AES011 lines 177-189) — exits early on match
7. Then checks strict suffix policy (AES010 lines 192-216)
8. For contract layer, maps to AES008 instead — line 209

### 8.2 Bugs Found

| # | Bug | Location | Impact | Fix |
|---|-----|----------|--------|-----|
| B1 | **Wrong severity label in violation message** | `capabilities_naming_checker.rs:203` | The hardcoded message reads `"AES011 SUFFIX_MISMATCH: ..."` even when the violation code is `"AES010"` (line 212). Users see the wrong rule code in the message body, causing confusion about which rule was violated. | Use the same `code` variable in the message format string, or build the message with `format!("{} SUFFIX_MISMATCH: ...", code)`. |
| B2 | **Zero line number** | `capabilities_naming_checker.rs:28` — `make_result` always uses `LineNumber::new(0)` | Violations report line 0, making it impossible to locate the offending file in large directories. While suffix violations are per-file (not per-line), reporting line 1 or the file's first meaningful line would be better UX. | Use `LineNumber::new(1)` for file-level violations. |
| B3 | **Missing unit tests** | Entire file | No tests for `check_domain_suffixes()`. Critical edge cases: barrel file skip, entry point skip, exception list, forbidden suffix higher priority than strict suffix, contract layer mapping to AES008, missing stem, no suffix, empty allowed list. | Add a `#[cfg(test)]` module with at least 10 test cases covering the above. |
| B4 | **FRD appendix path outdated** | `FRD_013.md:118` | References `layer-rules/capabilities_naming_checker.rs:124` (old path). Actual file is at `naming-rules/capabilities_naming_checker.rs:148`. | Update appendix path. |

### 8.3 What Needs to Be Added

1. **Unit test module** (`#[cfg(test)]` in `capabilities_naming_checker.rs`) covering:
   - Strict suffix violation → AES010 HIGH
   - Valid suffix → no violation
   - Barrel file → skipped
   - Entry point → skipped
   - Exception file → skipped
   - Forbidden suffix overrides strict (AES011 fires, not AES010)
   - Contract layer strict violation → AES008 HIGH (not AES010)
   - Flexible policy → skipped regardless of suffix
   - Missing underscore in filename → `get_suffix()` returns None → violation
2. **Integration test** — `test-project-rust/src-rust/root_violation.rs` fixture exists but no test assertion verifies AES010 is emitted for it.
3. **Message consistency fix** — the AES011/AES010 message prefix should match the emitted code dynamically.
4. **Config-driven suffix policy** — verify that `suffix_policy: "flexible"` correctly skips the strict check (line 192 gates on `== "strict"`).

### 8.4 What to Keep

1. **Proper skip logic** — Barrel files, entry points, and exceptions are all correctly excluded before any suffix analysis.
2. **AES008 delegation** — The contract layer → AES008 mapping at line 209 avoids double-reporting with AES008's dedicated checker.
3. **Forbidden suffix priority** — AES011 check runs before AES010 and returns early, preventing redundant violations.
4. **CLI integration** — Called centrally from `agent_checking_coordinator.rs:154` alongside all other per-file checks.
5. **Multi-protocol support** — The trait in `contract_rule_protocol.rs` allows other implementations for different project types.

### 8.5 Empirical Evidence from Test Projects

| Project | File | Expected | Actual (current) | Notes |
|---------|------|----------|------------------|-------|
| `test-project-rust` | `src-rust/root_violation.rs` | AES010 HIGH | ✅ Likely flagged | Root file has no suffix for a strict-policy layer — standard case. |
| `test-project-rust` | `capabilities/forbidden_suffix_vo.rs` | AES011 HIGH | ✅ Flagged as AES011 (forbidden suffix checked first) | AES011 check at line 177 returns before AES010 check at line 192. |
| `self-lint` | Any compliant file | No violation | ✅ No false positive | Valid suffix in allowed list passes. |
| `self-lint` | Contract layer file missing suffix | AES008 HIGH | ✅ Mapped correctly | Line 209 checks layer name. |
| `self-lint` | Barrel `mod.rs` | Skipped | ✅ Skipped | `is_barrel_file()` returns true at line 156. |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | `suffix_policy` and `allowed_suffix` | Missing config = no rules | Built-in defaults |

## 10. Appendices
- `src-rust/layer-rules/capabilities_naming_checker.rs:124` — `check_domain_suffixes()`
- `src-rust/shared-common/taxonomy_layer_vo.rs` — `suffix_policy`, `allowed_suffix`
- `docs/RULES_AES.md` — Allowed suffix lists per layer
- `docs/ARCHITECTURE.md` — Layer specifications
