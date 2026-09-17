---
name: create-agent
description: AES agent orchestrator scaffolding for Python Rust TS. Use when creating agent files.
metadata:
  tags: [python, rust, typescript, aes, agent, aggregate, orchestrator, structure, 3-block-structure, di, orchestration, vo]
  triggers:
    - "create agent"
    - "create agent python"
    - "create agent rust"
    - "create agent typescript"
    - "add agent"
    - "add agent python"
    - "add agent rust"
    - "fix agent structure"
    - "create aggregate"
    - "create aggregate python"
    - "create aggregate rust"
    - "create aggregate typescript"
    - "agent missing aggregate"
    - "validate agent logic"
    - "check agent"
    - "check agent python"
    - "audit agent"
    - "audit agent python"
    - "audit agent rust"
  dependencies: []
  related:
    - create-capabilities
    - create-taxonomy
    - create-contract
---
# Create Agent (AES)

The **agent layer performs orchestration only**. It receives a request, calls its aggregate
contract, and returns shared VOs. It never touches the filesystem, network, or database; it never
computes domain values; it never stores domain data locally.

Naming (AES101/AES102): `agent_<domain>_orchestrator.<ext>` — `_orchestrator` is the only allowed
suffix. It implements exactly one contract aggregate.

Read `references/python.md`, `references/rust.md`, or `references/typescript.md` for that
language's import lists, 3-block layout, templates, and verify command.

## Language split

| Language   | Aggregate contract        | DI mechanism                | Register / verify                |
| ------------ | --------------------------- | ----------------------------- | ---------------------------------- |
| Python     | ABC in `contract_*`       | Constructor param (ABC)     | `__init__.py`, `python -c "import <module>"` |
| Rust       | Trait in `contract_*`     | `Arc<dyn I<Name>Aggregate>` | `mod.rs`, `cargo check -p <crate-name>` |
| TypeScript | Interface in `contract_*` | Constructor param (interface) | `index.ts`, `npx tsc --noEmit`   |

Forbidden in every language: importing `capabilities_*`, other `agent_*`, or any `surface_*`;
and language I/O APIs (`open()`/`Path()`/`os.*`/HTTP+DB clients, `std::fs`/`reqwest`/`sqlx`,
`fs.*`/`fetch`/`axios`). Allowed: control flow, error propagation, awaiting/consuming injected
dependencies, collecting results into shared VOs.

## 3-Block structure (mandatory order)

1. **Block 1 — Type definition & constructor:** the struct/class and its injected dependencies.
2. **Block 2 — Aggregate method implementation:** only methods declared by the aggregate contract.
3. **Block 3 — Constructors, standard-protocol methods, factories, private helpers.**

Placement rule: anything that is a free/module-level function or a pure static with no access to
the type is **extracted to `*_utility`**; anything bound to the instance, a factory, or a dunder /
`Display` / `toString` goes to Block 3; contract methods go to Block 2.

## Hard rules

1. **No computation.** Arithmetic, totals, averages, folds/reduces, parsing, and normalization
   belong to capabilities. Iterating to call an injected dependency and routing its result is fine.
2. **No local domain data.** Domain types come from taxonomy VOs; behaviour comes from the
   aggregate contract. No raw primitives in signatures (`str`/`int`/`float`, `String`/`i32`/`f64`,
   `string`/`number`) — booleans only as semantic toggles.
3. **Never silently discard an error.** No `... or ""`, `unwrap_or_default()`, or `?? ""` on a
   result. Analysis orchestration returns a collection of result VOs with per-item handling;
   execution orchestration returns a result type (`Result[...]` / `Result<...>` / thrown error).
   I/O failures are mapped by capabilities; the agent only wraps them into a VO.
4. **No magic constants** — literals live in taxonomy `_constant` files.
5. **Size limits (AES405/AES301):** at least one aggregate implementation, at most 3 types per
   file. Rust: generic aggregate methods must be object-safe or gated `where Self: Sized`.
6. **Register** the agent in the crate/package barrel so it can be composed by the root layer.

## Workflow

1. Confirm the work is orchestration only — move computation to capabilities, data to taxonomy.
2. If no aggregate contract exists, create it with `create-contract` first.
3. Implement the 3 blocks from the language reference template.
4. Inject dependencies (Python/TS constructor param, Rust `Arc<dyn Trait>`); no direct construction.
5. Check imports, I/O, computation, error handling, and primitives against the rules above.
6. Register in `__init__.py` / `mod.rs` / `index.ts` and run the verify command.

## Checklist

- [ ] File is `agent_<domain>_orchestrator` (only `_orchestrator` suffix allowed).
- [ ] Block 1 → 2 → 3 order followed; Block 2 contains ONLY aggregate contract methods.
- [ ] Block 3 holds constructors, standard-protocol methods, factories, private helpers.
- [ ] ≥1 type implements the aggregate contract; ≤3 types total.
- [ ] No local domain data; DI through the injected contract; shared VOs only.
- [ ] Zero I/O, zero business logic, zero domain computation.
- [ ] No forbidden imports (capabilities, other agents, surface).
- [ ] No silently discarded errors, no raw primitives in contracts, no magic constants.
- [ ] Rust only: `Arc<dyn Trait>` DI, generic methods object-safe or `where Self: Sized`.
- [ ] Registered in `__init__.py` / `mod.rs` / `index.ts`; verify command passes.
