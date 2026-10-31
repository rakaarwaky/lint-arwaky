# DATA Template

```markdown
# DATA — <project-name>

## Data Overview

<One paragraph: what data shapes this kernel manages, who reads/writes them, what business domain they serve. No code.>

## Data Requirements

### DT-SHARED-001: <Short imperative name>

| Type | Description|
|------|-------------|
|      |           |

### DT-SHARED-002: <Short imperative name>
### DT-SHARED-003: <Short imperative name>
### DT-SHARED-004: <Short imperative name>
### DT-SHARED-005: <Short imperative name>

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
| Data Requirements | `DT-SHARED-NNN` items, each with Type / Description table.|
| Reference       | Separates spec promise from backlog claim.                         |
