# CLI Commands — DESIGN

> Output contract for the process surface. Renderers and CI consume the
> outcome vocabulary below to decide how to present a run and whether it passed.
> Condition: [ROADMAP.md](../../ROADMAP.md) (workspace) +
> [BACKLOG.md](BACKLOG.md).

## Design Overview

This surface writes text, not pixels, so it has no palette. What it owns is the
**outcome vocabulary** a caller branches on — the exit code, and nothing else.
Findings are rendered by `utility_output_text_formatter.rs`; the exit code is
the machine signal, and it is the only one.

## Design Requirements

`docs <path>` takes the **audit root, not a filter** — unlike every other
path-taking subcommand here, which scopes a workspace-wide run down to the
target. Pointing `docs` at a sub-directory audits that sub-tree as its own
document chain, so crosslink checks (AES604) that expect root-level
`PRD.md`/`ROADMAP.md` siblings behave differently than in a whole-workspace run.
See `crates/doc-rules/FRD.md`.

The `report` command never prints findings. It counts the scan's violations
per member and shows each count's change against the last run for that target.
A first run has no baseline, so it shows absolute counts and a note that the
next run will show the delta.

### DS-CLIC-001: Outcome Vocabulary

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-CLIC-002: Path Semantics

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-CLIC-003: Output Shapes

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-CLIC-004: Report

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-CLIC-005: Kind

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-CLIC-006: Request Shape

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-CLIC-007: Invariants

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-CLIC-008: Change Checklist

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-CLIC-009: States

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-CLIC-010: Error States

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|
