# 📄 Feature Requirements Document (FRD)
**Feature Name:** MCP Server — JSON-RPC 2.0 via `mcp-sdk-rs` 0.3.4
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
This document defines the MCP (Model Context Protocol) server that enables AI agents to interact with Lint Arwaky over JSON-RPC 2.0 via stdin/stdout using `mcp-sdk-rs` 0.3.4.

### 2.2 Scope
**In-Scope:** MCP server binary (`lint-arwaky-mcp`), JSON-RPC 2.0 transport over stdin/stdout, `mcp-sdk-rs` 0.3.4 integration, tool registration, lifecycle management.
**Out-of-Scope:** REST API, WebSocket transport, gRPC support.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **MCP** | Model Context Protocol — protocol for AI agent ↔ tool communication |
| **JSON-RPC 2.0** | Lightweight remote procedure call protocol |
| **mcp-sdk-rs** | Rust SDK implementing the MCP specification |

## 3. Feature Overview
### 3.1 Background & Problem
AI agents had no standardized way to invoke linting operations programmatically. Each integration required custom wiring.

### 3.2 Business Goals
- Provide a standardized MCP endpoint for AI agent integration
- Support all lint operations via JSON-RPC 2.0
- Enable tool discovery and schema introspection

### 3.3 Target Users
- **AI Agents (Claude, Cursor, etc.):** Invoke lint, fix, and analysis tools
- **Developers:** Test MCP tools locally via stdin/stdout

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As an AI agent, I want to list available tools, so I know what operations are supported.
- **US-002:** As an AI agent, I want to invoke `execute_command` with arguments, so I can run lint operations.
- **US-003:** As an AI agent, I want to check server health, so I can confirm connectivity.

### 4.2 Use Cases & Workflow
```
AI Agent ──JSON-RPC 2.0──► lint-arwaky-mcp (stdin/stdout)
                               │
                               ├─► execute_command(action, args)
                               ├─► list_commands(domain)
                               ├─► commands_schema(tool_name)
                               ├─► read_skill_context(section)
                               └─► health_check()
```

### 4.3 Business Rules
- Server reads from stdin, writes to stdout (no file I/O for transport)
- Stderr reserved for logging only
- Each request must include `jsonrpc: "2.0"`, `id`, `method`, `params`

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Request latency (local) | < 100ms |
| NFR-002 | Max concurrent requests | 1 (serial stdin/stdout) |
| NFR-003 | Startup time | < 200ms |

## 6. UI/UX Requirements
No UI. Communication is JSON-RPC over stdio.

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Server starts | `cargo run --bin lint-arwaky-mcp` | Server listens on stdin/stdout | Pending Review |
| AC-002 | Health check request | Client sends `health_check` | Returns `{"status": "ok"}` | Pending Review |
| AC-003 | Invalid JSON sent | Client sends malformed payload | Returns JSON-RPC error response | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| MCP server binary | `mcp-server/surface_mcp_entry.rs` | Pending Review |
| Tool registry | `mcp-server/agent_tool_registry.rs` | Pending Review |

### 8.2 Risks
- `mcp-sdk-rs` 0.3.4 may have breaking changes in future releases
- Stdin/stdout transport limited to single client

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Lint Pipeline) | MCP server invokes lint pipeline | Pipeline changes affect MCP | Versioned tool schemas |
| mcp-sdk-rs 0.3.4 | Rust MCP SDK | SDK instability | Pin exact version in Cargo.toml |
| stdin/stdout | Transport mechanism | Only one client at a time | Document limitation |

## 10. Appendices
- `src-rust/mcp-server/` — Feature folder
- `docs/FRD_055_lint_pipeline.md` — Lint pipeline dependency
