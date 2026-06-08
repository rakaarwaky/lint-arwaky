# Feature Requirements Document (FRD)
**Feature Name:** Orphan Code Detector (AES017)
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
This document defines the AES017 rule that detects **orphan code** — files that have no imports and are not entry points. Such files are unreachable from the rest of the system and likely represent dead code. The rule is implemented via `OrphanGraphResolver` in the coordinator pipeline.

### 2.2 Scope
**In-Scope:**
- Files with zero imports that are not designated entry points
- Barrel files (mod.rs, __init__.py, index.ts) excluded
- HIGH severity reporting

**Out-of-Scope:**
- Auto-fixing violations
- Orphan detection within packages/sub-modules
- Dynamically loaded modules

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES017** | Rule code for orphan code violation |
| **OrphanGraphResolver** | Project-wide analyzer for import graph and orphan detection |
| **Entry point** | File identified as having no inbound dependencies (e.g., main.rs, cli entry) |

## 3. Feature Overview
### 3.1 Background & Problem
Files that are not imported by any other file and are not entry points are orphaned — they exist in the codebase but are unreachable. They accumulate dead code, confuse developers, and waste maintenance effort.

### 3.2 Business Goals
- Detect and eliminate orphaned files
- Ensure every source file is reachable through imports or as an entry point
- Maintain a clean, navigable codebase

### 3.3 Target Users
- **Developers**: Get warned when creating files that aren't wired into the import graph
- **Architects**: Identify dead code during architectural reviews

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to be warned when a file I create has no imports and isn't used anywhere.
- **US-002:** As an architect, I want to see all orphaned files in a project-wide report.

### 4.2 Detection Pipeline
```
1. Build import graph from all project files
2. Identify entry points (files with no inbound edges)
3. For each file:
   a. Is it in the import graph with zero imports?
   b. Is it an entry point? → skip
   c. Is it a barrel file (mod.rs, __init__.py)? → skip
   d. If yes to a and no to b/c → AES017 HIGH
```

### 4.3 Business Rules
- Severity: HIGH
- Barrel files (mod.rs, __init__.py, index.ts, index.js) are excluded
- Entry points are identified via `OrphanGraphResolver::identify_entry_points()`
- Project-wide analysis (not per-file)

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per project | < 100ms for 1000 files |
| NFR-002 | False positive rate | 0% for properly wired files |
| NFR-003 | False negative rate | 0% for actual orphans |

## 6. UI/UX Requirements
```
AES017 HIGH - src-rust/orphan-detector/capabilities_orphan_checker.rs
  AES017 ORPHAN_CODE: File has no imports, not an entry point.
  WHY? Every source file must be reachable through imports.
  FIX: Import this file from another module or delete if unused.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | File with 0 imports, not entry point | Orphan scan runs | AES017 HIGH flagged | Pending Review |
| AC-002 | File with 0 imports, is entry point | Orphan scan runs | No AES017 | Pending Review |
| AC-003 | Barrel file (mod.rs) with 0 imports | Orphan scan runs | No AES017 | Pending Review |
| AC-004 | Barrel file (index.js) with 0 imports | Orphan scan runs | No AES017 | Pending Review index.js not excluded |
| AC-005 | File with >0 imports | Orphan scan runs | No AES017 | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation
- **Location**: `src-rust/pipeline-jobs/agent_checking_coordinator.rs:168-186`
- **Status**: **NEARLY COMPLETE** — one exclusion missing
- Uses `OrphanGraphResolver` for graph construction and entry point identification

### 8.2 Bugs/Gaps Found

1. **`index.js` not excluded** — Exclusion list checks `mod.rs`, `__init__.py`, `/index.ts` but misses `/index.js`

### 8.3 What Needs to Be Added
- Add `/index.js` to the exclusion list alongside `/index.ts`

### 8.4 What to Keep
- Import graph construction Pending Review
- Entry point identification Pending Review
- Barrel file exclusion for Rust/Python/TS Pending Review
- Orphan detection logic Pending Review

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| OrphanGraphResolver | Import graph building and entry point analysis | Graph may be incomplete for dynamic imports | Static analysis sufficient for Rust/Python |

## 10. Appendices
- `src-rust/pipeline-jobs/agent_checking_coordinator.rs:168` — Orphan detection loop
- `src-rust/orphan-detector/capabilities_orphan_analyzer.rs` — `OrphanGraphResolver`
