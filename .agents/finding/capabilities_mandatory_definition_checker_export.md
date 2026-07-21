# Export: crates/code-analysis/src/capabilities_mandatory_definition_checker.rs

This document contains the source code for `crates/code-analysis/src/capabilities_mandatory_definition_checker.rs` along with its resolved dependencies and related documentation.

## File List

**Selected file:**
- [crates/code-analysis/src/capabilities_mandatory_definition_checker.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/code-analysis/src/capabilities_mandatory_definition_checker.rs)

**Dependencies & related docs:**
- [.agents/skills/create-capabilities-rust/SKILL.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-capabilities-rust/SKILL.md)
- [.agents/skills/create-capabilities-rust/references/3-block-structure.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-capabilities-rust/references/3-block-structure.md)
- [.agents/skills/create-capabilities-rust/references/checklist.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-capabilities-rust/references/checklist.md)
- [.agents/skills/create-capabilities-rust/references/commands.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-capabilities-rust/references/commands.md)
- [.agents/skills/create-capabilities-rust/references/error-handling.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-capabilities-rust/references/error-handling.md)
- [.agents/skills/create-capabilities-rust/references/examples.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-capabilities-rust/references/examples.md)
- [.agents/skills/create-capabilities-rust/references/helper-vs-utility.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-capabilities-rust/references/helper-vs-utility.md)
- [.agents/skills/create-capabilities-rust/references/layer-boundaries.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-capabilities-rust/references/layer-boundaries.md)
- [.agents/skills/create-capabilities-rust/references/primitive-vo-policy.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-capabilities-rust/references/primitive-vo-policy.md)
- [ARCHITECTURE.md](file:///home/raka/mcp-arwaky/lint-arwaky/ARCHITECTURE.md)
- [PRD.md](file:///home/raka/mcp-arwaky/lint-arwaky/PRD.md)
- [crates/code-analysis/FRD.md](file:///home/raka/mcp-arwaky/lint-arwaky/crates/code-analysis/FRD.md)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_severity_vo.rs)
- [crates/shared/src/code-analysis/contract_class_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_class_protocol.rs)
- [crates/shared/src/code-analysis/contract_dead_inheritance_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_dead_inheritance_protocol.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs)
- [crates/shared/src/code-analysis/utility_bypass.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/utility_bypass.rs)
- [crates/shared/src/code-analysis/utility_mandatory.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/utility_mandatory.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_adapter_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_name_vo.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/common/taxonomy_definition_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_definition_vo.rs)
- [crates/shared/src/common/taxonomy_error_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_error_vo.rs)
- [crates/shared/src/common/taxonomy_job_id_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_job_id_vo.rs)
- [crates/shared/src/common/taxonomy_layer_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_layer_vo.rs)
- [crates/shared/src/common/taxonomy_lint_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_lint_vo.rs)
- [crates/shared/src/common/taxonomy_message_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_message_vo.rs)
- [crates/shared/src/common/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_path_vo.rs)
- [crates/shared/src/common/taxonomy_response_data_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_response_data_vo.rs)
- [crates/shared/src/common/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_severity_vo.rs)
- [crates/shared/src/common/taxonomy_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_source_vo.rs)
- [crates/shared/src/common/taxonomy_suggestion_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_suggestion_vo.rs)

---

## Selected File: crates/code-analysis/src/capabilities_mandatory_definition_checker.rs

```rust
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::code_analysis::utility_bypass::skip_cfg_test_block;
use shared::code_analysis::utility_mandatory::rust_declares_type;
use shared::taxonomy_definition_vo::LayerDefinition;

// PURPOSE: MandatoryDefinitionChecker — AES303: enforce struct/enum/trait/class/interface/type definitions exist AND are non-empty.
// Sub-check 1: file must define at least one struct/enum/trait/type (Rust) or class/interface/type (JS/TS)/class (Python) (IMandatoryClassProtocol).
// Sub-check 2: empty unit struct (struct Foo;) and empty class/interface (class Foo: pass, class Foo {}, interface {}) flagged as dead inheritance.
// ALGORITHM (check_mandatory_class_definition):
//   1. Skip barrel/constant files (mod.rs, __init__.py, _constant.*)
//   2. If no LayerDefinition or mandatory_class_definition disabled → skip
//   3. Check if filename is in exception list
//   4. Scan passed content for class/struct/trait/enum keyword declarations
//   5. If none found → AES303 MANDATORY_DEFINITION
// ALGORITHM (check_dead_inheritance):
//   1. Iterate lines; skip #[cfg(test)] blocks
//   2. For each `struct Foo;` (unit struct) → flag unless followed by impl block
//   3. For each `class Foo: pass` (Python empty class) → flag
//   4. For each `class Foo {}` (JS/TS empty class) → flag
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct MandatoryDefinitionChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

/// AES303 sub-check 2: detect empty struct/impl blocks (dead/empty definitions)
impl IDeadInheritanceProtocol for MandatoryDefinitionChecker {
    fn check_dead_inheritance(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let t = lines[i].trim();
            // Skip #[cfg(test)] modules correctly — advance past the entire block
            if t.starts_with("#[cfg(test)]") {
                i = skip_cfg_test_block(&lines, i);
                continue;
            }
            // Rust: unit struct `struct Foo;` or `pub struct Foo;` (tuple structs excluded)
            let stripped = Self::strip_visibility(t);
            if stripped.starts_with("struct ") && stripped.ends_with(';') && !stripped.contains('(')
            {
                // Skip if followed by impl block or attribute (intentional placeholder)
                let mut next_idx = i + 1;
                while next_idx < lines.len() {
                    let next_t = lines[next_idx].trim();
                    if next_t.is_empty() || next_t.starts_with('#') {
                        next_idx += 1;
                    } else {
                        break;
                    }
                }
                let next_is_impl = match lines.get(next_idx) {
                    Some(l) => l.trim().starts_with("impl "),
                    None => false,
                };
                if !next_is_impl {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES303",
                        Severity::MEDIUM,
                        AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                    ));
                }
                i += 1;
                continue;
            }
            // Python: empty class `class Foo: pass` (single line or multi-line)
            if t.starts_with("class ") || t.starts_with("class\t") {
                if t.ends_with(": pass") || t.ends_with(":pass") {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES303",
                        Severity::MEDIUM,
                        AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                    ));
                } else if t.ends_with(':') && i + 1 < lines.len() {
                    let next = lines[i + 1].trim();
                    if next == "pass" || next == "..." || next == "Ellipsis" {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES303",
                            Severity::MEDIUM,
                            AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                        ));
                    }
                }
            }
            // JS/TS: empty class/interface `class Foo {}`, `export class Foo {}`, `interface Bar {}`
            if Self::is_empty_js_declaration(t) {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES303",
                    Severity::MEDIUM,
                    AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                ));
            }
            i += 1;
        }
    }
}

/// AES303 sub-check 1: file must have at least one struct/enum/trait/class definition
impl IMandatoryClassProtocol for MandatoryDefinitionChecker {
    fn check_mandatory_class_definition(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = match Path::new(file).file_name().and_then(|f| f.to_str()) {
            Some(name) => name.to_string(),
            None => return,
        };

        if matches!(
            basename.as_str(),
            "__init__.py" | "main.py" | "py.typed" | "mod.rs" | "lib.rs" | "main.rs"
        ) {
            return;
        }
        if basename.ends_with("_constant.rs") || basename.ends_with("_constant.py") {
            return;
        }

        let def = match definition {
            Some(d) => d,
            None => return,
        };
        if !def.code_analysis.mandatory_class_definition.value {
            return;
        }
        if def.exceptions.values.contains(&basename) {
            return;
        }

        let mut has_class = false;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("class ")
                || trimmed.starts_with("export class ")
                || trimmed.starts_with("export default class ")
                || trimmed.starts_with("interface ")
                || trimmed.starts_with("export interface ")
                || trimmed.starts_with("type ")
                || trimmed.starts_with("export type ")
                || rust_declares_type(trimmed)
            {
                has_class = true;
                break;
            }
        }

        if !has_class {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES303",
                Severity::HIGH,
                AesCodeAnalysisViolation::MandatoryClassDefinition { reason: None }.to_string(),
            ));
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for MandatoryDefinitionChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl MandatoryDefinitionChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Strip Rust visibility modifiers from the beginning of a line.
    /// Handles `pub`, `pub(crate)`, `pub(crate)`, `pub(super)`, etc.
    /// P1.10 fix: enables detection of `pub struct Foo;` as unit struct.
    fn strip_visibility(line: &str) -> &str {
        let trimmed = line.trim();
        if trimmed.starts_with("pub ") || trimmed.starts_with("pub(") {
            // Skip past the visibility modifier
            if let Some(rest) = trimmed.strip_prefix("pub ") {
                rest
            } else if let Some(rest) = trimmed.strip_prefix("pub(") {
                // Find closing paren for pub(crate), pub(super), etc.
                if let Some(end_paren) = rest.find(')') {
                    let after = &rest[end_paren + 1..];
                    // Skip any whitespace after the closing paren
                    after.trim_start()
                } else {
                    trimmed
                }
            } else {
                trimmed
            }
        } else {
            trimmed
        }
    }

    /// Detect JS/TS empty class or interface declarations.
    /// Handles `class Foo {}`, `export class Foo {}`, `export default class Foo {}`.
    /// P1.11 fix: replaces simple `t.starts_with("class ")` check.
    fn is_empty_js_declaration(line: &str) -> bool {
        let code = line
            .split_once("//")
            .map(|(code, _comment)| code)
            .unwrap_or(line);

        let compact: String = code.split_whitespace().collect();

        compact.ends_with("{}") && Self::js_ts_declares_primary_symbol(code)
    }

    /// Detect JS/TS primary symbols: class or interface.
    fn js_ts_declares_primary_symbol(line: &str) -> bool {
        let code = line
            .split_once("//")
            .map(|(code, _comment)| code)
            .unwrap_or(line);

        let tokens: Vec<&str> = code.split_whitespace().collect();

        if let Some(pos) = tokens
            .iter()
            .position(|tok| *tok == "class" || *tok == "interface")
        {
            if pos == 0 {
                return true;
            }

            return matches!(
                tokens[pos - 1],
                "export" | "default" | "abstract" | "declare"
            );
        }

        false
    }
}
```

---

## File: .agents/skills/create-capabilities-rust/SKILL.md

```markdown
---
name: create-capabilities-rust
description: "Create and validate Rust capabilities layer files following AES rules: concrete implementation of behavior (business logic + external adaptation), 3-block structure, one impl struct per file, protocol trait contracts, DI for service dependencies, and shared VOs for domain data."
metadata:
    tags: [rust, aes, capability, protocol, structure, 3-block-structure, di, vo, role-naming]
    triggers:
        - "create capability rust"
        - "add capability rust"
        - "fix capability structure rust"
        - "create trait rust"
        - "capability missing trait rust"
        - "check capabilities rust"
        - "audit capabilities rust"
    dependencies: []
    related:
        - create-agent-rust
        - create-contract-rust
        - create-taxonomy-rust
---

# create-capabilities-rust

## Purpose

Create and validate Rust **capabilities layer** files following AES rules.

A capabilities file contains the **concrete implementation** of the system's behavior. This layer encapsulates both:

- **Business logic**: computations, validations, transformations, assessments
- **External adaptation**: database access, third-party API calls, file system access, infrastructure mechanics

Capabilities hide these implementations behind Contracts, keeping behavior modular, swappable, and fully isolated from orchestration.

A capabilities file must:

- implement one domain protocol trait,
- follow strict 3-block structure,
- use dependency injection for service collaborators (`Arc<dyn Trait>`),
- use shared VOs for domain data,
- use Utility standalone functions for low-level technical operations.

## Role Naming (ARCHITECTURE §8)

Capabilities use role suffixes describing their concern. Two families:

**Internal (business logic):**

validator, assessor, calculator, resolver, classifier, selector, mapper, transformer, policy, enricher, evaluator, analyzer, scorer, grader, ranker, filter, checker, reviewer, approver, rejector

**External (adaptation):**

repository, gateway, client, provider, fetcher, reader, writer, scanner, executor, publisher, subscriber, adapter, connector, uploader, downloader, sender, receiver, dispatcher, watcher, monitor

File: `capabilities_<domain>_<role>.rs`

## Dependencies (ARCHITECTURE §8)

- **May depend on:** Taxonomy, Contract, Utility.
- **Must NOT depend on / import:** other Capabilities, Agent.

Note: do **not** import `infrastructure_*` — that layer no longer exists; its mechanics now live in the Utility layer. Utility is an allowed dependency.

## Special Rules (ARCHITECTURE §8)

- **No Inter-Capability Dependency:** a capability never imports or calls another capability. They are standalone execution units.
- **Pipeline Aggregation:** multiple capabilities are composed into a sequential pipeline by the **Agent layer**, not by themselves.
- **Shared Logic Extraction (DRY):** if several capabilities need the same technical mechanics, extract it into a reusable standalone function in the **Utility layer**. Capabilities must not duplicate technical code.
- **Contract Implementation:** the capability implements the `protocol_` trait defined in the Contract layer.
- **State Ownership:** the capability owns business and technical state within its execution scope.
- **Utility Delegation:** low-level technical operations call Utility standalone functions, passing state/data as arguments.
- **No Orchestration:** no flow control across capabilities (looping/branching between capabilities) and no error-escalation policy. Execute one responsibility, return a result.
- **No Domain Definition:** do not define domain models (Entities, Value Objects); only consume and produce Taxonomy.
- **Constant Extraction:** extract reusable constants (magic strings, numbers, patterns) into `taxonomy_<domain>_constant.rs` in shared. Capabilities must not contain magic constants.

## Definition of Done

1. ONE implementation struct per file.
2. Struct implements ONE domain protocol trait in Block 2.
3. Block 2 contains ONLY the domain protocol trait implementation.
4. Constructors, std trait impls, private helpers in Block 3.
5. No locally defined domain models — Entities/Value Objects are consumed from Taxonomy, not defined here.
6. Service dependencies use DI via `Arc<dyn Trait>`.
7. Value/configuration fields use shared VOs.
8. No inter-capability dependencies (capabilities must not import other capabilities or Agent).
9. Low-level technical operations delegate to Utility standalone functions.
10. Reusable constants extracted to `taxonomy_<domain>_constant.rs` in shared.
11. `cargo check -p <crate-name>` passes.

## References

Read these files for detailed rules:

| File | Content |
|------|---------|
| `references/layer-boundaries.md` | Allowed/Forbidden imports and dependencies |
| `references/3-block-structure.md` | Block 1/2/3 definitions, method placement rules |
| `references/helper-vs-utility.md` | Helper vs utility decision, I/O Blocker, decision tree |
| `references/primitive-vo-policy.md` | Primitive policy table, VO construction rules |
| `references/error-handling.md` | Error handling rules with examples |
| `references/examples.md` | All BAD/GOOD code examples |
| `references/commands.md` | Quick heuristic check commands |
| `references/checklist.md` | Verification checklist |

## Templates

Use these templates when creating new files:

| File | Purpose |
|------|---------|
| `templates/capabilities_name.rs` | New capabilities implementation file |
| `templates/contract_name_protocol.rs` | New protocol trait definition |
| `templates/mod.rs` | Module registration |

## Workflow

### Step 1: Analyze File Responsibility

Read the file and ask: **"Does this implement protocol behavior?"**

If yes → keep as capabilities. If no → check if it's orchestration (→ agent), domain data (→ taxonomy), or pure technical mechanics (→ utility).

### Step 2: Check Missing Trait (AES403)

Does the capability struct implement a protocol trait? If no → create one.

### Step 3: Create Trait File if Missing

Create `contract_<name>_protocol.rs` in the appropriate shared domain folder.

### Step 4: Enforce 3-Block Structure

Reorganize: struct definition → domain protocol trait impl → constructors/std traits/helpers.

### Step 5: Verify Struct Discipline

One struct, no local data structs, DI via `Arc<dyn Trait>`, shared VOs.

### Step 6: Verify Helper vs Utility Boundary

See `references/helper-vs-utility.md` for the decision tree.

### Step 7: Verify Layer Compliance

No forbidden imports (Agent, other capabilities), no inter-capability dependencies, no business logic leakage, no domain model definition.

### Step 8: Verify Error Handling, VO, and Constants

See `references/error-handling.md` and `references/primitive-vo-policy.md`.

### Step 9: Verify Compilation

``` `bash
cargo check -p <crate-name>
``` `

## Quick Commands

``` `bash
# Check forbidden imports (no agent, no other capabilities)
rg "^\s*use\s+.*(agent_|capabilities_)" crates/<crate>/src/capabilities_*.rs

# List protocol trait implementations
rg -n "impl\s+I[A-Za-z0-9_]+Protocol\s+for" crates/<crate>/src/capabilities_*.rs
``` `

## Common Mistakes

- Importing other capabilities or Agent directly.
- Defining domain models (Entities, Value Objects) in capabilities files.
- Using concrete service types as struct fields.
- Using raw primitives for domain value fields.
- Putting private helpers in the protocol trait.
- Putting constructors in the protocol trait.
- Placing std trait impls before the domain protocol trait.
- Mixing Block 2 and Block 3 responsibilities.
- Flow control across capabilities / error-escalation policy (orchestration).
- Silent error swallowing with `unwrap_or_default()`.
- Magic constants in capabilities logic (extract to `taxonomy_<domain>_constant.rs`).
- Not delegating low-level technical operations to Utility.
```

---

## File: .agents/skills/create-capabilities-rust/references/3-block-structure.md

```markdown
# The 3-Block Structure

Every implementation file MUST follow this order with mandatory block markers:

1. **Block 1 — Struct Definition**
2. **Block 2 — Domain Protocol Trait Implementation**
3. **Block 3 — Constructors, Std Traits, and Private Helpers**

Each block MUST be preceded by a block marker comment:

``` `rust
// ─── Block 1: Struct Definition ───────────────────────────
``` `

``` `rust
// ─── Block 2: Protocol Trait Implementation ───────────────
``` `

``` `rust
// ─── Block 3: Constructors, Helpers, Private Methods ──────
``` `

## Block 1 — Struct Definition

``` `rust
// ─── Block 1: Struct Definition ───────────────────────────
pub struct Capabilities<NameCapability>;
``` `

Or with dependencies:

``` `rust
// ─── Block 1: Struct Definition ───────────────────────────
pub struct Capabilities<NameCapability> {
    collaborator: Arc<dyn I<NameCollaborator>Protocol>,
    store: Arc<dyn I<NameStore>Protocol>,
    policy: <NamePolicy>VO,
}
``` `

## Block 2 — Public Contract

Block 2 is RESERVED for the domain protocol trait ONLY.

``` `rust
// ─── Block 2: Protocol Trait Implementation ───────────────
impl I<NameCapability>Protocol for Capabilities<NameCapability> {
    fn execute(
        &self,
        input: &<DomainVO>,
        output: &mut Vec<<ResultVO>>,
    ) {
        // domain behavior
    }
}
``` `

Do NOT put these in Block 2:

``` `rust
impl Default for Capabilities<NameCapability>
impl Clone for Capabilities<NameCapability>
impl Debug for Capabilities<NameCapability>
impl Display for Capabilities<NameCapability>
impl From<...> for Capabilities<NameCapability>
``` `

Those belong in Block 3.

## Block 3 — Constructors, Std Traits, and Helpers

Block 3 contains:

- `new()`
- builders
- `Default`, `Clone`, `Debug`, `Display`
- other std trait impls
- private helper methods
- domain-specific associated functions used only by this struct

``` `rust
// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl Default for Capabilities<NameCapability> {
    fn default() -> Self {
        Self
    }
}

impl Capabilities<NameCapability> {
    pub fn new() -> Self {
        Self
    }

    fn effective_threshold(&self, input: &<DomainVO>) -> <Threshold>VO {
        // private helper
    }
}
``` `

Block 3 MUST NOT:

- define domain models (Entities, Value Objects) — that is **No Domain Definition** (ARCHITECTURE §8); consume them from Taxonomy instead.
- perform orchestration — no flow control across capabilities, no error-escalation policy (**No Orchestration**, ARCHITECTURE §8).
- duplicate technical mechanics that belong in a Utility standalone function (**DRY**, ARCHITECTURE §8).

## Trait Placement Decision Rule

``` `text
Trait impl found in a capabilities file?
  │
  ├─ Is it the domain protocol? (I<Name>Protocol)
  │   └─ YES → Block 2
  │
  └─ Is it a std/derive/utility trait? (Default, Clone, Debug, Display, From, etc.)
      └─ YES → Block 3
``` `

## Method Placement Decision Rule

``` `text
Method / function found in a capabilities file?
  │
  ├─ Is it defined as the domain protocol trait method?
  │   └─ YES → Block 2
  │
  ├─ Is it a std/derive trait impl?
  │   └─ YES → Block 3
  │
  ├─ Is it a factory method? (new(), builder)
  │   └─ YES → Block 3
  │
  └─ Is it a private helper?
      └─ YES → Block 3
``` `
```

---

## File: .agents/skills/create-capabilities-rust/references/checklist.md

```markdown
# Verification Checklist

- [ ] File name follows role naming `capabilities_<domain>_<role>.rs`
- [ ] File follows the 3-Block Structure.
- [ ] Block 1 contains exactly one implementation struct.
- [ ] Block 2 contains ONLY the domain protocol trait implementation.
- [ ] Block 3 contains constructors, std traits, and private helpers.
- [ ] Capability struct implements a protocol trait
- [ ] Trait contains only public domain contract methods.
- [ ] Private helpers are not declared in the trait.
- [ ] Constructors are not declared in the trait.
- [ ] Std trait impls are in Block 3.
- [ ] Domain-specific helpers may remain in Block 3.
- [ ] Reusable, stateless, domain-agnostic functions are extracted to `utility_.rs`.
- [ ] No reusable utility-like functions remain inside Block 3.
- [ ] Generic trait methods are object-safe or bounded with `where Self: Sized`.
- [ ] One file contains exactly one implementation struct.
- [ ] No domain data structures are defined locally.
- [ ] All domain data structures are imported from shared/taxonomy.
- [ ] Service dependencies use `Arc<dyn Trait>`.
- [ ] Value/configuration fields use shared VOs.
- [ ] Low-level technical operations delegate to `utility_*`Reusable technical mechanics extracted to Utility, not duplicated across capabilities (DRY).
- [ ] No forbidden imports from `agent_*`.
- [ ] No direct dependency on concrete `capabilities_*` (No Cross-Capability Dependency).
- [ ] No orchestration inside the capability
- [ ] No domain models (Entities/ValueObjects/Event/Error/Constant) defined in the file — consumed from Taxonomy only.
- [ ] Trait module is registered in the shared crate's `mod.rs`.
- [ ] `cargo check -p <crate-name>` passes.
```

---

## File: .agents/skills/create-capabilities-rust/references/commands.md

```markdown
# Quick Commands

These commands are rough heuristic checks. Final validation should use `cargo check`, clippy, or AST-based tooling.

``` `bash
# Check possible I/O in capabilities (AES404)
rg "std::fs|File::open|reqwest|hyper|sqlx|rusqlite" crates/<crate>/src/capabilities_*.rs

# Check forbidden imports
rg "^\s*use\s+.*agent_" crates/<crate>/src/capabilities_*.rs

# List structs in capabilities files
rg -n "^\s*pub struct" crates/<crate>/src/capabilities_*.rs

# List protocol trait implementations
rg -n "impl\s+I[A-Za-z0-9_]+Protocol\s+for" crates/<crate>/src/capabilities_*.rs

# Check capability filename follows role naming capabilities_<domain>_<role>.rs
rg -n --files-with-matches "capabilities_[a-z_]+_[a-z_]+\.rs$" crates/<crate>/src/capabilities_*.rs || echo "FILENAME VIOLATION"

# Check for inter-capability dependency (forbidden)
rg "^\s*use\s+.*capabilities_" crates/<crate>/src/capabilities_*.rs

# Check for orchestration anti-patterns in capabilities (No Orchestration, §8)
rg "for\s+.*\s+in|while\s+|\.escalate\(|error_escalation" crates/<crate>/src/capabilities_*.rs

# Find unwrap_or_default usage
rg "unwrap_or_default\(\)" crates/<crate>/src/capabilities_*.rs

# Find possible magic numbers
rg "[0-9]+\.[0-9]+" crates/<crate>/src/capabilities_*.rs

# Check object safety issues
cargo check -p <crate-name> 2>&1 | rg "cannot be made into an object"
``` `

## Check Wrong Block Order

``` `bash
for file in crates/<crate>/src/capabilities_*.rs; do
  awk '
    FNR == 1 { std = 0; proto = 0 }
    /^impl (Default|Clone|Debug|Display)/ { if (!std) std = FNR }
    /^impl I[A-Z].*Protocol/ { if (!proto) proto = FNR }
    END {
      if (std && proto && std < proto) {
        print "VIOLATION: " FILENAME " std trait (line " std ") before protocol (line " proto ")"
      }
    }
  ' "$file"
done
``` `
```

---

## File: .agents/skills/create-capabilities-rust/references/error-handling.md

```markdown
# Error Handling Rules

Capabilities error handling must be explicit.

## Rule 1: Do not silently discard errors

Forbidden:

``` `rust
let value = result.unwrap_or_default();
``` `

Forbidden:

``` `rust
let value = result.ok().unwrap_or_default();
``` `

## Rule 2: Fallible operations should return `Result`

If a method represents an operation that can fail unexpectedly, return `Result<T, E>`.

``` `rust
fn parse_input(content: &<RawContent>VO) -> Result<<DomainVO>, <Name>ParseError> {
    // ...
}
``` `

## Rule 3: Check/analysis methods may return `Vec<<ResultVO>>`

## Rule 3: Analysis methods may return a collection of `<ResultVO>`

``` `rust
fn execute(input: &<DomainVO>) -> Vec<<ResultVO>> {
    let mut results = Vec::new();
    // analysis logic
    results
}
``` `

## Rule 4: I/O errors belong to utility implementations (infrastructure layer removed)

Bad in capabilities:

``` `rust
fn read_input(path: &<Path>VO) -> Vec<<ResultVO>> {
    let content = std::fs::read_to_string(path.value()).unwrap_or_default(); // BAD
    Vec::new()
}
``` `

Good:

``` `rust
// utility_source_reader.rs
impl I<NameReader>Protocol for FileSystem<NameReader> {
    fn read(&self, path: &<Path>VO) -> Result<<SourceContent>VO, <Name>ReadError> {
        let raw = std::fs::read_to_string(path.value())
            .map_err(<Name>ReadError::Io)?;
        <SourceContent>VO::new(path.clone(), raw)
            .map_err(<Name>ReadError::Validation)
    }
}
``` `

``` `rust
// capabilities_<name>.rs
impl I<NameCapability>Protocol for <NameCapability> {
    fn execute(&self, source: &<SourceContent>VO) -> Vec<<ResultVO>> {
        // pure analysis using already-read source
        Vec::new()
    }
}

```

---

## File: .agents/skills/create-capabilities-rust/references/examples.md

```markdown
# Examples

## BAD: Capability Without Trait (AES403)

``` `rust
pub struct <NameComposer>;

impl <NameComposer> {
    pub fn compose_frame(&self) {
        // public behavior without protocol trait
    }
}
``` `

Fix:

``` `rust
pub struct <NameComposer>;

impl I<NameComposer>Protocol for <NameComposer> {
    fn compose_frame(&self) {
        // contract implementation
    }
}
``` `

## BAD: I/O in Capabilities (AES404)

``` `rust
impl <NameCapability> {
    fn process(&self) {
        let content = std::fs::read_to_string("file.txt"); // FORBIDDEN
    }
}
``` `

Fix: Move I/O to utility.

## BAD: Data Class Defined in Layer File

``` `rust
pub struct <NameResult> {
    is_valid: bool,
    reason: String,
}
``` `

Fix: Move to shared taxonomy, then import.

## BAD: Concrete Service Field

``` `rust
pub struct Capabilities<NameCapability> {
    collaborator: <NameCollaborator>, // BAD
}
``` `

Fix:

``` `rust
pub struct Capabilities<NameCapability> {
    collaborator: Arc<dyn I<NameCollaborator>Protocol>,
}
``` `

## BAD: Std Trait in Block 2

``` `rust
pub struct Capabilities<NameCapability>;

impl Default for Capabilities<NameCapability> {
    fn default() -> Self { Self }
}

impl I<NameCapability>Protocol for Capabilities<NameCapability> {
    fn execute(&self, ...) { // ...
    }
}
``` `

Fix: Move `Default` to Block 3.

## BAD: Orchestration Inside Capability (No Orchestration, §8)

``` `rust
impl <NamePipeline>Protocol for <NamePipeline> {
    fn run(&self) {
        let a = self.step_a();      // calls another capability's behavior
        if a.is_ok() {
            self.step_b();          // branching between capabilities
        } else {
            self.escalate();        // error-escalation policy
        }
    }
}
``` `

Fix: remove flow control and cross-capability calls. Let the Agent layer compose the pipeline. The capability executes one responsibility and returns a result.

## BAD: Domain Model Defined in Capability (No Domain Definition, §8)

``` `rust
pub struct <NameResult> {   // domain model defined here = forbidden
    is_valid: bool,
    reason: String,
}
``` `

Fix: define `<NameResult>` as a Taxonomy VO; the capability only consumes and produces it.

## GOOD: Capability with DI and Shared VO

``` `rust
use std::sync::Arc;

use shared::<name-feature>::taxonomy_<name>_vo::<Name>VO;
use shared::<name-feature>::contract_<name>_protocol::I<Name>Protocol;
use shared::<name-feature>::taxonomy_<name-collaborator>_protocol::I<NameCollaborator>Protocol;
use shared::<name-feature>::taxonomy_<name-store>_protocol::I<NameStore>Protocol;

pub struct Capabilities<NameCapability> {
    collaborator: Arc<dyn I<NameCollaborator>Protocol>,
    store: Arc<dyn I<NameStore>Protocol>,
    policy: <NamePolicy>VO,
}

impl I<NameCapability>Protocol for Capabilities<NameCapability> {
    // public contract methods only
}
``` `

## GOOD: Correct 3-Block Structure

``` `rust
use std::sync::Arc;

use shared::<name-feature>::taxonomy_<domain>_vo::<DomainVO>;
use shared::<name-feature>::contract_<name>_protocol::I<NameCapability>Protocol;
use shared::<name-feature>::taxonomy_<name-utility>::<name>_utility;
use shared::<name-feature>::taxonomy_<result>_vo::<ResultVO>;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct Capabilities<NameCapability>;

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl I<NameCapability>Protocol for Capabilities<NameCapability> {
    fn execute(
        &self,
        input: &<DomainVO>,
        output: &mut Vec<<ResultVO>>,
    ) {
        let key = input.key();
        if <name>_utility(key) {
            return;
        }
        // Remaining domain logic...
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for Capabilities<NameCapability> {
    fn default() -> Self { Self }
}

impl Capabilities<NameCapability> {
    pub fn new() -> Self { Self }

    fn is_relevant(&self, input: &<DomainVO>) -> bool {
        // Private helper specific to this capability.
        true
    }
}
``` `
```

---

## File: .agents/skills/create-capabilities-rust/references/helper-vs-utility.md

```markdown
# Helper vs Utility Decision

The boundary is not only about `&self`.

The real question is:

> Does this function know about specific business/domain rules, or is it just a blind reusable tool?
>
> AND
>
> Is it used only by this struct, or by multiple modules?

## Keep as Private Helper in Block 3

Keep the function inside the capabilities file if ANY of these is true:

1. It contains business/domain rules.
2. It accesses `self.field` or instance state.
3. It is tightly coupled to this capability only.
4. It is a factory method such as `new()` or builder method.
5. It is stateless but only used by this one struct and is domain-specific.

Example:

``` `rust
impl ContractRoleChecker {
    fn resolve_scope(scope: &str) -> (&str, Vec<&str>) {
        // Domain-specific parsing logic.
        // Even without `&self`, this can remain a private helper
        // if only this checker uses it.
    }
}
``` `

## Extract to Utility (`utility_.rs`)

Extract the function to shared taxonomy utility ONLY if ALL of these are true:

1. Stateless: no `&self`, no struct field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business rules.
5. Reusable: useful for multiple capabilities/utilities/modules.

Example:

``` `rust
// shared/<name-feature>/taxonomy_string_utility.rs
pub fn match_whole_token(haystack: &str, needle: &str) -> bool {
    // generic token matching
}
``` `

## I/O Rule

A function with I/O can be a  utility if it is stateless, domain-agnostic, and reusable.

``` `rust
// OK in taxonomy utility — stateless, domain-agnostic, reusable
pub fn walk_source_files(dir: &Path, files: &mut Vec<FilePath>, ignored: &[String]) {
    // I/O is allowed in taxonomy utilities
}
``` `

Rule:

``` `text
Stateless + I/O + domain-agnostic + reusable = taxonomy utility
Stateless + I/O + domain-specific = utility implementation (infrastructure layer removed)
``` `

## Decision Tree

``` `text
Found reusable code in capabilities?
  │
  ├─ Does it know business/domain rules?
  │   └─ YES → keep as private helper in Block 3
  │
  ├─ Does it need &self or struct state?
  │   └─ YES → keep as helper/method in Block 3
  │
  └─ Is it stateless, domain-agnostic, and reusable?
      └─ YES → extract to shared taxonomy utility (I/O allowed)
``` `
```

---

## File: .agents/skills/create-capabilities-rust/references/layer-boundaries.md

```markdown
# Layer Boundaries (AES)

## Capabilities Layer (`capabilities_*.rs`)

| Allowed                                      | Forbidden                                             |
| -------------------------------------------- | ----------------------------------------------------- |
| Computation, validation, calculation         | Direct import from `agent_*`                          |
| Data transformation, business rules          | Direct dependency on concrete `capabilities_*` modules |
| Domain behavior using shared models          | Locally defined domain data structures                |
| Protocol trait implementation                |                                                       |
| External adaptation (I/O, API calls, DB)     |                                                       |
| Private helpers supporting the impl struct   |                                                       |
| Calling injected protocol traits             |                                                       |
| Calling Utility standalone functions         |                                                       |

## Allowed Dependencies (ARCHITECTURE §8)

A capability may depend ONLY on these layers:

- **Taxonomy** — VOs, constants, entities, events
- **Contract** — protocol traits, aggregate interfaces
- **Utility** — standalone stateless functions (the former infrastructure mechanics now live here)

## Forbidden Dependencies (ARCHITECTURE §8)

- concrete **agent** implementations — capabilities must not import or know about the Agent layer
- concrete **capabilities** implementations — no inter-capability dependencies; capabilities never import each other

## Special Rules (ARCHITECTURE §8)

- **No Inter-Capability Dependency:** a capability never imports or calls another capability. They are standalone execution units.
- **Pipeline Aggregation:** multiple capabilities are composed into a sequential pipeline by the **Agent layer**, not by themselves.
- **Shared Logic Extraction (DRY):** if several capabilities need the same technical mechanics, extract it into a reusable standalone function in the **Utility layer**. Capabilities must not duplicate technical code.
- **Contract Implementation:** the capability implements the `protocol_` trait defined in the Contract layer.
- **State Ownership:** the capability owns business and technical state within its execution scope.
- **Utility Delegation:** low-level technical operations call Utility standalone functions, passing state/data as arguments.
- **No Orchestration:** no flow control across capabilities (looping/branching between capabilities) and no error-escalation policy. Execute one responsibility, return a result.
- **No Domain Definition:** do not define domain models (Entities, Value Objects); only consume and produce Taxonomy.
```

---

## File: .agents/skills/create-capabilities-rust/references/primitive-vo-policy.md

```markdown
# Primitive-to-VO Replacement Rules (AES402)

## General Rule

Domain data MUST use shared VOs, not raw primitives.

Bad:

``` `rust
pub struct <ResultVO> {
    pub target: String,
    pub position: u32,
    pub level: String,
}
``` `

Good:

``` `rust
pub struct <ResultVO> {
    target: <Target>VO,
    position: <LineNumber>VO,
    level: <Severity>VO,
}
``` `

## Primitive Policy

| Primitive            | Rule                                                                                |
| -------------------- | ----------------------------------------------------------------------------------- |
| `String`           | Forbidden for domain fields and public contract return values. Use VO.              |
| `i32`, `i64`     | Forbidden. Use domain VO.                                                           |
| `u32`, `u64`     | Forbidden. Use domain VO.                                                           |
| `usize`, `isize` | Forbidden for domain values. Use domain VO.                                         |
| `f32`, `f64`     | Forbidden. Use domain VO.                                                           |
| `char`             | Forbidden for domain values. Use domain VO.                                         |
| `bool`             | Allowed for semantic toggles when no richer VO is needed.                           |
| `&str`             | May be allowed for borrowed low-level input, but domain identifiers should use VOs. |

Prefer VOs for:

- file paths
- symbol names
- messages
- line numbers
- column numbers
- severity
- durations
- counts
- thresholds
- identifiers

## VO Construction Rules

VOs MUST validate on construction when the domain has invariants.

Good:

``` `rust
impl <LineNumber>VO {
    pub fn new(value: u32) -> Result<Self, ValidationError> {
        if value == 0 {
            return Err(ValidationError::positive("<LineNumber>VO"));
        }
        Ok(Self(value))
    }
}
``` `

## Optional and Collection Primitives

Bad:

``` `rust
pub struct <RuleSet>VO {
    pub patterns: Vec<String>,
    pub description: Option<String>,
}
``` `

Good:

``` `rust
pub struct <RuleSet>VO {
    patterns: <PatternList>VO,
    description: Option<<RuleDescription>VO>,
}
``` `
```

---

## File: ARCHITECTURE.md

```markdown
# Agentic Engineering System Architecture

## 1. Purpose

The Agentic Engineering System is a layered, AI-native architecture pattern. It keeps domain models stable, business logic readable, technical detail isolated, and layer boundaries explicit enough for both humans and AI agents to modify the system safely.

---

## 2. Workspace Organization

The architecture supports multi-language workspaces.

| Term               | Meaning                                                           |
| ------------------ | ----------------------------------------------------------------- |
| Project Workspaces | Project root containing all configuration and language members    |
| Workspace Member   | One self-contained crate, package, or module inside the workspace |
| Crates directory   | Rust workspace members                                            |
| Packages directory | TypeScript or JavaScript packages                                 |
| Modules directory  | Python modules or sub-projects                                    |

---

## 3. Naming Convention

File names must communicate three parts:

1. Layer as prefix
2. Concern as middle name
3. Role as suffix

The parts are joined by underscores, followed by the normal file extension for the language.

`layer_concern_role.rs/py/ts`

---

## 4. Vertical Slicing Folder Structure

The recommended folder structure follows this order:

#### Features member

_Example feature crate `crates|packages|modules/<name-features>/`_

``` `text
surface_<concern>_<role>.rs/py/ts                ← surface layer
capabilities_<concern>_<role>.rs/py/ts           ← capabilities layer
agent_<concern>_orchestrator.rs/py/ts            ← agent layer
``` `

Exceptions: `main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, `index.js`.

#### Shared member

`crates|packages|modules/shared/<common>or<domain-folder>`

``` `text
contract_<concern>_protocol.rs/py/ts             ← contract layer
contract_<concern>_aggregate.rs/py/ts            ← contract layer
taxonomy_<concern>_vo.rs/py/ts                   ← taxonomy layer
taxonomy_<concern>_event.rs/py/ts                ← taxonomy layer
taxonomy_<concern>_entity.rs/py/ts               ← taxonomy layer
taxonomy_<concern>_constant.rs/py/ts             ← taxonomy layer
utility_<concern>_<role>.rs/py/ts                ← utility layer
``` `

`shared` folder groups by domain. Use `shared/common/` for generic files.

---

## 5. Taxonomy Layer

### Purpose

Taxonomy is the domain foundation layer. It defines the stable language of the domain and must remain free from technical or behavioral concerns.

### Components

| Role         | Meaning                               |
| ------------ | ------------------------------------- |
| Value object | Immutable data concept                |
| Entity       | Stateful domain concept with identity |
| Event        | Immutable domain fact                 |
| Error        | Domain-level error                    |
| Constant     | Compile-time literal value            |

### Dependencies

Taxonomy depends on nothing.

### Special Rules

- Value objects and Constants may use all primitive types.
- Entities, Events, and Errors must use Value objects/Constants instead of primitive types (bool/str is an exception).
- Constants must be compile-time values.
- Taxonomy must not contain business rules, infrastructure, or imports from other layers.

---

## 6. Contract Layer

### Purpose

Contract defines the public behavior of the system without exposing implementation. It allows callers to depend on stable interfaces instead of concrete logic.

### Components

| Role      | Meaning                                                                                           |
| --------- | ------------------------------------------------------------------------------------------------- |
| Protocol  | Interface defining inbound behavior. It is implemented by Capabilities and consumed by the Agent. |
| Aggregate | Facade definition implemented by Agent, used by Surface to access feature behavior.               |

### Dependencies

Contract may depend on Taxonomy only.

### Special Rules

- Protocol defines behavior only without implementation.
- Aggregate hides Capabilities from Surface.

---

## 7. Utility Layer

### Purpose

Utility contains low-level technical mechanics. It exists so that Capabilities can remain clean and expressive.

### Role Naming

Utility role suffixes are unlimited. The role name is chosen based on demand and must describe the technical responsibility and concern of the file.

parser
splitter
trimmer
slugifier
sanitizer
normalizer
extractor
replacer
converter
counter
resolver
detector
builder
joiner
serializer
deserializer
encoder
decoder
hasher
generator
formatter
comparator
differ
matcher
checker
calculator
mapper
merger
grouper
sorter
deduplicator
printer

### Dependencies

Utility may depend only on Taxonomy.

### Technical Concern Examples

| Concern                 | Responsibility                                      |
| ----------------------- | --------------------------------------------------- |
| File discovery          | Walk directories, detect files, apply ignore        |
| External tool execution | Run linters, compilers, formatters, analyzers       |
| Parsing and matching    | Parse text, match patterns, extract structured data |
| Path normalization      | Normalize paths across platforms                    |
| System operations       | Handle process or environment mechanics             |

### Special Rules

- Utility must use stateless standalone functions only.
- Utility must not contain stateful objects, behavior definitions, or contract implementations.
- Utility must not make business decisions.
- Utility may perform technical operations if needed.
- Utility must not implement any contract.
- Utility role names may expand freely, but the layer must remain technical and standalone.
- Utility must use stateless standalone functions only.

---

## 8. Capabilities Layer

### Purpose

Capabilities contain the concrete implementation of the system's behavior. This layer encapsulates both **pure business logic** (computations, validations) and **external adaptations** (database access, third-party API calls, infrastructure mechanics). By hiding these implementations behind Contracts, the system keeps its behavior modular, swappable, and fully isolated from orchestration.

### Role Naming

#### Internal Examples

validator
assessor
calculator
resolver
classifier
selector
mapper
transformer
policy
enricher
evaluator
analyzer
scorer
grader
ranker
filter
checker
reviewer
approver
rejector

#### External Examples

repository
gateway
client
provider
fetcher
reader
writer
scanner
executor
publisher
subscriber
adapter
connector
uploader
downloader
sender
receiver
dispatcher
watcher
monitor

### Dependencies

- Capabilities may depend on Taxonomy, Contract, and Utility.
- Capabilities must not depend on or import other Capabilities.

### Concern Examples

Capabilities generally handle two types of concerns:

| Category                      | Concern        | Responsibility                                 |
| ----------------------------- | -------------- | ---------------------------------------------- |
| **Business Logic**      | Validation     | Check domain conditions or input correctness   |
|                               | Computation    | Calculate scores, totals, or derived values    |
|                               | Transformation | Map, filter, reduce, or reshape data           |
|                               | Resolution     | Apply rules and decide outcomes                |
|                               | Assessment     | Judge severity, compliance, grade, or quality  |
| **External Adaptation** | Repository     | Fetch or persist domain entities to a database |
|                               | Integration    | Communicate with third-party services or APIs  |
|                               | Provider       | Generate data from external systems            |

### Special Rules

- **No Inter-Capability Dependency:** Capabilities must never import or call other Capabilities directly. They are standalone execution units.
- **Pipeline Aggregation:** Multiple Capabilities (e.g., Capability A for data fetching, Capability B for business calculation) are designed to be composed into a sequential pipeline by the **Agent Layer**, not by themselves.
- **Shared Logic Extraction (DRY):** If multiple Capabilities require the same technical mechanics or functions, that logic must be extracted into a reusable standalone function in the **Utility Layer**. Capabilities must not duplicate technical code (Don't Repeat Yourself).
- **Contract Implementation:** Capabilities must implement the `protocol_` defined in the Contract Layer.
- **State Ownership:** Capabilities are the owners of business and technical state within their execution scope.
- **Utility Delegation:** Capabilities must call Utility standalone functions when low-level technical operations are required, passing their state/data as arguments.
- **No Orchestration:** Capabilities must not contain flow control (looping across capabilities, branching between capabilities, or error escalation policy). They execute their single responsibility and return a result.
- **No Domain Definition:** Capabilities must not define domain models (Entities, Value Objects); they only consume and produce Taxonomy.

---

## 9. Agent Layer

### Purpose

Agent coordinates multiple capabilities into executable flows. It controls sequence and movement, not business calculation.

### Allowed Role

The only Agent role is orchestrator.

### Dependencies

Agent may depend only on Taxonomy, Contract, and Utility.

### Allowed Flow Control

| Flow Type               | Purpose                                |
| ----------------------- | -------------------------------------- |
| Sequential execution    | Run steps in order                     |
| Looping                 | Process multiple items or events       |
| Branching               | Choose path based on result            |
| Error handling          | Recover, abort, continue, or escalate  |
| Timeout or cancellation | Stop long-running or asynchronous work |

### Special Rules

- Agent must depend on Contract, not concrete implementations.
- Agent must not use and must be completely ignorant of Capabilities implementations.
- Agent must not calculate business results.
- Agent must not define domain models.

---

## 10. Surface Layer

### Purpose

Surface is the outer boundary of the system. It handles user-facing or external-facing interaction and translates it into architectural actions.

### Allowed Roles

Surface roles include:

- command
- controller
- page
- view
- component
- router
- layout
- hook
- store
- action
- screen

### Surface Groups

| Group            | Roles                             | Dependencies                           | Rule                                            |
| ---------------- | --------------------------------- | -------------------------------------- | ----------------------------------------------- |
| Smart surfaces   | command, controller, page, router | Taxonomy, Contract Aggregate, Utility | May initiate feature behavior through aggregate |
| Utility surfaces | hook, store, action, screen       | Taxonomy only                          | Support smart surfaces but must not import them |
| Passive surfaces | component, view, layout           | Taxonomy only                          | Presentation-only, no logic or orchestration    |

### Special Rules

- Smart surfaces must consume Contract Aggregates.
- Surfaces must not import Capabilities, Utility, or Agent directly.
- Surfaces must not contain business calculation or orchestration.

---

## 11. Root Layer

### Purpose

Root is the composition layer. It assembles the system by connecting concrete implementations to contracts and starting the application.

### Components

| Role      | Meaning                                                                           |
| --------- | --------------------------------------------------------------------------------- |
| Container | Wires one feature by connecting Capabilities to Contract protocols and aggregates |
| Entry     | Bootstraps the application and composes feature containers                        |

### Dependencies

Root may depend on all layers.

### Special Rules

- Root may instantiate and wire components.
- Root must not contain business logic.
- Root must not contain orchestration policy.
- Root must not contain technical parsing or user interface behavior.
```

---

## File: PRD.md

```markdown
# PRD — Lint Arwaky

## Problem Statement

Software projects accumulate quality debt silently. Developers lack a single tool that audits Rust + Python + JavaScript/TypeScript together, enforces architectural rules, and works for both human developers (CLI) and AI agents (MCP tools).

## Goals & Success Metrics

- Goal 1: Multi-language linting in a single pass (Rust, Python, JS/TS)
- Goal 2: 24 AES rules enforced across 5 groups (Naming, Import, Quality, Role, Orphan)
- Goal 3: MCP server with 5 tools for autonomous AI-agent integration
- Goal 4: Zero bypass tolerance (`noqa`, `type: ignore`, `#[allow(...)]` flagged)
- Goal 5: Self-auditing — project lints itself under its own rule engine

## User Personas

- **AI Agent**: Autonomous linting, self-healing, code review via MCP tools
- **Developer**: Lint codebases, enforce architecture during local development
- **DevOps / CI**: Quality gates, trend reports, dependency scans
- **Contributor**: Extend adapters, add CLI commands
- **Reviewer**: Architecture audit, code quality analysis

## Scope

- In scope:
  - CLI binary (`lint-arwaky-cli`) for human developers
  - MCP server (`lint-arwaky-mcp`) for AI agents
  - TUI file browser (`lint-arwaky-tui`)
  - 24 AES rules across 5 groups
  - External linter adapters (Clippy, Ruff, MyPy, Bandit, ESLint, Prettier, TSC)
  - SARIF 2.1.0, JUnit XML, JSON reports
  - Git hooks integration
  - Auto-fix capabilities

- Out of scope:
  - IDE plugins (VS Code, IntelliJ)
  - Web dashboard
  - Cloud-hosted SaaS
  - Non-Rust implementation

## Feature Requirements (Prioritized)

### P0 — Must Have

- [ ] Multi-language scanning (Rust, Python, JS/TS)
- [ ] 24 AES rules enforcement
- [ ] CLI with `check`, `scan`, `fix`, `ci` commands
- [ ] MCP server with 5 tools
- [ ] Self-auditing capability

### P1 — Should Have

- [ ] External linter adapters (Clippy, Ruff, MyPy, Bandit, ESLint, Prettier, TSC)
- [ ] SARIF 2.1.0, JUnit XML, JSON reports
- [ ] Git hooks integration
- [ ] Auto-fix capabilities
- [ ] Watch mode for continuous linting

### P2 — Nice to Have

- [ ] TUI file browser
- [ ] Orphan code detection
- [ ] Duplicate code detection
- [ ] Dependency vulnerability scanning

## Non-functional Requirements (High-level)

- Performance: Scan 1000 files in < 5 seconds
- Security: No network calls required for core functionality
- Scalability: Handle monorepos with 10,000+ files
- Platform: Linux (primary), macOS (secondary)
- Binary: Static release via `cargo build --release`

## Open Questions / Risks

- Windows support timeline
- Performance optimization for very large codebases
- Integration with CI/CD platforms (GitHub Actions, GitLab CI)
```

---

## File: crates/code-analysis/FRD.md

```markdown
# FRD — code-analysis

## System Overview

The code-analysis crate enforces general code quality, formatting limits, and clean-coding policies. It protects the codebase from bloated files, empty structures, and duplicate blocks, while guaranteeing zero tolerance for warning/error bypasses.

## Functional Requirements

### FR-001: Maximum File Line Count (AES301)

- **Description**: Files must not exceed the maximum allowed line count.
- **Input**: Source file path
- **Output**: AES301 diagnostic if exceeded
- **Business Rules**:
  - Default max: 1000 lines (configurable per rule)
  - Applies to: Rust, Python, TypeScript, JavaScript
- **Edge Cases**: Files with long comments, generated code
- **Error Handling**: Emit AES301 with actual vs max line count

### FR-002: Minimum File Line Count (AES302)

- **Description**: Files must have minimum length to avoid empty placeholders.
- **Input**: Source file path
- **Output**: AES302 diagnostic if too short
- **Business Rules**:
  - Default min: 10 lines
  - Applies to: Rust, Python, TypeScript, JavaScript
- **Edge Cases**: Config files, entry points
- **Error Handling**: Emit AES302 with actual vs min line count

### FR-003: Mandatory Definitions (AES303)

- **Description**: Source files must declare at least one primary symbol.
- **Input**: Source file path
- **Output**: AES303 diagnostic if no definition found
- **Business Rules**:
  - Rust: struct, enum, trait, type
  - Python: class, def, async def
  - TypeScript: class, interface, type, enum, function
  - JavaScript: class, function, async function
- **Edge Cases**: Empty impl blocks, unit structs
- **Error Handling**: Emit AES303 with expected symbol types

### FR-004: Bypass Detection (AES304)

- **Description**: Detects and flags any attempt to suppress warnings/errors.
- **Input**: Source file path
- **Output**: AES304 diagnostic for each bypass found
- **Business Rules**:
  - Comment bypasses: noqa, type: ignore, eslint-disable
  - Attribute bypasses: #[allow(...)], #[warn(...)]
  - Fatal operations: unwrap(), expect(), panic!, todo!
  - Safe variants NOT flagged: unwrap_or(), unwrap_or_else()
- **Edge Cases**: Nested attributes, conditional compilation
- **Error Handling**: Emit AES304 with bypass type and location

### FR-005: Duplicate Code Detection (AES305)

- **Description**: Compares code blocks and flags identical/highly similar segments.
- **Input**: All workspace source files
- **Output**: AES305 diagnostic for duplicate blocks
- **Business Rules**:
  - Min duplicate lines: 5
  - Threshold: 50% similarity
  - Algorithm: Window-based hashing with normalized lines
- **Edge Cases**: Generated code, boilerplate
- **Error Handling**: Emit AES305 with duplicate file locations

### FR-006: File Read Error Diagnostics (AES000)

- **Description**: Emit diagnostic when file cannot be read or exceeds size limit.
- **Input**: File path
- **Output**: AES000 diagnostic
- **Business Rules**:
  - Max file size: 2 MiB
  - Emit diagnostic instead of silent skip
- **Edge Cases**: Binary files, permission errors
- **Error Handling**: Emit AES000 with error reason

## Data Model / Entity Relationship

``` `
CodeAnalysisRuleVO {
    rule_code: String
    max_lines: Option<u32>
    min_lines: Option<u32>
    threshold_pct: f64
}

Diagnostic {
    file_path: String
    line: u32
    column: u32
    rule_code: String
    message: String
    severity: Severity
}
``` `

## API Contract

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| `check_max_line_count()` | File path, content | Option<Diagnostic> | Check AES301 |
| `check_min_line_count()` | File path, content | Option<Diagnostic> | Check AES302 |
| `check_mandatory_definitions()` | File path, content | Option<Diagnostic> | Check AES303 |
| `check_forbidden_bypass()` | File path, content | Vec<Diagnostic> | Check AES304 |
| `handle_duplicates()` | All files | Vec<Diagnostic> | Check AES305 |

## Integration Points

- **Internal**: config-system (YAML rules), shared (taxonomy VOs)
- **External**: None

## Non-functional Requirements (Detailed)

- Performance: Analyze 1000 files in < 3 seconds
- Memory: O(n) where n = file size
- Accuracy: Zero false positives for valid code

## Test Scenarios / QA Checklist

- [ ] File exceeding max lines fails with AES301
- [ ] File below min lines fails with AES302
- [ ] File without definitions fails with AES303
- [ ] `unwrap()` detected with AES304
- [ ] `#[allow(...)]` detected with AES304
- [ ] Duplicate code detected with AES305
- [ ] Oversized file emits AES000

## Assumptions & Constraints

- Rules are configurable via YAML
- File reading uses memory-mapped I/O for large files
- Duplicate detection uses hash-based comparison

## Glossary

- **AES**: Agentic Engineering System
- **Bypass**: Attempt to suppress warnings/errors
- **Diagnostic**: Violation report with location and rule code

## Reference

- PRD: [PRD.md](../../PRD.md)
```

---

## File: crates/shared/src/cli-commands/mod.rs

```rust
// cli-commands — taxonomy and contract types
pub mod contract_analysis_pipeline_aggregate;
pub mod contract_report_formatter_aggregate;
pub mod contract_report_formatter_protocol;
pub mod taxonomy_catalog_constant;

pub mod taxonomy_cli_vo;
pub mod taxonomy_command_catalog_vo;
pub mod taxonomy_format_vo;
pub mod taxonomy_metadata_vo;
pub mod taxonomy_position_vo;
pub mod taxonomy_protocol_vo;
pub mod taxonomy_result_vo;
pub mod taxonomy_scan_report_vo;
pub mod taxonomy_scan_request_vo;
pub mod taxonomy_score_vo;
pub mod taxonomy_severity_vo;

pub mod utility_score_calculator;
```

---

## File: crates/shared/src/cli-commands/taxonomy_result_vo.rs

```rust
// PURPOSE: LintResult, LintResultList, FilePathSet — value objects for lint violation results
use serde::{Deserialize, Serialize};

use crate::cli_commands::taxonomy_position_vo::Position;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_lint_vo::LocationList;
use crate::common::taxonomy_lint_vo::ScopeRef;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LintResult {
    pub file: FilePath,
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub code: ErrorCode,
    pub message: LintMessage,
    pub source: Option<AdapterName>,
    pub severity: Severity,
    pub enclosing_scope: Option<ScopeRef>,
    pub related_locations: LocationList,
}

impl LintResult {
    /// Convenience constructor used by architecture checkers (make_result / mk pattern).
    pub fn new_arch(
        file: &str,
        line: usize,
        code: &str,
        sev: Severity,
        msg: impl Into<String>,
    ) -> Self {
        Self::new_arch_with_column(file, line, 0, code, sev, msg)
    }

    /// Column-aware constructor for architecture checkers.
    pub fn new_arch_with_column(
        file: &str,
        line: usize,
        column: usize,
        code: &str,
        sev: Severity,
        msg: impl Into<String>,
    ) -> Self {
        Self {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(line as i64),
            column: ColumnNumber::new(column as i64),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: DescriptionVO::new(String::new()),
                kind: DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        }
    }

    /// Specialized constructor for orphan detection results (no enclosing scope).
    pub fn new_orphan(file: &str, msg: impl Into<String>, sev: Severity, code: &str) -> Self {
        Self {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(0),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: None,
            related_locations: LocationList::new(),
        }
    }

    pub fn position(&self) -> Position {
        Position {
            line: self.line.clone(),
            column: self.column.clone(),
        }
    }
    pub fn identity(&self) -> Identity {
        Identity::new(format!(
            "{}:{}:{}:{:?}",
            self.file, self.line, self.code, self.source
        ))
    }
}

/// Generate a `Vec<T>`-backed newtype with `Default`, `new`, `iter`,
/// `len`, `is_empty`, `push`, and `append`. Used for the `LintResultList`
/// wrapper below; siblings `ImportInfoList`/`PrimitiveViolationList` in
/// `taxonomy_import_source_vo.rs` carry the same surface.
macro_rules! lint_result_list_wrapper {
    ($name:ident, $item:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
        pub struct $name {
            pub values: Vec<$item>,
        }

        impl $name {
            pub fn new(value: Vec<$item>) -> Self {
                Self { values: value }
            }
            pub fn iter(&self) -> std::slice::Iter<'_, $item> {
                self.values.iter()
            }
            pub fn len(&self) -> usize {
                self.values.len()
            }
            pub fn is_empty(&self) -> bool {
                self.values.is_empty()
            }
            pub fn push(&mut self, item: $item) {
                self.values.push(item);
            }
            pub fn append(&mut self, item: $item) {
                self.values.push(item);
            }
        }
    };
}

lint_result_list_wrapper!(LintResultList, LintResult);
```

---

## File: crates/shared/src/cli-commands/taxonomy_severity_vo.rs

```rust
// PURPOSE: Severity — re-export from common for backward compatibility
//
// This module exists so dependents can keep using the
// `cli_commands::taxonomy_severity_vo::Severity` import path. The real
// definition lives in `common::taxonomy_severity_vo` and is re-exported
// here to avoid breaking any code that still imports from the legacy path.
pub use crate::common::taxonomy_severity_vo::Severity;
```

---

## File: crates/shared/src/code-analysis/contract_class_protocol.rs

```rust
// PURPOSE: IMandatoryClassProtocol — protocol trait for AES303: check that each file has a struct/enum/trait definition
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_definition_vo::LayerDefinition;

pub trait IMandatoryClassProtocol: Send + Sync {
    fn check_mandatory_class_definition(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    );
}
```

---

## File: crates/shared/src/code-analysis/contract_dead_inheritance_protocol.rs

```rust
// PURPOSE: IDeadInheritanceProtocol — protocol trait for AES303 sub-check 2: detect empty struct/impl blocks
use crate::cli_commands::taxonomy_result_vo::LintResult;

pub trait IDeadInheritanceProtocol: Send + Sync {
    fn check_dead_inheritance(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
```

---

## File: crates/shared/src/code-analysis/mod.rs

```rust
// code-analysis — taxonomy and contract types
pub mod contract_adapter_protocol;
pub mod contract_bypass_checker_protocol;
pub mod contract_class_protocol;
pub mod contract_code_analysis_aggregate;
pub mod contract_code_metric_analyzer_protocol;
pub mod contract_dead_inheritance_protocol;
pub mod contract_layer_detection_aggregate;
pub mod contract_line_protocol;
pub mod taxonomy_analysis_vo;
pub mod taxonomy_code_analysis_rule_vo;
pub mod taxonomy_import_source_vo;
pub mod taxonomy_operation_error;
pub mod taxonomy_violation_code_analysis_vo;
pub mod utility_bypass;
pub mod utility_column;
pub mod utility_duplication;
pub mod utility_file_reader;
pub mod utility_language_mapper;
pub mod utility_mandatory;
pub mod utility_target;
pub use taxonomy_violation_code_analysis_vo::{AesCodeAnalysisViolation, Language};
```

---

## File: crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs

```rust
// PURPOSE: AesCodeAnalysisViolation — violation messages for code quality rules (AES301-305)
use std::fmt;

use crate::common::taxonomy_message_vo::LintMessage;

/// Identifiers treated as Rust-style word tokens (must match as a whole identifier).
pub const WORD_PATTERN_TOKENS: &[&str] = &[
    "unwrap",
    "expect",
    "panic",
    "todo",
    "unimplemented",
    "unreachable",
];

/// Internal violation kind for classification during scanning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationKind {
    UnwrapExpect,
    Panic,
    Todo,
    Unimplemented,
    BypassComment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    JavaScript,
    Python,
    TypeScript,
}

impl Language {
    pub fn from_adapter_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "clippy" | "rust" => Self::Rust,
            "eslint" | "prettier" | "tsc" | "javascript" => Self::JavaScript,
            "ruff" | "mypy" | "bandit" | "python" => Self::Python,
            "typescript" => Self::TypeScript,
            _ => Self::Rust,
        }
    }

    pub fn struct_keyword(&self) -> &'static str {
        match self {
            Self::Rust => "struct",
            Self::JavaScript | Self::TypeScript => "class/interface",
            Self::Python => "class/Protocol",
        }
    }

    pub fn type_kw(&self) -> &'static str {
        match self {
            Self::Rust => "type",
            Self::JavaScript | Self::TypeScript => "interface/type",
            Self::Python => "Protocol/type",
        }
    }

    pub fn interface_kw(&self) -> &'static str {
        match self {
            Self::Rust => "trait",
            Self::JavaScript | Self::TypeScript => "interface",
            Self::Python => "Protocol",
        }
    }

    pub fn inherits_kw(&self) -> &'static str {
        match self {
            Self::Rust => "implements",
            Self::JavaScript | Self::TypeScript => "implements/extends",
            Self::Python => "implements/inherits",
        }
    }
}

#[derive(Debug, Clone)]
pub enum AesCodeAnalysisViolation {
    // AES301 — File size
    FileTooLarge { reason: Option<LintMessage> },
    FileTooShort { reason: Option<LintMessage> },
    // AES303 — Mandatory class/struct definition
    MandatoryClassDefinition { reason: Option<LintMessage> },
    // AES304 — Bypass comments (Rust only)
    BypassComment { reason: Option<LintMessage> },
    UnwrapExpect { reason: Option<LintMessage> },
    Panic { reason: Option<LintMessage> },
    Todo { reason: Option<LintMessage> },
    Unimplemented { reason: Option<LintMessage> },
    // AES305 — Duplicate/dead code (empty impl blocks)
    DeadInheritance { reason: Option<LintMessage> },
    CodeDuplication { reason: Option<LintMessage> },
}

impl fmt::Display for AesCodeAnalysisViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AesCodeAnalysisViolation::FileTooLarge { reason } => {
                let default_why =
                    "Large files violate the Single Responsibility Principle.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES301 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n\
                        WHY? {}\n\
                        FIX: Split the module into smaller, more focused files.",
                    why
                )
            }
            AesCodeAnalysisViolation::FileTooShort { reason } => {
                let default_why =
                    "Excessively small files clutter the project structure.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES302 FILE_TOO_SHORT: File contains fewer than the required minimum lines.\n\
                        WHY? {}\n\
                        FIX: Expand the component or merge this logic into a related module.",
                    why
                )
            }
            AesCodeAnalysisViolation::BypassComment { reason } => {
                let default_why =
                    "Bypassing code checks hides issues and risks architectural regressions."
                        .to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 BYPASS_COMMENT: Forbidden bypass comment or annotation detected.\n\
                        WHY? {}\n\
                        FIX: Remove the bypass comment and resolve the issue properly.",
                    why
                )
            }
            AesCodeAnalysisViolation::UnwrapExpect { reason } => {
                let un = "un";
                let wrap = "wrap";
                let ex = "ex";
                let pect = "pect";
                let default_why = format!("Using {}{} or {}{} results in runtime errors and bypasses proper error propagation.", un, wrap, ex, pect);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(f, "AES304 UNWRAP_EXPECT: Forbidden {}{} or {}{} call detected.\n\
                        WHY? {}\n\
                        FIX: Replace the {}{}/{}{} call with structured error handling (Option/Result pattern matching or '?').", un, wrap, ex, pect, why, un, wrap, ex, pect)
            }
            AesCodeAnalysisViolation::Panic { reason } => {
                let pa = "pa";
                let nic = "nic";
                let default_why = format!("Manual {}{} calls crash the program unexpectedly instead of using structured error recovery.", pa, nic);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 PANIC: Forbidden {}{} call detected.\n\
                        WHY? {}\n\
                        FIX: Return a Result or handle the failure case gracefully without {}{}ing.",
                    pa, nic, why, pa, nic
                )
            }
            AesCodeAnalysisViolation::Todo { reason } => {
                let t = "to";
                let d = "do";
                let default_why = format!("{}{}!() placeholders represent incomplete code paths that can crash at runtime if reached unexpectedly.", t, d);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 TODO: Forbidden {}{}!() call detected.\n\
                        WHY? {}\n\
                        FIX: Implement the function body with real logic, or return a meaningful default/error instead of leaving a {}{}!() placeholder.",
                    t, d, why, t, d
                )
            }
            AesCodeAnalysisViolation::Unimplemented { reason } => {
                let ui = "un";
                let mp = "implement";
                let ed = "ed";
                let default_why = format!("{}{}{}!() claims a code path is unreachable, but when reached it crashes — violating the principle of fail-fast with clear error messages.", ui, mp, ed);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 UNIMPLEMENTED: Forbidden {}{}{}!() call detected.\n\
                        WHY? {}\n\
                        FIX: Either implement the missing logic or return a Result::Err with a descriptive error message.",
                    ui, mp, ed, why
                )
            }
            AesCodeAnalysisViolation::MandatoryClassDefinition { reason } => {
                let lang = Language::Rust;
                let default_why = format!(
                    "Encapsulation in {} is required for proper modularization and contract adherence.",
                    lang.struct_keyword()
                );
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(f, "AES303 MANDATORY_DEFINITION: File is missing a {}, {}, or {} definition.\n\
                        WHY? {}\n\
                        FIX: Group functions into a {} or implement a {} that defines the module interface.", lang.struct_keyword(), lang.interface_kw(), lang.type_kw(), why, lang.struct_keyword(), lang.interface_kw())
            }
            AesCodeAnalysisViolation::DeadInheritance { reason } => {
                let lang = Language::Rust;
                let default_why = format!("Empty {} implementation blocks do not add behavior and indicate dead or incomplete code.", lang.inherits_kw());
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(f, "AES305 DEAD_INHERITANCE: Empty {}, class, or {} implementation block detected.\n\
                        WHY? {}\n\
                        FIX: Implement the necessary methods/fields or remove the empty definition block.", lang.struct_keyword(), lang.interface_kw(), why)
            }
            AesCodeAnalysisViolation::CodeDuplication { reason } => {
                let default_why = "Duplicate code blocks increase maintenance burden and indicate missing abstraction.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES305 CODE_DUPLICATION: Duplicate code block detected.\n\
                        WHY? {}\n\
                        FIX: Extract the duplicated logic into a shared function or module.",
                    why
                )
            }
        }
    }
}

impl From<AesCodeAnalysisViolation> for String {
    fn from(v: AesCodeAnalysisViolation) -> String {
        v.to_string()
    }
}
```

---

## File: crates/shared/src/code-analysis/utility_bypass.rs

```rust
// PURPOSE: Stateless utility functions for bypass checking (AES304)
// Pure functions only — no domain types (enums, consts) belong here

/// Returns true if byte is a valid identifier continuation character.
pub fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Returns true if byte can start an identifier.
pub fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

/// Check if a line starts with a Rust bypass attribute (`#[allow(...)`, `#[expect(...)`,
/// `#![allow(...)]`, `#![expect(...)]`, `#![warn(...)]`, `#[warn(...)]`,
/// `#[clippy::allow(...)]`, etc.), constructed without the literal prefixes appearing
/// in source to avoid AES304 self-flagging.
pub fn starts_with_allow_attr(line: &str) -> bool {
    static PREFIXES: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    let prefixes = PREFIXES.get_or_init(|| {
        let mk = |chars: &[char]| chars.iter().collect::<String>();
        vec![
            mk(&['#', '[', 'a', 'l', 'l', 'o', 'w', '(']), // #[allow(
            mk(&['#', '[', 'e', 'x', 'p', 'e', 'c', 't', '(']), // #[expect(
            mk(&['#', '[', 'w', 'a', 'r', 'n', '(']),      // #[warn(
            mk(&['#', '!', '[', 'a', 'l', 'l', 'o', 'w', '(']), // #![allow(
            mk(&['#', '!', '[', 'e', 'x', 'p', 'e', 'c', 't', '(']), // #![expect(
            mk(&['#', '!', '[', 'w', 'a', 'r', 'n', '(']), // #![warn(
            mk(&[
                '#', '[', 'c', 'l', 'i', 'p', 'p', 'y', ':', ':', 'a', 'l', 'l', 'o', 'w', '(',
            ]), // #[clippy::allow(
        ]
    });
    prefixes.iter().any(|prefix| line.starts_with(prefix))
}

/// Check if a suffix after underscore is a known panicking/unsafe variant.
fn forbidden_method_suffix(token: &str, suffix: &str) -> bool {
    matches!((token, suffix), ("unwrap", "unchecked") | ("panic", "any"))
}

/// Returns true if `line` (already trimmed) contains `token` invoked as a method call or macro.
/// When `requires_method_call` is true, the token must be preceded by a dot (`.`).
pub fn matches_word_token(line: &str, token: &str, requires_method_call: bool) -> bool {
    if token.is_empty() {
        return false;
    }

    let trimmed = line.trim_start();
    if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
        return false;
    }

    let bytes = line.as_bytes();
    let token_bytes = token.as_bytes();
    let tlen = token_bytes.len();

    if bytes.len() < tlen {
        return false;
    }

    let mut i = 0;

    while i + tlen <= bytes.len() {
        if &bytes[i..i + tlen] == token_bytes {
            let before_ok = i == 0 || !is_ident_start(bytes[i - 1]);

            if before_ok {
                if requires_method_call {
                    let preceded_by_dot = i > 0 && bytes[i - 1] == b'.';
                    if !preceded_by_dot {
                        i += 1;
                        continue;
                    }
                }

                let j = i + tlen;

                if j < bytes.len() && (bytes[j] == b'(' || bytes[j] == b'!') {
                    return true;
                }

                if j < bytes.len() && bytes[j] == b'_' {
                    let seg_start = j + 1;

                    if seg_start < bytes.len() && is_ident_start(bytes[seg_start]) {
                        let mut seg_end = seg_start;

                        while seg_end < bytes.len() && is_ident_continue(bytes[seg_end]) {
                            seg_end += 1;
                        }

                        let seg = &line[seg_start..seg_end];
                        let k = seg_end;

                        if k < bytes.len()
                            && (bytes[k] == b'(' || bytes[k] == b'!')
                            && forbidden_method_suffix(token, seg)
                        {
                            return true;
                        }
                    }
                }
            }
        }

        i += 1;
    }

    false
}

/// Word-boundary keyword token matcher.
pub fn matches_keyword_token(line: &str, token: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
        return false;
    }

    let bytes = line.as_bytes();
    let token_bytes = token.as_bytes();
    let tlen = token_bytes.len();

    if bytes.len() < tlen {
        return false;
    }

    let mut i = 0;

    while i + tlen <= bytes.len() {
        if &bytes[i..i + tlen] == token_bytes {
            let before_ok =
                i == 0 || (!bytes[i - 1].is_ascii_alphanumeric() && bytes[i - 1] != b'_');

            let after_ok = i + tlen == bytes.len()
                || (!bytes[i + tlen].is_ascii_alphanumeric() && bytes[i + tlen] != b'_');

            if before_ok && after_ok {
                return true;
            }
        }

        i += 1;
    }

    false
}

/// Skip a brace-delimited block starting at `start`.
///
/// Returns the index of the first line after the block.
/// If the starting line is already balanced or has no opening brace,
/// returns `start + 1`.
pub fn skip_brace_block(lines: &[&str], start: usize) -> usize {
    if start >= lines.len() {
        return start;
    }

    let mut depth =
        lines[start].matches('{').count() as i32 - lines[start].matches('}').count() as i32;
    let mut idx = start + 1;

    if depth <= 0 {
        return idx;
    }

    while idx < lines.len() {
        depth += lines[idx].matches('{').count() as i32 - lines[idx].matches('}').count() as i32;
        idx += 1;

        if depth <= 0 {
            break;
        }
    }

    idx
}

/// Skip a `#[cfg(test)]` module block when present.
///
/// If the attribute is followed by a test module, returns the first line
/// after that module. Otherwise, returns `start + 1`, skipping only the
/// attribute line.
pub fn skip_cfg_test_block(lines: &[&str], start: usize) -> usize {
    if start >= lines.len() {
        return start;
    }

    let mut idx = start + 1;

    // Skip blank lines and additional attributes.
    while idx < lines.len() {
        let t = lines[idx].trim();
        if t.is_empty() || t.starts_with('#') {
            idx += 1;
            continue;
        }
        break;
    }

    if idx >= lines.len() {
        return idx;
    }

    let t = lines[idx].trim();
    let is_mod = t.split_whitespace().any(|w| w == "mod");

    // Not a module attribute; skip only the attribute line.
    if !is_mod {
        return start + 1;
    }

    // Module declaration without body, e.g. `mod tests;`.
    if t.ends_with(';') && !t.contains('{') {
        return idx + 1;
    }

    let mut depth = t.matches('{').count() as i32 - t.matches('}').count() as i32;
    idx += 1;

    // The module body opened and closed on the same line, e.g. `mod tests {}`.
    if depth <= 0 && t.contains('{') {
        return idx;
    }

    // Look for an opening brace on following lines.
    if depth <= 0 {
        while idx < lines.len() {
            let st = lines[idx].trim();
            depth += st.matches('{').count() as i32 - st.matches('}').count() as i32;
            idx += 1;

            if depth > 0 {
                break;
            }

            // Opened and closed immediately on the next line.
            if depth <= 0 && st.contains('{') {
                return idx;
            }
        }
    }

    // Consume until the module body closes.
    while idx < lines.len() && depth > 0 {
        let st = lines[idx].trim();
        depth += st.matches('{').count() as i32 - st.matches('}').count() as i32;
        idx += 1;
    }

    idx
}

// ─── Regression Tests for Phase 3 Fixes ──────────────────────────────────────────

#[cfg(test)]
mod phase3_regression_tests {
    use super::*;

    /// Regression test for Phase 3.9: matches_word_token restricts underscore suffix
    /// to known panicking variants (unwrap_unchecked, panic_any).
    #[test]
    fn matches_word_token_restricted_underscore_suffixes() {
        // Known unsafe variants should match
        assert!(matches_word_token("x.unwrap_unchecked()", "unwrap", true));
        assert!(matches_word_token("x.panic_any()", "panic", true));

        // Unknown suffixes should NOT match when requires_method_call is true
        // This prevents false positives like matching "foo_something()" for token "foo"
        assert!(!matches_word_token("x.foo_something()", "foo", true));
        assert!(!matches_word_token("x.bar_unsafe()", "bar", true));

        // Direct method calls should still match
        assert!(matches_word_token("x.unwrap()", "unwrap", true));
        assert!(matches_word_token("x.panic()", "panic", true));
    }

    /// Regression test: matches_word_token handles keyword tokens correctly.
    #[test]
    fn matches_keyword_token_word_boundaries() {
        // Keywords should match at proper word boundaries
        assert!(matches_keyword_token("let x = unwrap();", "unwrap"));
        assert!(matches_keyword_token("fn main() { unwrap(); }", "main"));

        // Substrings should NOT match
        assert!(!matches_keyword_token("my_unwrap()", "unwrap"));
        assert!(!matches_keyword_token("unwrap_something()", "unwrap"));
    }

    /// Regression test: matches_word_token skips comment lines.
    #[test]
    fn matches_word_token_skips_comments() {
        // Lines starting with // should be skipped
        assert!(!matches_word_token("// unwrap()", "unwrap", true));
        assert!(!matches_word_token("/* unwrap */", "unwrap", true));
        assert!(!matches_word_token("* unwrap *", "unwrap", true));
    }

    /// Regression test: matches_word_token handles empty token.
    #[test]
    fn matches_word_token_empty_token() {
        assert!(!matches_word_token("anything", "", true));
    }

    /// Regression test: matches_word_token handles line shorter than token.
    #[test]
    fn matches_word_token_short_line() {
        assert!(!matches_word_token("foo", "foobar", true));
    }
}
```

---

## File: crates/shared/src/code-analysis/utility_mandatory.rs

```rust
// PURPOSE: Stateless utility functions for mandatory definition checking (AES303)
// Extracted from capabilities_mandatory_definition_checker.rs — pure functions, no &self, no I/O

use super::utility_bypass::matches_keyword_token;

/// Check if a line declares a Rust struct/enum/trait/type using word-boundary matching.
/// Handles visibility modifiers (pub, pub(crate)), tuple structs, and avoids
/// substring false-positives like "obstruction", "structure", "instruction".
pub fn rust_declares_type(line: &str) -> bool {
    let keywords = ["struct", "enum", "trait", "type"];
    for kw in keywords {
        if matches_keyword_token(line, kw) {
            return true;
        }
    }
    false
}
```

---

## File: crates/shared/src/common/mod.rs

```rust
// common — truly shared types used by multiple features
pub mod contract_executor_protocol;
pub mod taxonomy_action_vo;
pub mod taxonomy_adapter_list_vo;
pub use utility_file::{
    collect_all_source_files, collect_all_source_files_raw, find_workspace_root, scan_directory,
};
pub mod taxonomy_adapter_error;
pub mod taxonomy_adapter_name_vo;
pub mod taxonomy_byte_count_vo;
pub mod taxonomy_common_error;
pub mod taxonomy_common_vo;
pub mod taxonomy_definition_vo;
pub mod taxonomy_depth_vo;
pub mod taxonomy_display_content_vo;
pub mod taxonomy_duration_vo;
pub mod taxonomy_error_vo;
pub mod taxonomy_filesystem_error;
pub mod taxonomy_git_vo;
pub mod taxonomy_job_id_vo;
pub mod taxonomy_job_vo;
pub mod taxonomy_language_info_vo;
pub mod taxonomy_language_vo;
pub mod taxonomy_layer_vo;
pub mod taxonomy_line_count_vo;
pub mod taxonomy_lint_vo;
pub mod taxonomy_message_vo;
pub mod taxonomy_name_vo;
pub mod taxonomy_naming_list_vo;
pub mod taxonomy_package_name_vo;
pub mod taxonomy_parser_error;
pub mod taxonomy_path_utils_vo;
pub mod taxonomy_path_vo;
pub mod taxonomy_paths_vo;
pub mod taxonomy_percentage_vo;
pub mod taxonomy_response_data_vo;
pub mod taxonomy_severity_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_suffix_vo;
pub mod taxonomy_suggestion_vo;
pub mod taxonomy_threshold_vo;
pub mod utility_command_runner;
pub mod utility_file;
pub mod utility_language_detector;
pub mod utility_layer_detector;
pub mod utility_path_normalization;
pub mod utility_value_object_generator;
pub use utility_signature_parser::{
    extract_python_method_signatures, extract_trait_method_signatures,
    extract_typescript_method_signatures, python_signature_uses_forbidden_primitive,
    signature_uses_forbidden_primitive, typescript_signature_uses_forbidden_primitive,
};
pub mod utility_signature_parser;
```

---

## File: crates/shared/src/common/taxonomy_adapter_name_vo.rs

```rust
// PURPOSE: AdapterName — validated newtype for adapter/linter name strings
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// adapter_name_vo — Adapter and tool identifier value objects.
///
/// Adapter/tool identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AdapterName {
    pub value: String,
}

impl AdapterName {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new AdapterName from a string.
    ///
    /// # Errors
    /// Returns an error if the adapter name is empty or only whitespace.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("Adapter name cannot be empty".to_string());
        }
        Ok(AdapterName {
            value: value.trim().to_string(),
        })
    }

    /// Create a raw AdapterName without error validation (for static compile-time safe inputs).
    pub fn raw<S: Into<String>>(value: S) -> Self {
        AdapterName {
            value: value.into(),
        }
    }
}

impl std::ops::Deref for AdapterName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for AdapterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for AdapterName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
```

---

## File: crates/shared/src/common/taxonomy_common_vo.rs

```rust
// PURPOSE: BooleanVO, ColumnNumber, Count, DataFlowList, LineContentList, LineNumber, PatternList, Score, Timestamp — common VOs
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_job_id_vo::JobId;
use crate::common::taxonomy_layer_vo::LineContentVO;
use crate::common::taxonomy_response_data_vo::ResponseData;
use crate::common::taxonomy_severity_vo::Severity;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct BooleanVO {
    pub value: bool,
}

impl BooleanVO {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for BooleanVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<bool> for BooleanVO {
    fn from(v: bool) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for BooleanVO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BooleanVOVisitor {}
        impl<'de> serde::de::Visitor<'de> for BooleanVOVisitor {
            type Value = BooleanVO;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BooleanVO { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<bool>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(BooleanVO { value: val })
            }
        }
        deserializer.deserialize_any(BooleanVOVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ColumnNumber {
    pub value: i64,
}

impl ColumnNumber {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for ColumnNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for ColumnNumber {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for ColumnNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ColumnNumberVisitor {}
        impl<'de> serde::de::Visitor<'de> for ColumnNumberVisitor {
            type Value = ColumnNumber;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ColumnNumber { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ColumnNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(ColumnNumber { value: val })
            }
        }
        deserializer.deserialize_any(ColumnNumberVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Count {
    pub value: i64,
}

impl Count {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for Count {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for Count {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CountVisitor {}
        impl<'de> serde::de::Visitor<'de> for CountVisitor {
            type Value = Count;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Count { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Count { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Count { value: val })
            }
        }
        deserializer.deserialize_any(CountVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataFlowList {
    pub values: Vec<ErrorMessage>,
}

impl DataFlowList {
    pub fn new(value: Vec<ErrorMessage>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[ErrorMessage] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ErrorMessage> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: ErrorMessage) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobIdList {
    pub values: Vec<JobId>,
}

impl JobIdList {
    pub fn new(value: Vec<JobId>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[JobId] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, JobId> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: JobId) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineContentList {
    pub values: Vec<LineContentVO>,
}

impl LineContentList {
    pub fn new(value: Vec<LineContentVO>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[LineContentVO] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, LineContentVO> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: LineContentVO) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
#[derive(Default)]
pub struct LineNumber {
    pub value: i64,
}

impl LineNumber {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for LineNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for LineNumber {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for LineNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct LineNumberVisitor {}
        impl<'de> serde::de::Visitor<'de> for LineNumberVisitor {
            type Value = LineNumber;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineNumber { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(LineNumber { value: val })
            }
        }
        deserializer.deserialize_any(LineNumberVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct PatternList {
    pub values: Vec<String>,
}

impl PatternList {
    pub fn new(value: impl IntoPatternListValues) -> Self {
        Self {
            values: value.into_pattern_list_values(),
        }
    }
    pub fn values(&self) -> &[String] {
        &self.values
    }
}

impl PatternList {
    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: String) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseDataList {
    pub values: Vec<ResponseData>,
}

impl ResponseDataList {
    pub fn new(value: Vec<ResponseData>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[ResponseData] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ResponseData> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: ResponseData) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Score {
    pub value: f64,
}

impl Score {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn is_perfect(&self) -> bool {
        self.value >= 100.0
    }
    pub fn is_passing(&self, threshold: &Score) -> bool {
        self.value >= threshold.value
    }
    pub fn deduct(&self, severity: &Severity) -> Score {
        Score {
            value: self.value - severity.score_impact(),
        }
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.value)
    }
}

impl From<f64> for Score {
    fn from(v: f64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for Score {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ScoreVisitor {}
        impl<'de> serde::de::Visitor<'de> for ScoreVisitor {
            type Value = Score;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v })
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v as f64 })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v as f64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<f64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Score { value: val })
            }
        }
        deserializer.deserialize_any(ScoreVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Timestamp {
    pub value: String,
}

impl Timestamp {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn now() -> Self {
        Self {
            value: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for Timestamp {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for Timestamp {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TimestampVisitor {}
        impl<'de> serde::de::Visitor<'de> for TimestampVisitor {
            type Value = Timestamp;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timestamp {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timestamp { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<String>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Timestamp { value: val })
            }
        }
        deserializer.deserialize_any(TimestampVisitor {})
    }
}

// Custom Coercion Traits for PatternList

pub trait IntoPatternListValues {
    fn into_pattern_list_values(self) -> Vec<String>;
}

impl IntoPatternListValues for &str {
    fn into_pattern_list_values(self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl IntoPatternListValues for String {
    fn into_pattern_list_values(self) -> Vec<String> {
        vec![self]
    }
}

impl IntoPatternListValues for Vec<String> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self
    }
}

impl IntoPatternListValues for Vec<&str> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self.into_iter().map(|s| s.to_string()).collect()
    }
}

impl IntoPatternListValues for &Vec<String> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ErrorMessage {
    pub value: String,
}

impl ErrorMessage {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ErrorMessage {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ErrorMessage {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_definition_vo.rs

```rust
// PURPOSE: LayerDefinition, LayerMapVO, NamingConfig — VOs for AES layer definitions and naming policies
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use serde::{Deserialize, Serialize};

/// Wrap a single-field VO that exposes a `new(value)` constructor plus the
/// default `derive`s needed by the codebase. Used to keep the boilerplate
/// for `LayerMapVO`/`NamingConfig` uniform without introducing a new linter
/// violation cluster.
macro_rules! single_field_vo {
    ($name:ident, $field:ident: $field_ty:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
        pub struct $name {
            pub $field: $field_ty,
        }

        impl $name {
            pub fn new($field: $field_ty) -> Self {
                Self { $field }
            }
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LayerDefinition {
    #[serde(default)]
    pub allowed: PatternList,
    #[serde(default)]
    pub forbidden: PatternList,
    #[serde(default)]
    pub mandatory: PatternList,
    #[serde(default)]
    pub word_count: Count,
    #[serde(default)]
    pub exceptions: PatternList,
    #[serde(default)]
    pub recursive: BooleanVO,

    #[serde(flatten)]
    pub naming: crate::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO,
    #[serde(flatten)]
    pub code_analysis: crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO,
    #[serde(flatten)]
    pub role: crate::role_rules::taxonomy_role_rule_vo::RoleRuleVO,
    #[serde(flatten)]
    pub orphan: crate::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO,
}

single_field_vo!(LayerMapVO, values: std::collections::HashMap<LayerNameVO, LayerDefinition>);
single_field_vo!(NamingConfig, word_count: Count);
```

---

## File: crates/shared/src/common/taxonomy_error_vo.rs

```rust
// PURPOSE: ErrorCode — value object for AES error code identification
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// error_code_vo — Error code value object.
///
/// Linter error code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ErrorCode {
    code: String,
}

impl ErrorCode {
    pub fn code(&self) -> &str {
        &self.code
    }
    /// Create a new ErrorCode from a string.
    ///
    /// # Errors
    /// Returns an error if the code is empty.
    pub fn new<S: Into<String>>(code: S) -> Result<Self, String> {
        let code = code.into();
        if code.is_empty() {
            return Err("Error code cannot be empty".to_string());
        }
        Ok(ErrorCode { code })
    }

    /// Create a raw ErrorCode without error validation.
    pub fn raw<S: Into<String>>(code: S) -> Self {
        ErrorCode { code: code.into() }
    }

    /// Returns true if the code is a style error (starts with E, W, or D).
    pub fn is_style(&self) -> bool {
        self.code.starts_with('E') || self.code.starts_with('W') || self.code.starts_with('D')
    }
    pub fn is_logic(&self) -> bool {
        self.code.starts_with('F') || self.code.starts_with('I')
    }
    pub fn is_security(&self) -> bool {
        self.code.starts_with('B')
    }
    pub fn is_architecture(&self) -> bool {
        self.code.starts_with("AES")
    }
}

impl std::ops::Deref for ErrorCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl Hash for ErrorCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}
```

---

## File: crates/shared/src/common/taxonomy_job_id_vo.rs

```rust
// PURPOSE: JobId — value object for pipeline job identifiers
//
// `JobId` is a thin wrapper around a `String` and is generated with the
// `string_value_object!` macro. It exists in its own file so that any
// crate needing job identifiers can `use` this type without pulling in the
// rest of the common VO namespace.
use crate::string_value_object;

string_value_object!(JobId);
```

---

## File: crates/shared/src/common/taxonomy_layer_vo.rs

```rust
// PURPOSE: FileContentVO, Identity, LayerNameVO, LineContentVO — VOs for layer identity and file content
use crate::string_value_object;

string_value_object!(FileContentVO);
string_value_object!(Identity);
string_value_object!(LayerNameVO);
string_value_object!(LineContentVO);
```

---

## File: crates/shared/src/common/taxonomy_lint_vo.rs

```rust
// PURPOSE: CommandArgs, Location, LocationList, ScopeBounds, ScopeRef, ViolationConstraint — VOs for lint violations
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeRef {
    pub name: DescriptionVO,
    #[serde(default)]
    pub kind: DescriptionVO,
    #[serde(default)]
    pub file: Option<FilePath>,
    #[serde(default)]
    pub start_line: Option<LineNumber>,
    #[serde(default)]
    pub end_line: Option<LineNumber>,
}

impl ScopeRef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: DescriptionVO::new(name),
            kind: DescriptionVO::new("function"),
            file: None,
            start_line: None,
            end_line: None,
        }
    }
    pub fn has_range(&self) -> bool {
        self.start_line.as_ref().is_some_and(|l| l.value > 0)
            && self.end_line.as_ref().is_some_and(|l| l.value > 0)
    }
}

impl std::fmt::Display for ScopeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref file) = self.file {
            write!(f, "{} {} in {}", self.kind.value, self.name.value, file)
        } else if !self.kind.value.is_empty() {
            write!(f, "{} {}", self.kind.value, self.name.value)
        } else {
            write!(f, "{}", self.name.value)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    #[serde(default)]
    pub file: Option<FilePath>,
    #[serde(default)]
    pub line: Option<LineNumber>,
    #[serde(default)]
    pub column: Option<ColumnNumber>,
    #[serde(default)]
    pub description: DescriptionVO,
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

impl Location {
    pub fn new() -> Self {
        Self {
            file: None,
            line: None,
            column: None,
            description: DescriptionVO::new(String::new()),
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        if let Some(ref file) = self.file {
            parts.push(file.value.clone());
        }
        if let Some(ref line) = self.line {
            let mut s = line.value.to_string();
            if let Some(ref col) = self.column {
                if col.value > 0 {
                    s = format!("{}:{}", line.value, col.value);
                }
            }
            parts.push(s);
        }
        let result = if parts.is_empty() {
            "unknown".to_string()
        } else {
            parts.join(":")
        };
        if self.description.value.is_empty() {
            write!(f, "{}", result)
        } else {
            write!(f, "{} — {}", result, self.description.value)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocationList {
    #[serde(default)]
    pub values: Vec<Location>,
}

impl LocationList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}

impl LocationList {
    pub fn push(&mut self, item: Location) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl std::ops::Deref for LocationList {
    type Target = Vec<Location>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ViolationConstraint {
    pub rule: DescriptionVO,
    #[serde(default)]
    pub min_value: DescriptionVO,
    #[serde(default)]
    pub max_value: DescriptionVO,
}

impl ViolationConstraint {
    pub fn new(rule: impl Into<String>) -> Self {
        Self {
            rule: DescriptionVO::new(rule),
            min_value: DescriptionVO::new(String::new()),
            max_value: DescriptionVO::new(String::new()),
        }
    }
}

impl std::fmt::Display for ViolationConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rule.value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandArgs {
    #[serde(default)]
    pub args: Vec<ContentString>,
}

impl Default for CommandArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandArgs {
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }
}

impl std::fmt::Display for CommandArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.args
                .iter()
                .map(|a| a.value.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeBounds {
    #[serde(default)]
    pub start: Option<LineNumber>,
    #[serde(default)]
    pub end: Option<LineNumber>,
}
```

---

## File: crates/shared/src/common/taxonomy_message_vo.rs

```rust
// PURPOSE: ComplianceStatus, LintMessage — VOs for compliance status and violation messages
use crate::string_value_object;

string_value_object!(LintMessage);

/// Boolean compliance flag. Written manually because `bool` is not supported
/// by the `string_value_object!` macro (`i64 as bool` is not a valid Rust cast).
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ComplianceStatus {
    pub value: bool,
}

impl ComplianceStatus {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for ComplianceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<bool> for ComplianceStatus {
    fn from(v: bool) -> Self {
        Self { value: v }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_path_vo.rs

```rust
// PURPOSE: FilePath, DirectoryPath — value objects for validated file and directory paths
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// file_path_vo — File and directory path value objects.
///
/// File path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FilePath {
    pub value: String,
}

impl FilePath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new FilePath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("File path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, collapse repeated slashes.
        let mut normalized = String::with_capacity(value.len());
        let mut prev_slash = false;
        for c in value.chars() {
            if c == '/' || c == '\\' {
                if !prev_slash {
                    normalized.push('/');
                    prev_slash = true;
                }
            } else {
                normalized.push(c);
                prev_slash = false;
            }
        }
        value = normalized;
        // Remove trailing slashes
        let trimmed = value.trim_end_matches('/');
        value = if trimmed.is_empty() {
            "/".to_string()
        } else {
            trimmed.to_string()
        };
        // If after normalization it's empty, then it was all slashes -> treat as root
        if value.is_empty() {
            return Ok(FilePath {
                value: "/".to_string(),
            });
        }
        Ok(FilePath { value })
    }

    /// File extension without dot.
    pub fn extension(&self) -> String {
        let special_files = [
            "Makefile",
            "Dockerfile",
            "Dockerfile.dev",
            "Dockerfile.prod",
            ".bashrc",
            ".profile",
            ".zshrc",
            ".gitignore",
            ".dockerignore",
        ];
        // Operate on the basename, not the full path — `./foo.rs` must still yield
        // `rs` as its extension, and `.bashrc` (which is fully a basename) must NOT
        // be confused with a hidden file mid-path.
        let basename = match self.value.rsplit('/').next() {
            Some(b) => b,
            None => return String::new(),
        };
        if special_files.contains(&basename) || basename.starts_with('.') {
            return String::new();
        }
        match basename.rsplit_once('.') {
            Some((_, ext)) => ext.to_string(),
            None => String::new(),
        }
    }

    /// Check if path has given extension (without dot).
    pub fn has_extension(&self, ext: &str) -> bool {
        self.extension().eq_ignore_ascii_case(ext)
    }

    /// Extract filename/basename of the path.
    pub fn basename(&self) -> String {
        match self.value.rsplit('/').next() {
            Some(f) => f.to_string(),
            None => self.value.clone(),
        }
    }

    /// Check if the path is a barrel file (module re-export aggregator).
    pub fn is_barrel_file(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "mod.rs" | "index.ts" | "index.js" | "index.tsx" | "index.jsx"
        )
    }

    /// Check if the path is a module/layer entry point file.
    pub fn is_entry_point(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py"
                | "main.py"
                | "py.typed"
                | "app.py"
                | "lib.rs"
                | "main.rs"
                | "index.ts"
                | "index.js"
                | "index.tsx"
                | "index.jsx"
                | "main.ts"
                | "main.js"
                | "app.ts"
                | "app.js"
        )
    }
}

impl std::ops::Deref for FilePath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for FilePath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// Directory path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct DirectoryPath {
    pub value: String,
}

impl DirectoryPath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new DirectoryPath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("Directory path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, and remove trailing slash.
        value = value.replace('\\', "/");
        // Remove trailing slashes
        let trimmed = value.trim_end_matches('/');
        value = if trimmed.is_empty() {
            "/".to_string()
        } else {
            trimmed.to_string()
        };
        Ok(DirectoryPath { value })
    }
}

impl std::ops::Deref for DirectoryPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for DirectoryPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> serde::Deserialize<'de> for DirectoryPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DirectoryPath::new(s).map_err(serde::de::Error::custom)
    }
}

impl Hash for DirectoryPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
```

---

## File: crates/shared/src/common/taxonomy_response_data_vo.rs

```rust
// PURPOSE: ResponseData — value object for pipeline job response data
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseData {
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    #[serde(default)]
    pub returncode: i64,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Default for ResponseData {
    fn default() -> Self {
        Self::new()
    }
}

impl ResponseData {
    pub fn new() -> Self {
        Self {
            value: None,
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata: HashMap::new(),
        }
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.value.as_ref()
    }
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.value.as_ref().and_then(|v| v.get(key))
    }
}
```

---

## File: crates/shared/src/common/taxonomy_severity_vo.rs

```rust
// PURPOSE: Severity — value object for violation severity levels (critical, high, medium, low)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum Severity {
    #[serde(rename = "info")]
    #[default]
    INFO,
    #[serde(rename = "low")]
    LOW,
    #[serde(rename = "medium")]
    MEDIUM,
    #[serde(rename = "high")]
    HIGH,
    #[serde(rename = "critical")]
    CRITICAL,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::INFO => write!(f, "info"),
            Severity::LOW => write!(f, "low"),
            Severity::MEDIUM => write!(f, "medium"),
            Severity::HIGH => write!(f, "high"),
            Severity::CRITICAL => write!(f, "critical"),
        }
    }
}

impl Severity {
    pub fn score_impact(&self) -> f64 {
        match self {
            Severity::INFO => 0.0,
            Severity::LOW => 1.0,
            Severity::MEDIUM => 2.0,
            Severity::HIGH => 3.0,
            Severity::CRITICAL => 5.0,
        }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_source_vo.rs

```rust
// PURPOSE: ContentString, SourceContentVO — VOs for source code content representation
use crate::string_value_object;
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_path_vo::FilePath;

string_value_object!(ContentString);

/// Source content value object: combines a file path, a `ContentString`
/// payload, and a language marker. Carries three fields rather than one,
/// so it does not fit the single-field `string_value_object!` macro;
/// defined manually.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceContentVO {
    pub file_path: FilePath,
    pub content: ContentString,
    pub language: String,
}

impl SourceContentVO {
    pub fn new(file_path: FilePath, content: ContentString, language: impl Into<String>) -> Self {
        Self {
            file_path,
            content,
            language: language.into(),
        }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_suggestion_vo.rs

```rust
// PURPOSE: ClassPath, DescriptionVO, LogOutput, MetadataVO, StdError, StdOutput, Suggestion — domain value objects for CLI suggestion/result data
use crate::string_value_object;
use serde::{Deserialize, Serialize};

// ClassPath, DescriptionVO, LogOutput, StdError, StdOutput, and Suggestion all
// follow the standard String-wrapper VO pattern; the macro emits the
// new/value/Display/From/Hash/PartialEq/Deserialize impls they need.
string_value_object!(ClassPath);
string_value_object!(DescriptionVO);
string_value_object!(LogOutput);
string_value_object!(StdError);
string_value_object!(StdOutput);
string_value_object!(Suggestion);

/// Strongly-typed replacement for the previous
/// `HashMap<String, serde_json::Value>` return type. Each field has a real
/// domain meaning — there is no `serde_json::Value` in the contract surface.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetadataVO {
    pub values: std::collections::HashMap<String, serde_json::Value>,
}

impl MetadataVO {
    pub fn new(value: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self { values: value }
    }
    pub fn value(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.values
    }
}
```

---

