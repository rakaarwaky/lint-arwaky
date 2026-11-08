# Shared BACKLOG Exemplar

```markdown
# BACKLOG — Ecommerce Shared Kernel

## Progress Overview

The shared layer value objects are declared but only half are implemented across feature crates. Products, cart items, orders, addresses, and payment methods are modeled as immutable domain objects with no behavior, and no feature crate defines its own copy. Remaining risk is migrating feature-local shapes onto the layer types.

## Backlog Status 

### DT-ECOM-001: Product value object

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Done                                        |
| Done         | Product value object                        |
| In Progress  | None                                        |
| Risk         | None                                        |
| Updated      | 2026-10-30                                  |

### DT-ECOM-002: CartItem value object

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Done                                        |
| Done         | CartItem value object                       |
| In Progress  | None                                        |
| Risk         | None                                        |
| Updated      | 2026-10-30                                  |

### DT-ECOM-003: Order value object

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | In Progress                                 |
| Done         | None                                        |
| In Progress  | Order value object                          |
| Risk         | None                                        |
| Updated      | 2026-10-30                                  |

### DT-ECOM-004: Address value object

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready                                       |
| Done         | None                                        |
| In Progress  | None                                        |
| Risk         | None                                        |
| Updated      | 2026-10-30                                  |

### DT-ECOM-005: PaymentMethod value object

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready                                       |
| Done         | None                                        |
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

- One entry per ID (`DT-ECOM-001`..`DT-ECOM-005`), each carrying one ID token per `extract_qualifier`.
- Five fixed rows per entry: State / Done / In Progress / Risk / Updated (`Blocked` and `Next` are no longer in the schema).
- Entry headings are unbraced and colon-terminated, matching FRD/DATA/DESIGN heading syntax.
- Reference links PRD, FRD, Roadmap, and Architecture per the template.
