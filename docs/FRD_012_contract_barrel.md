# 📄 Feature Requirements Document (FRD)
**Feature Name:** Contract Barrel Import Checker (AES007)  
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
This document defines the AES007 rule that enforces barrel-style imports from the contract layer. Contract-prefixed types (`contract_*`) must be imported via the feature folder's barrel (`mod.rs`) using `crate::contract::TypeName` instead of submodule paths.

### 2.2 Scope
**In-Scope:**
- Detecting `crate::contract::xxx::yyy` submodule import patterns
- Rust files only (Python/JS use different module systems)
- MEDIUM severity violations

**Out-of-Scope:**
- Python/JS import style validation
- Forbidden import rules (AES001)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES007** | Rule code for contract barrel import |
| **check_contract_barrel()** | Main detection method |
| **Barrel** | Feature folder's `mod.rs` that re-exports all contract types (e.g., `di-containers/contract_service_aggregate.rs`) |

## 3. Feature Overview
### 3.1 Background & Problem
Contract types were imported via submodule paths like `crate::contract::source_parser_port::ISourceParserPort`, creating coupling to internal module structure. The contract barrel (re-exported via `contract_service_aggregate.rs`) exists specifically to be the sole API surface — bypassing it defeats this purpose.

### 3.2 Business Goals
- Ensure contract barrel is the only entry point to contract types
- Standardize import patterns across the codebase
- Prevent coupling to internal contract module structure

### 3.3 Target Users
- **Developers**: Guided to use correct import style
- **Architects**: Ensure contract API surface is properly used

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be told when I use a submodule import path for contract types, so I can use the correct barrel style.

### 4.2 Use Cases & Workflow
**Detection:**
```
✅ Correct:  use crate::contract::ServiceContainerAggregate;
❌ Wrong:    use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;

Scan line: "use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate"
  └── Submodule path instead of barrel → VIOLATION (contract_-prefixed type via submodule)

Scan line: "use crate::contract::ServiceContainerAggregate"
  └── Barrel import → OK
```

### 4.3 Business Rules
- Severity: MEDIUM
- Only applies to Rust files
- Pattern: `crate::contract::<submodule>::<Type>` = violation

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per line | < 1ms |

## 6. UI/UX Requirements
```
AES007 MEDIUM - src-rust/cli-commands/surface_check_command.rs:5
  AES007 CONTRACT_BARREL: Contract import must be from barrel.
  Use: 'use crate::contract::ServiceContainerAggregate'
  Instead of: 'use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate'
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File uses `crate::contract::submodule::Type` | `check_contract_barrel()` runs | AES007 MEDIUM flagged | Pending Review |
| AC-002 | File uses `crate::contract::Type` | `check_contract_barrel()` runs | No violation | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| `check_contract_barrel()` | `code-analysis/agent_checking_coordinator.rs:398` | 16 lines | Active — called at line 68 per-file |
| `_get_contract_barrel_aliases()` | `layer-rules/capabilities_import_processor.rs:432` | 34 lines | Active — called via `_check_contract_layer` at line 264 |
| Test fixture | `test-project-rust/src-rust/capabilities/deep_import_processor.rs` | 10 lines | Present — imports `crate::contract::sub::module::WrongNamePort` |

There are **two separate implementations** of AES007 detection:
1. **Coordinator** (`agent_checking_coordinator.rs:398`): Simple line-by-line string matching — checks `use crate::di_containers::contract_service_aggregate::` prefix with >4 `::` segments.
2. **Import processor** (`capabilities_import_processor.rs:432`): AST-adjacent alias analysis — splits on `.` (Python-style paths), checks if `contract` segment exists and whether it's the barrel position (`parts[len-2] == "contract"`).

### 8.2 Bugs Found

| # | Bug | Location | Impact | Fix |
|---|-----|----------|--------|-----|
| B1 | **Coordinator regex too narrow** | `agent_checking_coordinator.rs:400-404` | Only catches `crate::di_containers::contract_service_aggregate::X` — misses the general `crate::contract::sub::module::Type` pattern documented in AC-001 and the FRD use-case. The test project fixture (`deep_import_processor.rs`) uses `crate::contract::sub::module::WrongNamePort`, which this checker **will not flag**. | Change prefix check to `t.starts_with("use crate::contract::")` and keep the `::` count > 3 logic (crate + contract + submodule + Type = 4 segments, barrel = 3 segments). |
| B2 | **Code duplication — two independent implementations** | `agent_checking_coordinator.rs:398` vs `capabilities_import_processor.rs:432` | Both claim AES007 but use different matching logic. The coordinator never delegates to the import processor. When both run, a violation could get reported twice — or not at all if neither catches the pattern. | Consolidate: remove the coordinator's inline checker and let the import processor (which has richer alias analysis) handle all AES007 detection. |
| B3 | **Import processor targets Python-style paths** | `capabilities_import_processor.rs:442` — `fullname.split('.')` | The import processor splits on `.` (Python dotted-path style) but Rust paths use `::`. For real Rust imports, the import processor will never see `contract` as a segment, so the guard `parts.contains(&"contract")` may never match. The AES007 check there is effectively dead code for Rust files. | Make the splitting style configurable per language, or convert Rust `::` paths to dotted paths before analysis. |
| B4 | **Zero line number** | `capabilities_import_processor.rs:454` — `LineNumber::new(0)` | Violations are reported with `line: 0`, meaning the output can't be mapped back to the source line. The coordinator version correctly uses `i + 1`. | Pass the actual line number from the import's span information. |
| B5 | **FRD appendix paths are outdated** | `FRD_012.md:100-101` | References `layer-rules/capabilities_compliance_analyzer.rs:414` (no such method — that file is only 390 lines) and `pipeline-jobs/agent_checking_coordinator.rs:191` (file moved to `code-analysis/`). | Update appendix paths to match current file structure. |

### 8.3 What Needs to Be Added

1. **Unified detection path** — eliminate the coordinator-level duplication; route all AES007 through `capabilities_import_processor.rs` with proper Rust `::` path splitting.
2. **Unit tests** — zero unit tests exist for AES007. Must cover:
   - Barrel import (`use crate::contract::TypeName`) → no violation
   - Submodule import (`use crate::contract::sub::TypeName`) → MEDIUM violation
   - Deep submodule import (`use crate::contract::a::b::c::TypeName`) → MEDIUM violation
   - Non-contract imports (`use crate::capabilities::foo::Bar`) → no violation
3. **Integration test assertions** — `test-project-rust/` fixture exists but no test script verifies that `scan` produces an AES007 finding for it. Wire the fixture into the test runner.
4. **Config toggle** — AES007 should be suppressible via config file (`lint_arwaky.config.rust.yaml`), per standard AES rule pattern.
5. **Multi-language support** — the import processor's `.` splitting works for Python but the coordinator only handles Rust. A language dispatch is needed.

### 8.4 What to Keep

1. **Coordinator's line number accuracy** — `i + 1` is correct and should be preserved when consolidating.
2. **Violation message clarity** — `"AES007 CONTRACT_BARREL: Must use barrel import (crate::contract::TypeName)."` clearly tells the developer what to do.
3. **Per-file execution** — running `check_contract_barrel()` for every file in the coordinator loop ensures blanket coverage even for files not processed by the import pipeline.
4. **Test fixture** — `deep_import_processor.rs` with the annotation `// AES007: import with 5+ segments` is a good starting point for test-driven development.

### 8.5 Empirical Evidence from Test Projects

| Project | File | Expected | Actual (current) | Notes |
|---------|------|----------|------------------|-------|
| `test-project-rust` | `capabilities/deep_import_processor.rs` | AES007 MEDIUM flagged | **Not flagged** (B1) | Coordinator prefix is `di_containers::contract_service_aggregate`, not `contract::sub::module`. |
| `self-lint` | `src-rust/cli-commands/surface_command_handler.rs` | No AES007 (barrel import) | ✅ No false positive | Barrel import `use crate::contract::...` correctly passes. |
| `self-lint` | Any file with `use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate` | AES007 MEDIUM flagged | ✅ Flagged | Coordinator's narrow prefix check does catch this specific pattern. |
| `test-project-python` | (any `.py` file) | AES007 MEDIUM flagged for non-barrel contract import | **Dead code** (B3) | Import processor splits on `.` but likely never receives Rust `::` paths; Python files may or may not flow through this path. |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| Rust module system | Only applies to Rust | Pending Review By design |
| Code duplication | Logic in 2 files | Maintenance burden | Consolidate to single path |

## 10. Appendices
- `src-rust/layer-rules/capabilities_compliance_analyzer.rs:414` — `check_contract_barrel()`
- `src-rust/pipeline-jobs/agent_checking_coordinator.rs:191` — Duplicated implementation
- `src-rust/di-containers/contract_service_aggregate.rs` — Contract barrel
