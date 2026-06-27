# Feature Requirement Document (FRD) - MCP Server

## 1. Feature Goal

The primary purpose of the `mcp-server` module is to implement a Model Context Protocol (MCP) server that exposes the linting pipeline as tools and resources accessible by AI agents. This server enables lint_arwaky integration with IDEs and other AI tools through a standard protocol.

## 2. Requirements & Scope

The `mcp-server` module is responsible for the MCP server based on the following specifications:

### Component Specifications

- **McpServerOrchestrator**: Coordinates MCP tool execution and request routing.
- **McpServerAggregate**: Aggregate root for all MCP capabilities.
- **McpCommandSurface**: Surface that handles MCP command requests.

### Tools

- **lint_scan**: Scans the workspace and returns violation results.
- **lint_fix**: Applies automatic fixes to specified files.
- **lint_check**: Checks a single file against AES rules.
- **lint_config**: Manages lint_arwaky configuration.
- **lint_setup**: Sets up a new project with AES structure.

### Inputs

- MCP JSON-RPC requests from clients.
- Tool parameters matching the schema.

### Outputs

- MCP response with linting results or operation status.

---

## 3. Success Indicators

The success of the `mcp-server` module is measured by:

- **Protocol Compliance**: Implementation conforms to MCP JSON-RPC standards.
- **Tool Discovery**: All tools can be discovered by AI clients.
- **Response Time**: Response time is under 5 seconds for standard operations.
- **Self-Audit Conformity**: The module itself passes AES rule checks.
