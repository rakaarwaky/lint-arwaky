# BACKLOG Template

Copy, fill, delete nothing.

```markdown
# BACKLOG — <feature-name>

## Progress Overview

<One paragraph of progress summary.>

## Backlog Status 

### {<DATA-NAME-ID> | <FEATURE-NAME-ID> | DESIGN-NAME-ID}

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready / In Progress / In Review / Done      |
| Done         | <evidence>                                  |
| In Progress  | <None / item name>                          |
| Risk         | <what ships broken / None>                  |
| Updated      | <YYYY-MM-DD>                                |

### {<DATA-NAME-ID> | <FEATURE-NAME-ID> | DESIGN-NAME-ID}
### {<DATA-NAME-ID> | <FEATURE-NAME-ID> | DESIGN-NAME-ID}
### {<DATA-NAME-ID> | <FEATURE-NAME-ID> | DESIGN-NAME-ID}

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
| Backlog Status   | `{<DATA-NAME-ID> &#124; <FEATURE-NAME-ID> &#124; DESIGN-NAME-ID}` entries with State / Done / In Progress / Risk / Updated. |
| Reference        | Connects the backlog to PRD, FRD, Roadmap, Architecture.               |
