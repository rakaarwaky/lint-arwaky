# 📄 Feature Requirements Document (FRD)
**Feature Name:** MCP Tool — `list_commands(domain)`
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
Defines the `list_commands(domain)` MCP tool that returns available commands optionally filtered by domain (e.g., `check`, `fix`, `config`).

### 2.2 Scope
**In-Scope:** Command enumeration by domain, domain validation, response formatting.
**Out-of-Scope:** Detailed schema — use `commands_schema` for that.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **domain** | Command group: `check`, `fix`, `config`, `scan`, `watch`, `hook` |

## 3. Feature Overview
### 3.1 Background & Problem
Agents needed a way to discover available commands without hardcoding tool names.

### 3.2 Business Goals
- Enable dynamic command discovery
- Support domain-based filtering for relevant results

### 3.3 Target Users
- AI agents exploring available capabilities

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As an AI agent, I want to list all commands, so I know what the tool can do.
- **US-002:** As an AI agent, I want to filter by domain `check`, so I only see relevant commands.

### 4.2 Use Cases & Workflow
```
Request:  list_commands("check")
Response: { "domain": "check", "commands": ["check", "check --fix", "check --format json"] }
```

### 4.3 Business Rules
- If `domain` is empty/null, all commands across all domains are returned
- Domains are predefined: `check`, `fix`, `config`, `scan`, `watch`, `hook`, `diff`, `suggest`

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Response time | < 50ms (no I/O) |

## 6. UI/UX Requirements
No UI. Returns structured JSON array.

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | No domain filter | `list_commands(null)` | Returns all commands | Pending Review |
| AC-002 | Valid domain | `list_commands("fix")` | Returns only fix commands | Pending Review |
| AC-003 | Invalid domain | `list_commands("bogus")` | Returns empty list | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Command registry | `mcp-server/capabilities_command_lister.rs` | Pending Review |
| Domain enum | `mcp-server/taxonomy_domain_vo.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-100 (MCP Server) | Requires running MCP server | Server must be running | Document startup requirement |

## 10. Appendices
- `src-rust/mcp-server/capabilities_command_lister.rs`
