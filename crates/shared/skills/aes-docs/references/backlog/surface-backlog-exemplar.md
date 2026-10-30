# Surface BACKLOG Exemplar

A filled BACKLOG against a surface (DESIGN.md) folder, matching the Template shape exactly. A surface folder carries `DESIGN.md` + `BACKLOG.md`. Each entry carries one ID from the `DS-<NAME>` family; the example below tracks three design-token IDs across entries.

```markdown
# BACKLOG — Ecommerce Web Surface

## Progress Overview

The web surface is in progress: catalog, cart, and checkout pages render against the contract aggregates, and the seller dashboard lists products and orders. Design tokens (DS-WEB-001..006) are declared; two of six are implemented. Remaining risk is the responsive grid below the mobile breakpoint.

## Backlog Status 

### DS-WEB-001: Color tokens

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Done                                        |
| Done         | DS-WEB-001 color tokens resolved in CSS vars|
| In Progress  | None                                        |
| Risk         | None                                        |
| Updated      | 2026-10-30                                  |

### DS-WEB-002: Spacing and layout grid

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | In Progress                                 |
| Done         | None                                        |
| In Progress  | DS-WEB-002 spacing and layout grid          |
| Risk         | Mobile single-column grid not yet verified  |
| Updated      | 2026-10-30                                  |

### DS-WEB-003: Typography scale

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready                                       |
| Done         | DS-WEB-003 typography scale tokens drafted  |
| In Progress  | None                                        |
| Risk         | None                                        |
| Updated      | 2026-10-30                                  |

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| PRD          | Business goals and scope                   | [PRD.md](PRD.md) |
| FRD          | Functional requirement specification       | [FRD.md](FRD.md) |
| Roadmap      | Timeline, phasing, and dependencies        | [ROADMAP.md](ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra   | [ARCHITECTURE.md](ARCHITECTURE.md) |
```

## Why this shape

- One entry per ID (`DS-WEB-001`, `DS-WEB-002`, `DS-WEB-003`), each carrying one ID token per `extract_qualifier`.
- Five fixed rows per entry: State / Done / In Progress / Risk / Updated (`Blocked` and `Next` are no longer in the schema).
- Entry headings are unbraced and colon-terminated, matching FRD/DATA/DESIGN heading syntax.
- Reference links PRD, FRD, Roadmap, and Architecture per the template.
