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

| Component                   | File                              | States                                    | Token map                            |
| --------------------------- | --------------------------------- | ----------------------------------------- | -------------------------------------|
| Event loop and app state    | `surface_tui_command.rs`          | running / idle                            | `BACKGROUND`                        |
| Action handler              | `surface_lint_action.rs`          | running / results / clean / error         | —                                    |
| Keystroke mapping           | `surface_event_action.rs`         | idle / ignored-while-running              | `KEY`                               |
| Path-entry screen           | `surface_path_screen.rs`          | empty / filled / focused / confirmed      | `PATH_INPUT`, `FOCUS_CONFIRM`        |
| File-tree pane              | `surface_tree_view.rs`            | browsing / selected                       | `DIRECTORY`, `LABEL`, `HIGHLIGHT`    |
| File-list pane              | `surface_file_list_view.rs`       | browsing / selected                       | `LABEL`, `HIGHLIGHT`                |
| Findings preview pane       | `surface_preview_view.rs`         | results / clean / error                   | `VIOLATIONS`, `CLEAN`                |
| Status line                 | `surface_status_component.rs`     | idle / busy / clean / error               | `PENDING`, `CLEAN`, `VIOLATIONS`     |
| Shortcut hint bar           | `surface_shortcut_component.rs`   | idle / busy-acknowledgement               | `LABEL`, `SEPARATOR`, `KEY`          |
| Log controller              | `surface_log_controller.rs`       | streaming / idle                          | `SEPARATOR`                         |
| Theme and colour resolution | `utility_tui_theme.rs`           | colour / no-colour                        | all tokens                          |
| Report rendering            | `utility_report_formatter.rs`     | rendered                                 | `LABEL`                             |
| Directory listing           | `utility_file_system.rs`          | ok / error                                | —                                    |

### DS-TUIC-001: Contrast Ownership

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Variant | Use case | Token | Value|
| — | — | — | — |

### DS-TUIC-002: Colors

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Variant | Use case | Token | Value|
| — | — | — | — |

### DS-TUIC-003: Typography

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Variant | Use case | Token | Value|
| — | — | — | — |

### DS-TUIC-004: Layout & Spacing

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Variant | Use case | Token | Value|
| — | — | — | — |

### DS-TUIC-005: Elevation & Depth

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Variant | Use case | Token | Value|
| — | — | — | — |

### DS-TUIC-006: Shapes

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Variant | Use case | Token | Value|
| — | — | — | — |

### DS-TUIC-007: Kind

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Variant | Use case | Token | Value|
| — | — | — | — |

### DS-TUIC-008: Request Shape

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Variant | Use case | Token | Value|
| — | — | — | — |

### DS-TUIC-009: Invariants

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Variant | Use case | Token | Value|
| — | — | — | — |

### DS-TUIC-010: Change Checklist

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Variant | Use case | Token | Value|
| — | — | — | — |

### DS-TUIC-011: States

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Variant | Use case | Token | Value|
| — | — | — | — |

### DS-TUIC-012: Error States

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Variant | Use case | Token | Value|
| — | — | — | — |

### DS-TUIC-013: Accessibility

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Variant | Use case | Token | Value|
| — | — | — | — |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|
