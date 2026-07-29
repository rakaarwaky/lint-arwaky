---
name: create-agent-rust
description: "Create and validate Rust agent layer files following AES rules: orchestration-only, zero I/O, zero business logic, zero domain computation, 3-block structure, max 3 types per file, aggregate contracts, DI for service dependencies, and shared VOs for domain data."
metadata:
  tags:
    [
      rust,
      aes,
      agent,
      aggregate,
      structure,
      3-block-structure,
      di,
      orchestration,
      vo,
    ]
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

Agent layer = **orchestration / pipeline execution only**. No I/O, no business logic, no domain computation, no local domain data. Depends only on Taxonomy + Contract + Utility layers.

## Definition of Done

1. At least one struct implements an aggregate trait (Block 2).
2. Block 2 contains ONLY the aggregate trait implementation.
3. Constructors, std trait impls, private helpers → Block 3.
4. Zero I/O, zero business logic, zero domain computation.
5. No locally defined domain data structures.
6. Service deps via `Arc<dyn Trait>`.
7. Config/value fields use shared VOs.
8. Aggregate signatures use shared VOs.
9. Total struct + enum ≤ 3.
10. `cargo check -p <crate-name>` passes.

---

## Layer Boundaries

| Allowed | Forbidden |
| --- | --- |
| Orchestration flow (`for`, `while`, `loop`) | Domain computation / arithmetic / analytics |
| Control flow (`if/else`, `match`) | Data transformation / business rules / domain validation |
| Calling injected aggregate/protocol methods | File I/O (`std::fs`, `File::open`) |
| Error propagation (`?`, `match Err`) | Network calls (`reqwest`, `hyper`) |
| Collecting results into shared VO types | Database operations (`sqlx`, `rusqlite`) |
| Async coordination (`tokio::join!`, `.await`) | Direct stdout/stderr / env / global mutation |
| Aggregate trait implementation + private helpers | Import from `capabilities_*`, `agent_*`, `surface_*` |
| | Locally defined domain data / raw primitives in aggregate contracts |

**Allowed imports:** `shared::*` — taxonomy VOs, constants, aggregate traits, protocol traits.

---

## The 3-Block Structure

Every file MUST follow this order:

```text
// ─── Block 1: Struct Definition ────────────────────────────
// ─── Block 2: Aggregate Trait Implementation ───────────────
// ─── Block 3: Constructors, Std Traits, Helpers ────────────
```

- **Block 1** — struct definition only. See `templates/agent_name.rs`.
- **Block 2** — ONLY `impl I<Name>Aggregate for <Name>Orchestrator { ... }`. No `impl Default`, `impl Display`, `fn new()` here.
- **Block 3** — `fn new()`, std trait impls (`Default`, `Clone`, `Display`), private helpers.

### Method Placement

```text
Free function (outside impl)?             → EXTRACT to *_utility.rs
In aggregate trait?                       → Block 2
std trait impl (Default/Clone/Display)?   → Block 3
fn new() / constructor?                   → Block 3
Private helper (uses &self)?              → Block 3
Pure fn, no struct dep?                   → EXTRACT to *_utility.rs
```

---

## Helper vs Utility

**Keep in Block 3** if ANY: accesses `self`, tightly coupled to this struct, constructor, agent-specific pipeline/domain knowledge, stateless but single-use.

**Extract to `*_utility.rs`** only if ALL: stateless (no `self`/`Self`), pure, no side effects, domain-agnostic, reusable across modules.

> I/O Rule: stateless + I/O + domain-agnostic = taxonomy utility. Stateless + I/O + domain-specific = capabilities.

```text
Found reusable code in agent?
  │
  ├─ Knows agent-specific or domain-specific details?
  │   └─ YES → Block 3
  ├─ Needs &self or struct state?
  │   └─ YES → Block 3
  └─ Stateless, domain-agnostic, reusable?
      └─ YES → extract to shared taxonomy utility (I/O allowed)
```

---

## Computation, Error Handling, and VO Rules

**Computation** — forbidden: arithmetic, totals, averages, counts as domain decisions, `.sum()`, `.fold()`, parsing, normalization. Allowed: iterating to call deps, routing results, propagating errors.

> Distinction: `for file in files { self.checker.check(file) }` = orchestration. `files.iter().map(|f| f.size()).sum()` = computation → capabilities.

**Error handling:**
- Rule 1: Never silently discard — forbidden: `checker.check().unwrap_or_default()`.
- Rule 2: Analysis orchestration → return `Vec<<ResultVO>>`, match per-item into VO.
- Rule 3: Execution orchestration → return `Result<ExecutionReport, AgentExecutionError>`.
- Rule 4: Delegate I/O errors to capabilities — agent only catches and wraps into VO.

**VOs** — aggregate contracts must use shared VOs, not raw primitives.

| Primitive | Rule |
| --- | --- |
| `String`, `i32`, `i64`, `u32`, `u64`, `usize`, `f32`, `f64`, `char` | Forbidden for domain fields/contracts. Use VO. |
| `bool` | Allowed for semantic toggles only. |
| `&str` | May be used for borrowed low-level input; domain identifiers → VO. |

---

## Templates

| File | Purpose |
| --- | --- |
| `templates/agent_name.rs` | Full agent implementation (3-block structure) |
| `templates/contract_name_aggregate.rs` | Aggregate trait definition |
| `templates/mod.rs` | Module registration |

---

## Workflow

1. **Analyze** — Is this orchestration only? If computation → capabilities. If domain data → taxonomy.
2. **Check aggregate** — Does agent struct implement an aggregate trait? If no → create `contract_<name>_aggregate.rs`.
3. **Enforce 3-Block** — Reorganize: struct def → aggregate trait impl → constructors/std traits/helpers.
4. **Type discipline** — ≥1 aggregate trait, ≤3 types (struct+enum), `Arc<dyn Trait>` for DI, shared VOs.
5. **Helper boundary** — Apply Helper vs Utility rules above.
6. **Layer compliance** — No forbidden imports, no I/O, no business logic, no computation.
7. **Error/VO/constants** — No silent swallowing, no raw primitives in contracts, no magic constants.
8. **Compile** — `cargo check -p <crate-name>`.

---

## Verification Checklist

- [ ] 3-Block Structure followed (Block 1 → 2 → 3).
- [ ] Block 1: exactly one implementation struct.
- [ ] Block 2: ONLY the aggregate trait implementation.
- [ ] Block 3: constructors, std trait impls, private helpers.
- [ ] Agent struct implements an aggregate trait.
- [ ] Aggregate: only public contract methods, no private helpers, no constructors.
- [ ] Reusable stateless domain-agnostic functions extracted to `*_utility.rs`.
- [ ] Generic aggregate methods are object-safe or bounded with `where Self: Sized`.
- [ ] One implementation struct per file, ≤3 total types.
- [ ] No local domain data — all imported from `shared/taxonomy`.
- [ ] Service deps via `Arc<dyn Trait>`.
- [ ] Shared VOs for config fields and aggregate signatures.
- [ ] Zero I/O, zero business logic, zero domain computation.
- [ ] No forbidden imports (`capabilities_*`, `agent_*`, `surface_*`).
- [ ] Aggregate registered in shared crate's `mod.rs`.
- [ ] `cargo check -p <crate-name>` passes.
