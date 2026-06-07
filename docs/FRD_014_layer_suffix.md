# 📄 Feature Requirements Document (FRD)
**Feature Name:** Layer Suffix Mismatch Detector (AES011)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 08/06/2026  
**Version:** v1.0  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |

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
File in capabilities/user_vo.rs
  1. get_stem("user_vo.rs") → "user_vo"
  2. get_suffix("user_vo") → "vo"
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
AES011 HIGH - src-rust/capabilities/user_vo.rs
  AES011 SUFFIX_MISMATCH: File uses a forbidden suffix for this layer.
  WHY? Forbidden suffixes prevent technical concepts from leaking.
  FIX: Rename or move to correct layer.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File in capabilities/ uses `_vo` suffix | `check_domain_suffixes()` runs | AES011 HIGH flagged | ✅ |
| AC-002 | File in infrastructure/ uses `_port` suffix | `check_domain_suffixes()` runs | AES011 HIGH flagged | ✅ |
| AC-003 | File uses allowed suffix | `check_domain_suffixes()` runs | No AES011 violation | ✅ |
| AC-004 | Forbidden match found | AES011 triggers | Early return (no AES010) | ✅ |

## 8. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| YAML config | `forbidden_suffix` per layer | Missing list = no rules | Configured by default |

## 9. Appendices
- `src-rust/capabilities/architecture_naming_checker.rs:152` — AES011 path
- `src-rust/taxonomy/layer_definition_vo.rs` — `forbidden_suffix` config field
- `docs/RULES_AES.md` — Forbidden suffix matrix
