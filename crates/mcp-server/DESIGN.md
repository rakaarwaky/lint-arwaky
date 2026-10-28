# MCP Server — DESIGN

> Protocol contract for the Model Context Protocol surface. Renderers and agent
> clients consume the response vocabulary below; the wire format is the design.
> Condition: [ROADMAP.md](../../ROADMAP.md) (workspace).

## Design Overview

This surface writes JSON-RPC frames, not pixels, so it has no palette. What it
owns is the **response vocabulary** a client must branch on — and the single
rule that governs it: a client branches on `exit_code`, never on JSON-RPC
success/failure.

## Design Requirements

### DS-MCPS-001: Outcome Vocabulary

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-MCPS-002: Kind

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-MCPS-003: Request Shape

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-MCPS-004: Invariants

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-MCPS-005: Change Checklist

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-MCPS-006: States

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
|         |          |       |       |

### DS-MCPS-007: Error States

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
