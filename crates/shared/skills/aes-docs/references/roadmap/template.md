# ROADMAP Template

```markdown
# ROADMAP — <workspace name>

## Progress Overview

<One paragraph of progress summary.>

## Roadmap Status

### PR-<PROJECT-NAME-ID>-001: <Short imperative name>

| Aspect       | Detail                                    |
|--------------|-------------------------------------------|
| Priority     | P0 / P1 / P2                              |
| State        | Ready / In Progress / In Review / Done    |
| Backlog      | [BACKLOG.md](crates/<feature>/BACKLOG.md) |

### PR-<PROJECT-NAME-ID>-002: <Short imperative name>

| Aspect       | Detail                                    |
|--------------|-------------------------------------------|
| Priority     | P0 / P1 / P2                              |
| State        | Ready / In Progress / In Review / Done    |
| Backlog      | [BACKLOG.md](crates/<feature>/BACKLOG.md) |

### PR-<PROJECT-NAME-ID>-003: <Short imperative name>

| Aspect       | Detail                                    |
|--------------|-------------------------------------------|
| Priority     | P0 / P1 / P2                              |
| State        | Ready / In Progress / In Review / Done    |
| Backlog      | [BACKLOG.md](crates/<feature>/BACKLOG.md) |

## Reference

| Artifact     | Purpose                                    | Link                       |
|--------------|--------------------------------------------|----------------------------|
| PRD          | Business goals, user stories, and scope    | [PRD.md](PRD.md)           |
| Architecture | System boundaries, tech stack, and infra   | [ARCHITECTURE.md](ARCHITECTURE.md) |
```

## Section Contract

Every section is required unless marked optional. Each exists for one reason.

| Section          | Why it belongs here                                                   |
| ---------------- | -----------------------------------------------------------------------|
| Progress Overview| Workspace progress summary in one paragraph.                          |
| Roadmap Status   | `PR-<PROJECT-NAME-ID>-NNN` items with Priority / State / Backlog.     |
| Reference        | Connects the roadmap to the problem spec and the layer contract.      |

Status Policy is removed from the template and not allowed (the `ROADMAP_DOC` allowed set is `[]`).