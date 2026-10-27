# BACKLOG — logging

---

## Progress Overview

The CLI and MCP entry points each carry their own `init_tracing()`
body, duplicating the `"warn,lint_arwaky::audit=info"` filter string.
No flag exists to raise the log level at runtime without exporting an
env var, so debugging a slow `la scan .` requires a rebuild or a
hard-coded filter change.

## Backlog Status

### {FR-Logging-001}

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready                                       |
| Done         | FR-Logging-001                                       |
| In Progress  | None                                        |
| Blocked      | None                                        |
| Next         | Ship                                        |
| Risk         | None                                        |
| Updated      | 2026-09-29                                  |

### {FR-Logging-002}

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready                                       |
| Done         | FR-Logging-002                                       |
| In Progress  | None                                        |
| Blocked      | None                                        |
| Next         | Ship                                        |
| Risk         | None                                        |
| Updated      | 2026-09-29                                  |

### {FR-Logging-003}

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready                                       |
| Done         | FR-Logging-003                                       |
| In Progress  | None                                        |
| Blocked      | None                                        |
| Next         | Ship                                        |
| Risk         | None                                        |
| Updated      | 2026-09-29                                  |

### {FR-Logging-004}

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready                                       |
| Done         | FR-Logging-004                                       |
| In Progress  | None                                        |
| Blocked      | None                                        |
| Next         | Ship                                        |
| Risk         | None                                        |
| Updated      | 2026-09-29                                  |

## Reference

| Prd | {Link to PRD.md} | [{link}](PRD.md) |
| Frd | {Link to FRD.md} | [{link}](FRD.md) |
| Roadmap | {Link to ROADMAP.md} | [{link}](ROADMAP.md) |
| Architecture | {Link to ARCHITECTURE.md} | [{link}](ARCHITECTURE.md) |
