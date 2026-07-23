# FRD Rules

## Purpose

FRD (Functional Requirements Document) describes **HOW** (functionally) for engineers, QA, and Tech Lead.

## Location

- Lives in **each feature module** (`packages/<feature>/FRD.md`)
- One FRD per feature/module

## Audience

- Engineers
- QA Engineers
- Tech Leads

## Focus

- Functional Requirements (with unique IDs: FR-001, FR-002)
- API Contract (endpoints, methods, payloads)
- Integration Points (3rd party, internal)
- Test Scenarios / QA Checklist
- Non-functional Requirements (detailed)
- Assumptions & Constraints
- Glossary

## Rules

1. **Use unique IDs** — FR-001, FR-002 for traceability
2. **Precise language** — avoid "secepatnya", "user-friendly"
3. **Include edge cases** — what happens at boundaries?
4. **Include error handling** — what happens when things fail?
5. **Reference PRD** — link to root PRD.md
6. **Include test scenarios** — QA can derive test cases
7. **One FRD per feature** — don't create monolith documents

## Anti-Patterns

- ❌ FRD without acceptance criteria → add testable conditions per FR
- ❌ FRD with vague language → use precise, measurable terms
- ❌ FRD without edge cases → add boundary conditions
- ❌ FRD without error handling → add failure scenarios
- ❌ FRD in root workspace → move to feature module
- ❌ FRD without PRD reference → add link to root PRD
