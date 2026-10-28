# DESIGN — Cli

## Design Overview

The fixture models a minimal terminal surface with a small token set and no pixel-level tokens.

## Design Requirements

### DS-CLI-001: Colors

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Base surface | pane background | terminal default | — |
| Primary accent | active selection | cyan | — |
| Secondary accent | inactive rows | gray | — |

### DS-CLI-002: Typography

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Section | pane title | bold | — |
| Body | file row | normal | — |
| Muted | path hint | dim | — |

### DS-CLI-003: Pane

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Title row | names the pane | — | — |
| Body | lists rows the pane owns | — | — |
| Status row | reports counts | — | — |

### DS-CLI-004: Row

| Variant | Use case | Token | Value|
|---------|----------|-------|------|
| Label | file name | — | — |
| Detail | violation codes | — | — |

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| PRD          | Business goals, user stories, and scope    | [PRD.md](../PRD.md) |
| Backlog      | Task tracking, epics, and implementation status | [BACKLOG.md](../BACKLOG.md) |
| Roadmap      | Timeline, phasing, and dependencies        | [ROADMAP.md](../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra   | [ARCHITECTURE.md](../ARCHITECTURE.md) |
