# HOW TO MAKE AGENT RUST

> **Purpose**: Orchestrate only: receive a request, call the aggregate contract, return shared VOs — no I/O, no computation.
>
> **Audience**: Agents and engineers scaffolding AES agent orchestrators.
>
> **Scope**: Rust `agent_<domain>_orchestrator` files — one aggregate, 3-block structure, zero I/O.
>
> **Location**: Feature domain package; depends only on shared contracts/taxonomy/utility.
>
> **Length**: One `_orchestrator` file per feature; ≥1 aggregate, ≤3 types.

---

## Rules

### Import rules

**Allowed imports:** `shared::*` — taxonomy VOs, constants, aggregate traits, protocol traits.
**Forbidden imports:** `capabilities_*`, `agent_*`, `surface_*`.

### 3-Block Structure

```text
// Block 1: Struct Definition
// Block 2: Aggregate Trait Implementation
// Block 3: Constructors, Std Traits, Helpers
```

Method placement:

```text
Free function (outside impl)?               → EXTRACT to *_utility.rs
In aggregate trait?                         → Block 2
std trait impl (Default/Clone/Display)?     → Block 3
fn new() / constructor?                     → Block 3
Private helper (uses &self)?                → Block 3
Pure fn, no struct dep?                     → EXTRACT to *_utility.rs
```

### Helper vs Utility

Keep in Block 3 if ANY: uses `&self`, coupled to this struct, constructor, agent-specific logic, single-use.
Extract to utility only if ALL: no `self`/`Self`, pure, no side effects, domain-agnostic, reusable.
I/O: stateless + I/O + domain-agnostic = taxonomy utility. Stateless + I/O + domain-specific = capabilities.

### Allowed / forbidden operations

**Allowed ops:** `for`/`while`/`loop`, `if/else`/`match`, `?`/`match Err`, `tokio::join!`/`.await`, collecting results into shared VOs.
**Forbidden ops:** `std::fs`, `File::open`, `reqwest`, `hyper`, `sqlx`, `rusqlite`, stdout/stderr write, env mutation, global state mutation.

### Computation, Errors, VOs

**Computation forbidden:** arithmetic, totals, averages, `.sum()`/`.fold()`, parsing, normalization.
Allowed: iteration to call deps, routing results, propagating errors.
e.g. `for file in files { self.checker.check(file) }` = OK. `files.iter().map(|f| f.size()).sum()` = capabilities.

**Error rules:**

- Rule 1: Never silently discard — no `checker.check().unwrap_or_default()`.
- Rule 2: Analysis orchestration → return the taxonomy-defined response enum variant; each variant
  holds VO-wrapped values, not raw primitives.
- Rule 3: Execution orchestration → return `Result<ExecutionReport, AgentExecutionError>` wrapped in a VO variant.
- Rule 4: Delegate I/O errors to capabilities — agent only wraps into VO.

**VO rules:** `String`/`i32`..`u64`/`f32`/`f64`/`char` forbidden for domain fields/contracts. `bool` for toggles; `&str` for borrowed non-domain input only.

### Workflow

1. Confirm orchestration only — computation → capabilities, domain data → taxonomy.
2. Struct implements aggregate trait? If no → create `contract_<name>_aggregate.rs`.
3. Enforce 3-Block.
4. ≥1 aggregate trait, ≤3 types (struct+enum), `Arc<dyn Trait>` for DI, shared VOs.
5. Generic aggregate methods: object-safe or `where Self: Sized`.
6. No forbidden imports, no I/O, no computation.
7. No silent errors, no raw primitives in contracts, no magic constants.
8. `cargo check -p <crate-name>`.

---

## Template

```rust
use std::sync::Arc;

use shared::<domain>::taxonomy_<name>_vo::<VO>;
use shared::<domain>::contract_<name>_aggregate::I<Name>Aggregate;

// ─── Block 1: Struct Definition ──────────────────────────
// Injected deps are protocol traits (what the agent dispatches to), not the aggregate.
pub struct Agent<Name> {
    // One field per capability seam, typed against the seam's protocol trait.
    dep: Arc<dyn I<Seam>Protocol>,
}

// ─── Block 2: Aggregate Trait Implementation ─────────────
// Exactly one method — the single entry point. Dispatch to the owning protocol trait.
impl I<Name>Aggregate for Agent<Name> {
    fn execute(&self, request: <RequestVO>) -> <ResponseVO> {
        // Orchestration only: match the request variant, delegate to the protocol.
        // No computation, no I/O, no silently discarded errors.
        match request {
            <RequestVO>::<Variant1> { .. } => <ResponseVO>::<Result1>(self.dep.<operation_1>(..)),
            <RequestVO>::<Variant2> { .. } => <ResponseVO>::<Result2>(self.dep.<operation_2>(..)),
        }
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
```

---

## Section Contract

| Check | Why it belongs here|
| ----- | -------------------|
| Block 1 → 2 → 3 order followed. | Required by AES layer rules and the linter; missing it is a defect.|
| Block 2: ONLY aggregate trait implementation. | Required by AES layer rules and the linter; missing it is a defect.|
| Block 3: constructors, std traits, private helpers. | Required by AES layer rules and the linter; missing it is a defect.|
| ≥1 struct implements aggregate trait; ≤3 total types. | Required by AES layer rules and the linter; missing it is a defect.|
| No local domain data; `Arc<dyn Trait>` for DI; shared VOs. | Required by AES layer rules and the linter; missing it is a defect.|
| Zero I/O, zero business logic, zero domain computation. | Required by AES layer rules and the linter; missing it is a defect.|
| No forbidden imports. | Required by AES layer rules and the linter; missing it is a defect.|
| Generic aggregate methods object-safe or `where Self: Sized`. | Required by AES layer rules and the linter; missing it is a defect.|
| Aggregate registered in shared crate `mod.rs`. | Required by AES layer rules and the linter; missing it is a defect.|
| `cargo check -p <crate-name>` passes. | Required by AES layer rules and the linter; missing it is a defect.|

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Manual (not machine-checked): zero I/O/computation; no silent error discard; 3-block order; helper-vs-utility matrix.
# Fallback compile gate: cargo check -p <crate-name>
```

---

## Placement (AES702, AES703)

Place every `agent_*_orchestrator` file in the feature folder the linter expects, with its FRD/BACKLOG document pair beside the source.

```
crates/<feature_name>/src/agent_<domain>_orchestrator.rs    # Rust
```

An `agent_*_orchestrator` file belongs in a **feature folder** — a subdirectory of a member
directory (`crates/`, `modules/`, `packages/`) that is neither `shared/` nor a surface folder.
A feature folder is defined as a folder holding at least one `agent_*_orchestrator` and one
`capabilities_*` file. If a folder holds only capabilities without an agent of its own it is
an **AES702** violation; if the agent sits in a surface-dominated folder it is also
**AES702** and the file should be moved out of the surface folder.

An orchestrator may sit at the **member root** (e.g. `packages/agent_calculator_orchestrator.ts`)
when it routes across features rather than serving one. That makes it a routing aggregate
over the features beneath it — it holds one orchestrator per feature and delegates to them.
It does **not** satisfy AES702 for those folders: each feature still owes an orchestrator of
its own, so a capabilities folder under a member-level orchestrator is still a split feature.

`agent_*_orchestrator` files are forbidden in `shared/` — that is an **AES701** violation and
the file must be moved to a feature folder.

## Feature folder documents

The feature folder holding this agent also carries two documents at its root, beside the
source:

```text
<member_dir>/<feature_name>/FRD.md       # what the feature does — requirements
<member_dir>/<feature_name>/BACKLOG.md   # where its work stands
```

The folder name matches the feature it serves. A folder named after a generic word
(`utils`, `common`, `core`) describes no feature and cannot carry a meaningful FRD.

A feature folder carrying an agent but neither document is an **AES702** violation — the
linter names the missing file and you write it.

---

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **Is this orchestration only — or does it compute / touch I/O / hold domain data?**
   - *Computes/I-O/data* → move to capabilities / taxonomy / utility.
2. **Does an `_aggregate` contract exist?**
   - *No* → create it first with `aes-contract`.
3. **Block 2 only aggregate methods? ≥1 aggregate? ≤3 types?**
   - *No* → restructure to 1→2→3; extract free functions to utility.
4. **Any silently discarded errors or raw primitives in signatures?**
   - *Yes* → propagate/`Result`; wrap primitives in VOs.
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?**
   - *No* → fix findings, re-scan.

## Workflow

1. Confirm orchestration only — computation → capabilities, domain data → taxonomy, pure helpers → utility.
2. Ensure an `_aggregate` contract exists — if missing, run `aes-contract` first.
3. Resolve the feature agent dir. Run `lint-arwaky-cli scan <layer-path>` — findings are your work list.
4. Implement the 3 blocks from § Template / § Section Contract; inject the aggregate via DI.
5. Check imports, I/O, computation, error handling, and primitives against § Rules.
6. Register in the shared barrel, verify with `lint-arwaky-cli scan`, then compose via `aes-root` / consume via `aes-surface`.
7. Fallback compile gate: ``cargo check -p <crate-name>``.

## Pre-flight Checklist

- [ ] `lint-arwaky-cli scan <layer-path>` exits 0.
- [ ] Every touched section contract was checked.
- [ ] File registered in `mod.rs`.
- [ ] Language fallback compile clean: ``cargo check -p <crate-name>``.
- [ ] Related skills considered for the next layer up/down.

## Common Mistakes (Anti-Patterns)

The linter covers naming, imports, and primitives. These need a reader (§ Rules):

- **Computation or I/O in the agent**: arithmetic/parsing/FS/network belong below the agent layer.
- **Silently discarded errors**: no silent default-on-error in signatures.
- **Agent implements protocol instead of aggregate (or both)**: exactly one aggregate, 3-block order.

---

## Completion

The Rust agent file is complete when:

- `lint-arwaky-cli scan <layer-path>` exits 0.
- Fallback compile gate passes: `cargo check -p <crate-name>`.
- `lint-arwaky-cli scan <layer-path>` reports no AES702 or AES703 findings.
- The feature folder root carries `FRD.md` and `BACKLOG.md` beside the source.
- No `agent_*_orchestrator` file sits in `shared/`.

