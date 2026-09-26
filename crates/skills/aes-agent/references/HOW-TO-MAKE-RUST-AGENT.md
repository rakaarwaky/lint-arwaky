# HOW TO MAKE AGENT RUST

> **Purpose**: Orchestrate only: receive a request, call the aggregate contract, return shared VOs — no I/O, no computation.
>
> **Audience**: Agents and engineers scaffolding AES agent orchestrators.
>
> **Scope**: Python, Rust, and TypeScript `agent_<domain>_orchestrator` files — one aggregate, 3-block structure, zero I/O.
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
- Rule 2: Analysis orchestration → `Vec<<ResultVO>>`, match per-item into VO.
- Rule 3: Execution orchestration → `Result<ExecutionReport, AgentExecutionError>`.
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
```

---

## Section Contract

| Check | Why it belongs here |
| ----- | ------------------- |
| Block 1 → 2 → 3 order followed. | Required by AES layer rules and the linter; missing it is a defect. |
| Block 2: ONLY aggregate trait implementation. | Required by AES layer rules and the linter; missing it is a defect. |
| Block 3: constructors, std traits, private helpers. | Required by AES layer rules and the linter; missing it is a defect. |
| ≥1 struct implements aggregate trait; ≤3 total types. | Required by AES layer rules and the linter; missing it is a defect. |
| No local domain data; `Arc<dyn Trait>` for DI; shared VOs. | Required by AES layer rules and the linter; missing it is a defect. |
| Zero I/O, zero business logic, zero domain computation. | Required by AES layer rules and the linter; missing it is a defect. |
| No forbidden imports. | Required by AES layer rules and the linter; missing it is a defect. |
| Generic aggregate methods object-safe or `where Self: Sized`. | Required by AES layer rules and the linter; missing it is a defect. |
| Aggregate registered in shared crate `mod.rs`. | Required by AES layer rules and the linter; missing it is a defect. |
| `cargo check -p <crate-name>` passes. | Required by AES layer rules and the linter; missing it is a defect. |

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Manual (not machine-checked): zero I/O/computation; no silent error discard; 3-block order; helper-vs-utility matrix.
# Fallback compile gate: cargo check -p <crate-name>
```
