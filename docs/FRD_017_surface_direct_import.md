# 📄 Feature Requirements Document (FRD)
**Feature Name:** Surface Direct Import Checker (AES023)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 09/06/2026  
**Version:** v1.1  

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 08/06/2026 | Raka | Initial document creation | [Stakeholder] |
| v1.1 | 09/06/2026 | Raka | Updated to prefix-based architecture: layers are filename prefixes, not directories; updated file paths for 26 feature folders | [Stakeholder] |

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
File: cli-commands/surface_check_command.rs

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
AES001 CRITICAL - src-rust/cli-commands/surface_check_command.rs:42
  [AES Layer Violation] Surfaces must NOT import infrastructure directly.
  File in 'surface' imports from 'infrastructure'.
  WHY? Surfaces must access infrastructure through ServiceContainerAggregate only.
  FIX: Inject dependencies via DI container instead of direct imports.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Surface imports infrastructure directly | `check_legacy_import_rules()` runs | AES023 CRITICAL flagged | Pending Review |
| AC-002 | Surface imports capabilities directly | `check_legacy_import_rules()` runs | AES023 CRITICAL flagged | Pending Review |
| AC-003 | Surface imports contract/taxonomy only | `check_legacy_import_rules()` runs | No violation | Pending Review |
| AC-004 | Agent file imports infrastructure | `check_legacy_import_rules()` runs | Skipped (allowed) | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| `check_legacy_import_rules()` | `layer-rules/capabilities_import_checker.rs:358` | 52 lines | Active — called at coordinator line 141, per file per layer |
| Inline surface import check (AES023) | `code-analysis/agent_checking_coordinator.rs:612` | 18 lines | Active — separate from `check_legacy_import_rules()`, runs for every file |
| Governance rules config | `config-system/taxonomy_config_vo.rs:20` | — | `LegacyLayerRuleList` — optional YAML section |
| Contract protocol trait | `layer-rules/contract_import_protocol.rs:44,101` | — | Trait + default impl for legacy import rules |
| Test fixture | `test-project-rust/` | — | **None present** |

There are **two separate implementations** of AES023-like checking:

1. **Config-driven** (`capabilities_import_checker.rs:358`): Reads `config.governance_rules` to define source→forbidden target pairs. Emits code **`AES001`** (not `AES023`) with CRITICAL severity and correct line numbers. Skips agent layer files (line 370).

2. **Coordinator inline** (`agent_checking_coordinator.rs:612`): String-matches `use ... ::capabilities::`, `::infrastructure::`, or `::agent::` in any file. Emits code `AES023` with **HIGH** severity (not CRITICAL) and **line 0**.

### 8.2 Bugs Found

| # | Bug | Location | Impact | Fix |
|---|-----|----------|--------|-----|
| B1 | **Wrong rule code in import checker** | `capabilities_import_checker.rs:399` — `"AES001"` | The config-driven implementation emits `AES001` (forbidden import) instead of `AES023` (surface direct import). Since AES001 is a different rule with different severity and message, this misclassifies the violation. | Change to `"AES023"` to match FRD specification. |
| B2 | **Code duplication — two AES023 paths** | `agent_checking_coordinator.rs:612` + `capabilities_import_checker.rs:358` | The coordinator inline check (lines 612-629) duplicates the config-driven check. Both can fire for the same import, producing two violations. Worse, they disagree on severity (HIGH vs CRITICAL) and rule code (`AES023` vs `AES001`). | Remove the coordinator inline checker; let the config-driven `check_legacy_import_rules()` handle all AES023 detection. |
| B3 | **Coordinator inline uses wrong severity** | `agent_checking_coordinator.rs:623` — `Severity::HIGH` | FRD section 4.3 specifies CRITICAL severity (auto-fail). The coordinator uses HIGH, which doesn't trigger auto-fail. | Change to `Severity::CRITICAL`. |
| B4 | **Coordinator inline uses line 0** | `agent_checking_coordinator.rs:621` | Violation reports line 0 instead of the actual import line. The config-driven version correctly reports `*line_num as i64`. | Use the actual line number from `enumerate()`. |
| B5 | **No AES023 test fixture** | `test-project-rust/` | Unlike AES007, AES010, AES011 which have fixtures, AES023 has zero test coverage. | Create `test-project-rust/src-rust/surfaces/direct_import_surface.rs` importing capabilities/infrastructure directly and expect AES023 CRITICAL. |
| B6 | **Coordinator inline check is too broad** | `agent_checking_coordinator.rs:615-617` | Checks for `::capabilities::`, `::infrastructure::`, `::agent::` in ANY file, not just surface files. Non-surface files (e.g., contract files) that import from capabilities would be incorrectly flagged. The coordinator only calls this at the file level without first checking the file's layer. | Gate the check: only run for files with `surface_` prefix, or rely solely on the config-driven version which already handles layer context. |

### 8.3 What Needs to Be Added

1. **Unified AES023 path** — remove the coordinator inline duplicate; fix the import checker to emit `AES023` instead of `AES001`.
2. **Governance rules YAML config** — ensure `lint_arwaky.config.rust.yaml` has a `governance_rules` section covering surface→infrastructure and surface→capabilities.
3. **Integration test fixture** — `test-project-rust/src-rust/surfaces/direct_import_surface.rs` importing from `crate::capabilities::...` and `crate::infrastructure::...`.
4. **Unit tests** — `check_legacy_import_rules()` with mock config:
   - Surface file importing infrastructure → AES023 CRITICAL
   - Surface file importing capabilities → AES023 CRITICAL
   - Surface file importing contract/taxonomy only → no violation
   - Agent file importing anything → skipped (line 370)
   - Empty governance rules → no-op (line 365 early return)

### 8.4 What to Keep

1. **Config-driven approach** — `check_legacy_import_rules()` uses governance rules from YAML, making the forbidden import pairs configurable without code changes.
2. **Agent layer skip** — line 370 correctly exempts agent files, which are allowed to import infrastructure/capabilities.
3. **Line-number accuracy** — the import checker version correctly reports `*line_num as i64` (unlike the coordinator version that uses line 0).
4. **Early return on empty rules** — line 365 checks `config.governance_rules.is_empty()` and returns immediately, avoiding unnecessary file I/O.
5. **Multi-separator support** — `detect_module_layer()` at line 411 handles both Rust `::` and Python `.` path separators.

### 8.5 Empirical Evidence from Test Projects

| Project | File | Expected | Actual (current) | Notes |
|---------|------|----------|------------------|-------|
| `test-project-rust` | — | AES023 CRITICAL | ❌ **No test fixture exists** | No surface file with direct infrastructure/capabilities import exists. |
| `self-lint` | `cli-commands/surface_check_command.rs` | No AES023 | ✅ (config-driven) | Uses contract/taxonomy imports only. But coordinator inline (B6) would actually flag this if the line contained `::capabilities::`. |
| `self-lint` | `cli-commands/surface_fix_command.rs` | No AES023 | ✅ | Uses `container.get_*()` pattern, no direct infra/cap imports. |
| `self-lint` | `code-analysis/agent_checking_coordinator.rs` | Skipped (agent) | ✅ Skipped | Line 370 skips agent layer files. |
| `self-lint` | Any surface file importing infrastructure | AES023 CRITICAL | ❌ **Wrong code** (B1) | Config-driven version emits AES001 instead of AES023. |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Import parsing for layer detection | Regex inaccuracy | Conservative matching |
| Governance rules | Source→target pairs in YAML | Missing rules = no enforcement | Configured by default |

## 10. Appendices
- `src-rust/layer-rules/capabilities_import_checker.rs:244` — `check_legacy_import_rules()`
- `src-rust/shared-common/taxonomy_config_vo.rs` — `governance_rules` config
- `src-rust/di-containers/contract_service_aggregate.rs` — DI contract
- `docs/RULES_AES.md` — Layer import rules
