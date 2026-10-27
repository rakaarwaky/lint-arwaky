# DESIGN — Cli

## Design Overview

The fixture models a minimal terminal surface with a small token set and no
pixel-level tokens.

## Design Requirements

### DS-CLI-001: Colors

| Role | Value | Use |
|------|-------|-----|
| Base surface | terminal default | The pane background. |
| Primary accent | cyan | The active selection. |
| Secondary accent | gray | Inactive rows. |
| Tertiary accent | yellow | A violation the user must act on. |

### DS-CLI-002: Typography

| Level | Weight | Use |
|-------|--------|-----|
| Section | bold | A pane title. |
| Body | normal | A file row. |
| Muted | dim | A path hint. |

### DS-CLI-003: Pane

| Part | Role |
|------|------|
| Title row | Names the pane. |
| Body | Lists the rows the pane owns. |
| Status row | Reports counts. |

### DS-CLI-004: Row

| Part | Role |
|------|------|
| Label | The file name. |
| Detail | The violation codes. |

## Reference

- PRD.md — what the product does and why
- ARCHITECTURE.md — the layer contract this design follows
- FRD.md — the requirements the interface serves

| Backlog | {Link to BACKLOG.md} | [{link}](BACKLOG.md) |
| Roadmap | {Link to ROADMAP.md} | [{link}](ROADMAP.md) |
