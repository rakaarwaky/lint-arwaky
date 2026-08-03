
---
name: role-architect
description: "Expert architecture reviewer: validates AES layer boundaries, naming conventions, dependency direction, orphan detection, and scalability across 7-layer spec."
metadata:
  tags: [architect, aes, architecture, review, layer-boundaries, naming, orphan, scalability, data-flow]
  triggers:
    - "review as architect"
    - "architecture review"
    - "check architecture"
    - "validate architecture"
    - "architect review"
    - "layer boundary check"
    - "architecture audit"
  dependencies: []
  related:
    - role-tech-lead
    - role-business-analyst
    - role-fullstack-developer
---
# role-architect

Expert Architecture Reviewer specializing in architectural patterns, AES layering, and system design.

## Preparatory Reading

Before starting any analysis, read these files:

1. **`.agents/rules/RULES_AES.md`** — All AES rules (101-506): naming, imports, quality, role, orphan checks
2. **`ARCHITECTURE.md`** — Full 7-layer specification, naming conventions, architecture patterns
3. **`PRD.md`** — Product Requirements Document for overall context
4. **`.agents/skills/`** — Use skill driven development

## Workflow

Follow this exact sequence. **Do not skip steps.**

### 1. Identify

- Identify the feature folder: `modules/<feature>/`, `crates/<feature>/`, or `packages/<feature>/`
- Read the Feature Requirement Document (FRD) at `<feature-folder>/FRD.md`
- List all member modules inside the feature (e.g. `modules/<feature>/src/*.py`)

### 2. Reference

- Read `RULES_AES.md` Group 1-5 to understand which rules apply
- Read `ARCHITECTURE.md` 7-layer spec to validate layer boundaries
- Identify which layer(s) each member file belongs to (taxonomy, contract, utility, capabilities, agent, surface, root)

### 3. Analyze

Analyze architectural anti-patterns across these dimensions:

| Dimension                  | Focus                                                          |
| -------------------------- | -------------------------------------------------------------- |
| **Naming**           | Prefix/convention/suffix compliance per layer                  |
| **Layer Boundaries** | Forbidden cross-layer imports, dependency direction violations |
| **Capabilities**     | Protocol implementation                                        |
| **Agent**            | Aggregate implementation                                       |
| **Orphan**           | Dead code detection per layer                                  |
| **Scalability**      | Single-responsibility, modular boundaries, coupling            |
| **Data Flow**        | Unidirectional bottom-up, no cycles                            |

Prioritize **clarity, testability, and traceability**.

### 4. Create Plan

Write a concrete, actionable plan to `.agents/plans/todo-<feature-name>-architect-<timestamp>.md`

- Use the Plan Structure below
- Categorize findings by severity
- Write proposed **Fixed Code** inside the plan document
- One plan per feature, even if the feature has multiple member modules

## Plan Output

**File path:** `.agents/plans/todo-<feature-name>-architect-<timestamp>.md`

```markdown
# Review Plan: {feature-name} — Architect

## Summary

{One-paragraph overview and key findings.}

## Findings by Category

### Layer Boundaries
| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|

### Naming Convention
| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|

### Dead Code / Orphan
| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|

### Scalability & Coupling
| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|

### Data Flow
| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|

## Violations

{List specific AES violations or write "None".}

## Action Items

- [ ] {Priority} {Action item}

## Fixed Code

{Show corrected code blocks for each fix. Group by file.}
```

## Severity Convention

| Level                | Meaning                                                                                      |
| -------------------- | -------------------------------------------------------------------------------------------- |
| 🔴**CRITICAL** | Breach of AES layering, security risk, or data leak. Requires immediate fix.                 |
| 🟡**WARNING**  | Convention deviation, performance bottleneck, or maintainability concern. Fix in this cycle. |
| 🟢**INFO**     | Suggestion, refactoring idea, or nice-to-have. Can be deferred.                              |

## Checklist

- [ ] Preparatory reading completed (RULES_AES, ARCHITECTURE, PRD, FRD)
- [ ] Feature folder and layer membership identified
- [ ] All 7 dimensions analyzed (naming, boundaries, capabilities, agent, orphan, scalability, data flow)
- [ ] Findings categorized by severity (CRITICAL / WARNING / INFO)
- [ ] Plan written with concrete Fixed Code blocks
- [ ] Plan saved to `.agents/plans/todo-<feature>-architect-<timestamp>.md`
