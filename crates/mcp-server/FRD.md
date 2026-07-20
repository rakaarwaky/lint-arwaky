# FRD — mcp-server

## Feature Goal
The mcp-server crate implements a Model Context Protocol (MCP) server that exposes the linting pipeline as tools and resources accessible by AI agents. It enables lint_arwaky integration with IDEs and other AI tools through a standard protocol.

## Requirements & Scope
- lint_scan — scan the workspace and return violation results.
- lint_fix — apply automatic fixes to specified files.
- lint_check — check a single file against AES rules.
- lint_config — manage lint_arwaky configuration.
- lint_setup — set up a new project with the AES structure.
- JSON-RPC conformance; tool discovery by AI clients.

## Success Indicators
- [ ] Protocol compliance — implementation conforms to MCP JSON-RPC standards.
- [ ] Tool discovery — all tools discoverable by AI clients.
- [ ] Response time — under 5 seconds for standard operations.
- [ ] Rule conformance — the crate's own source complies with AES rules (no agent_*, capabilities_*, or infrastructure_* imports; taxonomy/contract only) when complete.
