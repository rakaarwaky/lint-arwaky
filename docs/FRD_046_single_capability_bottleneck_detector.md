# Feature Requirements Document (FRD)
**Feature Name:** Single Capability Bottleneck Detector (AES031)  
**Product:** Lint Arwaky v1.10.2  
**Author:** Raka  
**Date:** 09/06/2026  
**Version:** v1.0

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 09/06/2026 | Raka | Initial document creation | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
This document defines the AES031 rule that detects when all dispatch routes in a project go to a single capability class, despite multiple specialized capability classes being available. The rule is implemented in `_check_single_capability_bottleneck()` within `DispatchRoutingChecker` in `cli-transport/capabilities_routing_processor.rs`, with a secondary heuristic implementation in `check_single_bottleneck()` in `agent_checking_coordinator.rs`. AES031 ensures that dispatch routing distributes actions across capabilities by concern rather than funneling everything through one class.

### 2.2 Scope
**In-Scope:**
- Analysis of dispatch routing references to determine per-class route distribution
- Detection when all routes (>= 3) go to a single class while other capability classes exist
- Heuristic detection: capability files with > 30 functions or > 5 impl blocks
- MEDIUM severity reporting

**Out-of-Scope:**
- Runtime performance analysis of dispatch distribution
- Automatic refactoring of dispatch configuration
- Per-action concern classification (requires domain knowledge)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES031** | Rule code for single capability bottleneck violation |
| **DispatchRoutingChecker** | Main checker in `capabilities_routing_processor.rs` |
| **check_single_bottleneck()** | Heuristic function-count checker in `agent_checking_coordinator.rs` |
| **Bottleneck** | A single class handling all or most dispatch routes |
| **Capability class** | A struct/trait implementing a specific concern or domain operation |

## 3. Feature Overview
### 3.1 Background & Problem
Without AES031, dispatch architectures can degrade into "god capability" anti-patterns where one class accumulates all action handlers. As new features are added, developers tend to add routes to the existing capability that already "does everything," rather than creating specialized capabilities. This leads to bloated classes (500+ lines, 40+ methods), poor separation of concerns, and difficult testing.

### 3.2 Business Goals
- Prevent god-class capability bottlenecks in dispatch routing
- Encourage creation of specialized capability classes per concern
- Provide clear, actionable messages suggesting alternative capability classes
- Detect bloat early via function count and impl block heuristics

### 3.3 Target Users
- **Developers**: Get warned when their dispatch funnel is too narrow
- **Architects**: Monitor dispatch distribution to ensure proper separation of concerns

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when all dispatch routes go to a single capability class despite other specialized options existing.
- **US-002:** As a developer, I want to be warned when my capability file has too many functions (> 30) indicating it should be split.
- **US-003:** As an architect, I want the violation message to list the other available capability classes so I can redistribute routes.

### 4.2 Use Cases & Workflow
**Primary Pipeline (routing-based):**
```
File: agent/dispatch_orchestrator.py (COMMAND_CATALOG)

1. Parse all capability references → group by class_name
2. Count total unique classes referenced
3. If ONLY ONE class is referenced AND it has >= 3 routes:
   a. Count other defined capability classes (from ClassDefinitionMap)
   b. If other classes exist → AES031 MEDIUM for each route
```

**Secondary Pipeline (heuristic — agent_checking_coordinator):**
```
File: capabilities/overloaded_handler.rs

1. Is layer == "capabilities"? → YES
2. Count "fn " occurrences in content
3. If > 30 → AES031 MEDIUM: "31 functions"
4. Count "impl " blocks in content
5. If > 5 → AES031 MEDIUM: "6 impl blocks"
```

### 4.3 Business Rules
- Severity: MEDIUM
- Routing-based check: only triggers when ALL routes go to one class
- Routing-based check: only triggers when the bottleneck class has >= 3 routes
- Routing-based check: other capability classes must exist in the project
- Heuristic check: capability files with > 30 functions flagged
- Heuristic check: capability files with > 5 impl blocks flagged

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per project | < 50ms |
| NFR-002 | False positive rate | < 5% (heuristic may flag intentionally large classes) |
| NFR-003 | False negative rate | 0% for single-class bottlenecks |

## 6. UI/UX Requirements
```
AES031 MEDIUM - src-rust/cli-transport/agent/dispatch_orchestrator.py
  AES031 SINGLE_CAPABILITY_BOTTLENECK: Action 'process_order' routes to 'OrderManager' but 4 other capability classes exist (PaymentHandler, InventoryManager, NotificationService, AuditLogger). Actions should be distributed to the correct capability.
  WHY? Dispatch should distribute actions across capabilities by concern. A single class handling everything indicates under-utilization of the architecture.
  FIX: Create or use specialized capability classes for distinct action types.

AES031 MEDIUM - src-rust/code-analysis/capabilities/overloaded_handler.rs
  AES031 SINGLE_CAPABILITY_BOTTLENECK: 31 functions.
  WHY? Too many functions in a single capability class suggests it should be split by concern.
  FIX: Split the capability class into multiple specialized classes.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | All 5 dispatch routes go to "OrderManager", 3 other capability classes exist | `_check_single_capability_bottleneck()` runs | AES031 MEDIUM flagged for each route | Pending Review |
| AC-002 | Dispatch routes distributed across 3 classes equally | `_check_single_capability_bottleneck()` runs | No AES031 | Pending Review |
| AC-003 | Only 2 routes to single class (< 3 threshold) | Bottleneck checker runs | Skipped (below threshold) | Pending Review |
| AC-004 | Capability file with 35 functions | `check_single_bottleneck()` runs | AES031 MEDIUM flagged | Pending Review |
| AC-005 | Capability file with 3 impl blocks (< 5 threshold) | `check_single_bottleneck()` runs | No AES031 | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation
- **Location (routing)**: `src-rust/cli-transport/capabilities_routing_processor.rs:268-352`
- **Location (heuristic)**: `src-rust/code-analysis/agent_checking_coordinator.rs:563-586`
- **Status**: **FULLY IMPLEMENTED** — both paths are active
- Routing check groups by class, fires when 1 class has all >= 3 routes
- Heuristic check counts `fn ` and `impl ` tokens in capability files

### 8.2 Bugs Found

1. **Two different implementations, different triggers** (`routing_processor.rs:268` vs `agent_checking_coordinator.rs:563`)
   - Routing checker: triggered by dispatch catalog analysis (Python-only)
   - Heuristic checker: triggered by `fn `/`impl ` counts (all languages)
   - Both emit AES031 with different messages
   - **Impact**: user may see AES031 from two sources with different semantics
   - **Fix**: either merge into one checker or rename one rule code

2. **Routing checker Python-only** (`routing_processor.rs:154`)
   - `if !path.ends_with(".py") { continue; }`
   - Rust dispatch routing (trait-based `Command` enum dispatch, `match` arms) never analyzed
   - **Impact**: Rust projects get zero AES031 from the routing checker
   - **Fix**: add Rust enum-dispatch analysis in `_check_capability_by_layer`

3. **Heuristic threshold is arbitrary** (`agent_checking_coordinator.rs:569,578`)
   - `> 30` functions and `> 5` impl blocks hardcoded
   - Not configurable via YAML
   - **Impact**: cannot tune per project or per layer
   - **Fix**: move thresholds to config YAML under an `aes031` section

4. **`_group_capabilities_by_class` only uses Python refs** (`routing_processor.rs:305-324`)
   - Groups are built from `CapabilityReferenceList` which only contains Python catalog entries
   - Does not include Rust `match` arm dispatch targets
   - **Impact**: underestimates total dispatch coverage for Rust projects

### 8.3 What Needs to Be Added
- **Rust dispatch analysis**: parse `match` arms that call capability methods
- **Configurable thresholds**: add `max_functions`/`max_impl_blocks` to YAML config
- **Merge heuristic into routing checker**: single source of AES031 truth
- **Integration test fixtures**: Python catalog with all routes to one class

### 8.4 What to Keep
- **Routing-based bottleneck detection** Pending Review (correct semantic for dispatch)
- **Other-class listing in message** Pending Review (line 346-349, actionable)
- **Heuristic function/impl count** Pending Review (catches bloat even without dispatch analysis)

### 8.5 Empirical Evidence from Test Projects
- `test-project-python/` — No COMMAND_CATALOG fixture exists for AES031
- `test-project-rust/` — No oversized capability fixture exists
- **No test fixture exercises AES031** — needs at least one Python dispatch fixture with single-class routing

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Class/method extraction for dispatch | Python-only | Extend to Rust match arms |
| Config YAML | Threshold configurability | Hardcoded 30 fn / 5 impl | Add to config YAML |
| Test fixtures | Python catalog with single-class routing | No fixture exists | Create fixture |
| capabilities_routing_processor | Shared with AES030/AES032 | Changes affect 3 rules | Add unit tests per rule |

## 10. Appendices
- `src-rust/cli-transport/capabilities_routing_processor.rs:268` — `_check_single_capability_bottleneck()`
- `src-rust/cli-transport/capabilities_routing_processor.rs:305` — `_group_capabilities_by_class()`
- `src-rust/cli-transport/capabilities_routing_processor.rs:327` — `_report_class_bottleneck()`
- `src-rust/code-analysis/agent_checking_coordinator.rs:563` — `check_single_bottleneck()`
- `lint_arwaky.config.rust.yaml:264` — AES031 config message
