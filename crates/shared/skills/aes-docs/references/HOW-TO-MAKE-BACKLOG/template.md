# BACKLOG Template

Copy, fill, delete nothing.

```markdown
# BACKLOG — <feature-name>

## Progress Overview

<One paragraph of progress summary.>

## Backlog Status 

### <FEATURE-NAME-ID>: <entry title>

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready / In Progress / In Review / Done      |
| Done         | <evidence>                                  |
| In Progress  | <None / item name>                          |
| Risk         | <what ships broken / None>                  |
| Updated      | <YYYY-MM-DD>                                |

### <FEATURE-NAME-ID>: <entry title>
### <DS-NAME-ID>: <entry title>
### <DT-NAME-ID>: <entry title>

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| PRD          | <Business goals and scope>                 | [PRD.md](PRD.md) |
| FRD          | <Functional requirements specification>    | [FRD.md](FRD.md) |
| Roadmap      | <Timeline, phasing, and dependencies>      | [ROADMAP.md](ROADMAP.md) |
| Architecture | <System boundaries, tech stack, and infra> | [ARCHITECTURE.md](ARCHITECTURE.md) |
```

## Section Contract


| Section          | Why it belongs here                                                     |
| ---------------- | ----------------------------------------------------------------------- |
| Progress Overview| Gives the truth before the table, not after it.                         |
| Backlog Status   | `<FEATURE-NAME-ID>`, `<DS-NAME-ID>`, or `<DT-NAME-ID>` entries (unbraced, colon-terminated) with State / Done / In Progress / Risk / Updated. |
| Reference        | Connects the backlog to PRD, FRD, Roadmap, Architecture.               |
