# Feature Requirements Document (FRD)
**Feature Name:** MCP Schema Violation Detector (AES025)
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
This document defines the AES025 rule that detects **MCP schema violations** in Python files using the MCP framework. MCP tools must declare valid JSON Schema for their input parameters and provide descriptive docstrings. The rule has two implementations: (1) a full dedicated checker in `McpSchemaChecker`, and (2) a simplistic heuristic checker in `check_mcp_schema()`.

### 2.2 Scope
**In-Scope:**
- Missing docstring on MCP tool functions (CRITICAL — docstring <10 chars after stripping)
- Missing type annotation on MCP tool parameters (CRITICAL — except `self`, `ctx`)
- Invalid JSON Schema in inline `parameters=`/`input_schema=` dicts (CRITICAL — missing `type` or `properties`)
- Python files using `@mcp.tool`, `server.add_tool`, or `register_tool`

**Out-of-Scope:**
- Auto-fixing violations
- Non-MCP Python files
- Rust MCP tool implementations
- Schema validation for Pydantic BaseModel-derived tools (handled implicitly by Pydantic)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **AES025** | Rule code for MCP schema violation |
| **McpSchemaChecker** | Full dedicated checker in `mcp-server/capabilities_schema_checker.rs` |
| **check_mcp_schema()** | Active heuristic checker in `coordinator` |
| **MCP** | Model Context Protocol — tool interface for LLM clients |
| **JSON Schema** | Schema format for validating MCP tool input parameters |

## 3. Feature Overview
### 3.1 Background & Problem
MCP tools must declare valid JSON Schema so LLM clients can validate input before tool calls. Docstrings become tool descriptions in `tools/list` responses. Without enforcement, tools may have missing descriptions (hidden capability), untyped parameters (no schema mapping), or invalid JSON Schema (runtime errors on tool calls).

### 3.2 Business Goals
- Ensure every MCP tool has a meaningful docstring
- Ensure every tool parameter has a type annotation
- Ensure all inline JSON Schema dicts have valid `type` and `properties` keys
- Maintain 100% schema compliance for all MCP tool endpoints

### 3.3 Target Users
- **LLM Developers**: Get clear tool descriptions and parameter types from tools/list
- **MCP Tool Authors**: Get immediate feedback on missing schema declarations
- **Architects**: Enforce schema correctness across all MCP tooling

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As an MCP tool author, I want to be warned when my tool function has no docstring.
- **US-002:** As an MCP tool author, I want to be warned when a parameter lacks a type annotation.
- **US-003:** As an MCP tool author, I want to be warned when my inline schema is missing `type` or `properties`.
- **US-004:** As an architect, I want all three sub-checks reported with CRITICAL severity.

### 4.2 Detection Pipeline
**Full Pipeline (McpSchemaChecker):**
```
File: src-rust/mcp-server/capabilities_schema_checker.rs

1. Scan file for MCP tool declarations:
   a. @<obj>.tool('name') decorator
   b. server.add_tool(func, ...)
   c. register_tool(func, ...)

2. For each tool function found:
   a. Extract function definition (async) def <name>(<params>):
   b. CHECK: Does function have a docstring ("""...""") ≥10 non-whitespace chars?
      → If not → AES025 CRITICAL (missing docstring)
   c. For each parameter:
      - CHECK: Does param have `: type` annotation?
      → If no → AES025 CRITICAL (missing type annotation)
   d. If tool has inline `parameters=` or `input_schema=` dict:
      - CHECK: Does dict contain "type" key?
      - CHECK: Does dict contain "properties" key?
      - CHECK: Is "type" value a valid JSON Schema type (string, number, etc.)?
      → If any fails → AES025 CRITICAL (invalid JSON Schema)

3. Return all violations collected
```

**Active Pipeline (coordinator):**
```
File: src-rust/code-analysis/agent_checking_coordinator.rs

1. Does filename contain "mcp_" or "_schema"? → NO → skip
2. Does content contain "fn " + ("tool" or "Tool" or "schema")? → NO → skip
3. Content > 50 chars? → flag AES025 MEDIUM
```

### 4.3 Business Rules
| Rule | Severity | Condition |
|------|----------|-----------|
| Missing docstring | CRITICAL | Tool function has no docstring or docstring <10 chars after stripping quotes |
| Missing type annotation | CRITICAL | Tool parameter lacks `: type` annotation (except `self`, `ctx`) |
| Invalid JSON Schema | CRITICAL | Inline `parameters=`/`input_schema=` dict missing `type` or `properties`, or has invalid `type` value |
| Active checker | MEDIUM | Heuristic: filename contains "mcp_" + has "fn " + has "tool"/"schema" but no actual function found |

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Detection per file (full) | < 100ms |
| NFR-002 | Detection per file (active) | < 5ms |
| NFR-003 | False positive rate | 0% for well-formed MCP tools |
| NFR-004 | Python-only enforcement | No Rust/JS false positives |

## 6. UI/UX Requirements
```
AES025 CRITICAL - src-python/mcp_server/tools/agent_chat_tool.py:15
  AES025 MCP_TOOL_SCHEMA_VIOLATION: MCP tool 'chat' is missing a descriptive docstring.
  WHY? Docstrings become tool descriptions in tools/list responses for LLM clients.
  FIX: Add a docstring ("""...""") of at least 10 characters describing the tool.

AES025 CRITICAL - src-python/mcp_server/tools/agent_chat_tool.py:18
  AES025 MCP_TOOL_SCHEMA_VIOLATION: MCP tool 'chat' parameter 'message' lacks a type annotation.
  WHY? Untyped parameters cannot map to JSON Schema entries.
  FIX: Add a type annotation (e.g., message: str).

AES025 CRITICAL - src-python/mcp_server/tools/agent_search_tool.py:22
  AES025 MCP_TOOL_SCHEMA_VIOLATION: MCP tool 'search' has an invalid JSON Schema:
    Missing required key 'properties' in inline schema dict.
  WHY? JSON Schema must declare both 'type' and 'properties' for dict-based schemas.
  FIX: Add 'properties' key with valid field definitions.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | MCP tool without docstring | `McpSchemaChecker` runs | AES025 CRITICAL flagged | Pending Review |
| AC-002 | MCP tool with untyped param | Checker runs | AES025 CRITICAL flagged | Pending Review |
| AC-003 | MCP tool with missing `properties` in schema | Checker runs | AES025 CRITICAL flagged | Pending Review |
| AC-004 | MCP tool with invalid schema `type` | Checker runs | AES025 CRITICAL flagged | Pending Review |
| AC-005 | Well-formed MCP tool with docstring + typed params + valid schema | Checker runs | No AES025 | Pending Review |
| AC-006 | Non-MCP Python file | Checker runs | Skipped | Pending Review |
| AC-007 | MCP file with no tool function | Active `check_mcp_schema()` runs | AES025 MEDIUM flagged | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Two Implementations — Full vs Heuristic

#### 8.1.1 McpSchemaChecker (FULL) — `src-rust/mcp-server/capabilities_schema_checker.rs`
- **Location**: `src-rust/mcp-server/capabilities_schema_checker.rs:107-475`
- **Status**: **FULLY IMPLEMENTED** — 368 lines, three sub-checks
- Implements `check_mcp_tool_schema()` with:
  - Docstring validation (lines 249-260)
  - Type annotation validation (lines 295-305)
  - JSON Schema validation (lines 468-475)
- Uses regex-based Python file parsing (`FUNC_DEF_RE`, `TOOL_DECORATOR_PATTERNS`)
- **Called from**: Need to verify integration

#### 8.1.2 Active Heuristic Checker (SIMPLE) — `agent_checking_coordinator.rs:620-635`
```rust
fn check_mcp_schema(file: &str, content: &str, violations: &mut Vec<LintResult>) {
    if !file.contains("mcp_") && !file.contains("_schema") { return; }
    let has = content.contains("fn ")
        && (content.contains("tool") || content.contains("Tool") || content.contains("schema"));
    if !has && content.len() > 50 {
        violations.push(Self::mk(file, 0, "AES025", Severity::MEDIUM, "..."));
    }
}
```
**Status**: **PARTIALLY IMPLEMENTED** — heuristic only flags files missing expected keywords. Does NOT perform actual schema inspection. Called in `run_all_checks()` line 80.

### 8.2 Bugs/Gaps Found

1. **Active checker is too simplistic** — only checks if filename contains "mcp_" and content has "fn " + "tool"/"schema". No actual docstring, type annotation, or schema validation.
2. **McpSchemaChecker integration unclear** — It's in mcp-server/ module but there's no evidence of it being called from the coordinator pipeline.
3. **Heuristic false positives** — Any file with "mcp_" prefix and file content > 50 chars triggers AES025 MEDIUM even if valid.

### 8.3 What Needs to Be Added
- **Wire McpSchemaChecker into coordinator** — replace or supplement the heuristic with the full checker
- **Test fixtures** for Python MCP tools without docstrings, without type annotations, and with invalid schemas
- **Rust MCP detection** — AES025 is documented as Python-only but Rust projects may have MCP servers too

### 8.4 What to Keep
- **McpSchemaChecker full implementation** Pending Review (correctly does 3 sub-checks)
- **Heuristic filename-based guard** Pending Review (fast pre-filter before full check)

### 8.5 Empirical Evidence from Test Projects
- `test-project-rust/src-rust/infrastructure/mcp_empty_infra.rs` — no function handler → flagged by heuristic Pending Review
- `test-project-rust/src-rust/capabilities/mcp_tool_processor.rs` — tool without schema → NOT flagged by heuristic Pending Review

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (AST scanning) | Python file parsing for MCP tools | Regex-based parsing may miss edge cases | Improve with proper Python AST |
| McpSchemaChecker | Full schema checker exists but may be unwired | Dead code | Wire into coordinator pipeline |
| JSON Schema validation | Inline dict validation | Complex schema structures may produce false positives | Add comprehensive test fixtures |

## 10. Appendices
- `src-rust/mcp-server/capabilities_schema_checker.rs:107` — `McpSchemaChecker::check_mcp_tool_schema()`
- `src-rust/code-analysis/agent_checking_coordinator.rs:620` — Active heuristic `check_mcp_schema()`
- `test-project-rust/src-rust/infrastructure/mcp_empty_infra.rs` — Empty MCP fixture
- `test-project-rust/src-rust/capabilities/mcp_tool_processor.rs` — Tool without schema fixture
