# 📄 Feature Requirements Document (FRD)
**Feature Name:** MCP Tool — `commands_schema(tool_name)`
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
Defines the `commands_schema(tool_name)` MCP tool that returns JSON Schema for a specific tool's parameters, enabling agents to construct valid requests.

### 2.2 Scope
**In-Scope:** JSON Schema generation for each tool, parameter validation metadata, type information.
**Out-of-Scope:** OpenAPI/Swagger generation, HTML documentation.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **JSON Schema** | Draft 2020-12 schema describing tool parameters |
| **tool_name** | Name of the MCP tool to inspect |

## 3. Feature Overview
### 3.1 Background & Problem
Agents had to guess parameter formats. Schema introspection enables type-safe tool invocation.

### 3.2 Business Goals
- Enable agent-side parameter validation
- Support auto-complete and type checking
- Reduce invocation errors

### 3.3 Target Users
- AI agents requiring parameter type information

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As an AI agent, I want to get JSON Schema for `execute_command`, so I know which parameters are required.

### 4.2 Use Cases & Workflow
```
Request:  commands_schema("execute_command")
Response: {
  "name": "execute_command",
  "parameters": {
    "action": { "type": "string", "enum": ["check", "fix", "scan", "suggest", "diff"] },
    "args": { "type": "object", "additionalProperties": true }
  }
}
```

### 4.3 Business Rules
- Returns schema for one tool at a time
- Unknown `tool_name` returns error with available tool list
- Schema conforms to JSON Schema Draft 2020-12

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Schema generation | < 20ms |

## 6. UI/UX Requirements
No UI. Returns JSON Schema.

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Valid tool name | `commands_schema("execute_command")` | Returns valid JSON Schema | Pending Review |
| AC-002 | Invalid tool name | `commands_schema("bogus")` | Returns error with valid tools | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Schema generator | `mcp-server/capabilities_schema_provider.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-100 (MCP Server) | Requires running MCP server | Server must be running | Document startup requirement |

## 10. Appendices
- `src-rust/mcp-server/capabilities_schema_provider.rs`
