---
name: add-docs-rust
description: "Add proper doc comments, type annotations, and crate-level PRD.md/FRD.md/README.md to Rust crates following project conventions."
metadata:
    tags: [rust, docs, doc-comments, prd, frd, readme]
    triggers:
        - "add docs rust"
        - "add crate readme rust"
        - "add prd rust"
        - "add frd rust"
        - "add doc comments rust"
        - "document public api rust"
    dependencies: []
    related:
        - lint-arwaky-rust
        - cleanup-files-rust
---
# add-docs-rust

## Rules

- Every crate directory MUST contain THREE crate-level docs: `PRD.md`, `FRD.md`, and `README.md`.
- **PRD.md** = Product Requirements Document — describes **WHAT** and **WHY** for stakeholders, PM, Design, and Eng alignment.
- **FRD.md** = Functional Requirements Document — describes **HOW** (functionally) for engineers, QA, and Tech Lead.
- **README.md** = Developer onboarding — describes **HOW TO USE/RUN** for developers.
- Relationship: **PRD (what/why) → FRD (how) → README (how to use)**. Each serves a different audience.
- All public structs and methods MUST have `///` doc comments (visible in `cargo doc`).
- Doc comments MUST explain "what" and "why", not "how" (code shows how).
- Example code in doc comments MUST be valid Rust.

## Purpose

Add crate-level documentation and `///` doc comments:

- `PRD.md` — stakeholder alignment (Problem Statement / Goals & Success Metrics / User Personas / Scope / Feature Requirements / Non-functional Requirements).
- `FRD.md` — engineering specs (Functional Requirements with IDs / Data Model / API Contract / Integration Points / Test Scenarios).
- `README.md` — developer onboarding (Quick Start / Architecture / Project Structure / Available Commands / Configuration / Testing / Contributing).
- `///` doc comments on all public items for `cargo doc` visibility.

## When to Use

- New crate has no `PRD.md`, `FRD.md`, or `README.md`.
- Documents are conflated (wrong audience for wrong doc) — split them.
- Public structs/methods lack `///` doc comments.
- `cargo doc` output is incomplete or missing.
- User asks to document the crate or add docs.

## The Fundamental Question

> **"Can a stakeholder understand this crate's purpose in 30 seconds?"**

If no -> **Add PRD.md (what/why).**

> **"Can an engineer implement this from the spec?"**

If no -> **Add FRD.md (how).**

> **"Can a developer clone → build → run in < 10 minutes?"**

If no -> **Add README.md (how to use).**

## Document Audience Matrix

| Document  | Audience                     | Focus                  | Length    |
| --------- | ---------------------------- | ---------------------- | --------- |
| PRD.md    | Stakeholder, PM, Design, Eng | *What* & *Why*     | 1-2 pages |
| FRD.md    | Engineer, QA, Tech Lead      | *How* (functionally) | 2-5 pages |
| README.md | Developer (new/existing)     | *How to use/run*     | 1-2 pages |

## Detection Patterns

### Missing PRD.md / FRD.md / README.md (Create)

```
crates/<name-folder>/
├── src/
│   ├── lib.rs
│   └── ...
├── tests/
├── PRD.md        # stakeholder alignment (what/why)
├── FRD.md        # engineering specs (how)
└── README.md     # developer onboarding (how to use)
```

### Missing Doc Comments (Add)

```rust
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

## PRD.md Template (STAKEHOLDER ALIGNMENT — what/why)

```markdown
# PRD — <crate-name>

> Product Requirements Document. Describes WHAT this crate does and WHY.
> Audience: Stakeholders, PM, Design, Engineering leads.

## Problem Statement
<One paragraph: what problem does this crate solve?>

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

## FRD.md Template (ENGINEERING SPECS — how)

```markdown
# FRD — <crate-name>

> Functional Requirements Document. Describes HOW this crate works functionally.
> Audience: Engineers, QA, Tech Lead.

## Reference
- PRD: <link to PRD.md>

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

## README.md Template (DEVELOPER ONBOARDING — how to use)

```markdown
# <crate-name>

> One-liner: what this crate does and who it's for.

## Prerequisites
- Rust 1.70+
- <other dependencies>

## Quick Start
```bash
git clone ...
cd crates/<name>
cargo build
cargo run
```

## Architecture

<High-level diagram or link to full docs>

## Project Structure

```
src/
├── lib.rs
├── modules/
└── ...
```

## Available Commands

| Command         | Description     |
| --------------- | --------------- |
| `cargo build` | Build the crate |
| `cargo test`  | Run tests       |
| `cargo run`   | Run the binary  |

## Configuration

<Environment variables, config files>

## Testing

```bash
cargo test
```

## Contributing

<Branching strategy, PR conventions>

## License

<License type>
```

## Workflow

### Step 1: Analyze Crate

- List files in `crates/<name>/src/`
- Identify public structs and methods
- Check existing docs (PRD.md / FRD.md / README.md / `///` comments)

### Step 2: Create / Fix PRD.md (stakeholder alignment)

Write crate-level PRD.md following the PRD template. It MUST contain:

1. Problem Statement
2. Goals & Success Metrics
3. User Personas
4. Scope
5. Feature Requirements (prioritized)
6. Non-functional Requirements (high-level)

Write for non-engineers. Avoid technical jargon. Use acceptance criteria.

### Step 3: Create / Fix FRD.md (engineering specs)

Write crate-level FRD.md following the FRD template. It MUST contain:

1. Reference to PRD
2. System Overview
3. Functional Requirements (with unique IDs: FR-001, FR-002)
4. Data Model
5. API Contract
6. Integration Points
7. Test Scenarios

Use precise, unambiguous language. Include edge cases and error handling.

### Step 4: Create / Update README.md (developer onboarding)

Write README.md following the README template. It MUST contain:

1. Quick Start (clone → build → run in < 10 minutes)
2. Architecture (high-level)
3. Project Structure
4. Available Commands
5. Configuration
6. Testing
7. Contributing

Keep concise. Link to PRD/FRD for details. Update when setup changes.

### Step 5: Add Doc Comments

For each public struct and method:

1. Convert `//` comments to `///` doc comments
2. Add summary line
3. Add explanation if >10 lines of logic
4. Add `# Example` block if applicable

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

### Step 6: Add Type Annotations

- Use Rust type annotations for all function parameters and return types
- Use traits for abstract behavior
- Use enums for sum types

```rust
pub fn validate(&self, data: &HashMap<String, Value>) -> Result<(bool, String), Error> {
    // ...
}
```

## Verification Checklist

- [ ] PRD.md exists with Problem Statement, Goals, Personas, Scope, Features
- [ ] FRD.md exists with Functional Requirements (FR-001 IDs), Data Model, API Contract
- [ ] README.md exists with Quick Start, Architecture, Commands, Testing
- [ ] Documents serve correct audience (PRD=stakeholders, FRD=engineers, README=developers)
- [ ] All public structs have `///` doc comments
- [ ] All public methods have `///` doc comments with Args/Returns/Errors
- [ ] All function signatures use type annotations
- [ ] Example code in doc comments is valid Rust

## Quick Commands

```bash
# Check files without doc comments
find crates/ -name "*.rs" | while read f; do
    head -1 "$f" | grep -q '^///' || echo "NO DOC COMMENT: $f"
done

# Run cargo doc
cargo doc --open
```

## Common Mistakes (AVOID)

- ❌ **PRD contains SQL schema or API details** — move to FRD
- ❌ **FRD without acceptance criteria** — add testable conditions per FR
- ❌ **README = essay 10 pages** — keep concise, link to other docs
- ❌ **One document for all audiences** — split by audience
- ❌ **Documents "write & forget"** — review each sprint/release
- ❌ **Missing doc comments**: Every public item needs `///` doc comment
- ❌ **Using `//` instead of `///`**: Use `///` for cargo doc visibility
- ❌ **Incomplete parameter documentation**: All parameters must be documented
