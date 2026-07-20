# Feature Requirement Document (FRD) - MCP Server

See [README.md](../../../README.md) for MCP setup and [TEST.md](../../../TEST.md) for smoke-test criteria.

## 1. Feature Goal

The primary purpose of the `mcp-server` module is to implement a Model Context Protocol (MCP) server that exposes the linting pipeline as tools and resources accessible by AI agents. This server enables lint_arwaky integration with IDEs and other AI tools through a standard protocol.

## 2. Requirements & Scope

The `mcp-server` module is responsible for the MCP server based on the following specifications:

- **lint_scan**: Scans the workspace and returns violation results.
- **lint_fix**: Applies automatic fixes to specified files.
- **lint_check**: Checks a single file against AES rules.
- **lint_config**: Manages lint_arwaky configuration.
- **lint_setup**: Sets up a new project with AES structure.

## 3. Success Indicators

The success of the `mcp-server` module is measured by:

- **Protocol Compliance**: Implementation conforms to MCP JSON-RPC standards.
- **Tool Discovery**: All tools can be discovered by AI clients.
- **Response Time**: Response time is under 5 seconds for standard operations.
- **Rule Conformance**: When complete, the module's own source complies with AES rules (no `agent_*`, `capabilities_*`, or `infrastructure_*` imports; taxonomy/contract only).
