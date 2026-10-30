# Feature BACKLOG Exemplar

A filled BACKLOG against a feature (FRD.md) folder, matching the Template shape exactly. A feature folder carries `FRD.md` + `BACKLOG.md`. Each entry carries one ID from the `FR-<NAME>` family; the example below tracks three checkout requirement IDs across entries.

```markdown
# BACKLOG — Ecommerce Checkout

## Progress Overview

Checkout flow is in progress: cart validation, stock reservation, voucher application, and payment all compile against the contract aggregates, but end-to-end order placement is not yet wired to the notification service. Remaining risk is idempotency key handling under concurrent retries.

## Backlog Status 

### {FR-ECOMMERCE-001}

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Done                                        |
| Done         | Catalog page lists products with search     |
| In Progress  | None                                        |
| Risk         | None                                        |
| Updated      | 2026-10-30                                  |

### {FR-ECOMMERCE-002}

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | In Progress                                 |
| Done         | None                                        |
| In Progress  | Checkout flow with saved address and payment|
| Risk         | None                                        |
| Updated      | 2026-10-30                                  |

### {FR-ECOMMERCE-003}

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready                                       |
| Done         | Voucher validation and discount calculation |
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

- One entry per ID (`{FR-ECOMMERCE-001}`, `{FR-ECOMMERCE-002}`, `{FR-ECOMMERCE-003}`), each carrying one ID token per `extract_qualifier`.
- Five fixed rows per entry: State / Done / In Progress / Risk / Updated (`Blocked` and `Next` are no longer in the schema).
- Reference links PRD, FRD, Roadmap, and Architecture per the template.
