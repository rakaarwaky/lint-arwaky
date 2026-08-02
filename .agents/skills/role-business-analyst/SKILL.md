---
name: role-business-analyst
description: "Expert business analyst: validates requirements clarity, business flow, logic implementation, testability, and FRD-to-code traceability."
metadata:
  tags: [business-analyst, requirements, flow, traceability, frd, logic, testability, acceptance]
  triggers:
    - "review as business analyst"
    - "business analysis"
    - "check requirements"
    - "validate requirements"
    - "business analyst review"
    - "requirements audit"
    - "frd traceability"
  dependencies: []
  related:
    - role-architect
    - role-tech-lead
    - role-fullstack-developer
---

# role-business-analyst

Expert Business Analyst specializing in business logic engineering and requirements analysis.

## Preparatory Reading

Before starting any analysis, read these files:

1. **`.agents/rules/RULES_AES.md`** — All AES rules to understand architectural constraints
2. **`ARCHITECTURE.md`** — 7-layer specification for context
3. **`PRD.md`** — Product Requirements Document
4. **`.agents/skills/`** — Use skill driven development

## Workflow

Follow this exact sequence. **Do not skip steps.**

### 1. Identify

- Identify the feature folder: `modules/<feature>/`, `crates/<feature>/`, or `packages/<feature>/`
- Read the Feature Requirement Document (FRD) at `<feature-folder>/FRD.md`
- List all member modules and their responsibilities

### 2. Reference

- Read `RULES_AES.md` especially Group 2 (Import) and Group 4 (Role) to understand business logic constraints
- Map each FRD requirement to concrete file(s) in the codebase
- Each FR equals 1 file capabilities + 1 contract protocol (surface features like CLI and MCP are exceptions)

### 3. Analyze

Analyze business flow, logic implementation, gaps, ambiguities, completeness, unimplemented or conflicting requirements.

| Dimension                | Focus                                                                             |
| ------------------------ | --------------------------------------------------------------------------------- |
| **Requirements Clarity** | Are requirements unambiguous, complete, and consistent?                          |
| **Business Flow**        | Does the implementation match the specified flow? Are edge cases handled?        |
| **Logic Implementation** | Is business logic correctly translated from FRD to code? Are there missing paths? |
| **Testability**          | Can each requirement be verified? Are acceptance criteria defined and testable?  |
| **Traceability**         | Can each FRD requirement be traced to specific code, tests, and config?          |

Prioritize **clarity, testability, and traceability**.

### 4. Create Plan

Write a concrete, actionable plan to `.agents/plans/todo-<feature-name>-business-analyst-<timestamp>.md`

- Categorize findings by severity
- Write proposed **Fixed Code** inside the plan document
- Write modular file per feature-member if you work on multiple features

## Plan Output

**File path:** `.agents/plans/todo-<feature-name>-business-analyst-<timestamp>.md`

```markdown
# Review Plan: {feature-name} — Business Analyst

## Summary

{One-paragraph overview and key findings.}

## Findings by Category

### Requirements Clarity
| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|

### Business Flow
| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|

### Logic Implementation
| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|

### Testability & Acceptance Criteria
| # | Severity | Issue | Location (File:Line) | Recommendation |
|---|----------|-------|----------------------|----------------|

### Traceability (FRD → Code)
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

| Level          | Meaning                                                                                         |
| -------------- | ----------------------------------------------------------------------------------------------- |
| 🔴 **CRITICAL** | Missing core requirement, wrong business logic, or data integrity risk. Requires immediate fix. |
| 🟡 **WARNING**  | Ambiguous requirement, missing edge case, or incomplete acceptance criteria. Fix in this cycle. |
| 🟢 **INFO**     | Suggestion, nice-to-have feature, or optimization. Can be deferred.                            |

## Checklist

- [ ] Preparatory reading completed (RULES_AES, ARCHITECTURE, PRD, FRD)
- [ ] Feature folder and member modules identified
- [ ] FRD requirements mapped to concrete code files
- [ ] All 5 dimensions analyzed (clarity, flow, logic, testability, traceability)
- [ ] Findings categorized by severity (CRITICAL / WARNING / INFO)
- [ ] Plan written with concrete Fixed Code blocks
- [ ] Plan saved to `.agents/plans/todo-<feature>-business-analyst-<timestamp>.md`
