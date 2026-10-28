# Dispatcher — DESIGN

> Routing contract between the rule groups and every surface. Renderers read
> the outcome vocabulary below to decide how to present a run.
> Condition: [ROADMAP.md](../../ROADMAP.md) (workspace) +
> [BACKLOG.md](BACKLOG.md).

## Design Overview

This surface draws nothing, so it has no palette. What it owns instead is the
**outcome vocabulary** every renderer must honour: a collector never decides
that a run is acceptable, it only reports what the groups found and lets the
surface map that onto the process exit-code contract.

## Design Requirements

Every collector has the same shape: `collect_<group>(target, aggregates) -> Vec<Finding>`.

### DS-DISP-001: Outcome Vocabulary

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-DISP-002: Kind

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-DISP-003: Request Shape

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-DISP-004: Invariants

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-DISP-005: Change Checklist

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-DISP-006: States

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-DISP-007: Error States

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
