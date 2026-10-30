# DATA Template

Copy, fill, delete nothing.

```markdown
# DATA — <project-name>

## Data Overview

<One paragraph: what data shapes this kernel manages, who reads/writes them, what business domain they serve. No code.>

## Data Requirements

### DT-<DATA-NAME-ID>-001: <Short imperative name>

| Type | Description|
|------|-------------|
|      |           |

### DT-<DATA-NAME-ID>-002: <Short imperative name>
### DT-<DATA-NAME-ID>-003: <Short imperative name>
### DT-<DATA-NAME-ID>-004: <Short imperative name>
### DT-<DATA-NAME-ID>-005: <Short imperative name>

## Reference

| Artifact     | Purpose                                         | Link|
|--------------|-------------------------------------------------|------|
| PRD          | <Business goals, user stories, and scope>       | [PRD.md](PRD.md)|
| Backlog      | <Task tracking, epics, and implementation status>| [BACKLOG.md](BACKLOG.md)|
| Roadmap      | <Timeline, phasing, and dependencies>           | [ROADMAP.md](ROADMAP.md)|
| Architecture | <System boundaries, tech stack, and infra>      | [ARCHITECTURE.md](ARCHITECTURE.md)|
```

## Section Contract

Only `Data Overview`, `Data Requirements`, `Reference` are required. Each exists for one reason.

| Section         | Why it belongs here                                                |
| --------------- | --------------------------------------------------------------------|
| Data Overview   | Orients the reader before the data items begin.                    |
| Data Requirements | `DT-<DATA-NAME-ID>-NNN` items, each with Type / Description table.|
| Reference       | Separates spec promise from backlog claim.                         |
