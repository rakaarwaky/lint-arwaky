---
name: aes-capabilities
description: AES capability implementation scaffolding for Python Rust TS. Use when creating capability files.
metadata:
  tags:
    - python
    - rust
    - typescript
    - aes
    - capabilities
    - protocol
    - role-naming
    - 3-block-structure
    - di
    - vo
  related_skills:
    - aes-lint-arwaky
    - aes-agent
    - aes-taxonomy
    - aes-contract
    - aes-utility
  triggers:
    - create capabilities
    - create capabilities python
    - create capabilities rust
    - create capabilities typescript
    - add capabilities
    - add capabilities python
    - add capabilities rust
    - fix capabilities structure
    - create protocol
    - create protocol python
    - create protocol rust
    - create protocol typescript
    - capabilities missing protocol
    - validate capabilities logic
    - check capabilities
    - check capabilities python
    - audit capabilities
    - audit capabilities python
    - audit capabilities rust
---

# aes-capabilities

> **Purpose**: Scaffold AES capability files — concrete protocol implementations (domain rules + external adaptation).
> **Audience**: The agent creating or validating a capability file.
> **Scope**: Python, Rust, and TypeScript `capabilities_<domain>_<role>` files — 3-block structure, ≥1 protocol implementor, ≤3 types.

## Capability vs Utility — Defining the Boundary

A capability and a utility are fundamentally different layers. Confusing them causes misplaced code, wrong imports, and linter violations.

| | **Capability** (`capabilities_*`) | **Utility** (`utility_*`) |
|---|---|---|
| **Core question** | What can the system **do**? | How is a small task **done**? |
| **Nature** | Concrete protocol behaviour — domain rules, external adaptation, DI-wired | Stateless, domain-agnostic, pure free functions |
| **Responsibility** | Implements a `_protocol` contract; orchestrates inputs into domain outcomes | Performs narrow, reusable operations with no business logic |
| **State** | Has instance state via injected dependencies (`Arc<dyn Trait>`, DI params) | Absolutely no state — pure functions only |
| **Location** | Feature folder (`crates/<feature>/`, `modules/<feature>/`, `packages/<feature>/`) | Shared folder (`shared/`) beside taxonomy and contract files |
| **Consumers** | Consumed by agent orchestrators or root composition | Consumed by capabilities, agents, surfaces, or other utilities |
| **Examples** | `capabilities_user_auth`, `capabilities_data_persist`, `capabilities_rate_limit` | `utility_string_sanitizer`, `utility_date_formatter`, `utility_hasher` |

**Decision rule**: Ask "Does this code implement a protocol behaviour with business rules?" If yes → capability. Ask "Does this code perform a narrow, stateless, reusable operation with no business logic?" If yes → utility. If it does both, split it: business logic stays in the capability; the stateless helper moves to utility.

The **layer** decides which suffix, which imports, and which structure apply.
Rules, templates, section contracts, and Verify blocks live in the language HOW-TUs under [`references/`](references/).

| Language | Focus | Body rule | HOW-TO |
| -------- | ----- | --------- | ------ |
| Python | Protocol implementation | 3-block; ≥1 protocol; ≤3 types | [references/HOW-TO-MAKE-PYTHON-CAPABILITIES.md](references/HOW-TO-MAKE-PYTHON-CAPABILITIES.md) |
| Rust | Protocol implementation | 3-block; ≥1 protocol; ≤3 types | [references/HOW-TO-MAKE-RUST-CAPABILITIES.md](references/HOW-TO-MAKE-RUST-CAPABILITIES.md) |
| TypeScript | Protocol implementation | 3-block; ≥1 protocol; ≤3 types | [references/HOW-TO-MAKE-TYPESCRIPT-CAPABILITIES.md](references/HOW-TO-MAKE-TYPESCRIPT-CAPABILITIES.md) |

**The layer chain:**

`contract_*_protocol` (created via aes-contract) → **capabilities implement** → agent / root consume

Each file answers one layer's job. A method or import in the wrong layer is the defect this skill exists to prevent.

---

## Invariants

Every rule is machine-checked by `lint-arwaky-cli scan <layer-path>` (see each HOW-TO § Verify).
A rule cannot drift from the gate. Cite the linter, not this file, when pointing at a rule.

| Layer | Rule |
| ----- | ---- |
| Naming | File `capabilities_<domain>_<role>` — role from internal/external naming lists; forbidden suffixes `_vo`/`_entity`/`_error`/`_event`/`_constant`/`_constants`/`_protocol`/`_aggregate`/`_utility`/`_request`/`_response` (AES101/AES102). |
| Structure | 3-block order: Block 1 type+ctor → Block 2 protocol methods only → Block 3 factories/dunders/helpers. ≥1 protocol implementor, ≤3 types (AES403). |
| Imports | Taxonomy + `_protocol` contracts + utility only — never agents, siblings, surface, local domain models (AES201–AES205). |
| DI | Protocol interfaces only (`Arc<dyn Trait>` in Rust); shared VOs in fields/signatures. |
| Helpers | Stateless domain-agnostic ≥2-consumer functions move to Utility; constants to `taxonomy_*_constant`. |
| Register | Shared barrel: `__init__.py` / `mod.rs` / `index.ts`. |
| Verify | `lint-arwaky-cli scan <layer-path>` → 0. Language compile is fallback only. |

Split details, templates, and Section Contract tables: **read the language HOW-TO** — do not restate them here.

---

## Placement (AES702, AES703)

A `capabilities_*` file belongs in a **feature folder** — a subdirectory of a member
directory (`crates/`, `modules/`, `packages/`) that is neither `shared/` nor a surface folder.
The folder name must match the feature it serves:

```text
crates/<feature_name>/src/capabilities_<domain>_<role>.*    # Rust
modules/<feature_name>/src/capabilities_<domain>_<role>.py  # Python
packages/<feature_name>/src/capabilities_<domain>_<role>.ts # TypeScript
```

The prefix is `capabilities_` in every language. A file named `capability_x.py` is not
a capability file to the linter and is not counted toward a feature folder.

Every feature folder needs **at least one `capabilities_*` file and at least one
`agent_*_orchestrator` file**. A feature folder with only capabilities has nothing
orchestrating them — an **AES702** violation.

`capabilities_*` files are forbidden in two places:

- **`shared/`** — the shared folder holds taxonomy, utility, and contract files only (**AES701**).
- **A surface folder** — a folder where surface files outnumber everything else carries surface files only (**AES703**).

Both violations name the misplaced file and direct you to move it into a feature folder.

### Feature folder documents

A feature folder carries two documents at its root, beside the source:

```text
crates/<feature_name>/FRD.md       # what the feature does — requirements
crates/<feature_name>/BACKLOG.md   # where its work stands
```

The folder name matches the feature it serves: `crates/calculator/` documents the calculator,
`modules/naming_violations/` documents naming violations. A folder named after a generic
word (`utils`, `common`, `core`) describes no feature and cannot carry a meaningful FRD.

A feature folder carrying capabilities or an agent but neither document is an
**AES702** violation — the linter names the missing file and you write it. A folder
carrying neither capabilities nor an agent is not a feature and owes no document pair.

---

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **Is this protocol behaviour (not a stateless helper or pure function)?**
   - *No* → it is a utility, not a capability. See `aes-utility` skill.
2. **Does a `_protocol` contract exist for this behaviour?**
   - *No* → create it first with `aes-contract`.
3. **Block 2 contain only protocol methods? ≥1 implementor? ≤3 types?**
   - *No* → restructure to 1→2→3; split or extract helpers.
4. **Any forbidden imports or local domain models?**
   - *Yes* → strip; shared needs go through contract or root.
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?**
   - *No* → fix findings, re-scan.

---

## Workflow

1. Confirm the work is protocol behaviour (not orchestration / data / mechanics).
2. Ensure a `_protocol` contract exists — if missing, run `aes-contract` first.
3. Resolve the feature capability dir. Run `lint-arwaky-cli scan <layer-path>` — findings are your work list.
4. Implement the 3 blocks from the language HOW-TO § Template / § Section Contract.
5. Strip forbidden imports / inter-capability deps / local domain models; move constants and pure helpers out.
6. Register in the shared barrel, verify with `lint-arwaky-cli scan`, then wire via `aes-agent` / `aes-root`.

---

## Verification

### Machine Checks

```bash
lint-arwaky-cli scan <layer-path>   # AES101/102, AES201–205, AES401–406 → must be 0
# Fallback only: language compile (python -c import / cargo check / npx tsc --noEmit)

```text

A pass means naming, imports, primitives, and roles are clean. Structural judgement
(tier choice, block order, helper-vs-utility, "orchestration only") is **manual** — see HOW-TO § Rules.

### Human Checks

A machine pass does not mean the file is right. Layer purpose, structural order, and
"only what this layer may do" still need a reader (HOW-TO § Rules).

---

## Pre-flight Checklist

- [ ] `lint-arwaky-cli scan <layer-path>` exits 0.
- [ ] Every touched HOW-TO's `Verify` block was executed.
- [ ] File registered in `__init__.py` / `mod.rs` / `index.ts`.
- [ ] Language fallback compile clean if the HOW-TO lists it.
- [ ] Related skills considered for the next layer up/down.

---

## Common Mistakes (Anti-Patterns)

The linter covers naming, imports, and primitives. These need a reader (HOW-TO § Rules):

- **Stateless helper with no protocol behaviour in a capability file**: extract to `utility_*` — it belongs in shared, not the feature folder.
- **Business logic mixed with a pure helper**: split — business rules stay; stateless logic moves to utility.
- **Capabilities importing each other**: shared needs go through contract or root, not sibling imports.
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

---

## Related Skills

- `aes-contract`
- `aes-utility`
- `aes-agent`
