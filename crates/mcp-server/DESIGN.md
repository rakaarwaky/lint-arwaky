# MCP Server — DESIGN

## Kind

`mcp` — the Model Context Protocol surface. Exposes the lint toolset to an MCP
client over JSON-RPC 2.0 on stdin/stdout, so an agent can run scans, read
configuration, and pull skill documentation without shelling out.

## Entry Points

| File | Tool | Purpose |
| --- | --- | --- |
| `surface_mcp_tool_command.rs` | `execute_command` | Run any CLI command. The primary tool. |
| `surface_mcp_tool_command.rs` | `list_commands` | List available commands, optionally filtered by domain. |
| `surface_mcp_tool_command.rs` | `read_skill` | Read a skill's documentation by section. |
| `surface_mcp_tool_command.rs` | `health_check` | Report adapter availability and system state. |
| `surface_mcp_tool_command.rs` | `get_config` | Return the effective architecture config for a path and language. |
| `surface_mcp_action_command.rs` | — | The JSON-RPC request loop: parse, route, respond. |

## Request Shape

A request arrives as one JSON-RPC 2.0 message on stdin. The action command
parses it, matches the method against the tool router, and returns a response on
stdout. Tool arguments are `Parameters<T>` structs, so a malformed argument is
rejected before any lint work starts.

### `execute_command` response schema

Every `execute_command` call returns a **successful** JSON-RPC tool result whose
text content is one JSON object. The wrapped command's outcome is carried
inside that object, never on the JSON-RPC envelope:

| Field | Type | Meaning |
| --- | --- | --- |
| `exit_code` | integer | The CLI exit-code contract value: `0` Ok, `1` PolicyFail, `2` RuntimeError, `3` PrerequisiteMissing. Always present. |
| `status` | string | A short, human-oriented label for the same outcome (`ok`, `warning`, or `error`). Action-specific detail is carried by `result`. |
| `action` | string | The command that ran, echoed back. |
| `error` | string | Present only when the call could not run at all; it is accompanied by `exit_code: 2`. |
| other | — | Per-command payload (e.g. `total_violations`, `results`, `items`, `message`). |

A client **MUST** branch on `exit_code`. Branching on JSON-RPC success/failure,
or on `status`, is wrong: a `scan` that found violations is a JSON-RPC success
carrying `exit_code: 1`, and an agent that ignores the field reads a failing
lint run as "nothing to do". `PRD.md`'s Exit Code Contract states the same
requirement for MCP responses.

### End-to-end call chain and its timeouts

```mermaid
sequenceDiagram
    participant C as MCP client (AI agent)
    participant S as MCP server (single-threaded)
    participant CLI as CLI command layer
    participant D as dispatcher
    participant X as external-lint
    participant P as tool subprocess

    C->>S: execute_command { action: "scan", path }
    Note over S: no server-side timeout; the loop is blocked for the whole call
    S->>CLI: route the action
    CLI->>D: collect_<group>(target, aggregates)
    D->>X: run the adapter set (sequential)
    loop up to 10 adapters, one at a time
        X->>P: spawn tool
        Note over X,P: per-adapter ceiling: 180s Clippy · 120s Rustfmt/cargo-audit · 60s each Python/JS/Markdown tool
        P-->>X: output, or timeout → adapter error, scan continues
    end
    X-->>D: normalized findings
    D-->>CLI: findings
    CLI-->>S: output + exit code
    S-->>C: { exit_code, status, … }
```

Only the subprocess layer has a timeout. Nothing above it — adapter phase,
dispatcher, CLI, MCP — imposes a budget of its own, so the per-adapter ceilings
**sum** rather than cap. Worst case, with every language present and every
adapter running to its ceiling: 180 + 120 + 120 + (7 × 60) ≈ **840 s (~14
minutes)** for one `execute_command` call. See the `timeout` row in Error
States.

## States

| State | Condition | Response |
| --- | --- | --- |
| `uninitialized` | A request arrived before the MCP handshake | A protocol error; the server waits |
| `ready` | The handshake completed and the tool router is live | Normal tool results |
| `busy` | A tool call is executing | The client waits; the server is single-threaded. No server-side time limit applies: the call runs to completion, up to the ~14-minute worst case traced in Request Shape |
| `error` | The request was malformed, or the tool failed | A JSON-RPC error object carrying the cause |

## Error States

| Condition | Behaviour |
| --- | --- |
| Malformed JSON on stdin | Return a JSON-RPC parse error and keep the process alive |
| Unknown method | Return a method-not-found error; do not exit |
| Tool argument fails to deserialize | Return an invalid-params error naming the field |
| The underlying CLI command fails | Return the command's output and exit code as a successful tool result, so the client can read why — in the `execute_command` response schema above, with `exit_code` carrying `1`/`2`/`3` |
| A tool call runs long (e.g. `execute_command` running `scan`/`ci` over a large mixed-language workspace) | **No server-side timeout and no cancellation method exist.** The call blocks the single-threaded loop until the underlying command returns; queued calls from the same client wait behind it. The only bound is the external-lint per-adapter ceiling chain (~14 minutes worst case) |
| A client needs to abandon a blocked call | The protocol surface offers no cancel request. Recovery is client-side: set a client timeout above the worst-case bound, or kill and restart the server process and re-issue the call |

The server never terminates on a client error. A malformed request is answered
and the loop continues, because an MCP client may recover and retry.

### Response Contract

Every action returns `exit_code` as the primary machine signal (`0` success,
`1` policy findings, `2` invalid request/runtime error, `3` missing tool). The
shared `status` vocabulary is `ok`, `warning`, or `error`; action-specific
outcomes such as `clean`, `violations`, `pass`, and `fail` are reported in
`result`.

The server confines all client paths to the canonical startup working directory.
It starts read-only; operators must set `LINT_ARWAKY_MCP_ALLOW_MUTATIONS=true`
to permit `fix`, hook changes, or project initialization.

**Timeout decision (current contract).** Overall MCP calls remain unbounded;
the subprocess fallback has a configurable 60-second per-linter timeout, while
in-process and adapter phases run to completion. Clients should retain their own
overall timeout above the documented worst case.

## Invariants

- Tools reach lint behavior through the CLI command layer, never by importing a
  capabilities file directly.
- stdout carries protocol frames only. Diagnostics go to stderr, so a stray log
  line cannot corrupt a response.
- A tool returns text the client can read directly; it does not return raw
  protocol objects.
- Every `execute_command` result carries a top-level integer `exit_code`, for
  every action and every outcome. A payload that omits it is a contract
  violation, not a formatting detail.

## Change Checklist

Adding a tool means adding one `#[tool]` method with its `Parameters<T>` args
struct, registering it in the tool router, and adding a row to the table above.
Changing the transport touches `surface_mcp_action_command.rs` only.
