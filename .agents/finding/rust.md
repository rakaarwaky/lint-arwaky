# Skills: Rust (12 skills, 12 files)

This document contains all rust skills from `.agents/skills/`.

## Table of Contents

- [add-docs-rust](#add-docs-rust)
- [cleanup-consolidate-rust](#cleanup-consolidate-rust)
- [create-agent-rust](#create-agent-rust)
- [create-capabilities-rust](#create-capabilities-rust)
- [create-contract-rust](#create-contract-rust)
- [create-root-rust](#create-root-rust)
- [create-surface-rust](#create-surface-rust)
- [create-taxonomy-rust](#create-taxonomy-rust)
- [create-test-rust](#create-test-rust)
- [create-utility-rust](#create-utility-rust)
- [fix-bypass-rust](#fix-bypass-rust)
- [lint-arwaky-rust](#lint-arwaky-rust)

---

# add-docs-rust

**Files:** 1

## File List

- [.agents/skills/add-docs-rust/SKILL.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/add-docs-rust/SKILL.md)

---

## File: .agents/skills/add-docs-rust/SKILL.md

```markdown
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
    - cleanup-consolidate-rust
---
# add-docs-rust

## Rules

- **PRD.md** = Product Requirements Document — **1 per project root** — describes **WHAT** and **WHY** for stakeholders.
- **README.md** = Developer onboarding — **1 per project root** — describes **HOW TO USE/RUN** for developers.
- **FRD.md** = Functional Requirements Document — **1 per feature crate** — describes **HOW** (functionally) for engineers.
- Relationship: **PRD (what/why) → FRD (how) → README (how to use)**. Each serves a different audience.
- All public structs and methods MUST have `///` doc comments (visible in `cargo doc`).
- Doc comments MUST explain "what" and "why", not "how" (code shows how).
- Example code in doc comments MUST be valid Rust.

## Purpose

Add crate-level documentation and `///` doc comments:

- `PRD.md` — stakeholder alignment (Problem Statement / Goals & Success Metrics / User Personas / Scope / Feature Requirements / Non-functional Requirements).
- `FRD.md` — engineering specs (Functional Requirements with IDs / API Contract / Integration Points / Test Scenarios).
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


| Document  | Audience                     | Focus                | Length    |
| ----------- | ------------------------------ | ---------------------- | ----------- |
| PRD.md    | Stakeholder, PM, Design, Eng | _What_ & _Why_       | 1-2 pages |
| FRD.md    | Engineer, QA, Tech Lead      | _How_ (functionally) | 2-5 pages |
| README.md | Developer (new/existing)     | _How to use/run_     | 1-2 pages |

## Detection Patterns

### Missing docs (Create)

Project root:
``` `
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
``` `

### Missing Doc Comments 

``` `rust
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
``` `

## PRD.md 

``` `markdown
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
``` `

## FRD.md Template 

``` `markdown
# FRD — <feature-name>

> Functional Requirements Document. Describes HOW this feature works functionally.
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
``` `

## README.md 

``` `markdown
# <crate-name>

> One-liner: what this crate does and who it's for.

## Prerequisites

- Rust 1.70+
- <other dependencies>

## Quick Start

``` `bash
git clone ...
cd crates/<name>
cargo build
cargo run
``` `

``` ``

## Architecture

<High-level diagram or link to full docs>

## Project Structure

``` `
src/
├── lib.rs
├── modules/
└── ...
``` `

## Available Commands

| Command       | Description     |
| ------------- | --------------- |
| `cargo build` | Build the crate |
| `cargo test`  | Run tests       |
| `cargo run`   | Run the binary  |

## Configuration

<Environment variables, config files>

## Testing

``` `bash
cargo test
``` `

## Contributing

<Branching strategy, PR conventions>

## License

<License type>
``` `

## Workflow

### Step 1: Analyze Crate

- List files in `crates/<name>/src/`
- Identify public structs and methods
- Check existing docs (PRD.md / FRD.md / README.md / `///` comments)

### Step 2: Create / Fix PRD.md (stakeholder alignment)

Write project-root PRD.md following the PRD template. It MUST contain:

1. Problem Statement
2. Goals & Success Metrics
3. User Personas
4. Scope
5. Feature Requirements (prioritized)
6. Non-functional Requirements (high-level)

Write for non-engineers. Avoid technical jargon. Use acceptance criteria.

### Step 3: Create / Fix FRD.md (engineering specs)

For each feature crate, write FRD.md following the FRD template. It MUST contain:

1. Reference to PRD
2. System Overview
3. Functional Requirements (with unique IDs: FR-001, FR-002)
4. API Contract
5. Integration Points
6. Test Scenarios

Use precise, unambiguous language. Include edge cases and error handling.

### Step 4: Create / Update README.md (developer onboarding)

Write project-root README.md following the README template. It MUST contain:

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

``` ``rust
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
/// ``` `
/// let rule = ImportRuleVO::new("*.test.ts", "Test file");
/// assert!(rule.check("foo.test.ts"));
/// ``` `
pub fn check(&self, path: &str) -> Result<bool, Error> {
    // ...
}
``` ``

### Step 6: Add Type Annotations

- Use Rust type annotations for all function parameters and return types
- Use traits for abstract behavior
- Use enums for sum types

``` `rust
pub fn validate(&self, data: &HashMap<String, Value>) -> Result<(bool, String), Error> {
    // ...
}
``` `

## Verification Checklist

- [ ]  PRD.md at project root with Problem Statement, Goals, Personas, Scope, Features
- [ ]  README.md at project root with Quick Start, Architecture, Commands, Testing
- [ ]  FRD.md in each feature crate with Functional Requirements (FR-001 IDs), API Contract
- [ ]  Documents serve correct audience (PRD=stakeholders, FRD=engineers, README=developers)
- [ ]  All public structs have `///` doc comments
- [ ]  All public methods have `///` doc comments with Args/Returns/Errors
- [ ]  All function signatures use type annotations
- [ ]  Example code in doc comments is valid Rust

## Quick Commands

``` `bash
# Check files without doc comments
find crates/ -name "*.rs" | while read f; do
    head -1 "$f" | grep -q '^///' || echo "NO DOC COMMENT: $f"
done

# Run cargo doc
cargo doc --open
``` `

## Common Mistakes (AVOID)

- ❌ **PRD contains SQL schema or API details** — move to FRD
- ❌ **FRD without acceptance criteria** — add testable conditions per FR
- ❌ **README = essay 10 pages** — keep concise, link to other docs
- ❌ **One document for all audiences** — split by audience
- ❌ **Documents "write & forget"** — review each sprint/release
- ❌ **Missing doc comments**: Every public item needs `///` doc comment
- ❌ **Using `//` instead of `///`**: Use `///` for cargo doc visibility
- ❌ **Incomplete parameter documentation**: All parameters must be documented
```

---

# cleanup-consolidate-rust

**Files:** 1

## File List

- [.agents/skills/cleanup-consolidate-rust/SKILL.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/cleanup-consolidate-rust/SKILL.md)

---

## File: .agents/skills/cleanup-consolidate-rust/SKILL.md

```markdown
---
name: cleanup-consolidate-rust
description: "Find and remove dead code, unused files, stubs, thin wrappers, and duplicates across Rust crates, then merge overlapping files into single cohesive modules."
metadata:
  tags:
    [
      rust,
      cleanup,
      consolidation,
      bloat,
      stubs,
      thin-wrappers,
      dead-code,
      orphan,
      unused-files,
      merge,
      deduplication,
      single-file,
      single-struct,
      aes,
    ]
  triggers:
    - "cleanup rust"
    - "clean bloat rust"
    - "remove stubs rust"
    - "remove thin wrappers rust"
    - "find unused files rust"
    - "find dead code rust"
    - "remove dead code rust"
    - "cleanup crate rust"
    - "merge two files into one"
    - "combine two impl files"
    - "consolidate files"
    - "merge capabilities files"
    - "merge agent files"
    - "merge overlap rust"
    - "deduplicate modules rust"
  dependencies: []
  related:
    - add-docs-rust
    - create-capabilities-rust
    - create-agent-rust
---

# cleanup-consolidate-rust

## Purpose

Unified Rust codebase cleanup skill combining **dead code removal** and **file consolidation**. First find and remove dead code, unused files, stubs, thin wrappers, and duplicates. Then detect overlapping files that share the same domain and merge them into single cohesive modules. The result is a cleaner codebase with fewer files, less bloat, and maximum signal-to-noise ratio.

**CRITICAL: Two-Phase Approach** — Phase 1 removes dead code. Phase 2 merges overlapping files. Never skip Phase 1 — consolidating files with dead code wastes effort.

---

## Rules

- **Never remove real logic** — only remove code not relevant to FRD scope
- **Always update trait** — when removing methods from impl, remove from trait too
- **Always run lint after changes** — verify no compilation errors or regressions
- **Always snapshot before cleanup** — git commit or stash before any deletion
- **File with 0 inbound references** = likely unused (verify with multi-pattern check)
- **File with only re-exports** = likely bloat (consider consolidation)
- **File not referenced by any other file, test, or build script** = candidate for deletion
- **Respect `#[allow(dead_code)]`** — investigate intent before removing
- **Respect `#[cfg(...)]` gates** — code behind feature flags or test cfg is NOT dead
- **One Struct Per File** (consolidation): merge two impl files into single file with single struct
- **Target Selection**: keep file with most logic as target; move unique functions from source files into target

---

## When to Use

- After refactoring capability modules
- Before committing capability changes
- When user asks to clean bloat from a module
- After refactoring a crate (find orphaned files)
- When cleaning up accumulated dead code
- Before release (final bloat pass)
- Two impl files share the same domain and can be unified
- Multiple files implement the same concept (e.g., 7 coordinate transform files)
- Multiple files handle the same feature (e.g., cursor drawer + cursor renderer)
- Multiple adapter files for the same technology (e.g., 3 FFmpeg adapters)

---

## The Fundamental Questions

### For Cleanup (Phase 1)

Before keeping any function or file, ask:

> **"Why does this function/file need to exist?"**

| Answer | Verdict |
| ------------------------------------------------------------- | ---------- |
| "Because it was always there" | **REMOVE** |
| "Because it might be useful someday" | **REMOVE** |
| "Because it handles edge cases we don't have" | **REMOVE** |
| "Because it's required by FRD" | **KEEP** |
| "Because it's called by a method required by FRD" | **KEEP** |
| "Because it's behind a feature flag we still ship" | **KEEP** |
| "Because it's used by tests that validate FRD behavior" | **KEEP** |
| "Because a proc macro / derive generates code referencing it" | **KEEP** |
| "Because `build.rs` or integration tests reference it" | **KEEP** |

### For Consolidation (Phase 2)

> **"Do these files do the same thing or share the same domain?"**

If yes → **Merge them into 1 file**

---

## Phase 1: Dead Code Cleanup

### Detection Patterns: Function-Level Bloat

#### Thin Wrappers (Remove)

``` `rust
// ❌ Simple attribute return — direct access is simpler
fn get_something(&self, obj: &Obj) -> f64 {
    obj.attribute
}

// ❌ Simple enum comparison — comparison is already trivial
fn should_force_x(&self, hint: &ActionHint) -> bool {
    *hint == ActionHint::X
}

// ❌ Single-field delegation — no logic added
fn name(&self) -> &str {
    &self.inner.name
}
``` `

**Exception — KEEP thin wrappers when:**

- They are part of a public trait implementation (removing breaks the trait contract)
- They add documentation value (`/// Converts meters to kilometers`)
- They are the sole implementation of a trait method used polymorphically

#### Stubs (Remove)

``` `rust
// ❌ Empty implementations providing no value
fn method(&self) -> Option<()> { None }
fn method(&self) -> String { String::new() }
fn method(&self) -> Vec<Item> { vec![] }
fn method(&self) -> Result<(), Error> { Ok(()) }
fn method(&self) -> bool { false }
fn method(&self) -> i32 { 0 }
``` `

**Exception — KEEP stubs when:**

- They are required by a trait definition that external crates implement
- They are placeholder for a confirmed next-sprint FRD item (add `// TODO(FRD-XXX): implement` comment)

#### Duplicate Functions (Remove)

Same function logic in multiple capability files — keep in the file that **owns the domain logic**.

``` `rust
// ❌ In capabilities_movement.rs AND capabilities_physics.rs:
fn clamp_velocity(v: f64, max: f64) -> f64 {
    v.clamp(-max, max)
}
// KEEP in the file that owns velocity logic. Remove from the other.
``` `

**Detection:** Match on function body similarity, not just name. Two functions with different names but identical bodies are also duplicates.

#### Overengineered Patterns (Remove)

``` `rust
// ❌ Temporal enforcer, circular dependency detection, plugin registries, etc.
// if NOT in MVP → REMOVE
``` `

**3-Point Decision Test — ALL must be true to remove:**

1. ✅ The pattern is **NOT referenced** in any FRD requirement document
2. ✅ Removing it does **NOT break** any existing test (`cargo test` passes)
3. ✅ The pattern adds **>20 lines** of code for **<3 lines** of actual consumed logic

If **any** check fails → **KEEP** and add comment: `// REVIEW: candidate for removal post-MVP`

### Detection Patterns: File-Level Orphans

#### Unused Files

Files not imported, declared, or referenced by any other file in the crate:

``` `
crates/my-crate/src/capabilities_orphan_feature.rs  // 0 inbound refs
``` `

#### Re-Export Only Files

Files that only re-export from another module — bloat if the re-export adds no value:

``` `rust
// ❌ capabilities_reexport.rs — just a passthrough
pub use super::capabilities_real_impl::MyStruct;
pub use super::capabilities_real_impl::MyTrait;
// WHY: Consolidate into the real impl file or into mod.rs directly.
``` `

**Exception — KEEP re-export files when:**

- They form a deliberate public API surface (`pub use` in `lib.rs` pattern)
- Multiple downstream crates import from the re-export path (changing would be a breaking change)

### Exceptions (NEVER Remove Without Explicit Approval)

| File/Pattern | Reason |
| ---------------------------------------------------- | ------------------------------------------------------------ |
| `lib.rs` | Crate entry point |
| `mod.rs` | Module declarations |
| `main.rs` | Binary entry point |
| `contract_*.rs` / `traits.rs` | Trait definitions (may be used by external crates) |
| `build.rs` | Build script |
| Files behind `#[cfg(feature = "...")]` | Conditionally compiled — verify feature is truly deprecated |
| `#[cfg(test)]` modules / `tests/` directory | Test code — check `cargo test` not just `cargo check` |
| Files referenced by `build.rs` | Build-time code generation |
| Files referenced by integration tests (`tests/*.rs`) | Not visible from `src/` imports |
| Files referenced by proc macros / derive macros | Invisible to grep — referenced via macro expansion |
| Items with `#[allow(dead_code)]` | Developer explicitly marked as intentional — investigate WHY |
| Taxonomy / utility files referenced by any layer | Cross-cutting concerns |

### AES Layer-Specific Orphan Detection (AES501–AES506)

After generic orphan detection, run layer-specific orphan checks using the `orphan-detector` tool:

``` `bash
# Run full orphan scan (detects AES501–AES506 layer violations)
cargo run --bin lint-arwaky-cli -- orphan <project-path> --format json
``` `

The tool builds a full import reachability graph and checks:

| Rule | Layer | Orphan If... | Severity |
|------|-------|-------------|----------|
| **AES501** | Taxonomy | No non-taxonomy file imports it | MEDIUM |
| **AES502** | Contract | No implementation (`impl Trait for Type`) exists, or no callers | MEDIUM |
| **AES503** | Capabilities | Not wired in any container and not reachable from entry points | HIGH |
| **AES504** | Utility | Imported only by other utility files (utility-only chain = dead) | MEDIUM |
| **AES505** | Agent | Not referenced by any surface, entry point, or container | **HIGH** |
| **AES506** | Surface | Not reachable in `Entry→Smart→Utility→Passive` chain | MEDIUM |

### Phase 1 Workflow

#### Step 1.1: Safety Snapshot

``` `bash
# ALWAYS do this first — non-negotiable
git add -A && git commit -m "pre-cleanup snapshot: <crate-name>" --allow-empty
git checkout -b cleanup/<crate-name>-$(date +%Y%m%d)
``` `

If anything goes wrong:

``` `bash
git checkout main
git branch -D cleanup/<crate-name>-$(date +%Y%m%d)
# Or restore specific files:
git checkout HEAD~1 -- crates/<crate>/src/<file>.rs
``` `

#### Step 1.2: Read Requirements

Read the FRD / requirements document to understand MVP scope. List all required capabilities, traits, and behaviors.

#### Step 1.3: Run Primary Detection (Tooling)

Use Rust-native tooling FIRST — it understands cfg, macros, and the module system:

``` `bash
# Primary: cargo clippy dead code detection
cargo clippy -p <crate-name> --all-features -- -W dead_code -W unused_imports -W unused_variables 2>&1 | tee /tmp/clippy_report.txt

# Secondary: cargo-udeps (finds unused dependencies and unreachable modules)
cargo udeps -p <crate-name> --all-features 2>&1 | tee /tmp/udeps_report.txt

# Tertiary: cargo check with all features (catches cfg-gated code)
cargo check -p <crate-name> --all-features 2>&1 | tee /tmp/check_report.txt

# Test compilation (catches test-only references)
cargo test -p <crate-name> --no-run --all-features 2>&1 | tee /tmp/test_report.txt
``` `

#### Step 1.4: Run Secondary Detection (File-Level Scan)

Multi-pattern scan for files not referenced anywhere:

``` `bash
#!/usr/bin/env bash
# find_unused_files.sh — comprehensive orphan detection
CRATE_DIR="crates/<crate-name>/src"

for f in "$CRATE_DIR"/*.rs "$CRATE_DIR"/**/*.rs; do
  [ -f "$f" ] || continue
  name=$(basename "$f" .rs)

  # Skip protected files
  [[ "$name" =~ ^(lib|mod|main|build)$ ]] && continue
  [[ "$name" =~ ^contract_ ]] && continue

  # Check ALL reference patterns:
  refs=0
  refs=$((refs + $(grep -rnE "(mod|pub mod)\s+${name}\s*;" "$CRATE_DIR" | grep -v "^$f:" | wc -l)))
  refs=$((refs + $(grep -rnE "use\s+.*\b${name}\b" "$CRATE_DIR" | grep -v "^$f:" | wc -l)))
  refs=$((refs + $(grep -rnE "(crate|super|self)::${name}\b" "$CRATE_DIR" | grep -v "^$f:" | wc -l)))
  refs=$((refs + $(grep -rnE "\b${name}\b" crates/<crate-name>/build.rs 2>/dev/null | wc -l)))
  refs=$((refs + $(grep -rnE "\b${name}\b" crates/<crate-name>/tests/ 2>/dev/null | wc -l)))

  parent_dir=$(dirname "$f")
  glob_refs=$(grep -rnE "use\s+(super|self)::\*" "$parent_dir" 2>/dev/null | grep -v "^$f:" | wc -l)

  if [ "$refs" -eq 0 ] && [ "$glob_refs" -eq 0 ]; then
    echo "UNUSED: $f (0 references, 0 glob imports in parent)"
  elif [ "$refs" -eq 0 ] && [ "$glob_refs" -gt 0 ]; then
    echo "MAYBE_UNUSED: $f (0 direct refs, but $glob_refs glob import(s) in parent — verify manually)"
  fi
done
``` `

#### Step 1.5: Detect Function-Level Bloat

``` `bash
# Find stubs (methods returning trivial values)
grep -rnP "fn\s+\w+\s*\([^)]*\)\s*(->\s*\S+)?\s*\{\s*(None|Some\(\(\)\)|String::new\(\)|vec!\[\]|Ok\(\(\)\)|false|0|Default::default\(\))\s*\}" \
  "$CRATE_DIR" | head -40

# Find thin wrappers (single-expression bodies, multi-line aware)
rg -U "fn\s+\w+\s*\([^)]*\)[^{]*\{\s*\n\s*(self\.\w+|&self\.\w+|\*\w+\s*==\s*\S+)\s*\n\s*\}" \
  "$CRATE_DIR" | head -30

# Find duplicate function names across files
grep -rn "^\s*pub fn \|^\s*fn " "$CRATE_DIR" | \
  sed 's/.*fn \([a-z_0-9]*\).*/\1/' | sort | uniq -d | while read dup; do
    echo "DUPLICATE: $dup"
    grep -rn "fn ${dup}" "$CRATE_DIR"
    echo "---"
  done

# Find #[allow(dead_code)] items (investigate, don't auto-remove)
grep -rn "#\[allow(dead_code)\]" "$CRATE_DIR" | head -20

# Find cfg-gated code (DO NOT remove without verifying feature status)
grep -rn "#\[cfg(feature" "$CRATE_DIR" | head -20
grep -rn "#\[cfg(test)\]" "$CRATE_DIR" | head -20
``` `

#### Step 1.6: Analyze and Categorize

For each flagged item, apply **The Fundamental Question**. Categorize findings:

| Category | What It Is | Action | Confidence |
| -------------------- | ------------------------------------------- | -------------------------------- | --------------- |
| **Stubs** | Empty or trivial-return methods | Remove | High |
| **Thin Wrappers** | Direct attribute access, simple comparisons | Remove (unless trait impl) | High |
| **Duplicates** | Same logic in multiple files | Keep in owning file, remove rest | High |
| **Overengineered** | Patterns failing 3-point test | Remove | Medium — verify |
| **Unused Files** | 0 inbound refs (all patterns checked) | Delete | High |
| **Re-export Only** | Files with only `pub use` passthrough | Consolidate | Medium |
| **Maybe Unused** | 0 direct refs but glob import in parent | Manual review | Low — verify |
| **cfg-gated** | Behind `#[cfg(feature/test)]` | KEEP unless feature deprecated | N/A |
| **allow(dead_code)** | Explicitly marked by developer | Investigate intent | Low — ask |

#### Step 1.7: Report Phase 1

Generate a per-file report:

``` `markdown
## Cleanup Report: <crate-name>

### Summary

- Files scanned: X
- Functions analyzed: Y
- Items flagged for removal: Z
- Estimated lines removed: N

### Per-File Findings

#### `capabilities_movement.rs`

| Item               | Type         | Lines | Verdict | Reason                             |
| ------------------ | ------------ | ----- | ------- | ---------------------------------- |
| `get_velocity()`   | Thin wrapper | 3     | REMOVE  | Direct `self.velocity` access      |
| `clamp_velocity()` | Duplicate    | 5     | REMOVE  | Owned by `capabilities_physics.rs` |
| `apply_force()`    | Real logic   | 22    | KEEP    | Required by FRD-012                |

#### `capabilities_orphan_feature.rs`

| Item        | Type        | Lines | Verdict | Reason                                        |
| ----------- | ----------- | ----- | ------- | --------------------------------------------- |
| Entire file | Unused file | 87    | DELETE  | 0 inbound refs, no glob imports, not in tests |

### Items Requiring Manual Review

- `utils_temporal.rs` — `#[allow(dead_code)]` on 3 items. Developer intent unclear.
- `capabilities_experimental.rs` — Behind `#[cfg(feature = "experimental")]`. Is feature deprecated?
``` `

#### Step 1.8: Get Approval for Phase 1

Present report to user. Get **explicit per-file approval** before making changes.

For "Maybe Unused" and "cfg-gated" items, require **explicit confirmation** — do not batch-remove.

#### Step 1.9: Execute Phase 1 Cleanup

``` `bash
# Remove unused file(s)
rm crates/<crate>/src/capabilities_orphan_feature.rs

# Update mod.rs — remove module declaration
sed -i '/mod capabilities_orphan_feature;/d' crates/<crate>/src/mod.rs

# Update trait definitions — remove removed methods
# (Manual: open trait file, delete method signatures matching removed impls)

# Remove thin wrappers / stubs from impl blocks
# (Manual: edit file, remove function, update trait if applicable)
``` `

#### Step 1.10: Verify Phase 1

``` `bash
# Compilation check (all features to catch cfg-gated breakage)
cargo check -p <crate-name> --all-features 2>&1 | grep -E "^error"

# Test compilation
cargo test -p <crate-name> --no-run --all-features 2>&1 | grep -E "^error"

# Full test run (if fast enough)
cargo test -p <crate-name> --all-features 2>&1 | tail -5

# Clippy clean
cargo clippy -p <crate-name> --all-features -- -D warnings 2>&1 | grep -E "^error|^warning"

# Check downstream crates that depend on this one
cargo check --workspace --all-features 2>&1 | grep -E "^error"
``` `

---

## Phase 2: File Consolidation

### Detection Patterns: Same-Concept Files (Merge)

``` `rust
capabilities_world_to_camera.rs
capabilities_camera_to_world.rs
capabilities_camera_to_viewport.rs
// All do coordinate transforms → merge into capabilities_coordinate_mapper.rs
``` `

### Detection Patterns: Same-Feature Files (Merge)

``` `rust
capabilities_brush_cursor_drawer.rs
capabilities_drag_cursor_drawer.rs
capabilities_cursor_data_renderer.rs
// All render cursors → merge into capabilities_cursor_renderer.rs
``` `

### Detection Patterns: Same-Technology Adapters (Merge)

``` `rust
utility_ffmpeg_adapter.rs
utility_video_ffmpeg_adapter.rs
// Both use FFmpeg → merge into 1 adapter
``` `

### The Consolidation Pattern

#### Before Merge (Two Files)

``` `
crates/<crate>/src/capabilities_<name1>.rs
  - StructA implements TraitA
  - Fields: field_a, field_b
  - Methods: method_a, helper_a

crates/<crate>/src/capabilities_<name2>.rs
  - StructB implements TraitB
  - Fields: field_c, field_d
  - Methods: method_b, helper_b
``` `

#### After Merge (One File)

``` `rust
use async_trait::async_trait;
use shared::...;

/// Unified struct combining StructA and StructB for [domain description].
pub struct UnifiedStruct {
    // Fields from BOTH old structs (merge all fields)
    field_a: TypeA,
    field_b: TypeB,
    field_c: TypeC,
    field_d: TypeD,
}

#[async_trait]
impl TraitA for UnifiedStruct {
    fn method_a(&self, ...) -> ... {
        self.do_method_a(...)  // wrapper calls do_* method
    }

    fn do_method_a(&self, ...) -> ... {
        // merged logic from old StructA
    }
}

#[async_trait]
impl TraitB for UnifiedStruct {
    fn method_b(&self, ...) -> ... {
        self.do_method_b(...)  // wrapper calls do_* method
    }

    fn do_method_b(&self, ...) -> ... {
        // merged logic from old StructB
    }
}

// Free functions — keep as standalone or make methods
fn helper_a(...) -> ... { ... }
fn helper_b(...) -> ... { ... }
``` `

### Phase 2 Workflow

#### Step 2.1: Detect Overlaps and Analyze Files

Group files by concept/feature/technology. Read each file to understand:

- What structs/classes exist
- What traits they implement
- What fields each struct has
- What methods each impl block has
- What free functions exist
- What imports are used

``` `bash
# Group files by capability name pattern
ls crates/<crate>/src/capabilities_*.rs

# Analyze both files
wc -l crates/<crate>/src/file1.rs crates/<crate>/src/file2.rs
grep -c "^pub struct" crates/<crate>/src/file1.rs
grep -c "^    fn \|^    pub fn " crates/<crate>/src/file1.rs
``` `

#### Step 2.2: Pick Target File

Select the file with the most logic (most lines, most methods, most fields) as the merge target.

#### Step 2.3: Merge Imports

Combine imports from all files, remove duplicates:

``` `rust
// From file1 + file2 — deduplicated
use async_trait::async_trait;
use shared::common::...;
use shared::import_rules::...;
use std::collections::{HashMap, HashSet};
``` `

#### Step 2.4: Merge Structs

Combine fields from all old structs into one struct:

``` `rust
pub struct UnifiedStruct {
    // Fields from StructA
    field_a: TypeA,
    field_b: TypeB,

    // Fields from StructB
    field_c: TypeC,
    field_d: TypeD,
}
``` `

**Merge carefully**: If both structs have the same field (e.g., `_config`), keep only one.

#### Step 2.5: Merge Impl Blocks

Put ALL methods into impl blocks. If multiple traits exist, create separate impl blocks for each trait.

**For each trait:**

- Trait method (public) → wrapper calling `do_*` method
- Internal implementation → `do_*` prefix

``` `rust
impl TraitA for UnifiedStruct {
    fn public_method(&self, ...) -> ... {
        self.do_public_method(...)  // calls internal method
    }

    fn do_public_method(&self, ...) -> ... {
        // actual logic from old StructA
    }
}

impl TraitB for UnifiedStruct {
    fn public_method(&self, ...) -> ... {
        self.do_public_method(...)  // calls internal method
    }

    fn do_public_method(&self, ...) -> ... {
        // actual logic from old StructB
    }
}
``` `

#### Step 2.6: Merge Free Functions

Keep free functions as standalone (outside impl block) or convert to methods:

``` `rust
// Option A: Keep as standalone free functions
fn helper_a(...) -> ... { ... }
fn helper_b(...) -> ... { ... }

// Option B: Convert to methods (if they need self)
impl UnifiedStruct {
    fn do_helper_a(&self, ...) -> ... { ... }
    fn do_helper_b(&self, ...) -> ... { ... }
}
``` `

#### Step 2.7: Update All References

Find and update ALL references across the codebase:

``` `bash
# Find all references to old names
grep -r "OldStructA\|OldStructB\|TraitA\|TraitB" crates/

# Update lib.rs exports
# Update root container wiring
# Update test files
``` `

#### Step 2.8: Delete Source File(s)

Remove the file(s) whose functionality was merged:

``` `bash
rm crates/<crate>/src/file2.rs
``` `

#### Step 2.9: Verify Phase 2

``` `bash
cargo check -p <crate-name> 2>&1 | grep -E "error|cannot find"
``` `

---

## Final Verification (Both Phases)

``` `bash
# Compilation check
cargo check -p <crate-name> --all-features 2>&1 | grep -E "^error"

# Test compilation
cargo test -p <crate-name> --no-run --all-features 2>&1 | grep -E "^error"

# Full test run
cargo test -p <crate-name> --all-features 2>&1 | tail -5

# Clippy clean
cargo clippy -p <crate-name> --all-features -- -D warnings 2>&1 | grep -E "^error|^warning"

# Check downstream crates
cargo check --workspace --all-features 2>&1 | grep -E "^error"
``` `

---

## Commit

``` `bash
git add -A
git commit -m "cleanup(<crate-name>): remove N dead items + merge M files (K lines)

Removed:
- X stubs
- Y thin wrappers
- Z duplicate functions
- W unused files

Consolidated:
- A files merged into B files

All cargo check/test/clippy passing with --all-features."
``` `

---

## Verification Checklist

### Phase 1: Dead Code Cleanup

- [ ] Git snapshot created before any changes
- [ ] Working on dedicated cleanup branch
- [ ] FRD / requirements read and MVP scope understood
- [ ] `cargo clippy --all-features` run as primary detection
- [ ] File-level scan uses multi-pattern detection (mod, use, path, glob, build.rs, tests)
- [ ] Each function evaluated against Fundamental Question
- [ ] `#[cfg(feature)]` and `#[cfg(test)]` items NOT auto-removed
- [ ] `#[allow(dead_code)]` items investigated, not auto-removed
- [ ] Proc macro / derive macro references checked
- [ ] Integration tests (`tests/`) checked for references
- [ ] Report generated showing keep/remove per file with reasons
- [ ] Approval received before making changes
- [ ] Traits updated when methods removed from impl
- [ ] `mod.rs` updated when modules deleted
- [ ] `cargo check -p <crate> --all-features` passes
- [ ] `cargo test -p <crate> --all-features` passes
- [ ] `cargo clippy -p <crate> --all-features -- -D warnings` passes
- [ ] `cargo check --workspace --all-features` passes (downstream crates)

### Phase 2: File Consolidation

- [ ] Files analyzed and overlaps confirmed
- [ ] Target file selected (most logic)
- [ ] Imports merged and deduplicated
- [ ] Structs combined into one struct with all fields
- [ ] All methods moved to impl blocks (trait impl + inherent impl)
- [ ] Free functions kept as standalone or converted to methods
- [ ] Source file(s) deleted
- [ ] All references updated (lib.rs, root container, tests)
- [ ] `cargo check -p <crate-name>` passes without warnings or errors

### Final

- [ ] Committed with descriptive message

---

## Quick Reference Commands

``` `bash
# === PHASE 1: PRIMARY DETECTION ===
cargo clippy -p <crate> --all-features -- -W dead_code -W unused_imports 2>&1
cargo udeps -p <crate> --all-features 2>&1

# === PHASE 1: FILE-LEVEL ORPHAN SCAN ===
# (Use the full script from Step 1.4 above)

# === PHASE 1: FUNCTION-LEVEL BLOAT ===
# Stubs:
rg "fn\s+\w+\([^)]*\)\s*(->\s*\S+)?\s*\{\s*(None|String::new|vec!\[\]|Ok\(\(\)\)|false|0)\s*\}" crates/<crate>/src/

# Thin wrappers (multiline):
rg -U "fn\s+\w+\([^)]*\)[^{]*\{\s*\n\s*(self\.\w+|&self\.\w+)\s*\n\s*\}" crates/<crate>/src/

# Duplicates:
grep -rn "fn " crates/<crate>/src/ | sed 's/.*fn \([a-z_0-9]*\).*/\1/' | sort | uniq -d

# cfg-gated code (DO NOT REMOVE):
rg "#\[cfg\(" crates/<crate>/src/

# allow(dead_code) (INVESTIGATE):
rg "#\[allow\(dead_code\)\]" crates/<crate>/src/

# === PHASE 2: OVERLAP DETECTION ===
ls crates/<crate>/src/capabilities_*.rs | xargs -n1 basename | sort
wc -l crates/<crate>/src/file1.rs crates/<crate>/src/file2.rs
grep -c "^pub struct" crates/<crate>/src/file1.rs
grep -c "^    fn \|^    pub fn " crates/<crate>/src/file1.rs

# === VERIFICATION ===
cargo check -p <crate> --all-features 2>&1 | grep "^error"
cargo test -p <crate> --all-features 2>&1 | tail -3
cargo clippy -p <crate> --all-features -- -D warnings 2>&1 | grep "^error"
cargo check --workspace --all-features 2>&1 | grep "^error"

# === ROLLBACK ===
git checkout HEAD~1 -- crates/<crate>/src/<file>.rs   # restore one file
git reset --hard HEAD~1                                  # nuclear option
``` `

---

## Common Mistakes (AVOID)

| Mistake | Why It's Dangerous | Prevention |
| -------------------------------------------------- | --------------------------------------------------------- | ----------------------------------------------- |
| Removing real MVP logic | Breaks required functionality | Fundamental Question + FRD cross-reference |
| Forgetting to update traits | Compilation errors in downstream crates | Always edit trait file when editing impl |
| Deleting files without updating `mod.rs` | Compilation error: "file not found for module" | Checklist item; grep for `mod <name>;` |
| Removing `contract_*.rs` / trait files | Breaks external crate consumers | Exception list; check `Cargo.toml` dependents |
| Skipping `--all-features` in verification | Misses breakage in cfg-gated code | Always use `--all-features` in check/test/clippy |
| Removing `#[cfg(test)]` code | Breaks `cargo test` | Run `cargo test --no-run` as verification step |
| Removing code behind `#[cfg(feature)]` | Breaks feature-gated builds | Check `Cargo.toml` `[features]` section first |
| Ignoring glob imports (`use super::*`) | File appears unused but is imported via glob | Check parent module for `*` imports |
| Ignoring proc macro / derive references | File is referenced via macro expansion, invisible to grep | Check `#[derive(...)]` and proc macro crates |
| Skipping git snapshot | Cannot rollback if cleanup breaks something | Step 1.1 is non-negotiable |
| Batch-removing "Maybe Unused" items | Glob imports or macros may reference them | Require manual review + explicit approval |
| Removing `#[allow(dead_code)]` items without asking | Developer had a reason to mark it | Investigate git blame / ask author |
| Consolidating files with dead code | Wastes effort merging code that should be deleted | Always run Phase 1 before Phase 2 |
| Forgetting to update lib.rs exports after merge | Compilation error: "unresolved import" | Grep for old module names after merge |
| Leaving orphan references after merge | Runtime errors from stale imports | Grep for old struct/trait names after merge |

---

## Decision Flowchart

``` `
START
│
├─ PHASE 1: DEAD CODE CLEANUP
│  │
│  ├─ Item flagged for removal
│  │  │
│  │  ├─ Is it in the Exceptions list?
│  │  │  └─ YES → KEEP (stop)
│  │  │
│  │  ├─ Is it behind #[cfg(feature/test)]?
│  │  │  └─ YES → KEEP unless feature is confirmed deprecated (stop)
│  │  │
│  │  ├─ Does it have #[allow(dead_code)]?
│  │  │  └─ YES → Investigate intent. Ask author. Do NOT auto-remove. (stop)
│  │  │
│  │  ├─ Is it referenced by proc macro / derive / build.rs / integration test?
│  │  │  └─ YES → KEEP (stop)
│  │  │
│  │  ├─ Apply Fundamental Question:
│  │  │  ├─ "Required by FRD?" → KEEP
│  │  │  ├─ "Called by FRD-required method?" → KEEP
│  │  │  ├─ "Always there / might be useful / edge case?" → REMOVE
│  │  │  └─ Unclear? → Flag for manual review (do NOT auto-remove)
│  │  │
│  │  ├─ If Overengineered pattern:
│  │  │  └─ Pass 3-point test? → REMOVE. Fail any point? → KEEP + comment.
│  │  │
│  │  └─ Execute removal → Update trait → Update mod.rs → Verify
│  │
│  └─ Phase 1 Complete → Proceed to Phase 2
│
├─ PHASE 2: FILE CONSOLIDATION
│  │
│  ├─ Do files share the same domain/concept/feature?
│  │  └─ NO → Skip consolidation for these files
│  │
│  ├─ YES → Merge into single file:
│  │  ├─ Pick target (most logic)
│  │  ├─ Merge imports (deduplicate)
│  │  ├─ Merge structs (combine fields)
│  │  ├─ Merge impl blocks (one per trait)
│  │  ├─ Merge free functions
│  │  ├─ Update all references
│  │  ├─ Delete source file(s)
│  │  └─ Verify compilation
│  │
│  └─ Phase 2 Complete → Final Verification
│
└─ FINAL VERIFICATION
   ├─ cargo check --all-features
   ├─ cargo test --all-features
   ├─ cargo clippy --all-features -- -D warnings
   ├─ cargo check --workspace --all-features
   └─ Commit with descriptive message
``` `

---

## Dry-Run Mode

When user requests `--dry-run` or says "just show me what you'd remove":

1. Run Phase 1 Steps 1.1–1.6 (detection + analysis)
2. Run Phase 2 Step 2.1 (overlap detection)
3. Generate the full report (Phase 1 Step 1.7 + Phase 2 findings)
4. **Do NOT execute any deletions or edits**
5. Present report and wait for explicit approval to proceed

This is the **default mode** for first-time runs on a crate.
```

---

# create-agent-rust

**Files:** 1

## File List

- [.agents/skills/create-agent-rust/SKILL.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-agent-rust/SKILL.md)

---

## File: .agents/skills/create-agent-rust/SKILL.md

```markdown
---
name: create-agent-rust
description: "Create and validate Rust agent layer files following AES rules: orchestration-only, zero I/O, zero business logic, zero domain computation, 3-block structure, max 3 types per file, aggregate contracts, DI for service dependencies, and shared VOs for domain data."
metadata:
  tags: [rust, aes, agent, aggregate, structure, 3-block-structure, di, orchestration, vo]
  triggers:
    - "create agent rust"
    - "add agent rust"
    - "fix agent structure rust"
    - "create aggregate rust"
    - "agent missing aggregate rust"
    - "validate agent logic rust"
    - "check agent rust"
    - "audit agent rust"
  dependencies: []
  related:
    - create-capabilities-rust
    - create-taxonomy-rust
    - create-contract-rust
---

# create-agent-rust

Agent = orchestration only. No I/O, no business logic, no domain computation, no local domain data.

**Allowed imports:** `shared::*` — taxonomy VOs, constants, aggregate traits, protocol traits.
**Forbidden imports:** `capabilities_*`, `agent_*`, `surface_*`.

**Allowed ops:** `for`/`while`/`loop`, `if/else`/`match`, `?`/`match Err`, `tokio::join!`/`.await`, collecting results into shared VOs.
**Forbidden ops:** `std::fs`, `File::open`, `reqwest`, `hyper`, `sqlx`, `rusqlite`, stdout/stderr write, env mutation, global state mutation.

## 3-Block Structure

``` `text
// Block 1: Struct Definition
// Block 2: Aggregate Trait Implementation
// Block 3: Constructors, Std Traits, Helpers
``` `

Method placement:

``` `text
Free function (outside impl)?               → EXTRACT to *_utility.rs
In aggregate trait?                         → Block 2
std trait impl (Default/Clone/Display)?     → Block 3
fn new() / constructor?                     → Block 3
Private helper (uses &self)?                → Block 3
Pure fn, no struct dep?                     → EXTRACT to *_utility.rs
``` `

## Helper vs Utility

Keep in Block 3 if ANY: uses `&self`, coupled to this struct, constructor, agent-specific logic, single-use.
Extract to utility only if ALL: no `self`/`Self`, pure, no side effects, domain-agnostic, reusable.
I/O: stateless + I/O + domain-agnostic = taxonomy utility. Stateless + I/O + domain-specific = capabilities.

## Computation, Errors, VOs

**Computation forbidden:** arithmetic, totals, averages, `.sum()`/`.fold()`, parsing, normalization.
Allowed: iteration to call deps, routing results, propagating errors.
e.g. `for file in files { self.checker.check(file) }` = OK. `files.iter().map(|f| f.size()).sum()` = capabilities.

**Error rules:**
- Rule 1: Never silently discard — no `checker.check().unwrap_or_default()`.
- Rule 2: Analysis orchestration → `Vec<<ResultVO>>`, match per-item into VO.
- Rule 3: Execution orchestration → `Result<ExecutionReport, AgentExecutionError>`.
- Rule 4: Delegate I/O errors to capabilities — agent only wraps into VO.

**VO rules:** `String`/`i32`..`u64`/`f32`/`f64`/`char` forbidden for domain fields/contracts. `bool` for toggles; `&str` for borrowed non-domain input only.

## Templates

``` `rust
use std::sync::Arc;

use shared::<domain>::taxonomy_<name>_vo::<VO>;
use shared::<domain>::contract_<name>_aggregate::I<Name>Aggregate;

// ─── Block 1: Struct Definition ──────────────────────────
pub struct Agent<Name> {
    aggregate: Arc<dyn I<Name>Aggregate>,
}

// ─── Block 2: Aggregate Trait Implementation ─────────────
impl I<Name>Aggregate for Agent<Name> {
    fn execute(&self, request: &<RequestVO>) -> Vec<<ResultVO>> {
        // orchestration only — delegate to aggregate
        self.aggregate.process(request)
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Agent<Name> {
    pub fn new(aggregate: Arc<dyn I<Name>Aggregate>) -> Self {
        Self { aggregate }
    }
}

impl Default for Agent<Name> {
    fn default() -> Self {
        Self {
            aggregate: Arc::new(PlaceholderAggregate),
        }
    }
}
``` `
## Workflow

1. Confirm orchestration only — computation → capabilities, domain data → taxonomy.
2. Struct implements aggregate trait? If no → create `contract_<name>_aggregate.rs`.
3. Enforce 3-Block.
4. ≥1 aggregate trait, ≤3 types (struct+enum), `Arc<dyn Trait>` for DI, shared VOs.
5. Generic aggregate methods: object-safe or `where Self: Sized`.
6. No forbidden imports, no I/O, no computation.
7. No silent errors, no raw primitives in contracts, no magic constants.
8. `cargo check -p <crate-name>`.

## Checklist

- [ ] Block 1 → 2 → 3 order followed.
- [ ] Block 2: ONLY aggregate trait implementation.
- [ ] Block 3: constructors, std traits, private helpers.
- [ ] ≥1 struct implements aggregate trait; ≤3 total types.
- [ ] No local domain data; `Arc<dyn Trait>` for DI; shared VOs.
- [ ] Zero I/O, zero business logic, zero domain computation.
- [ ] No forbidden imports.
- [ ] Generic aggregate methods object-safe or `where Self: Sized`.
- [ ] Aggregate registered in shared crate `mod.rs`.
- [ ] `cargo check -p <crate-name>` passes.
```

---

# create-capabilities-rust

**Files:** 1

## File List

- [.agents/skills/create-capabilities-rust/SKILL.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-capabilities-rust/SKILL.md)

---

## File: .agents/skills/create-capabilities-rust/SKILL.md

```markdown
---
name: create-capabilities-rust
description: "Create and validate Rust capabilities layer files following AES rules: concrete implementation of behavior (business logic + external adaptation), 3-block structure, max 3 types per file, protocol trait contracts, DI for service dependencies, and shared VOs for domain data."
metadata:
  tags: [rust, aes, capabilities, protocol, 3-block-structure, di, vo]
  triggers:
    - "create capabilities rust"
    - "add capabilities rust"
    - "fix capabilities structure rust"
    - "create protocol rust"
    - "capabilities missing protocol rust"
    - "validate capabilities logic rust"
    - "check capabilities rust"
    - "audit capabilities rust"
  dependencies: []
  related:
    - create-agent-rust
    - create-taxonomy-rust
    - create-contract-rust
    - create-utility-rust
---
# create-capabilities-rust

Capabilities = concrete protocol trait implementation. File: `capabilities_<domain>_<role>.rs`.

**Allowed imports:** Taxonomy, Contract (`_protocol` only), Utility.
**Forbidden:** `agent_*`, other `capabilities_*`, `surface_*`, local domain models, magic constants.

## Examples Role Naming

**Internal:** validator, assessor, calculator, resolver, classifier, selector, mapper, transformer, policy, enricher, evaluator, analyzer, scorer, grader, ranker, filter, checker, reviewer, approver, rejector
**External:** repository, gateway, client, provider, fetcher, reader, writer, scanner, executor, publisher, subscriber, adapter, connector, uploader, downloader, sender, receiver, dispatcher, watcher, monitor

## Structure Rules

- Rule 1: Internal helper structs without trait impl → ALLOWED.
- Rule 2: ≥1 struct implements a protocol trait.
- Rule 3: Total struct + enum ≤ 3.

## 3-Block Structure

``` `text
// Block 1: Struct Definition
// Block 2: Protocol Trait Implementation
// Block 3: Constructors, Std Traits, Helpers
``` `

## Helper vs Utility Decision Matrix

**Keep in Block 3** if ANY of these apply:

- Uses `&self` or instance state.
- Domain-specific (contains business rules).
- Single consumer (used only within this file/module).
- Acts as a constructor or builder for the struct.

**Extract to Utility** ONLY if ALL of these apply:

- No `self` (stateless free function).
- Pure / deterministic (or domain-agnostic I/O like serialization).
- Domain-agnostic (no business rules).
- ≥2 consumers (reusable across modules).

## Templates

``` `rust
use std::sync::Arc;

use shared::<name-feature>::taxonomy_<name-policy>_vo::<NamePolicy>VO;
use shared::<name-feature>::contract_<name-store>_protocol::I<NameStore>Protocol;
use shared::<name-feature>::contract_<name-collaborator>_protocol::I<NameCollaborator>Protocol;
use shared::<name-feature>::contract_<name-capability>_protocol::I<NameCapability>Protocol;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct Capabilities<NameCapability> {
    collaborator: Arc<dyn I<NameCollaborator>Protocol>,
    store: Arc<dyn I<NameStore>Protocol>,
    policy: <NamePolicy>VO,
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl I<NameCapability>Protocol for Capabilities<NameCapability> {
    fn execute(&self, input: &<DomainVO>) -> Vec<<ResultVO>> {
        let mut results = Vec::new();
        // domain logic using injected dependencies
        results
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Capabilities<NameCapability> {
    pub fn new(
        collaborator: Arc<dyn I<NameCollaborator>Protocol>,
        store: Arc<dyn I<NameStore>Protocol>,
        policy: <NamePolicy>VO,
    ) -> Self {
        Self {
            collaborator,
            store,
            policy,
        }
    }

    // HELPERS: Should be `private` (no `pub`) or `pub(crate)` for testing.
    // If a helper needs to be fully `pub` and reusable across modules, extract it to Utility.
    fn helper_method(&self) -> bool {
        // internal logic
        true
    }
}
``` `

## Workflow

1. Confirm implements protocol behavior (not orchestration/data/mechanics).
2. File `use shared::..._protocol::I<Name>` — if missing → flag `CapabilityNoProtocol`.
3. Create `contract_<name>_protocol.rs` if missing.
4. Enforce 3-Block with explicit `// Block 1:`, `// Block 2:`, `// Block 3:` comments.
5. AES403: ≥1 trait implementor, ≤3 types, `Arc<dyn Trait>` for DI, shared VOs.
6. No forbidden imports, no inter-capability deps, no local domain models.
7. `cargo check -p <crate-name>`.

## Checklist

- [ ]  Block 1 → 2 → 3 order followed with explicit comments.
- [ ]  Block 2: ONLY `impl I<Name>Protocol for ...`.
- [ ]  ≥1 struct implements protocol trait; ≤3 total struct+enum.
- [ ]  Imports from `_protocol` module or Utility only.
- [ ]  No local domain models, no agent/capability imports.
- [ ]  `Arc<dyn Trait>` for DI; shared VOs for fields and trait signatures.
- [ ]  Constants → `taxonomy_<domain>_constant.rs`.
- [ ]  Helper functions in Block 3 are `private` or `pub(crate)` (not fully `pub` unless justified).
- [ ]  Low-level, reusable, stateless ops → moved to Utility.
- [ ]  `cargo check -p <crate-name>` passes.

``` `
``` `
```

---

# create-contract-rust

**Files:** 1

## File List

- [.agents/skills/create-contract-rust/SKILL.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-contract-rust/SKILL.md)

---

## File: .agents/skills/create-contract-rust/SKILL.md

```markdown
---
name: create-contract-rust
description: "Create and validate Rust contract layer files in shared domain: pure trait definitions for protocols and aggregates. Contracts define public promises only, with no implementation, no layer imports, and domain-safe VO-based signatures."
metadata:
  tags: [rust, aes, contract, protocol, aggregate, trait, vo]
  triggers:
    - "create contract rust"
    - "add contract rust"
    - "create protocol rust"
    - "create aggregate rust"
    - "contract missing rust"
    - "validate contract rust"
    - "check contract rust"
  dependencies: []
  related:
    - create-capabilities-rust
    - create-agent-rust
    - create-taxonomy-rust
---
# create-contract-rust

Contract = pure trait definitions. No default implementations. File: `contract_<concept>_<suffix>.rs`.

**Allowed imports:** taxonomy types, other contract types.
**Forbidden:** capabilities, agents, surface, root.

## Contract Roles


| Suffix       | Implemented By | Used By |
| -------------- | ---------------- | --------- |
| `_protocol`  | Capabilities   | Agent   |
| `_aggregate` | Agent          | Surface |

Naming: `I<Name>Protocol`, `I<Name>Aggregate`.

## Rules

- `pub trait` only — methods end with `;`, no bodies.
- No private helper signatures.
- All methods type-annotated.
- Object-safe by default.
- Signatures use shared VOs — no `String`/`i32`..`u64`/`f32`/`f64`/`Vec<String>` for domain values.
- `bool` and `&str` (for non-domain input) allowed with care.
- Register in shared `mod.rs`.

## Templates

### Protocol trait

``` `rust
use shared::<domain>::taxonomy_<name>_vo::<VO>;

pub trait I<Name>Protocol: Send + Sync {
    fn method_name(
        &self,
        param: &VO,
    );
}
``` `

### Aggregate trait

``` `rust
use shared::<domain>::taxonomy_<name>_vo::<VO>;

pub trait I<Name>Aggregate: Send + Sync {
    fn execute(
        &self,
        request: &ScanRequest,
    ) -> Vec<LintResult>;
}

pub trait I<Name>Aggregate:
    I<Name>rotocol
    + I<Name>Protocol
    + I<Name>Protocol
    + I<Name>Protocol
    + I<Name>Protocol
{
    /// All discovered source files .
    fn <Name>_<Name>(&self) -> &[FileEntry];

    /// Read file content from bounded cache.
    fn <Name>_<Name>(&self, path: &FilePath) -> ContentString;

    /// Get cached file content (after scan).
    fn <Name>_<Name>(&self, path: &Path) -> Option<String>;

    /// Check if a file is in the cache.
    fn <Name>_<Name>(&self, path: &Path) -> bool;
}
``` `

### mod.rs

``` `rust
// <domain> — contract traits for <domain> operations
pub mod contract_<name>_protocol;
pub mod contract_<name>_aggregate;
``` `

## Workflow

1. Which layer implements this? Capabilities → `_protocol`. Agent → `_aggregate`.
2. Golden Rule: only methods called by outer layers go in the trait.
3. Create `contract_<concept>_<suffix>.rs` in shared domain.
4. Register in `mod.rs`.
5. `cargo check -p <crate-name>`.

## Checklist

- [ ]  Correct suffix `_protocol` or `_aggregate`.
- [ ]  `pub trait` only — no default method bodies.
- [ ]  All methods type-annotated.
- [ ]  No imports from capabilities, agents, surface.
- [ ]  Signatures use shared VOs.
- [ ]  Registered in shared `mod.rs`.
- [ ]  `cargo check -p <crate-name>` passes.
```

---

# create-root-rust

**Files:** 1

## File List

- [.agents/skills/create-root-rust/SKILL.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-root-rust/SKILL.md)

---

## File: .agents/skills/create-root-rust/SKILL.md

```markdown
---
name: create-root-rust
description: "Create and validate Rust root layer files: composition root that wires Capabilities to Contract traits/aggregates and bootstraps the application. Container connects implementations, Entry starts the system."
metadata:
  tags: [rust, aes, root, container, entry, composition, di, wiring]
  triggers:
    - "create root rust"
    - "add root rust"
    - "create container rust"
    - "create entry rust"
    - "wire dependencies rust"
    - "check root rust"
    - "audit root rust"
  dependencies: []
  related:
    - create-capabilities-rust
    - create-agent-rust
    - create-contract-rust
    - create-taxonomy-rust
---

# create-root-rust

Root = **composition layer** that assembles the system. Connects concrete implementations to contracts and starts the application. May depend on all layers.

## Two Root Roles

| Role | Suffix | Responsibility |
| --- | --- | --- |
| Container | `_container` | Wire one feature's Capabilities to Contracts |
| Entry | `_entry` | Bootstrap application, compose feature containers |

## Definition of Done

1. Correct suffix: `_container` or `_entry`.
2. Container: wires Capabilities to Contract traits/aggregates (via `Arc<dyn Trait>`).
3. Entry: bootstraps application and composes feature containers.
4. May instantiate and wire components.
5. No business logic.
6. No orchestration policy.
7. No technical parsing or UI behavior.
8. `cargo check -p <crate-name>` passes.

## Workflow

1. **Determine role** — Container (wire one feature) or Entry (bootstrap all)?
2. **Create file** → `root_<concept>_<suffix>.rs`.
3. **Wire deps** → Connect Capabilities to Contract traits via `Arc::new(impl)`.
4. **Register** → update `mod.rs`.
5. **Verify** → `cargo check -p <crate-name>`.
```

---

# create-surface-rust

**Files:** 1

## File List

- [.agents/skills/create-surface-rust/SKILL.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-surface-rust/SKILL.md)

---

## File: .agents/skills/create-surface-rust/SKILL.md

```markdown
---
name: create-surface-rust
description: "Create and validate Rust surface layer files following AES406: smart/utility/passive surfaces, strict import rules, delegate to aggregates, zero direct lower-layer imports, zero business logic, VO-based state, and explicit error handling."
metadata:
  tags: [rust, aes, surface, smart, utility, passive, di, vo]
  triggers:
    - "create surface rust"
    - "add surface rust"
    - "fix surface structure rust"
    - "create command rust"
    - "create controller rust"
    - "check surface rust"
    - "audit surface rust"
  dependencies: []
  related:
    - create-agent-rust
    - create-taxonomy-rust
    - create-contract-rust
---
# create-surface-rust

Surface = entry points and UI adapters. No business logic. Delegate to aggregates. File: `surface_<domain>_<role>.rs`.

## Three Types (AES406)


| Type    | Suffixes                                     | Imports                          | Forbidden                            |
| --------- | ---------------------------------------------- | ---------------------------------- | -------------------------------------- |
| Smart   | `_command`, `_controller`, `_page`, `_entry` | taxonomy +`contract_*_aggregate` | capabilities, concrete agents        |
| Utility | `_hook`, `_store`, `_action`, `_screen`      | taxonomy + passive surfaces      | smart surfaces, capabilities, agents |
| Passive | `_component`, `_view`, `_layout`             | taxonomy only                    | all other layers                     |

## Rules

- Smart: inject `Arc<dyn I<Name>Aggregate>` via DI, delegate, return `Result<State, SurfaceError>`.
- Utility: map events → VOs, hold minimal UI state, compose passive.
- Passive: render from VOs only — no computation, no orchestration.
- **Never silently discard errors:** forbidden `self.runner.run(&r).unwrap_or_default()`. Use `Ok/Err` or update error state VO.
- All state fields use shared VOs.

## Helper vs Utility

Keep in surface file if ANY: uses `&self`, surface-specific mapping, constructor.
Extract to taxonomy utility only if ALL: no `self`, pure, domain-agnostic, reusable.

## Templates

``` `rust
use std::sync::Arc;

use shared::<domain>::taxonomy_<name>_vo::<VO>;
use shared::<domain>::contract_<name>_aggregate::I<Name>Aggregate;

pub struct Surface<Name> {
    aggregate: Arc<dyn I<Name>Aggregate>,
}

impl Surface<Name> {
    pub fn new(aggregate: Arc<dyn I<Name>Aggregate>) -> Self {
        Self { aggregate }
    }

    pub fn handle(&self, event: &TuiEvent) -> Result<UiState, SurfaceError> {
        // orchestration only
        Ok(UiState::idle())
    }
}
``` `

## Workflow

1. Determine type (Smart/Utility/Passive), choose suffix.
2. Enforce import rules for that type.
3. No silent error discard.
4. `cargo check -p <crate-name>`.

## Checklist

- [ ]  Correct suffix for surface type.
- [ ]  Smart: only taxonomy + `contract_*_aggregate` imports.
- [ ]  Utility: only taxonomy + passive surface imports.
- [ ]  Passive: only taxonomy imports.
- [ ]  Smart delegates via `Arc<dyn Trait>`.
- [ ]  Zero business logic and computation.
- [ ]  No silent error discarding.
- [ ]  All state fields use shared VOs.
- [ ]  `cargo check -p <crate-name>` passes.
```

---

# create-taxonomy-rust

**Files:** 1

## File List

- [.agents/skills/create-taxonomy-rust/SKILL.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-taxonomy-rust/SKILL.md)

---

## File: .agents/skills/create-taxonomy-rust/SKILL.md

```markdown
---
name: create-taxonomy-rust
description: "Create and validate Rust taxonomy layer files in shared taxonomy: VOs, entities, errors, events, and constants. Taxonomy is the domain foundation layer — stable language of the domain, free from technical or behavioral concerns."
metadata:
  tags: [rust, aes, taxonomy, shared, vo, entity, error, event, constant, primitive-to-vo]
  triggers:
    - "create taxonomy rust"
    - "add taxonomy rust"
    - "move dataclass to taxonomy rust"
    - "create vo rust"
    - "create error taxonomy rust"
    - "create constant taxonomy rust"
    - "check taxonomy rust"
    - "audit taxonomy rust"
  dependencies: []
  related:
    - create-capabilities-rust
    - create-agent-rust
    - create-contract-rust
---
# create-taxonomy-rust

Taxonomy = stable domain language. Single source of truth for VOs, entities, errors, events, constants. Location: `crates/shared/src/<domain>/`.

**Allowed imports:** other taxonomy types, std.
**Forbidden:** capabilities, agents, surface, root, contracts, `std::fs`/network/database (in VOs/entities/errors/events/constants).

## File Types


| Suffix         | Content                | Key constraint                               |
| ---------------- | ------------------------ | ---------------------------------------------- |
| `_vo.rs`       | Value Objects          | Validate in`new()`, immutable fields, no I/O |
| `_entity.rs`   | Entities with identity | Identity VO field required                   |
| `_error.rs`    | Domain errors          | Implement`std::error::Error` + `Display`     |
| `_event.rs`    | Domain events          | Immutable, VO payload fields                 |
| `_constant.rs` | Compile-time constants | `pub const` only — no functions             |
| `_utility.rs`  | Stateless helpers      | No struct, no`impl`, domain-agnostic         |

## VO Rules (AES401/AES402)

Forbidden for domain fields: `String`, `i32`..`u64`, `f32`/`f64`, `Vec<String>`.
`bool` and `&str` (for non-domain borrowed input) allowed with care.

## Templates

### Value Object

``` `rust
use crate::common::taxonomy_validation_error::ValidationError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct <Name>(String);

impl <Name> {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ValidationError::empty("<Name>"));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for <Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
``` `

### Entity

``` `rust
use crate::common::taxonomy_validation_error::ValidationError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct <Name>(String);

impl <Name> {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ValidationError::empty("<Name>"));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for <Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
``` `

### Error

``` `rust
use thiserror::Error;

use crate::<domain>::taxonomy_<name>_vo::<VO>;

#[derive(Debug, Error)]
pub enum <Name>Error {
    #[error("Error message: {0}")]
    Variant(#[source] std::io::Error),
}
``` `

### Constants

``` `rust
/// Default value description.
pub const <NAME>_DEFAULT: f64 = 24.0;

/// Minimum value description.
pub const <NAME>_MIN: f64 = 0.5;

/// Filename constant.
pub const <NAME>_FILENAME: &str = "file.json";
``` `

## Workflow

1. Determine type (VO/Entity/Error/Event/Constant/Utility).
2. Create `taxonomy_<domain>_<type>.rs` in `shared/src/<domain>/`.
3. VOs: `fn new(...) -> Result<Self, DomainError>` or invariant check in `new`.
4. Errors: impl `std::error::Error` + `Display`.
5. Constants: `pub const NAME: Type = value;` only.
6. Register in `mod.rs`.
7. `cargo check -p <crate-name>`.

## Checklist

- [ ]  Correct suffix.
- [ ]  VOs validate on construction; composite VOs use other VOs (no raw primitives).
- [ ]  Errors implement `std::error::Error`.
- [ ]  Constants are `pub const` pure literal values.
- [ ]  No import from capabilities, agents, surface, root, contracts.
- [ ]  No I/O, network, or database in taxonomy files.
- [ ]  Registered in shared `mod.rs`.
- [ ]  `cargo check -p <crate-name>` passes.
```

---

# create-test-rust

**Files:** 1

## File List

- [.agents/skills/create-test-rust/SKILL.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-test-rust/SKILL.md)

---

## File: .agents/skills/create-test-rust/SKILL.md

```markdown
---
name: create-test-rust
description: "Generates contract, unit, integration, E2E, acceptance, and smoke test suites in tests/ (flat prefix naming), plus benchmark suites in benches/ (separate directory). Use when adding a new capability crate, increasing coverage, preparing a release, or validating performance. Triggers: create tests rust, add tests rust, create test suite rust, crate tests rust, e2e tests rust, benchmark rust."
metadata:
  tags: [rust, testing, criterion, contract, unit, integration, e2e, acceptance, smoke, benchmark]
  related: [create-test-python, create-test-typescript]
---

# Create Rust Test Suite

## Directory Layout

``` `
crates/<name>/
├── src/
│   └── capabilities_my_struct.rs   # NO inline tests. Clean.
├── tests/                          # All test types, flat prefix naming
│   ├── contract_<crate>.rs
│   ├── unit_<crate>_<module>.rs
│   ├── integration_<crate>.rs
│   ├── smoke_<app>.rs
│   ├── e2e_<flow>.rs
│   └── acceptance_<FR_id>.rs
├── benches/                        # Benchmark tests only
│   └── bench_<subject>.rs
└── Cargo.toml                      # [[bench]] path → benches/bench_*.rs
``` `

## Rules

- **Tests** (`tests/`): flat, prefix IS the virtual folder — no real subdirectories.
- **Benchmarks** (`benches/`): use `criterion` — never hand-rolled timing.
- Prefix pattern: `<type>_<subject>.rs`
- Contract tests verify trait implementation.
- Unit tests: happy path, edge cases, error paths.
- Integration tests: use real DI container.
- E2E tests: hit real entry point, assert on real output.
- Acceptance tests: map 1:1 to business requirement (FRD/PRD ID).
- Smoke tests: must complete in under 5 seconds.

## Cargo.toml for Benchmarks

``` `toml
[[bench]]
name = "bench_<subject>"
path = "benches/bench_<subject>.rs"
harness = false
``` `

## Test Types

| Prefix | Directory | Scope | Speed | Runs when |
| --- | --- | --- | --- | --- |
| `contract_` | tests/ | Trait impl exists | ms | Every PR |
| `unit_` | tests/ | One public function | ms | Every PR |
| `integration_` | tests/ | Crate / DI wiring | ms–s | Every PR |
| `smoke_` | tests/ | App boots + responds | <5s | Every PR |
| `e2e_` | tests/ | Full request lifecycle | s | Every PR (critical path) |
| `acceptance_` | tests/ | Business requirement met | s | Every PR / release gate |
| `bench_` | benches/ | Performance regression | s–min | Release gate / nightly |

## Coverage Targets

| Layer | Minimum |
| --- | --- |
| Capabilities | 70% |
| Agent | 60% |
| Utility | 50% |

## Workflow

``` `
- [ ] Step 1: Analyze crate / app structure
- [ ] Step 2: Identify untested public API
- [ ] Step 3: Write tests/contract_<crate>.rs
- [ ] Step 4: Write tests/unit_<crate>_<module>.rs
- [ ] Step 5: Write tests/integration_<crate>.rs
- [ ] Step 6: Write tests/smoke_<app>.rs
- [ ] Step 7: Write tests/e2e_<flow>.rs
- [ ] Step 8: Write tests/acceptance_<FR_id>.rs
- [ ] Step 9: Write benches/bench_<subject>.rs + register in Cargo.toml
- [ ] Step 10: cargo test --workspace
- [ ] Step 11: Verify coverage targets met
``` `
```

---

# create-utility-rust

**Files:** 1

## File List

- [.agents/skills/create-utility-rust/SKILL.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-utility-rust/SKILL.md)

---

## File: .agents/skills/create-utility-rust/SKILL.md

```markdown
---
name: create-utility-rust
description: "Create and validate Rust utility layer files following AES rules: stateless standalone functions, no struct, no trait impl, pure functions, domain-agnostic, reusable across modules."
metadata:
  tags: [rust, aes, utility, stateless, pure-functions, domain-agnostic, reusability, taxonomy]
  triggers:
    - "create utility rust"
    - "add utility rust"
    - "extract to utility rust"
    - "move to utility rust"
    - "check utility rust"
  dependencies: []
  related:
    - create-capabilities-rust
    - cleanup-consolidate-rust
---
# create-utility-rust

Utility = stateless standalone functions. No struct, no `impl`, no domain rules. File: `utility_<domain>_<role>.rs`.

**Allowed imports:** Taxonomy only (`shared::taxonomy_*`).
**Forbidden:** `use` from Capabilities, Agent, Surface, Contract, or other Utility modules.

## Examples role Naming

parser, splitter, trimmer, slugifier, sanitizer, normalizer, extractor, replacer, converter, counter, resolver, detector, builder, joiner, serializer, deserializer, encoder, decoder, hasher, generator, formatter, comparator, differ, matcher, checker, calculator, mapper, merger, grouper, sorter, deduplicator, printer

## Templates

### utility_name.rs

``` `rust
// PURPOSE: <Domain> utility functions — stateless, pure, domain-agnostic
// Free functions only — no struct, no impl blocks.
use shared::taxonomy::<domain>_vo::<VO>;

/// <Description of what this function does>
///
/// # Arguments
/// * `<param_name>` — <description>
///
/// # Returns
/// <description of return value>
pub fn <function_name>(<param_name>: &<Type>) -> <ReturnType> {
    // pure function logic here
}
``` `

## Rules

1. **Structure:** Only `pub fn` free functions — absolutely no `struct`, no `impl` blocks, no traits.
2. **State & Side Effects:** Stateless & deterministic. Side-effects are strictly limited to domain-agnostic operations (e.g., generic serialization, hashing, format conversion). No business logic, no `rand`, no `SystemTime::now()`, no global mutable state.
3. **Domain Awareness:** Domain-agnostic — no business rules, no layer-name knowledge.
4. **Reusability:** Must be used by ≥2 modules. If it has a single consumer, keep it as a private helper in the consuming module.
5. **I/O Constraint:** I/O is allowed ONLY if it strictly adheres to Rules 1, 2, and 3 (e.g., a generic JSON serializer, not a "save user to database" function).

## Helper vs Utility Decision Matrix

**Keep as private helper** (in Capabilities/Agent) if ANY of these apply:

- Uses `&self` or instance state.
- Domain-specific (contains business rules).
- Single consumer.

**Extract to Utility** ONLY if ALL of these apply:

- No `self` (stateless free function).
- Pure / deterministic (or domain-agnostic I/O).
- Domain-agnostic (no business rules).
- ≥2 consumers (reusable across modules).

## Workflow

1. Confirm ≥2 consumers, stateless, and domain-agnostic.
2. Create `utility_<domain>_<role>.rs`.
3. Register in `mod.rs`.
4. `cargo check -p <crate-name>`.

## Checklist

- [ ]  Only free functions — no struct, no impl, no traits.
- [ ]  No `&self`, no instance state.
- [ ]  Pure/deterministic (or I/O strictly limited to domain-agnostic ops like serialization/hashing).
- [ ]  No business rules or layer-name knowledge.
- [ ]  Used by ≥2 modules (not a single-consumer helper).
- [ ]  No `use` from Capabilities, Agent, Surface, or Contract.
- [ ]  No magic constants (→ move to `taxonomy_*_constant.rs`).
- [ ]  `cargo check -p <crate-name>` passes.
```

---

# fix-bypass-rust

**Files:** 1

## File List

- [.agents/skills/fix-bypass-rust/SKILL.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/fix-bypass-rust/SKILL.md)

---

## File: .agents/skills/fix-bypass-rust/SKILL.md

```markdown
---
name: fix-bypass-rust
description: "Fix Rust bypass comments (#[allow], unwrap, expect, panic) by addressing root causes instead of suppressing errors."
metadata:
  tags: [rust, bypass, comments, aes304, allow, unwrap]
  triggers:
    - "fix bypass rust"
    - "fix bypass comments rust"
    - "remove allow rust"
    - "remove unwrap rust"
  dependencies: []
  related:
    - cleanup-consolidate-rust
---

# fix-bypass-rust

## Rules

- NO `#[allow(...)]` allowed (except in config exceptions)
- NO `unwrap()` allowed
- NO `expect()` allowed
- NO `panic!()` allowed
- Fix the root cause instead

## Purpose

Remove `#[allow(...)]`, `unwrap()`, `expect()`, `panic!()` and fix the underlying issue.

## When to Use

- File has bypass comments
- File uses unwrap/expect/panic

## The Fundamental Question

> **"Is there a bypass comment or unsafe call?"**

If yes -> **Fix root cause and remove**

## Workflow

### Step 1: Find Bypass Comments

Read code and find bypass comments and unsafe calls.

### Step 2: Fix Root Cause

Fix underlying type/error.

### Step 3: Remove Comment/Call

Remove the bypass comment or unsafe call.

## Common Violations

| Violation               | Fix                                            |
| ----------------------- | ---------------------------------------------- |
| `#[allow(dead_code)]`   | Remove unused code or add to config exceptions |
| `#[allow(clippy::...)]` | Fix the clippy warning                         |
| `unwrap()`              | Use `?` or `match` for error handling          |
| `expect("msg")`         | Use `?` or `match` for error handling          |
| `panic!("msg")`         | Return `Result::Err` instead                   |
```

---

# lint-arwaky-rust

**Files:** 1

## File List

- [.agents/skills/lint-arwaky-rust/SKILL.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/lint-arwaky-rust/SKILL.md)

---

## File: .agents/skills/lint-arwaky-rust/SKILL.md

```markdown
---
name: lint-arwaky-rust
description: "Run lint-arwaky CLI scanner and MCP server for Rust projects — validate AES compliance, check layer violations, and fix architecture issues."
metadata:
  tags: [rust, lint, aes, compliance, scanning, mcp, clippy]
  triggers:
    - "lint arwaky rust"
    - "lint code rust"
    - "check compliance rust"
    - "scan rust project"
  dependencies: []
  related:
    - cleanup-consolidate-rust
    - build-verify-all
---

# lint-arwaky-rust — Complete Command & Argument Reference

Run linters (`clippy`, `rustfmt`, `lint-arwaky-cli`) and enforce 7-layer Architecture Enforcement System (AES) compliance rules for Rust crates and workspaces.

---

## Shell Aliases

Shortcut aliases are available for fast terminal access (automatically added to `~/.bashrc` / `~/.zshrc`):

| Alias | Target Binary | Description | Example Usage |
| :--- | :--- | :--- | :--- |
| `lac` | `lint-arwaky-cli` | Primary CLI gatekeeper & scanner | `lac scan .`, `lac fix crates/`, `lac ci` |
| `lat` | `lint-arwaky-tui` | Terminal User Interface (TUI) dashboard | `lat` |
| `lam` | `lint-arwaky-mcp` | MCP Server (STDIO backend for AI clients) | Configured in Claude / Cursor / Windsurf |

---

## 1. Global CLI Options

These options apply globally across all `lint-arwaky-cli` subcommands:

| Option | Long Flag | Description |
| :--- | :--- | :--- |
| `-v` | `--verbose` | Enable debug logging and detailed diagnostic traces. |
| `-q` | `--quiet` | Minimize console output (suppress non-error messages). |
| `-o` | `--output-dir <DIR>` | Directory to save generated reports (overrides active configuration). |
| | `--filter <CODE>` | Filter scan results by specific AES rule code (e.g. `AES101`, `AES301`, `AES401`). |
| `-h` | `--help` | Print help information for the CLI or specific subcommand. |
| `-V` | `--version` | Print CLI binary version. |

---

## 2. Complete Commands & Subcommands Reference

### `scan` / `check`
Scans target Rust workspace, discovers workspace members, and runs all linters.

``` `bash
# Basic scan (defaults to text format)
lint-arwaky-cli scan workspaces-bad/crates

# Scan with specific output format (text | json | sarif | junit)
lint-arwaky-cli scan workspaces-bad/crates --format json

# Scan single workspace member by name
lint-arwaky-cli scan workspaces-bad/crates --member shared

# Filter results by specific AES rule ID
lint-arwaky-cli scan workspaces-bad/crates --filter AES401

# Save reports to custom directory
lint-arwaky-cli scan workspaces-bad/crates --format json --output-dir ~/.local/share/lint-arwaky/reports
``` `

**Arguments & Flags**:
* `[PATH]`: Target path to scan (defaults to current directory `.`).
* `--format <FORMAT>`: Output format (`text`, `json`, `sarif`, `junit`).
* `--member <NAME>`: Target single workspace member by package name.
* `--filter <CODE>`: Filter violations by AES rule ID.
* `-o, --output-dir <DIR>`: Output directory path to save report files.

---

### `fix`
Applies safe automatic fixes to compliance violations across the codebase.

``` `bash
# Apply automatic fixes
lint-arwaky-cli fix crates/

# Preview changes without modifying files (Dry Run)
lint-arwaky-cli fix crates/ --dry-run

# Preview fixes for specific rule code
lint-arwaky-cli fix crates/ --dry-run --filter AES101
``` `

**Arguments & Flags**:
* `[PATH]`: Target path to fix (defaults to `.`).
* `--dry-run`: Perform a dry run showing diffs without modifying files.
* `--filter <CODE>`: Apply fixes only for a specific AES rule ID.

---

### `ci`
Continuous Integration quality gate mode. Evaluates compliance score against a threshold.

``` `bash
# CI mode with default threshold
lint-arwaky-cli ci crates/

# CI mode with custom score threshold (exits with status 1 if score < 80)
lint-arwaky-cli ci crates/ --threshold 80 --format junit
``` `

**Arguments & Flags**:
* `[PATH]`: Target path (defaults to `.`).
* `--threshold <SCORE>`: Minimum acceptable quality score (0–100, default: 80).
* `--format <FORMAT>`: Output format (`text`, `json`, `sarif`, `junit`).

---

### `quality`, `import`, `naming`, `role`, `orphan`, `external`
Run a single linter independently for targeted analysis.

``` `bash
# Run only naming rules
lint-arwaky-cli naming crates/

# Run only orphan detection with JSON output
lint-arwaky-cli orphan crates/ --format json

# Run orphan on a specific member
lint-arwaky-cli orphan crates/ --member shared_common

# Run only import rules on a specific path
lint-arwaky-cli import crates/code_analysis

# Run only role rules
lint-arwaky-cli role crates/

# Run only external linters (clippy)
lint-arwaky-cli external crates/

# Run only quality analysis
lint-arwaky-cli quality crates/
``` `

**Arguments & Flags**:
* `[PATH]`: Target path to scan (defaults to `.`).
* `--format <FORMAT>`: Output format (`text`, `json`, `sarif`, `junit`).
* `--member <NAME>`: (orphan only) Target specific workspace member.

---

### `security` & `dependencies`
Scans for security vulnerabilities and library dependency CVEs.

``` `bash
# Scan code for security issues (Bandit, Cargo Audit, ESLint Security)
lint-arwaky-cli security crates/

# Scan Rust library dependencies for vulnerabilities
lint-arwaky-cli dependencies crates/
``` `

---

### `watch`
Monitors file system changes and re-runs linting automatically upon file save.

``` `bash
# Watch directory and re-lint on changes
lint-arwaky-cli watch crates/
``` `

---

### `install-hook` & `uninstall-hook`
Manages Git pre-commit hook integration.

``` `bash
# Install git pre-commit hook
lint-arwaky-cli install-hook

# Uninstall git pre-commit hook
lint-arwaky-cli uninstall-hook
``` `

---

### `init` & `install`
Initializes workspace configuration and installs linter adapter dependencies.

``` `bash
# Create default lint_arwaky.config.yaml in workspace
lint-arwaky-cli init

# Install required external linter tools (clippy, rustfmt, etc.)
lint-arwaky-cli install
``` `

---

### `config-show`, `adapters`, & `mcp-config`
Displays workspace configuration and active integrations.

``` `bash
# Show active configuration tokens and rules
lint-arwaky-cli config-show

# List all active linter adapters (Clippy, Rustfmt, etc.)
lint-arwaky-cli adapters

# Print MCP server configuration JSON for AI client integration
lint-arwaky-cli mcp-config
``` `

---

### `doctor` & `version`
Environment diagnostic tools.

``` `bash
# Health check for Rust tooling and environment
lint-arwaky-cli doctor

# Display binary version information
lint-arwaky-cli version
``` `

---

## MCP Server Tools Reference (`lint-arwaky-mcp`)

`lint-arwaky-mcp` exposes 5 JSON-RPC 2.0 tools over STDIO for AI clients (Claude Code, Cursor, Windsurf, Hermes):

| Tool Name | Description | Arguments / Parameters |
| :--- | :--- | :--- |
| `execute_command` | Execute any CLI command action | `action` (required: `"scan"`, `"check"`, `"fix"`, `"security"`, `"doctor"`, etc.), `args` (optional JSON object, e.g. `{"path": "/abs/path"}`) |
| `list_commands` | List available CLI commands catalog | `domain` (optional: filter by domain string, e.g. `"setup"`, `"check"`) |
| `read_skill` | Read `SKILL.md` documentation by section | `section` (optional: header name to extract) |
| `health_check` | Check MCP server & adapter health | None (0 parameters) |
| `get_config` | Get active architecture config | `path` (optional project path), `language` (optional: `"rust"`, `"python"`, `"javascript"`) |

### Example MCP JSON-RPC Payload

``` `json
// execute_command: run Rust scan
{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"execute_command","arguments":{"action":"scan","args":{"path":"workspaces-bad/crates"}}}}

// health_check: check Rust adapters (clippy)
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"health_check","arguments":{}}}

// get_config: retrieve Rust architecture configuration
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_config","arguments":{"language":"rust"}}}
``` `

---

## 3. Native Rust Tooling Commands

``` `bash
# Auto-format Rust code
cargo fmt --all

# Check Clippy lints
cargo clippy --all-targets -- -D warnings

# Per-crate build/check/test
cargo check -p <crate-name>
cargo test -p <crate-name>
cargo test --workspace
``` `

---

## 4. Report Redirection & XDG Storage

Output can be saved directly to the XDG `reports` directory (`~/.local/share/lint-arwaky/reports/`):

``` `bash
# Save JSON report
lint-arwaky-cli scan crates/ --format json > ~/.local/share/lint-arwaky/reports/scan_rust.json

# Save SARIF report for GitHub Code Scanning
lint-arwaky-cli scan crates/ --format sarif > ~/.local/share/lint-arwaky/reports/scan_rust.sarif
``` `

---

## 5. Verification Checklist

- [ ] `cargo fmt --all` clean
- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] `cargo test --workspace` passes
- [ ] `lint-arwaky-cli scan .` reports 0 violations
```

---

