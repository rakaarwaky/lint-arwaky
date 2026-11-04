# TUI — DESIGN

> Visual and interaction contract for the terminal surface. Tokens, panel
> geometry, and component anatomy.
> Condition: [ROADMAP.md](../../ROADMAP.md) (workspace) +
> [BACKLOG.md](BACKLOG.md).

## Design Overview

`utility_tui_theme.rs` is the single source of truth for every token below. A
renderer resolves a token through the `color()` helper rather than applying a
palette constant, so `NO_COLOR` collapses the whole surface to the terminal's
monochrome default in one place.

## Design Requirements

### DS-TUIC-001: Contrast Ownership

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-TUIC-002: Colors

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-TUIC-003: Typography

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-TUIC-004: Layout & Spacing

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-TUIC-005: Elevation & Depth

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-TUIC-006: Shapes

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-TUIC-007: Kind

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-TUIC-008: Request Shape

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-TUIC-009: Invariants

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-TUIC-010: Change Checklist

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-TUIC-011: States

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-TUIC-012: Error States

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-TUIC-013: Accessibility

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|
