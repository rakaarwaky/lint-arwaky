# FRD — mcp-server

## Feature Goal

The mcp-server crate implements a Model Context Protocol (MCP) server that exposes the linting pipeline as tools and resources accessible by AI agents. It enables lint_arwaky integration with IDEs and other AI tools through a standard protocol.

## Requirements & Scope

- execute_command — execute any CLI command via the MCP interface.
- list_commands — list available CLI commands with descriptions and examples.
- read_skill — read SKILL.md documentation by section.
- health_check — check system health: adapters and system state.
- JSON-RPC conformance; tool discovery by AI clients.

## Success Indicators

- [ ] Protocol compliance — implementation conforms to MCP JSON-RPC standards.
- [ ] Tool discovery — all tools discoverable by AI clients.
- [ ] Response time — under 5 seconds for standard operations.
- [ ] Rule conformance — the crate's own source complies with AES rules (no agent__, capabilities__, or infrastructure_* imports; taxonomy/contract only) when complete.
