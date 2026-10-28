# DATA — Shared

## Data Overview

The shared kernel declares the protocols each feature implements. It holds no
data of its own: callers supply the request, and each feature returns the resolved value.

## Data Requirements

### DT-SHAR-001: Protocol shape

| Type | Description|
|------|-------------|
| Request | The shape callers supply before a run. |
| Response | The resolved value returned to the caller. |

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| PRD          | Business goals, user stories, and scope    | [PRD.md](../../PRD.md) |
| Backlog      | Task tracking, epics, and implementation status | [BACKLOG.md](../../BACKLOG.md) |
| Roadmap      | Timeline, phasing, and dependencies        | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra   | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
