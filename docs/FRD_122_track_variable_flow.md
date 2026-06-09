# 📄 Feature Requirements Document (FRD)
**Feature Name:** Track Variable Flow — Data Flow Analysis
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
Defines the variable data flow tracking feature that traces how a variable is assigned, transformed, and consumed within its scope, enabling developers to understand data provenance.

### 2.2 Scope
**In-Scope:** Variable assignment tracking, re-assignment tracking, function parameter flow, return value flow, scope-local tracking.
**Out-of-Scope:** Cross-function taint tracking, heap/alias analysis, concurrency data flow.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Data flow** | Path of data from assignment through transformations to consumption |
| **Assignment** | `let x = ...`, `x = ...`, `x.foo = ...` |

## 3. Feature Overview
### 3.1 Background & Problem
Developers had no way to track how data flows through variables within a function, making debugging and auditing difficult.

### 3.2 Business Goals
- Trace variable origin through re-assignments
- Show all read/write operations on a variable
- Support debugging and code review use cases

### 3.3 Target Users
- Developers debugging data flow issues
- Code reviewers auditing variable usage

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to track all assignments and reads of `config`, so I understand how configuration flows.

### 4.2 Use Cases & Workflow
```
Input:  track variable "config" in scope of function "load_config"
Output:
  Line 10: let config = read_yaml("config.yaml")     ← WRITE
  Line 15: config = config.merge(defaults)            ← READ + WRITE
  Line 20: let path = config.get("path")              ← READ
  Line 25: return config                              ← READ (return)
```

### 4.3 Business Rules
- Tracks within a single function scope only
- Detects: `=`, `+=`, `.method()`, function arguments, return values
- Multiple variables with same name in nested scopes handled independently

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Flow tracking per variable | < 10ms |
| NFR-002 | Accuracy for simple patterns | > 80% |

## 6. UI/UX Requirements
```
$ lint-arwaky-cli check --track-variable config --scope "load_config" file.rs
 Data flow for `config` in `load_config`:
  [WRITE] L10: let config = read_yaml("config.yaml")
  [READ]  L15: config.merge(defaults)
  [WRITE] L15: config = config.merge(defaults)
  [READ]  L20: config.get("path")
  [READ]  L25: return config
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Variable with assignments | Track variable | All writes and reads shown | Pending Review |
| AC-002 | Variable not found | Track unknown var | Empty result with scope variable list | Pending Review |
| AC-003 | Nested scope | Same var name in inner scope | Distinguishable entries | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Variable tracker | `semantic-analysis/capabilities_var_tracker.rs` | Pending Review |
| Flow reporter | `semantic-analysis/capabilities_flow_reporter.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Variable declaration and usage extraction | Regex limited for complex patterns | Line-based heuristic; document limitations |

## 10. Appendices
- `src-rust/semantic-analysis/capabilities_var_tracker.rs`
