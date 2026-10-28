# FRD — cli-commands

## Feature Overview

The cli-commands crate is the terminal surface of the tool: one surface command
module per subcommand (scan, check, fix, ci, report, git, watch, setup, plugin,
skill, config, maintenance) parses arguments, invokes the dispatch layer, and
formats the returned result. Formatting helpers — output text rendering, report
snapshots and deltas, and skill hint resolution — live in the shared kernel
module for this feature so every surface formats results the same way.

## Functional Requirements

Each subcommand's surface module owns argument handling, progress reporting,
and result formatting only; the workspace work itself routes through the
dispatcher action for that verb, so CLI behavior stays aligned with the MCP and
TUI surfaces. Report-oriented subcommands render snapshots, per-member deltas,
and formatted text through the kernel output helpers, keeping message text
stable for scripts that parse it. The skill subcommand resolves contextual
hints through the kernel skill hint resolver and prints them with the same
output formatter. Every command exits with the code the workspace exit-code
contract assigns to its outcome, so callers can branch on status without
parsing output.

## Public Interface

| Aspect | Detail|
|--------|-------|
| Surface | scan, check, fix, ci, report, git, watch, setup, plugin, skill, config, maintenance |
| Entry | one surface command module per subcommand |
| Delegation | workspace work routes through the dispatcher actions |
| Formatting | output text formatter, report snapshot and delta renderer, skill hint resolver |
| Contract | argument handling and result formatting only; business logic lives in the dispatcher |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|
