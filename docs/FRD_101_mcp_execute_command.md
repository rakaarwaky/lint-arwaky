# 📄 Feature Requirements Document (FRD)
**Feature Name:** MCP Tool — `execute_command(action, args)`
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
Defines the `execute_command(action, args)` MCP tool that executes lint actions (check, fix, scan) via JSON-RPC, returning structured results to the AI agent.

### 2.2 Scope
**In-Scope:** `execute_command` method, argument validation, result serialization, error handling.
**Out-of-Scope:** Direct filesystem access, arbitrary shell execution.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **action** | Lint action enum: `check`, `fix`, `scan`, `suggest` |
| **args** | JSON object mapping argument names to values |

## 3. Feature Overview
### 3.1 Background & Problem
Agents needed a single entry point to trigger all lint operations without knowing CLI flag syntax.

### 3.2 Business Goals
- Provide a unified `execute_command` interface for all lint actions
- Return structured JSON results for agent consumption
- Validate arguments server-side before execution

### 3.3 Target Users
- AI agents calling lint operations programmatically

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As an AI agent, I want to call `execute_command("check", {"path": "."})` to lint a project.
- **US-002:** As an AI agent, I want to call `execute_command("fix", {"path": ".", "dry_run": true})` to preview fixes.

### 4.2 Use Cases & Workflow
```
Request:  execute_command("check", {"path": "test-project-rust/"})
Response: { "status": "completed", "violations": 12, "aes_codes": ["AES001", "AES003"] }
```

### 4.3 Business Rules
- `action` must be one of: `check`, `fix`, `scan`, `suggest`, `diff`
- Unknown actions return error with valid actions list
- `args` validated against JSON Schema for the action

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Execution latency | Same as CLI equivalent |
| NFR-002 | Argument validation | < 10ms |

## 6. UI/UX Requirements
No UI. Returns structured JSON response.

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Valid action + args | `execute_command("check", {"path": "."})` | Returns result with violation count | Pending Review |
| AC-002 | Invalid action | `execute_command("unknown", {})` | Returns error with valid actions | Pending Review |
| AC-003 | Missing required args | `execute_command("check", {})` | Returns argument validation error | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Execute command handler | `mcp-server/agent_command_executor.rs` | Pending Review |
| Action enum | `mcp-server/taxonomy_action_vo.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-100 (MCP Server) | Requires running MCP server | Server must be started first | Document startup order |
| FR-055 (Lint Pipeline) | Delegates to pipeline | Pipeline errors propagate | Wrap in Result type |

## 10. Appendices
- `src-rust/mcp-server/agent_command_executor.rs`
