# FRD — mcp-server

## Feature Overview

The mcp-server crate exposes the tool to MCP clients over a local server: a
command catalog clients can list and filter, execute and config tools wired
straight into the dispatcher actions, and a skill reader for prompt material.
The crate is surface-only — it has no contract or capabilities layers of its
own and calls the dispatch layer directly, keeping MCP behavior identical to
the CLI for the same operation.

## Functional Requirements

### FR-McpServer-001: Command Catalog and Tool Execution

| Aspect | Detail|
|--------|-------|
| Description | Publish a listable command catalog and run those commands through MCP tools wired into the dispatcher. |
| Business Rules | Every catalog entry carries a name, description, and example; the list tool can filter the catalog by domain and yields an empty result for an unknown domain; the execute tool passes its action straight into the dispatch layer so MCP results match the CLI; version queries return a stable envelope. |
| Edge Cases | An unknown catalog entry is rejected rather than guessed; a nonexistent filter yields an empty list, not an error; a missing config file is reported loudly instead of silently defaulted. |
| Capability | Tool handlers for list_commands, execute_command, and get_config that delegate to the dispatcher. |
| Protocol | —; the crate is surface-only and calls the dispatch layer directly, with no capability seam of its own. |
| Method | handle list commands with optional domain filter, handle execute command with path validation, handle get config, handle read skill. |
| Input | MCP tool call arguments (command name, path, domain filter); source is the MCP client request. |
| Output | catalog entries or command results carrying the dispatcher response. |
| Error | unknown command or config absence is reported loudly in the tool result. |
| Event | None; each tool call is a single synchronous round trip. |
| Test | test_command_catalog_is_nonempty, test_catalog_entries_have_name_description_example, test_catalog_contains_expected_commands, handle_list_commands_returns_full_catalog_without_filter, and handle_execute_command_wires_action_into_dispatch assert catalog and execution wiring. |
| Metric | Every seeded catalog entry satisfies the name/description/example triple; each execute call reaches exactly one dispatcher action. |

### FR-McpServer-002: Workspace Confinement and Read-Only Authorization

| Aspect | Detail|
|--------|-------|
| Description | Confine client-supplied paths to the workspace and classify mutating actions for read-only authorization. |
| Business Rules | Every path argument is resolved and rejected if it escapes the workspace; mutating actions are classified so a read-only authorization mode can refuse them before they run; non-mutating reads stay available. |
| Edge Cases | A path with traversal segments outside the workspace is rejected; a mutating action under read-only authorization is refused with a clear result; an unknown classification defaults to the safe side. |
| Capability | Path validation step before dispatch and the mutating-action classifier. |
| Protocol | —; confinement runs in the tool handlers before they call the dispatcher. |
| Method | reject an out-of-workspace path; classify an action as mutating or read-only. |
| Input | client path arguments and the action name with its authorization mode. |
| Output | accepted paths reach the dispatcher; rejected paths and refused mutations return structured tool errors. |
| Error | out-of-workspace or unauthorized requests fail closed. |
| Event | None. |
| Test | client_paths_are_confined_to_workspace and mutating_actions_are_classified_for_read_only_authorization assert confinement and classification. |
| Metric | Every seeded traversal path is rejected; every seeded mutating action under read-only authorization is refused. |

## Public Interface

| Aspect | Detail|
|--------|-------|
| Surface | MCP tools: list_commands, execute_command, get_config, read_skill, docs scaffold. |
| Entry | tool handlers in the server loop, one module per command family. |
| Delegation | tool results come from dispatcher actions; the server adds no business logic. |
| Taxonomy | request and response value objects live in this crate as its only consumer needs them. |
| Contract | argument validation, catalog rules, and authorization before dispatch. |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|
