# DATA — Shared

## Data Overview

The shared kernel declares the protocols each feature implements. It holds no
data of its own: callers supply the request, and each feature returns the resolved value.

## Data Requirements

### DT-SHARED-001: Request value object

| Type | Description |
|------|-------------|
| Request | The shape callers supply before a run. |

### DT-SHARED-002: Response value object

| Type | Description |
|------|-------------|
| Response | The resolved value returned to the caller. |

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| PRD          | Business goals, user stories, and scope    | [PRD.md](../PRD.md) |
| Backlog      | Task tracking, epics, and implementation status | [BACKLOG.md](../BACKLOG.md) |
| Roadmap      | Timeline, phasing, and dependencies        | [ROADMAP.md](../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra   | [ARCHITECTURE.md](../ARCHITECTURE.md) |
