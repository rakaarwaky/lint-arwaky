# FRD Template

```markdown
# FRD — <feature-name>

## Feature Overview

<One diagram or ≤ 3 sentences: where this feature sits, what calls it, what it calls.>

## Functional Requirements

### FR-<FEATURE-NAME-ID>-001: <Short imperative name>

**Description:** <what it does — one paragraph>

#### Business Logic
- <validation logic>
- <validation logic>
- <validation logic>

#### Edge Cases
- <boundary condition & handling>
- <boundary condition & handling>
- <boundary condition & handling>

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | <shape, source> |
| **Output** | <shape, destination> |
| **Protocol** | <protocol trait / interface> |
| **Method** | <capability method> |
| **Event** | <domain event emitted, if any> |
| **Error** | <what fails, what the caller sees> |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| <context> | <action> | <observable result> |

#### Metric
| Target Metric | Value |
|---------------|-------|
| <metric, e.g. Latency / Success Rate> | <target value, e.g. 200ms / 99.9%> |

### FR-<FEATURE-NAME-ID>-002: <Short imperative name>
### FR-<FEATURE-NAME-ID>-003: <Short imperative name>
### FR-<FEATURE-NAME-ID>-004: <Short imperative name>
### FR-<FEATURE-NAME-ID>-005: <Short imperative name>

## Public Interface

| Aspect     | Detail            |
|------------|-------------------|
| Agent      | <agent name>      |
| Aggregate  | <aggregate name>  |
| Method     | <execute>         |
| Request    | <request>         |
| Response   | <response>        |

## Reference

| Artifact     | Purpose                                         | Link |
|--------------|-------------------------------------------------|------|
| PRD          | <Business goals, user stories, and scope>       | [PRD.md](PRD.md) |
| Backlog      | <Task tracking, epics, and implementation status>| [BACKLOG.md](BACKLOG.md) |
| Roadmap      | <Timeline, phasing, and dependencies>           | [ROADMAP.md](ROADMAP.md) |
| Architecture | <System boundaries, tech stack, and infra>      | [ARCHITECTURE.md](ARCHITECTURE.md) |
```
