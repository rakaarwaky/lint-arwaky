# 📄 Feature Requirements Document (FRD)
**Feature Name:** Trace Call Chains — Call Graph Analysis
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
Defines the call chain tracing feature that traces all callers and callees for a given function across the project, producing a call graph for impact analysis.

### 2.2 Scope
**In-Scope:** Forward trace (who does this function call?), backward trace (who calls this function?), call depth limiting, output as tree or flat list.
**Out-of-Scope:** Dynamic dispatch resolution, async call chain tracing, recursive call cycle detection.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Call chain** | Sequence of function calls from entry point to target |
| **Forward trace** | Functions called by the target |
| **Backward trace** | Functions that call the target |

## 3. Feature Overview
### 3.1 Background & Problem
Developers had no tooling to understand call relationships across the project when refactoring or debugging.

### 3.2 Business Goals
- Support impact analysis for refactoring
- Visualize call relationships across modules
- Limit trace depth to prevent explosion

### 3.3 Target Users
- Developers performing refactoring
- Code reviewers understanding code flow

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to trace who calls `check_imports()`, so I understand its impact before refactoring.

### 4.2 Use Cases & Workflow
```
Input:  trace callers of "check_imports" in project/
Output:
  lint_processor::run() → compliance_orchestrator::check_all()
    → import_checker::check_imports()
```

### 4.3 Business Rules
- Default max depth: 5 levels
- Case-sensitive symbol matching
- Only intra-project calls traced; external library calls excluded

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Trace generation (1000 files) | < 5s |
| NFR-002 | Max depth enforced | 5 (configurable) |

## 6. UI/UX Requirements
```
 Call chain for: check_imports (capabilities_import_checker.rs:42)
 Backward trace (callers):
   ├─ compliance_orchestrator::check_all() (agent_compliance_orchestrator.rs:120)
   │  └─ lint_processor::run() (agent_lint_orchestrator.rs:55)
   │     └─ surface_check_command::execute() (surface_check_command.rs:10)
 Forward trace (callees):
   └─ source_parser::extract_imports() (infrastructure_rust_scanner.rs:200)
      └─ regex::Regex::find() (std)
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Target function exists | Trace `check_imports` | Callers and callees listed | Pending Review |
| AC-002 | Target not found | Trace `nonexistent_fn` | Empty result with suggestion | Pending Review |
| AC-003 | Depth limit | Trace with max_depth=2 | Trace truncated at depth 2 | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Call graph builder | `semantic-analysis/capabilities_call_graph.rs` | Pending Review |
| Trace CLI command | `semantic-analysis/surface_trace_command.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Function call extraction | Regex may miss method calls | Support basic `fn()`, `obj.method()` patterns |

## 10. Appendices
- `src-rust/semantic-analysis/capabilities_call_graph.rs`
