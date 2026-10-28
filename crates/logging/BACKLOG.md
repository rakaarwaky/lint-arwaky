# BACKLOG — logging

---

## Progress Overview

The CLI and MCP entry points each carry their own `init_tracing()`
body, duplicating the `"warn,lint_arwaky::audit=info"` filter string.
No flag exists to raise the log level at runtime without exporting an
env var, so debugging a slow `la scan .` requires a rebuild or a
hard-coded filter change.

## Backlog Status

### {Logging}

| Aspect       | Detail                                     |
|--------------|---------------------------------------------|
| State        | Ready                                      |
| Done         | Logging                                      |
| In Progress  | None                                       |
| Blocked      | None                                       |
| Next         | Ship                                       |
| Risk         | None                                       |
| Updated      | 2026-09-29                                 |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|
