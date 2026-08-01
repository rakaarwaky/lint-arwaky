---
name: add-docs-typescript
description: "Add proper JSDoc comments, type annotations, and project-level PRD.md/README.md + per-feature FRD.md to TypeScript packages following project conventions."
metadata:
  tags: [typescript, docs, jsdoc, type-hints, prd, frd, readme]
  triggers:
    - "add docs typescript"
    - "add jsdoc typescript"
    - "add type hints typescript"
    - "add prd typescript"
    - "add frd typescript"
    - "add package readme typescript"
  dependencies: []
  related:
    - cleanup-consolidate-typescript
    - add-docs-rust
---
# add-docs-typescript

## Purpose

Add documentation at correct locations following project conventions.

## Rules

- **PRD.md** = Product Requirements Document — **1 per project root** — describes **WHAT** and **WHY** for stakeholders.
- **README.md** = Developer onboarding — **1 per project root** — describes **HOW TO USE/RUN** for developers.
- **FRD.md** = Functional Requirements Document — **1 per feature module** — describes **HOW** (functionally) for engineers.
- Relationship: **PRD (what/why) → FRD (how) → README (how to use)**. Each serves a different audience.
- All public classes and methods MUST have JSDoc docstrings.
- Docstrings MUST explain "what" and "why", not "how" (code shows how).

## Document Location Matrix


| Document  | Location            | Audience                     | Focus                |
| ----------- | --------------------- | ------------------------------ | ---------------------- |
| PRD.md    | Root workspace      | Stakeholder, PM, Design, Eng | _What_ & _Why_       |
| README.md | Root workspace      | Developer (new/existing)     | _How to use/run_     |
| FRD.md    | Each feature module | Engineer, QA, Tech Lead      | _How_ (functionally) |

## Templates

### PRD.md

```markdown
# PRD — <project-name>

## Problem Statement

<One paragraph: what problem does this project solve?>

## Goals & Success Metrics

- Goal 1: <measurable outcome>
- Goal 2: <measurable outcome>

## User Personas

- **Persona 1**: <who they are, what they need>
- **Persona 2**: <...>

## Scope

- In scope: <...>
- Out of scope: <...>

## Feature Requirements (Prioritized)

### P0 — Must Have

- [ ] <feature with acceptance criteria>

### P1 — Should Have

- [ ] <feature with acceptance criteria>

### P2 — Nice to Have

- [ ] <feature with acceptance criteria>

## Non-functional Requirements (High-level)

- Performance: <...>
- Security: <...>
- Scalability: <...>

## Open Questions / Risks

- <question or risk>
```

### FRD.md

```markdown
# FRD — <feature-name>

## System Overview

<Architecture diagram or high-level description>

## Functional Requirements

### FR-001: <Feature Name>

- **Description**: <what it does>
- **Input**: <input data>
- **Output**: <output data>
- **Business Rules**: <validation logic>
- **Edge Cases**: <edge case handling>
- **Error Handling**: <error scenarios>

### FR-002: <Feature Name>

- ...

## API Contract

| Operation | Input | Output | Description |
|-----------|-------|--------|-------------|
| `<name>`  | ...   | ...    | ...         |

## Integration Points

- **3rd Party**: <service name, purpose>
- **Internal**: <service name, purpose>

## Non-functional Requirements (Detailed)

- Performance: <response time, throughput>
- Security: <auth, encryption, compliance>
- SLA: <availability, uptime>

## Test Scenarios / QA Checklist

- [ ] <test scenario with expected result>

## Assumptions & Constraints

- <assumption or constraint>

## Glossary

- **Term**: <definition>

## Reference

- PRD: <link to root PRD.md>
```

### README.md

```markdown
# <project-name>

> One-liner: what this project does and who it's for.

## Prerequisites

- Node 20+
- <other dependencies>

## Quick Start

```bash
git clone ...
cd <project>
npm install
npm run dev
```

## Architecture

<High-level diagram or link to full docs>

## Project Structure

```
packages/
├── feature-a/
│   └── FRD.md        # feature specs
├── feature-b/
│   └── FRD.md        # feature specs
└── ...
```

## Available Scripts


| Command         | Description          |
| ----------------- | ---------------------- |
| `npm run dev`   | Start development    |
| `npm run build` | Build for production |
| `npm test`      | Run tests            |

## Configuration

<Environment variables, config files>

## Testing

```bash
npm test
```

## Contributing

<Branching strategy, PR conventions>

## License

<License type>

```

## Definition of Done

1. PRD.md exists at root with Problem Statement, Goals, Personas, Scope, Features.
2. README.md exists at root with Quick Start, Architecture, Scripts, Testing.
3. FRD.md exists in each feature module with Functional Requirements (FR-001 IDs).
4. Documents serve correct audience (PRD=stakeholders, FRD=engineers, README=developers).
5. All modules have one-liner JSDoc docstrings.
6. All classes have descriptive JSDoc docstrings.
7. All public methods have parameter/return documentation.
8. All function signatures use type annotations.
9. Complex types use interfaces or type aliases.

## Workflow

### Step 1: Analyze Project

- List feature modules in `packages/`
- Identify public modules, classes, and functions
- Check existing docs (PRD.md / README.md / FRD.md / JSDoc / type annotations)

### Step 2: Create / Fix PRD.md (root workspace)

Write root-level PRD.md following the PRD.md template above. See `references/prd-rules.md` for rules.

### Step 3: Create / Fix FRD.md (each feature module)

For each feature module, write FRD.md following the FRD.md template above. See `references/frd-rules.md` for rules.

### Step 4: Create / Update README.md (root workspace)

Write root-level README.md following the README.md template above. See `references/readme-rules.md` for rules.

### Step 5: Add JSDoc Comments

See `references/jsdoc-rules.md` for rules and templates.

### Step 6: Add Type Annotations

See `references/type-annotation-rules.md` for rules and patterns.

## Quick Commands

```bash
# Check files without docstrings
find packages/ -name "*.ts" | while read f; do
    head -1 "$f" | grep -q '^/\*\*' || echo "NO DOCSTRING: $f"
done

# Run TypeScript type checker
npx tsc --noEmit
```

## Common Mistakes

- PRD contains SQL schema or API details → move to FRD.
- FRD without acceptance criteria → add testable conditions per FR.
- README = essay 10 pages → keep concise, link to other docs.
- One document for all audiences → split by audience.
- Documents "write & forget" → review each sprint/release.
- FRD in root instead of feature module → FRD belongs with the feature code.
- Missing module docstrings → every file needs a one-liner at the top.
- Incomplete parameter documentation → all parameters must be documented.
- Using @ts-ignore without reason → fix the root cause instead of suppressing errors.
