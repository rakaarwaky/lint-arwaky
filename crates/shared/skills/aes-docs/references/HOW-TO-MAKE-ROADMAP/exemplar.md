# ROADMAP Exemplar

A filled ROADMAP against the Ecommerce Application project, matching the Template shape exactly. Entry priorities match the paired PRD exemplar (PR-ECOMMERCE-000 P0, PR-ECOMMERCE-001 P1, PR-ECOMMERCE-002–004 P2), and the roll-up stays in strict P0 → P1 → P2 order.

```markdown
# ROADMAP — Ecommerce Application

## Current Condition

Ecommerce Application is mid-build: the shared kernel shipped and is evidence-verified (v3.12.2 tag), product catalog is in review behind an open PR, and shopping cart, checkout, and web surfaces are specified but not started. The blocking risk is that checkout and order creation depend on a payment provider integration that is not yet provisioned.

## State Definitions

| State       | Meaning                                            |
|-------------|----------------------------------------------------|
| Ready       | Specified enough to start; nobody has started it.  |
| In Progress | Someone is in it now.                              |
| In Review   | PR open, awaiting review/CI.                       |
| Done        | Evidenced complete — cites the command/commit/PR.  |

## Feature Roll-up

### PR-ECOMMERCE-000: Shared Kernel
<!-- Tier P0: entry -000 is the first P0 entry. -->

| Aspect       | Detail                                    |
|--------------|---------------------------------------------|
| Priority     | P0                                          |
| State        | Done                                        |
| Dependencies | —                                          |
| Updated      | 2026-10-02                                  |

### PR-ECOMMERCE-001: Product Catalog
<!-- Tier P1: the first P1 entry. -->

| Aspect       | Detail                                    |
|--------------|---------------------------------------------|
| Priority     | P1                                          |
| State        | In Review                                   |
| Dependencies | PR-ECOMMERCE-000                           |
| Updated      | 2026-10-15                                  |

### PR-ECOMMERCE-002: Shopping Cart
<!-- Tier P2: the first P2 entry. -->

| Aspect       | Detail                                    |
|--------------|---------------------------------------------|
| Priority     | P2                                          |
| State        | Ready                                       |
| Dependencies | PR-ECOMMERCE-001                           |
| Updated      | 2026-10-18                                  |

### PR-ECOMMERCE-003: Checkout and Order

| Aspect       | Detail                                    |
|--------------|---------------------------------------------|
| Priority     | P2                                          |
| State        | Ready                                       |
| Dependencies | PR-ECOMMERCE-002 / PAYMENT-PROVIDER-001   |
| Updated      | 2026-10-18                                  |

### PR-ECOMMERCE-004: Web

| Aspect       | Detail                                    |
|--------------|---------------------------------------------|
| Priority     | P2                                          |
| State        | Ready                                       |
| Dependencies | PR-ECOMMERCE-001 / PR-ECOMMERCE-003       |
| Updated      | 2026-10-18                                  |

## Risk Register

- PAYMENT-PROVIDER-001: payment provider integration is not yet provisioned, so PR-ECOMMERCE-003 cannot reach In Progress (issue #142, open).
- CATALOG-STOCK-SYNC: product catalog stock is not synced with the shared kernel's inventory model; resolves when PR-ECOMMERCE-001 merges (issue #155, open).
```

## Notes

- The roll-up is in strict priority order: the P0 kernel, then the P1 catalog, then the three P2 entries (cart, checkout, web). No tier jumps.
- Each entry's `Priority` row matches the tier the paired PRD exemplar assigned to the same PR-ID — the ROADMAP does not promote or demote an entry the PRD declared at a different tier.
- `State` values are drawn only from the four-row `State Definitions` table; `Done` rows cite their evidence (the version tag in `Current Condition`).
- `Dependencies` carry cross-cutting PR-IDs and external integration IDs, kept to items that span multiple features or infrastructure.
- `Risk Register` lists open and closed risks with issue numbers.
