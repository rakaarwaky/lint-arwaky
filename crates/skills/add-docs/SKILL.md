---
name: add-docs
description: Adds docstrings, doc comments, JSDoc and types. Use when documenting Python, Rust, TS, PRD, FRD.
metadata:
  tags:
    [
      python,
      rust,
      typescript,
      docs,
      docstring,
      doc-comments,
      jsdoc,
      type-hints,
      prd,
      frd,
      readme,
      pep257,
    ]
  triggers:
    - "add docs python"
    - "add docstring python"
    - "add type hints python"
    - "add prd python"
    - "add frd python"
    - "add package readme python"
    - "add docs rust"
    - "add crate readme rust"
    - "add prd rust"
    - "add frd rust"
    - "add doc comments rust"
    - "document public api rust"
    - "add docs typescript"
    - "add jsdoc typescript"
    - "add type hints typescript"
    - "add prd typescript"
    - "add frd typescript"
    - "add package readme typescript"
  dependencies: []
  related:
    - cleanup-consolidate
    - fix-bypass
    - lint-arwaky
---
# add-docs

## Purpose

Add documentation at correct locations following project conventions — crate/module-level documentation plus native doc comments (`"""` docstrings, `///` doc comments, JSDoc):

- `PRD.md` — stakeholder alignment (Problem Statement / Goals & Success Metrics / User Personas / Scope / Feature Requirements / Non-functional Requirements).
- `FRD.md` — engineering specs (Functional Requirements with IDs / API Contract / Integration Points / Test Scenarios).
- `README.md` — developer onboarding (Quick Start / Architecture / Project Structure / Available Commands / Configuration / Testing / Contributing).
- Doc comments on all public items, in the language's native form.

## Document Location Matrix

| Document  | Location                     | Audience                     | Focus                | Length    |
| --------- | ---------------------------- | ---------------------------- | -------------------- | --------- |
| PRD.md    | Root workspace               | Stakeholder, PM, Design, Eng | _What_ & _Why_       | 1-2 pages |
| README.md | Root workspace               | Developer (new/existing)     | _How to use/run_     | 1-2 pages |
| FRD.md    | Each feature module/crate/pkg | Engineer, QA, Tech Lead      | _How_ (functionally) | 2-5 pages |

## Rules

- **PRD.md** = Product Requirements Document — **1 per project root** — describes **WHAT** and **WHY** for stakeholders.
- **README.md** = Developer onboarding — **1 per project root** — describes **HOW TO USE/RUN** for developers.
- **FRD.md** = Functional Requirements Document — **1 per feature module** (Python/TypeScript) / **1 per feature crate** (Rust) — describes **HOW** (functionally) for engineers.
- Relationship: **PRD (what/why) → FRD (how) → README (how to use)**. Each serves a different audience.
- Doc comments MUST explain "what" and "why", not "how" (code shows how).
- **Python:** all public classes and functions MUST have docstrings (PEP 257).
- **Rust:** all public structs and methods MUST have `///` doc comments (visible in `cargo doc`); example code in doc comments MUST be valid Rust.
- **TypeScript:** all public classes and methods MUST have JSDoc docstrings.

### When to Use

- New module/crate/package has no `PRD.md`, `FRD.md`, or `README.md`.
- Documents are conflated (wrong audience for wrong doc) — split them.
- Public items lack doc comments, or the doc-comment output (`cargo doc`, typed API surface) is incomplete.
- The user asks to document a module/crate/package or add docs.

### The Fundamental Question

> **"Can a stakeholder understand this project's purpose in 30 seconds?"** — If no → Add PRD.md (what/why).
> **"Can an engineer implement this from the spec?"** — If no → Add FRD.md (how).
> **"Can a developer clone → build → run in < 10 minutes?"** — If no → Add README.md (how to use).

## Detection Patterns (Rust)

### Missing docs (Create)

```
project-root/
├── PRD.md          # stakeholder alignment (what/why) — 1 per project
├── README.md       # developer onboarding (how to use) — 1 per project
├── crates/
│   ├── feature-a/
│   │   ├── src/
│   │   └── FRD.md  # engineering specs (how) — per feature crate
│   └── feature-b/
│       ├── src/
│       └── FRD.md  # engineering specs (how) — per feature crate
```

### Missing Doc Comments

```rust
// [BAD] no doc comment — invisible to cargo doc
// PURPOSE explain file in one sentence
pub struct ImportOrchestrator {
    mandatory: Arc<dyn IImportMandatoryProtocol>,
}

// [OK] /// doc comment — appears in cargo doc
/// Orchestrates <name-feature>.
///
/// Execution order:
/// 1.
/// 2.
/// 3.
/// 4.
pub struct ImportOrchestrator {
    mandatory: Arc<dyn IImportMandatoryProtocol>,
}
```

## Templates

### PRD.md

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

### FRD.md

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
```

### README.md

```markdown
# <project-name>

> One-liner: what this project does and who it's for.

## Prerequisites

- <see per-language table below>
- <other dependencies>

## Quick Start

<see per-language table below>

## Architecture

<High-level diagram or link to full docs>

## Project Structure

<see per-language table below>

## Available Scripts / Available Commands

<see per-language table below — Rust titles this section "Available Commands", Python and TypeScript title it "Available Scripts">

## Configuration

<Environment variables, config files>

## Testing

<per-language test command>

## Contributing

<Branching strategy, PR conventions>

## License

<License type>
```

Per-language README variables:

| Language   | Prerequisites | Quick Start                                                             | Project Structure                                            | Section title         | Command table rows                                                                       |
| ---------- | ------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------ | --------------------- | ---------------------------------------------------------------------------------------- |
| Python     | Python 3.10+  | `git clone ...` → `cd <project>` → `pip install -e .` → `python -m <package>` | `modules/feature-a/FRD.md`, `modules/feature-b/FRD.md`       | Available Scripts     | `python -m <package>` = Run the package; `pytest` = Run tests; `ruff check .` = Lint code |
| Rust       | Rust 1.70+    | `git clone ...` → `cd crates/<name>` → `cargo build` → `cargo run`            | `src/lib.rs`, `src/modules/`                                 | Available Commands    | `cargo build` = Build the crate; `cargo test` = Run tests; `cargo run` = Run the binary   |
| TypeScript | Node 20+      | `git clone ...` → `cd <project>` → `npm install` → `npm run dev`              | `packages/feature-a/FRD.md`, `packages/feature-b/FRD.md`     | Available Scripts     | `npm run dev` = Start development; `npm run build` = Build for production; `npm test` = Run tests |

Rust templates say `<crate-name>` / "this crate" where the shared template above says `<project-name>` / "this project", and `Build the crate` for the `cargo build` row.

Per-language `## Project Structure` block for the README:

Python:

```
modules/
├── feature-a/
│   └── FRD.md        # feature specs
├── feature-b/
│   └── FRD.md        # feature specs
└── ...
```

Rust:

```
src/
├── lib.rs
├── modules/
└── ...
```

TypeScript:

```
packages/
├── feature-a/
│   └── FRD.md        # feature specs
├── feature-b/
│   └── FRD.md        # feature specs
└── ...
```

## Docstring Conventions

### Python (PEP 257)

Docstrings on all public classes and functions, explaining what/why, with `Args`/`Returns` sections. Verify importability with `python -c "import <module>"`.

### Rust (`///`)

For each public struct and method: convert `//` comments to `///` doc comments, add summary line, add explanation if >10 lines of logic, add `# Example` block if applicable. Use type annotations for all function parameters and return types, traits for abstract behavior, enums for sum types.

```rust
/// Taxonomy value objects for import rules.

/// Value object representing an import rule with pattern and message.
pub struct ImportRuleVO {
    pattern: String,
    message: String,
}

/// Check if path matches the import rule.
///
/// # Arguments
///
/// * `path` - File path to check
///
/// # Returns
///
/// `true` if path matches the rule
///
/// # Errors
///
/// Returns `Err` if path is empty
///
/// # Example
///
/// ```
/// let rule = ImportRuleVO::new("*.test.ts", "Test file");
/// assert!(rule.check("foo.test.ts"));
/// ```
pub fn check(&self, path: &str) -> Result<bool, Error> {
    // ...
}
```

A crate-level / module-level `///` comment (the `/// Taxonomy value objects for import rules.` line above) states in one sentence what the file is for. Annotated signature example:

```rust
pub fn validate(&self, data: &HashMap<String, Value>) -> Result<(bool, String), Error> {
    // ...
}
```

### TypeScript (JSDoc / TSDoc)

Every module gets a one-liner `/** */` docstring at the top; every class a descriptive docstring; every public method parameter/return documentation (`@param`/`@returns`). All function signatures use type annotations; complex types use interfaces or type aliases.

## Workflow

1. **Analyze** — list feature modules/crates/packages, identify public modules, classes, structs, and functions, check existing docs (PRD.md / README.md / FRD.md / doc comments / type annotations). Rust: list files in `crates/<name>/src/`.
2. **Create / Fix PRD.md** (project root) per the template above. It MUST contain: Problem Statement, Goals & Success Metrics, User Personas, Scope, Feature Requirements (prioritized), Non-functional Requirements (high-level). Write for non-engineers; avoid technical jargon; use acceptance criteria.
3. **Create / Fix FRD.md** (each feature module/crate) per the template above. It MUST contain: Reference to PRD, System Overview, Functional Requirements with unique IDs (FR-001, FR-002), API Contract, Integration Points, Test Scenarios. Use precise, unambiguous language; include edge cases and error handling.
4. **Create / Update README.md** (project root) per the template above. It MUST contain: Quick Start (clone → build → run in < 10 minutes), Architecture, Project Structure, Available Scripts/Commands, Configuration, Testing, Contributing. Keep concise; link to PRD/FRD for details; update when setup changes.
5. **Add doc comments** to all public items using the language convention above.
6. **Add type annotations** to all signatures.
7. **Verify** with the Quick Commands below.

## Quick Commands

```bash
# Rust — check files without doc comments, then build docs
find crates/ -name "*.rs" | while read f; do
    head -1 "$f" | grep -q '^///' || echo "NO DOC COMMENT: $f"
done
cargo doc --open
```

```bash
# TypeScript — check files without docstrings, then type check
find packages/ -name "*.ts" | while read f; do
    head -1 "$f" | grep -q '^/\*\*' || echo "NO DOCSTRING: $f"
done
npx tsc --noEmit
```

```bash
# Python — verify the package imports
python -c "import <module>"
```

## Checklist

Shared (all languages):

- [ ] PRD.md at project root with Problem Statement, Goals, Personas, Scope, Features.
- [ ] README.md at project root with Quick Start, Architecture, Commands/Scripts, Testing.
- [ ] FRD.md in each feature module/crate with Functional Requirements (FR-001 IDs), API Contract.
- [ ] Documents serve correct audience (PRD=stakeholders, FRD=engineers, README=developers).

Python:

- [ ] All public classes have docstrings.
- [ ] All public functions have docstrings with Args/Returns.

Rust:

- [ ] All public structs have `///` doc comments.
- [ ] All public methods have `///` doc comments with Args/Returns/Errors.
- [ ] All function signatures use type annotations.
- [ ] Example code in doc comments is valid Rust.

TypeScript (Definition of Done):

- [ ] All modules have one-liner JSDoc docstrings.
- [ ] All classes have descriptive JSDoc docstrings.
- [ ] All public methods have parameter/return documentation.
- [ ] All function signatures use type annotations.
- [ ] Complex types use interfaces or type aliases.

## Common Mistakes (AVOID)

- ❌ **PRD contains SQL schema or API details** — move to FRD.
- ❌ **FRD without acceptance criteria** — add testable conditions per FR.
- ❌ **README = essay 10 pages** — keep concise, link to other docs.
- ❌ **One document for all audiences** — split by audience.
- ❌ **Documents "write & forget"** — review each sprint/release.
- ❌ **FRD in root instead of feature module** — FRD belongs with the feature code.
- ❌ **Missing module docstrings** — every file needs a one-liner at the top.
- ❌ **Incomplete parameter documentation** — all parameters must be documented.
- ❌ **Missing doc comments** (Rust) — every public item needs `///`; `//` is invisible to `cargo doc`.
- ❌ **Using `@ts-ignore` without reason** (TypeScript) — fix the root cause instead of suppressing errors.
