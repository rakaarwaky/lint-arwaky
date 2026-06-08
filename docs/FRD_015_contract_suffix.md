# 📄 Feature Requirements Document (FRD)
**Feature Name:** Contract Suffix Mismatch Detector (AES008)  
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
This document defines the AES008 rule that enforces contract-specific suffixes. Every `contract_`-prefixed file must use `_port`, `_protocol`, or `_aggregate` suffix to declare its architectural role.

### 2.2 Scope
**In-Scope:**
- Files with `contract_` filename prefix
- Required suffixes: `_port`, `_protocol`, `_aggregate`
- Skipping barrel files (`mod.rs`)
- HIGH severity violations

**Out-of-Scope:**
- Other layers' suffix rules (AES010, AES011 — separate FRDs)
- Content validation of contract files

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES008** | Rule code for contract suffix mismatch |
| **check_domain_suffixes()** | Main detection method (AES008 path at line 186) |
| **_port** | Outbound interface suffix |
| **_protocol** | Inbound interface suffix |
| **_aggregate** | Composition facade suffix |

## 3. Feature Overview
### 3.1 Background & Problem
Files in the contract layer had ambiguous names like `contract_helpers.rs` or `types.rs` that didn't communicate their architectural role. The architecture mandates exactly three roles: ports (outbound), protocols (inbound), and aggregates (composition). Every file must declare which role it serves.

### 3.2 Business Goals
- Every contract file clearly identifies its role via suffix
- Enforce exactly three allowed suffixes in contract layer
- Provide clear naming guidance

### 3.3 Target Users
- **Developers**: Use the correct suffix when adding contract files
- **Architects**: Maintain clear contract layer structure

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer adding a contract_-prefixed file, I want to be told if my filename doesn't end with _port, _protocol, or _aggregate, so I follow the naming convention.

### 4.2 Use Cases & Workflow
**Detection:**
```
File: contract_helpers.rs
  filename starts with "contract_" → layer: contract
  suffix_policy: strict
  allowed_suffix: ["port", "protocol", "aggregate"]

  get_stem("contract_helpers.rs") → "contract_helpers"
  get_suffix("contract_helpers") → "helpers"

  "helpers" NOT in allowed_suffix
  layer == "contract" → emit AES008 code
```

**Valid files:**
```
service_container_aggregate.rs → suffix "aggregate" Pending Review
source_parser_port.rs          → suffix "port" Pending Review
arch_rule_protocol.rs          → suffix "protocol" Pending Review
```

### 4.3 Business Rules
- Severity: HIGH
- Same code path as AES010 but emits AES008 code for contract layer
- Skip mod.rs barrel files

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |

## 6. UI/UX Requirements
```
AES008 HIGH - src-rust/di-containers/contract_project_helpers.rs
  AES008 SUFFIX_MISMATCH: File is missing a required strict suffix.
  WHY? Contract files must use _port, _protocol, or _aggregate suffix.
  FIX: Add one of: port, protocol, aggregate.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Contract file without port/protocol/aggregate suffix | `check_domain_suffixes()` runs | AES008 HIGH flagged | Pending Review |
| AC-002 | Contract file with valid suffix | `check_domain_suffixes()` runs | No violation | Pending Review |
| AC-003 | Barrel file (mod.rs) | `check_domain_suffixes()` runs | Skipped (barrel) | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| AES008 code path (in `check_domain_suffixes()`) | `naming-rules/capabilities_naming_checker.rs:209` | 6 lines (209-214) | Active — conditional branch inside the AES010/AES011 method |
| Contract suffix validation (inheritance context) | `layer-rules/capabilities_inheritance_checker.rs:82` | 6 lines | Active — checks `_port`, `_protocol`, `_aggregate` for import/inheritance resolution |
| Contract layer config | `lint_arwaky.config.rust.yaml:48-49` | — | `strict: ["port", "protocol", "aggregate"]` |
| Test fixture | `test-project-rust/` | — | **None present** — no AES008 fixture exists |

AES008 is not a standalone method. It is a **code-path branch** inside `check_domain_suffixes()`:
1. The AES010 strict suffix check runs identically for all layers (line 192)
2. If a violation is found AND the layer is `"contract"`, the emitted code is `"AES008"` (line 209)
3. For all other layers, the emitted code is `"AES010"` (line 212)
4. The violation message is **identical** for AES008 and AES010 — it hardcodes `"AES011"` as the prefix regardless

### 8.2 Bugs Found

| # | Bug | Location | Impact | Fix |
|---|-----|----------|--------|-----|
| B1 | **Wrong code in violation message** | `capabilities_naming_checker.rs:203` | The violation message always starts with `"AES011 SUFFIX_MISMATCH: ..."` regardless of the actual code (`AES008` for contract, `AES010` for non-contract). An AES008 violation shows the wrong rule code in its message body, confusing developers about which rule was triggered. | Change the message format to use the `code` variable: `format!("{} SUFFIX_MISMATCH: ...", code)` or use `def.suffix_violation_message` which correctly references `AES008` in the YAML (line 242). |
| B2 | **No test fixture for AES008** | `test-project-rust/` | Unlike AES010 and AES011 which have test fixtures, AES008 has **zero** test coverage. The contract-layer-only branch (line 209) is never exercised by any test project. | Create `test-project-rust/src-rust/contract/helpers_contract.rs` with a forbidden suffix and expect AES008. |
| B3 | **Zero line number** | `capabilities_naming_checker.rs:28` | Same as AES010/AES011 — violation reports line 0. | Use `LineNumber::new(1)` for file-level violations. |
| B4 | **AES008 depends on AES010's config** | `capabilities_naming_checker.rs:192` | AES008 only fires when `suffix_policy == "strict"` AND suffix is not in `allowed_suffix`. If someone changes the contract layer's policy to `flexible`, AES008 violations silently disappear. The rule should enforce contract suffixes regardless of policy. | Add a dedicated contract-suffix check outside the AES010/AES011 flow, or make contract layer always strict in code regardless of config. |

### 8.3 What Needs to Be Added

1. **Dedicated AES008 checker method** — AES008's logic is a 6-line branch inside a shared method. Extract it to a standalone `check_contract_suffix()` method for clarity and testability.
2. **Unit tests**:
   - `contract_helpers.rs` → AES008 HIGH (`_helpers` not in allowed list)
   - `contract_service_aggregate.rs` → no violation (`_aggregate` is allowed)
   - `contract/mod.rs` → skipped (barrel file)
   - Contract layer with `suffix_policy: flexible` → should still enforce AES008
3. **Integration test fixture** — add `test-project-rust/src-rust/contract/` directory with a violating file and wire it into the test runner.
4. **Message fix** — use dynamic rule code in the violation message instead of hardcoding `"AES011"`.

### 8.4 What to Keep

1. **Config-driven allowed suffixes** — contract allowed suffixes are defined in YAML (`strict: ["port", "protocol", "aggregate"]`), making them customizable without code changes.
2. **Shared code path reuse** — reusing `check_domain_suffixes()` for AES008 avoids duplication of the stem/suffix extraction logic and the strict policy check.
3. **Barrel file skipping** — the check inherits `is_barrel_file()` and `is_entry_point()` filtering correctly.
4. **YAML message override** — the YAML has a specific `suffix_violation_message` for the contract scope (config line 241-244) mentioning AES008. If `def.suffix_violation_message.value` is set, the correct message is used — the hardcoded fallback at line 203 is only reached when no override exists.

### 8.5 Empirical Evidence from Test Projects

| Project | File | Expected | Actual (current) | Notes |
|---------|------|----------|------------------|-------|
| `test-project-rust` | — | AES008 | ❌ **No test fixture exists** | No contract-layer file in test-project-rust exercises AES008. |
| `self-lint` | `contract_adapter_port.rs` | No violation | ✅ | Correct `_port` suffix passes. |
| `self-lint` | `contract_fix_aggregate.rs` | No violation | ✅ | Correct `_aggregate` suffix passes. |
| `self-lint` | `contract_reporting_protocol.rs` | No violation | ✅ | Correct `_protocol` suffix passes. |
| `self-lint` | `di-containers/contract_service_aggregate.rs` | No violation | ✅ | Correct suffix, though in `di-containers/` feature folder. |
| `self-lint` | (hypothetical) `contract_helpers.rs` | AES008 HIGH | ❌ Would fire but with message saying `AES011` (B1) | The wrong-code-in-message bug affects all contract-layer violations. |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | `allowed_suffix` for contract layer | Must include port, protocol, aggregate | Configured by default in YAML |

## 10. Appendices
- `src-rust/layer-rules/capabilities_naming_checker.rs:186` — AES008 code path
- `src-rust/shared-common/taxonomy_layer_vo.rs` — `allowed_suffix` config field
- `docs/ARCHITECTURE.md` — Contract layer specification
