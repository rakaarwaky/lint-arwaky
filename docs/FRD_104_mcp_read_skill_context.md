# 📄 Feature Requirements Document (FRD)
**Feature Name:** MCP Tool — `read_skill_context(section)`
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
Defines the `read_skill_context(section)` MCP tool that reads documentation sections from the project's skill/AGENTS.md files, enabling agents to access guidance dynamically.

### 2.2 Scope
**In-Scope:** Reading named sections from AGENTS.md, SKILLS.md, and docs/ files; section extraction by heading.
**Out-of-Scope:** Writing documentation, arbitrary file access outside docs/.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **section** | Markdown heading name (e.g., `Build & dev`, `Architecture`) |

## 3. Feature Overview
### 3.1 Background & Problem
Agents lacked runtime access to project documentation, causing them to operate without context of project conventions.

### 3.2 Business Goals
- Provide agents with on-demand access to project docs
- Support section-level granularity to reduce token usage

### 3.3 Target Users
- AI agents needing project-specific guidance

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As an AI agent, I want to read the `Build & dev` section to know how to compile the project.

### 4.2 Use Cases & Workflow
```
Request:  read_skill_context("Build & dev")
Response: { "section": "Build & dev", "content": "cargo build --release\ncargo test...", "source": "AGENTS.md" }
```

### 4.3 Business Rules
- Only reads from `AGENTS.md`, `SKILLS.md`, and `docs/*.md`
- Returns first matching heading; case-insensitive matching
- Non-existent sections return empty content with available sections list

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Section lookup | < 30ms |

## 6. UI/UX Requirements
No UI. Returns markdown content as string.

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Existing section | `read_skill_context("Build & dev")` | Returns section content | Pending Review |
| AC-002 | Non-existent section | `read_skill_context("Bogus")` | Returns empty with available sections | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Section reader | `mcp-server/capabilities_skill_reader.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-100 (MCP Server) | Requires running MCP server | File I/O may fail if docs missing | Graceful error with suggestions |
| AGENTS.md | Documentation source | Missing AGENTS.md returns empty | Fallback to static guide |

## 10. Appendices
- `src-rust/mcp-server/capabilities_skill_reader.rs`
- `AGENTS.md` — Source documentation
