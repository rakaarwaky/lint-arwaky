# 📄 Feature Requirements Document (FRD)
**Feature Name:** Multi-Project Aggregate Lint (`multi-project` subcommand)
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
This document defines the multi-project aggregate lint feature that runs lint rules across multiple projects in a workspace and aggregates results into a single report. It supports monorepo structures with multiple language projects side-by-side.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli multi-project [workspace-path]` — lint all projects in workspace
- Automatic discovery of projects in a workspace directory
- Separate lint run per project with per-project config
- Aggregated summary with global and per-project scores
- Cross-project violation deduplication

**Out-of-Scope:**
- Distributed/parallel execution across machines
- Dependency graph analysis between projects
- Centralized config inheritance (each project uses its own config)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Workspace** | Directory containing multiple independent projects (subdirectories) |
| **Project** | Directory with its own config file (`lint_arwaky.config.*.yaml`) |
| **Aggregate score** | Combined score across all projects (weighted by project size) |
| **Cross-project dedup** | Suppressing identical violations that span project boundaries |

## 3. Feature Overview
### 3.1 Background & Problem
Monorepo workspaces (e.g., Cargo workspace, npm workspaces) contain multiple independent projects. Running lint on each project individually is tedious. There is no unified view of code quality across the entire workspace.

### 3.2 Business Goals
- Provide a single command to lint all projects in a workspace
- Show per-project and aggregate quality scores
- Enable workspace-wide quality gates in CI
- Reduce friction for monorepo teams

### 3.3 Target Users
- **Monorepo Teams**: Run lint across entire workspace
- **CI/CD Pipelines**: Aggregate quality gate across all projects
- **Engineering Managers**: See workspace-wide quality at a glance

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a monorepo developer, I want to run `lint-arwaky-cli multi-project .` to lint all projects in the workspace, so I can fix issues before pushing.
- **US-002:** As a CI pipeline, I want an aggregate quality score across all projects to enforce workspace-wide standards.
- **US-003:** As a project lead, I want to see per-project breakdown to identify which projects need attention.

### 4.2 Use Cases & Workflow
**Multi-Project Lint Pipeline:**
```
lint-arwaky-cli multi-project .
  │
  ├─► 1. Discover projects in workspace
  │     ├── Scan subdirectories for config files
  │     ├── Detect Cargo.toml (Rust), pyproject.toml (Python), package.json (JS)
  │     └── Build project list with detected languages
  │
  ├─► 2. Run lint on each project (serial or parallel)
  │     ├── project-a/  →  3 violations, score 92
  │     ├── project-b/  →  12 violations, score 78
  │     └── project-c/  →  0 violations, score 100
  │
  ├─► 3. Aggregate results
  │     ├── Total violations: 15
  │     ├── Weighted score: (92*1000 + 78*500 + 100*200) / 1700 = 88.8
  │     └── Weight: lines of code per project
  │
  └─► 4. Report
        "Workspace score: 88.8 / 100"
        "3 projects, 15 violations"
```

**Example Output:**
```
$ lint-arwaky-cli multi-project .
🏗️  Multi-Project Lint — /home/user/workspace
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Projects found: 3
  ─────────────────────────────────────────────────
  Project                  Violations    Score    LOC
  ─────────────────────────────────────────────────
  core-lib/                3             92.0     1000
  web-app/                 12            78.0     500
  cli-tool/                0             100.0    200
  ─────────────────────────────────────────────────
  Aggregate Score:         88.8 / 100
  Total Violations:        15 (2 CRITICAL, 5 HIGH, 8 LOW)
  ─────────────────────────────────────────────────
  ⚠️  web-app/ has the most violations — consider a focused lint session
```

### 4.3 Business Rules
- Projects identified by presence of config file or manifest
- Weighted score: each project's score weighted by its LOC
- Critical violations in any project → non-zero exit code
- Per-project config files honored independently
- Workspace config (optional) can set aggregate thresholds

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Project discovery for 100 subdirectories | < 200ms |
| NFR-002 | Aggregate computation (10 projects) | < 50ms |
| NFR-003 | Parallel lint execution (10 projects) | < 30s |
| NFR-004 | Report generation | < 100ms |

## 6. UI/UX Requirements
```
$ lint-arwaky-cli multi-project . --parallel
🏗️  Multi-Project Lint (parallel mode) — /home/user/workspace
  Linting 3 projects...
  [████████████████████████████████████] 3/3 (100%)

  ┌──────────────┬──────────┬───────┬───────┐
  │ Project      │ Viol.    │ Score │ LOC   │
  ├──────────────┼──────────┼───────┼───────┤
  │ core-lib/    │ 3        │ 92.0  │ 1000  │
  │ web-app/     │ 12       │ 78.0  │ 500   │
  │ cli-tool/    │ 0        │ 100.0 │ 200   │
  └──────────────┴──────────┴───────┴───────┘
  Aggregate: 88.8/100  15 violations
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Workspace with 3 Rust projects | `multi-project .` runs | All 3 projects linted, aggregate score displayed | Pending Review |
| AC-002 | Mixed-language workspace (Rust + Python) | `multi-project .` runs | Each project linted with correct language adapter | Pending Review |
| AC-003 | One project has CRITICAL violation | `multi-project .` runs | Exit code 1, CRITICAL highlighted in summary | Pending Review |
| AC-004 | No projects found in directory | `multi-project .` runs | "No projects found" message | Pending Review |
| AC-005 | `--parallel` flag used | `multi-project . --parallel` runs | Projects linted concurrently | Pending Review |

## 8. Empirical Findings (Code Audit)
### 8.1 Current Implementation
| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| Multi-project command | `multi-project/` | — | **NOT IMPLEMENTED** |
| Project discovery | `multi-project/` | — | **NOT IMPLEMENTED** |
| Aggregate calculator | `multi-project/` | — | **NOT IMPLEMENTED** |

### 8.2 What Needs to Be Added
- `handle_multi_project()` handler
- Project discovery by manifest scanning
- Parallel project execution (thread pool)
- Weighted aggregate score computation
- Per-project config loading
- Workspace-level report formatting

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Lint Core) | Core lint engine reused per project | Engine must support programmatic invocation | Ensure `lint_project()` public API |
| FR-072 (Language Adapters) | Language detection per project | Unknown language → skip | Warn and continue with remaining projects |
| File system | Large workspace with 100+ projects | Discovery overhead | Optimize with parallel discovery |

## 10. Appendices
- `multi-project/surface_multi_project_command.rs` — CLI handler
- `multi-project/agent_multi_project_orchestrator.rs` — Orchestrator
- `multi-project/contract_governance_protocol.rs` — Governance protocol
- `multi-project/infrastructure_project_discovery.rs` — Project scanner
