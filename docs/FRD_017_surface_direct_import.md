# 📄 Feature Requirements Document (FRD)
**Feature Name:** Surface Direct Import Checker (AES023)  
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
This document defines the AES023 rule that prevents surfaces from directly importing infrastructure or capabilities. Surfaces must access these layers ONLY through `ServiceContainerAggregate` in the contract layer.

### 2.2 Scope
**In-Scope:**
- Surface files importing from `infrastructure/` or `capabilities/` directly
- CRITICAL severity (auto-fail)
- Governance rules from YAML config

**Out-of-Scope:**
- Other surface rules (AES022 — separate FRD)
- Non-surface layer import validation

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES023** | Rule code for surface direct import |
| **check_legacy_import_rules()** | Main detection method |
| **governance_rules** | Config field — source→forbidden_target pairs |
| **ServiceContainerAggregate** | Contract aggregate — the ONLY way surfaces access infra/cap |

## 3. Feature Overview
### 3.1 Background & Problem
Surfaces imported infrastructure and capabilities directly instead of using the DI container. This created tight coupling between surfaces and concrete implementations, defeating the purpose of dependency inversion and making the code harder to test and maintain.

### 3.2 Business Goals
- Enforce dependency inversion for surface layer
- Surfaces must use `ServiceContainerAggregate` exclusively
- CRITICAL severity ensures immediate attention

### 3.3 Target Users
- **Developers**: Remember to use DI container instead of direct imports
- **Architects**: Maintain decoupled architecture

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be blocked immediately if I import infrastructure directly from a surface, so I use the DI container instead.

### 4.2 Use Cases & Workflow
**Detection:**
```
File: surfaces/cli_check_command.rs

1. Parse: "use crate::infrastructure::python_ruff::RuffAdapter" ← infra
             "use crate::contract::ServiceContainerAggregate" ← contract (allowed)
             "use crate::taxonomy::FilePath" ← taxonomy (allowed)

2. For each import:
   - detect_module_layer("infrastructure") → "infrastructure"
   - Check governance_rules: { source: "surface", target: "infrastructure" } → MATCH
   - VIOLATION AES023 CRITICAL

3. Contract and taxonomy imports:
   - No governance rule matches → OK
```

**Correct pattern:**
```rust
// Surface using DI container — CORRECT
let linter = container.get_architecture_linter();
linter.run_self_lint(path).await;
```

### 4.3 Business Rules
- Severity: CRITICAL (auto-fail)
- Only applies to surface files
- Agent files are SKIPPED (`if file_layer == "agent" { return; }`)
- Governance rules from `ArchitectureConfig.governance_rules`

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per surface file | < 10ms |

## 6. UI/UX Requirements
```
AES001 CRITICAL - src-rust/surfaces/cli_check_command.rs:42
  [AES Layer Violation] Surfaces must NOT import infrastructure directly.
  File in 'surface' imports from 'infrastructure'.
  WHY? Surfaces must access infrastructure through ServiceContainerAggregate only.
  FIX: Inject dependencies via DI container instead of direct imports.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Surface imports infrastructure directly | `check_legacy_import_rules()` runs | AES023 CRITICAL flagged | ✅ |
| AC-002 | Surface imports capabilities directly | `check_legacy_import_rules()` runs | AES023 CRITICAL flagged | ✅ |
| AC-003 | Surface imports contract/taxonomy only | `check_legacy_import_rules()` runs | No violation | ✅ |
| AC-004 | Agent file imports infrastructure | `check_legacy_import_rules()` runs | Skipped (allowed) | ✅ |

## 8. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Import parsing for layer detection | Regex inaccuracy | Conservative matching |
| Governance rules | Source→target pairs in YAML | Missing rules = no enforcement | Configured by default |

## 9. Appendices
- `src-rust/capabilities/architecture_import_checker.rs:244` — `check_legacy_import_rules()`
- `src-rust/taxonomy/architecture_config_vo.rs` — `governance_rules` config
- `src-rust/contract/service_container_aggregate.rs` — DI contract
- `docs/RULES_AES.md` — Layer import rules
