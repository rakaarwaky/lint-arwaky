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

## Workflow

1. **Triage (diagnostic tree)** — ask in order; the first "No" dictates the next action: orchestration only (no I/O, no computation, no local domain data)? an `_aggregate` contract exists? — no: create it first with `aes-contract`. Block 2 holds only aggregate methods; ≥1 aggregate; ≤3 types? no silently discarded errors; no raw primitives in signatures (wrap in VOs; propagate errors)? `lint-arwaky-cli scan <layer-path>` exits 0?
2. Confirm orchestration only — computation → capabilities, domain data → taxonomy, pure helpers → utility.
3. Ensure an `_aggregate` contract exists — if missing, run `aes-contract` first.
4. Resolve the feature agent dir. Run `lint-arwaky-cli scan <layer-path>` — findings are the work list.
5. Implement the 3 blocks from § Template / § Section Contract; the class implements the aggregate trait via DI.
6. Check imports, I/O, computation, error handling, and primitives against § Rules.
7. Register in `mod.rs`, then compose via `aes-root` / consume via `aes-surface`.
8. **Placement (AES702/AES703)** — the file sits in a feature folder (`crates/<feature_name>/src/agent_<domain>_orchestrator`), not in `shared/` or a surface-dominated folder; `FRD.md` + `BACKLOG.md` at the feature folder root beside the source; a capabilities-only folder without its own agent is a split feature.
9. Fallback compile gate: `cargo check -p <crate-name>`.

## Review

- Block 1 → 2 → 3 order; Block 2 holds only aggregate methods; helper-vs-utility split respected (free functions extracted to `*`*_utility.rs`) — § Rules § 3-Block Structure, § Helper vs Utility.
- Orchestration only: zero I/O, zero computation, zero domain data; DI via protocol traits; shared VOs in signatures — § Rules § Allowed / forbidden operations, § Computation, Errors, VOs.
- No silently discarded errors (`unwrap_or_default()` / `?? ""` on results); errors propagate or wrap into VOs — § Rules Error rules.
- No agent file in `shared/`; capabilities-only folders carry an agent of their own — § Workflow step 8.
- `contract_<name>_aggregate.rs` exists; ≥1 aggregate, ≤3 types.

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Language compile is a fallback gate only: `cargo check -p <crate-name>`
```
