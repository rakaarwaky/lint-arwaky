---
name: add-docs-python
description: "Add proper docstrings, type hints, and crate-level PRD.md/FRD.md/README.md to Python packages following PEP 257 and project conventions."
version: 1.3.0
category: documentation
tags: [python, docs, docstring, type-hints, prd, frd, readme, pep257]
triggers:
  - "add docs python"
  - "add docstring python"
  - "add type hints python"
  - "add prd python"
  - "add frd python"
  - "add package readme python"
dependencies: []
related:
  - cleanup-files-python
  - consolidate-files-python
---

# add-docs-python

## Rules

- **PRD.md** and **README.md** live in the ROOT workspace only (project-level docs).
- **FRD.md** lives in each feature module directory (feature-level specs).
- **PRD.md** = Product Requirements Document — describes **WHAT** and **WHY** for stakeholders, PM, Design, and Eng alignment.
- **FRD.md** = Functional Requirements Document — describes **HOW** (functionally) for engineers, QA, and Tech Lead.
- **README.md** = Developer onboarding — describes **HOW TO USE/RUN** for developers.
- Relationship: **PRD (what/why) → FRD (how) → README (how to use)**. Each serves a different audience.
- All public modules, classes, and functions MUST have docstrings (PEP 257) and type hints.
- Docstrings MUST explain "what" and "why", not "how" (code shows how).

## Purpose

Add documentation at correct locations:
- Root workspace:
  - `PRD.md` — stakeholder alignment (Problem Statement / Goals & Success Metrics / User Personas / Scope / Feature Requirements / Non-functional Requirements).
  - `README.md` — developer onboarding (Quick Start / Architecture / Project Structure / Available Scripts / Configuration / Testing / Contributing).
- Each feature module:
  - `FRD.md` — engineering specs (Functional Requirements with IDs / Data Model / API Contract / Integration Points / Test Scenarios).
- PEP 257 docstrings + type hints on all public items.

## When to Use

- Root workspace has no `PRD.md` or `README.md`.
- Feature module has no `FRD.md`.
- Documents are conflated (wrong audience for wrong doc) — split them.
- Public modules/classes/functions lack docstrings or type hints.
- User asks to document the package or add docs.

## The Fundamental Question

> **"Can a stakeholder understand this project's purpose in 30 seconds?"**

If no -> **Add PRD.md at root (what/why).**

> **"Can an engineer implement this feature from the spec?"**

If no -> **Add FRD.md in feature module (how).**

> **"Can a developer clone → run → contribute in < 10 minutes?"**

If no -> **Add README.md at root (how to use).**

## Document Location Matrix

| Document | Location | Audience | Focus |
|----------|----------|----------|-------|
| PRD.md | Root workspace | Stakeholder, PM, Design, Eng | *What* & *Why* |
| README.md | Root workspace | Developer (new/existing) | *How to use/run* |
| FRD.md | Each feature module | Engineer, QA, Tech Lead | *How* (functionally) |

## Detection Patterns

### Missing PRD.md / README.md (Create at Root)

```
<workspace-root>/
├── modules/
│   ├── feature-a/
│   │   ├── src/
│   │   ├── tests/
│   │   └── FRD.md        # feature-level engineering specs
│   └── feature-b/
│       ├── src/
│       ├── tests/
│       └── FRD.md        # feature-level engineering specs
├── PRD.md                # project-level stakeholder alignment
└── README.md             # project-level developer onboarding
```

### Missing FRD.md (Create in Feature Module)

```
modules/<feature-name>/
├── src/
│   ├── __init__.py
│   └── ...
├── tests/
└── FRD.md                # feature-level engineering specs
```

### Missing Docstrings / Type Hints (Add)

```python
# PURPOSE explain file in one sentence
class ImportRuleVO:
    ...

# [OK] docstring + type hints
class ImportRuleVO:
    """Value object representing an import rule with pattern and message."""
```

## PRD.md Template (ROOT — stakeholder alignment)

```markdown
# PRD — <project-name>

> Product Requirements Document. Describes WHAT this project does and WHY.
> Audience: Stakeholders, PM, Design, Engineering leads.

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

## FRD.md Template (FEATURE MODULE — engineering specs)

```markdown
# FRD — <feature-name>

> Functional Requirements Document. Describes HOW this feature works functionally.
> Audience: Engineers, QA, Tech Lead.

## Reference
- PRD: <link to root PRD.md>

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

## Data Model / Entity Relationship
<Entity diagram or data structure definitions>

## API Contract
| Endpoint | Method | Payload | Response |
|----------|--------|---------|----------|
| `/path` | GET | - | `{...}` |

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
```

## README.md Template (ROOT — developer onboarding)

```markdown
# <project-name>

> One-liner: what this project does and who it's for.

## Prerequisites
- Python 3.10+
- <other dependencies>

## Quick Start
```bash
git clone ...
cd <project>
pip install -e .
python -m <package>
```

## Architecture
<High-level diagram or link to full docs>

## Project Structure
```
modules/
├── feature-a/
│   └── FRD.md        # feature specs
├── feature-b/
│   └── FRD.md        # feature specs
└── ...
```

## Available Scripts
| Command | Description |
|---------|-------------|
| `python -m <package>` | Run the package |
| `pytest` | Run tests |
| `ruff check .` | Lint code |

## Configuration
<Environment variables, config files>

## Testing
```bash
pytest
```

## Contributing
<Branching strategy, PR conventions>

## License
<License type>
```

## Workflow

### Step 1: Analyze Project

- List feature modules in `modules/`
- Identify public modules, classes, and functions
- Check existing docs (PRD.md / README.md / FRD.md / docstrings / type hints)

### Step 2: Create / Fix PRD.md (root workspace)

Write root-level PRD.md following the PRD template. It MUST contain:

1. Problem Statement
2. Goals & Success Metrics
3. User Personas
4. Scope
5. Feature Requirements (prioritized)
6. Non-functional Requirements (high-level)

Write for non-engineers. Avoid technical jargon. Use acceptance criteria.

### Step 3: Create / Fix FRD.md (each feature module)

For each feature module, write FRD.md following the FRD template. It MUST contain:

1. Reference to root PRD
2. System Overview
3. Functional Requirements (with unique IDs: FR-001, FR-002)
4. Data Model
5. API Contract
6. Integration Points
7. Test Scenarios

Use precise, unambiguous language. Include edge cases and error handling.

### Step 4: Create / Update README.md (root workspace)

Write root-level README.md following the README template. It MUST contain:

1. Quick Start (clone → run in < 10 minutes)
2. Architecture (high-level)
3. Project Structure (show FRD.md locations)
4. Available Scripts
5. Configuration
6. Testing
7. Contributing

Keep concise. Link to PRD/FRD for details. Update when setup changes.

### Step 5: Add Docstrings (PEP 257)

- **Module docstrings**: One-liner at top of file describing module purpose
- **Class docstrings**: One-liner describing class purpose and behavior
- **Function/method docstrings**: Describe purpose, parameters, return values, and exceptions

```python
"""Taxonomy value objects for import rules."""


class ImportRuleVO:
    """Value object representing an import rule with pattern and message."""


def check(self, path: str) -> bool:
    """Check if path matches the import rule.

    Args:
        path: File path to check

    Returns:
        True if path matches the rule

    Raises:
        ValueError: If path is empty
    """
```

### Step 6: Add Type Hints

- Use Python 3.5+ type hint syntax (`def foo(x: int) -> str:`)
- Import `from __future__ import annotations` for forward references
- Use `typing` module for complex types (`List`, `Dict`, `Optional`, `Union`)

```python
def validate(self, data: dict[str, Any]) -> tuple[bool, str]:
    """Validate data against the import rule."""
```

## Verification Checklist

- [ ] PRD.md exists at root with Problem Statement, Goals, Personas, Scope, Features
- [ ] README.md exists at root with Quick Start, Architecture, Scripts, Testing
- [ ] FRD.md exists in each feature module with Functional Requirements (FR-001 IDs)
- [ ] Documents serve correct audience (PRD=stakeholders, FRD=engineers, README=developers)
- [ ] All modules have one-liner docstrings
- [ ] All classes have descriptive docstrings
- [ ] All public functions have parameter/return documentation
- [ ] All function signatures use type hints
- [ ] Forward references use string quotes or `__future__.annotations`
- [ ] Complex types use `typing` module

## Quick Commands

```bash
# Check files without docstrings
find modules/ -name "*.py" | while read f; do
    head -1 "$f" | grep -q '^"""' || echo "NO DOCSTRING: $f"
done

# Run mypy for type checking
python -m mypy modules/ --ignore-missing-imports
```

## Common Mistakes (AVOID)

- ❌ **PRD contains SQL schema or API details** — move to FRD
- ❌ **FRD without acceptance criteria** — add testable conditions per FR
- ❌ **README = essay 10 pages** — keep concise, link to other docs
- ❌ **One document for all audiences** — split by audience
- ❌ **Documents "write & forget"** — review each sprint/release
- ❌ **FRD in root instead of feature module** — FRD belongs with the feature code
- ❌ **Missing module docstrings**: Every file needs a one-liner at the top
- ❌ **Incomplete parameter documentation**: All parameters must be documented
- ❌ **Using type: ignore without reason**: Fix the root cause instead of suppressing errors
