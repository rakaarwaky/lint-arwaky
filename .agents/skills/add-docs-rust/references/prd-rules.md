# PRD Rules

## Purpose

PRD (Product Requirements Document) describes **WHAT** and **WHY** for stakeholders, PM, Design, and Engineering leads.

## Location

- Lives in **root workspace only** (`<workspace-root>/PRD.md`)
- One PRD per project (not per feature)

## Audience

- Stakeholders
- Product Managers
- Designers
- Engineering Leads

## Focus

- Problem Statement
- Goals & Success Metrics
- User Personas
- Scope (in/out)
- Feature Requirements (prioritized P0/P1/P2)
- Non-functional Requirements (high-level)

## Rules

1. **Write for non-engineers** — avoid technical jargon
2. **Use acceptance criteria** — measurable conditions for each feature
3. **Prioritize features** — P0 (must have), P1 (should have), P2 (nice to have)
4. **Include user personas** — who are we building for?
5. **Define success metrics** — how do we know we succeeded?
6. **No technical details** — SQL schemas, API contracts belong in FRD
7. **Version and date** — track document evolution

## Anti-Patterns

- ❌ PRD contains SQL schema or API details → move to FRD
- ❌ PRD written in technical language → rewrite for stakeholders
- ❌ PRD without acceptance criteria → add measurable conditions
- ❌ PRD without user personas → add who we're building for
- ❌ PRD without success metrics → add KPIs/OKRs
