# 📄 Feature Requirements Document (FRD)
**Feature Name:** Layer Suffix Mismatch Detector (AES011)  
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
This document defines the AES011 rule that detects forbidden suffixes in layers. Certain suffixes are reserved for specific layers (e.g., `_vo` belongs in taxonomy only) — if they appear in disallowed layers, they are flagged.

### 2.2 Scope
**In-Scope:**
- Checking suffix against `forbidden_suffix.values` per layer
- Capabilities forbidden: `_vo`, `_entity`, `_error`, `_event`, `_port`, `_protocol`, `_aggregate`, `_io`
- Infrastructure forbidden: same list
- Early return after first match (no cascade to AES010)
- HIGH severity violations

**Out-of-Scope:**
- Allowed suffix enforcement (AES010 — separate FRD)
- Contract-specific rules (AES008 — separate FRD)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES011** | Rule code for forbidden suffix violation |
| **check_domain_suffixes()** | Main detection method (AES011 path) |
| **forbidden_suffix** | Config field — list of prohibited suffixes per layer |

## 3. Feature Overview
### 3.1 Background & Problem
Domain suffixes (`_vo`, `_entity`) were appearing in capabilities and infrastructure layers where they don't belong. Contract suffixes (`_port`, `_protocol`) were leaking outside the contract layer. These suffixes have specific architectural meanings and must stay in their designated layers.

### 3.2 Business Goals
- Prevent domain suffixes from leaking into non-domain layers
- Prevent contract suffixes from leaking into non-contract layers
- Clear violation messages explaining which suffix is forbidden

### 3.3 Target Users
- **Developers**: Told when using a suffix reserved for another layer
- **Architects**: Maintain clear separation of concerns

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned if I use a suffix that belongs to another layer, so I don't violate architectural boundaries.

### 4.2 Use Cases & Workflow
**Detection:**
```
File prefixed capabilities_ (e.g., capabilities_user_vo.rs)
  filename starts with "capabilities_" → layer = "capabilities"
  1. get_stem("capabilities_user_vo.rs") → "capabilities_user_vo"
  2. get_suffix("capabilities_user_vo") → "vo"
  3. Look up forbidden_suffix for capabilities:
     → ["vo", "entity", "error", "event", "port", "protocol", "aggregate", "io"]
  4. "vo" IS in forbidden list
  5. Flag AES011 HIGH
  6. EARLY RETURN (no AES010 allowed-suffix check)
```

### 4.3 Business Rules
- Severity: HIGH
- Runs BEFORE AES010 strict policy check
- Early return prevents double-flagging
- Forbidden list configurable via YAML

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file | < 5ms |

## 6. UI/UX Requirements
```
AES011 HIGH - src-rust/layer-rules/capabilities_user_vo.rs
  AES011 SUFFIX_MISMATCH: File uses a forbidden suffix for this layer.
  WHY? Forbidden suffixes prevent technical concepts from leaking.
  FIX: Rename or move to correct layer.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File prefixed capabilities_ uses `_vo` suffix | `check_domain_suffixes()` runs | AES011 HIGH flagged | Pending Review |
| AC-002 | File prefixed infrastructure_ uses `_port` suffix | `check_domain_suffixes()` runs | AES011 HIGH flagged | Pending Review |
| AC-003 | File uses allowed suffix | `check_domain_suffixes()` runs | No AES011 violation | Pending Review |
| AC-004 | Forbidden match found | AES011 triggers | Early return (no AES010) | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| `check_domain_suffixes()` AES011 path | `naming-rules/capabilities_naming_checker.rs:177` | 13 lines | Active — runs before AES010 strict check |
| Forbidden suffix config | `lint_arwaky.config.rust.yaml:62,76` | — | Capabilities/Infra forbid: `vo, entity, error, event, port, protocol, aggregate, io` |
| Test fixture | `test-project-rust/src-rust/capabilities/forbidden_suffix_vo.rs` | 3 lines | Present — `_vo` suffix in capabilities layer |

The AES011 detection is the first branch inside `check_domain_suffixes()`:
1. Extract suffix from filename stem via `get_suffix()` — line 174
2. Check if suffix exists in `def.forbidden_suffix.values` — line 178
3. If matched: emit AES011 HIGH, **return early** (line 187) — AES010 is never evaluated
4. If no match: fall through to AES010 strict policy check at line 192

This early-return design is intentional per FRD AC-004: "Forbidden match found → AES011 triggers → Early return (no AES010)."

### 8.2 Bugs Found

| # | Bug | Location | Impact | Fix |
|---|-----|----------|--------|-----|
| B1 | **Zero line number** | `capabilities_naming_checker.rs:28` — `make_result` | All AES011 violations report line 0. Per-file violations should report line 1 for navigability. | Change `LineNumber::new(0)` to `LineNumber::new(1)`. |
| B2 | **No unit tests for the forbidden suffix codepath** | `capabilities_naming_checker.rs` — no `#[cfg(test)]` | Forbidden suffix logic (early return, config-driven list) has zero test coverage. A config change that accidentally removes the forbidden list goes undetected. | Add tests covering: (a) forbidden suffix → AES011, (b) early return prevents AES010, (c) allowed suffix → no AES011, (d) empty forbidden list → no AES011. |
| B3 | **Forbidden suffix config only defined for capabilities and infrastructure** | `lint_arwaky.config.rust.yaml:62,76` | The FRD says forbidden suffixes should apply to capabilities and infrastructure. But surfaces, agent, taxonomy, and root layers have no `forbidden_suffix` list in the YAML (they use `strict` policy instead). If a layer is misconfigured with both `strict` and no `forbidden`, the forbidden check is a no-op (empty list). This is by design but should be documented. | No code fix needed — document this layering in the FRD or as a YAML comment. |
| B4 | **No `forbidden_suffix` in Python/JS configs** | `lint_arwaky.config.python.yaml`, `lint_arwaky.config.javascript.yaml` | AES011 only enforced for Rust. For cross-language projects, Python/JS files with forbidden suffixes pass silently. | Add equivalent forbidden suffix lists to Python and JS configs, or document as intentional. |

### 8.3 What Needs to Be Added

1. **Unit tests** for `check_domain_suffixes()` AES011 path:
   - `capabilities/user_vo.rs` → AES011 HIGH (forbidden `_vo`)
   - `infrastructure/db_port.rs` → AES011 HIGH (forbidden `_port`)
   - `capabilities/user_helper.rs` → no AES011 (`_helper` not forbidden)
   - Capabilities with no forbidden list configured → no AES011
   - Verify early return: forbidden match prevents AES010 from running
2. **Integration test assertion** — test-project-rust fixture `capabilities/forbidden_suffix_vo.rs` must have a corresponding `expect` entry in the test runner.
3. **Cross-language parity** — add `forbidden_suffix` to Python and JS configs (or document as WONTFIX).

### 8.4 What to Keep

1. **Early return design** — AES011 fires before AES010 and returns immediately. This prevents redundant/conflicting violations and is exactly what AC-004 specifies.
2. **Config-driven lists** — the forbidden suffix list is externalized in YAML, allowing per-project customization without code changes.
3. **Just the right set of forbidden suffixes** — `vo, entity, error, event, port, protocol, aggregate, io` covers all taxonomy and contract suffixes that should never appear in capabilities/infrastructure layers.
4. **Test fixture clarity** — `forbidden_suffix_vo.rs` with `// AES011: capabilities file with forbidden _vo suffix` is a good self-documenting test case.

### 8.5 Empirical Evidence from Test Projects

| Project | File | Expected | Actual (current) | Notes |
|---------|------|----------|------------------|-------|
| `test-project-rust` | `capabilities/forbidden_suffix_vo.rs` | AES011 HIGH | ✅ Likely flagged | `_vo` suffix in capabilities layer is in the forbidden list. Early return prevents AES010. |
| `test-project-rust` | Any `capabilities/*.rs` with allowed suffix | No AES011 | ✅ No false positive | Suffixes like `_checker`, `_processor` are not in the forbidden list. |
| `self-lint` | `capabilities_naming_checker.rs` (in `naming-rules/`) | No AES011 | ✅ | Suffix `_checker` is allowed. |
| `self-lint` | Any `infrastructure/` file with `_adapter` suffix | No AES011 | ✅ | `_adapter` not in forbidden list for infrastructure. |
| `test-project-python` | Capabilities file with `_vo.py` suffix | AES011 HIGH | ❌ Not enforced | Python config has no `forbidden_suffix` entries (B4). |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | `forbidden_suffix` per layer | Missing list = no rules | Configured by default |

## 10. Appendices
- `src-rust/layer-rules/capabilities_naming_checker.rs:152` — AES011 path
- `src-rust/shared-common/taxonomy_layer_vo.rs` — `forbidden_suffix` config field
- `docs/RULES_AES.md` — Forbidden suffix matrix
