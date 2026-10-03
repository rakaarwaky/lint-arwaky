# DESIGN — Cli

## Brand & Style

### Colors

| Role | Value | Use |
|------|-------|-----|
| Base surface | terminal default | The pane background. |
| Primary accent | cyan | The active selection. |
| Secondary accent | gray | Inactive rows. |
| Tertiary accent | yellow | A violation the user must act on. |

### Typography

| Level | Weight | Use |
|-------|--------|-----|
| Section | bold | A pane title. |
| Body | normal | A file row. |
| Muted | dim | A path hint. |

## Components

### Pane

| Part | Role |
|------|------|
| Title row | Names the pane. |
| Body | Lists the rows the pane owns. |
| Status row | Reports counts. |

### Row

| Part | Role |
|------|------|
| Label | The file name. |
| Detail | The violation codes. |

## Reference

- PRD.md — what the product does and why
- ARCHITECTURE.md — the layer contract this design follows
- FRD.md — the requirements the interface serves
