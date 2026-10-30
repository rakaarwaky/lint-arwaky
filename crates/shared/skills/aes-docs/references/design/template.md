# DESIGN Template

Copy, fill, delete nothing.

```markdown
# DESIGN — <project-name>

## Design Overview

<One paragraph of what the design looks like.>

## Design Requirements

### DS-<DESIGN-NAME-ID>-001: <Short imperative name>

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |     |

### DS-<DESIGN-NAME-ID>-002: <Short imperative name>
### DS-<DESIGN-NAME-ID>-003: <Short imperative name>
### DS-<DESIGN-NAME-ID>-004: <Short imperative name>
### DS-<DESIGN-NAME-ID>-005: <Short imperative name>
### DS-<DESIGN-NAME-ID>-006: <Short imperative name>

## Reference

| Artifact     | Purpose                                         | Link|
|--------------|-------------------------------------------------|------|
| PRD          | <Business goals, user stories, and scope>       | [PRD.md](PRD.md)|
| Backlog      | <Task tracking, epics, and implementation status>| [BACKLOG.md](BACKLOG.md)|
| Roadmap      | <Timeline, phasing, and dependencies>           | [ROADMAP.md](ROADMAP.md)|
| Architecture | <System boundaries, tech stack, and infra>      | [ARCHITECTURE.md](ARCHITECTURE.md)|
```

## Section Contract

Only `Design Overview`, `Design Requirements`, `Reference` are required. Each exists for one reason.

| Section              | Why it belongs here                                         |
| -------------------- | -------------------------------------------------------------|
| Design Overview      | Orients the reader before the requirements begin.           |
| Design Requirements  | `DS-<DESIGN-NAME-ID>-NNN` items the surface must carry.     |
| Reference            | Connects the spec back to the spec chain.                   |
