# HOW TO MAKE CONTRACT RUST

> **Purpose**: State a Rust contract's public promises (protocol or aggregate) so outer  
> layers can depend on the seam without importing a concretion.
>
> **Audience**: Agents and engineers scaffolding AES contract traits in the shared domain.
>
> **Scope**: Pure trait definitions in `contract_<concept>_<suffix>.rs` — `_protocol` or  
> `_aggregate` only. No default bodies.
>
> **Location**: Shared domain crate next to taxonomy, registered in the shared `mod.rs`.
>
> **Length**: `_protocol` = every method the capabilities expose; `_aggregate` = one entry  
> point. Only methods outer layers actually call.

---

## Rules

Seven rules. Each one prevents a specific failure mode.

1. **Suffix is strictly** `_protocol` **or** `_aggregate`**.** Type names: `I<Name>Protocol`,  
 `I<Name>Aggregate`. File: `contract_<concept>_<suffix>.rs`.
2. `pub trait` **only — methods end with** `;`**, no bodies.** Never default  
 implementations, never private-helper signatures (`AES101`/`AES102`).
3. **Protocol = rich, one method per capability operation.** The trait declares every  
 operation its capabilities implement, each with its own named method and its own typed  
 signature. A capability implements the whole trait. No `execute(op, …)` dispatch, no  
 untyped argument bag, no enum-of-everything return — every method has one concrete  
 return type.
4. **Aggregate = ONE method.** The aggregate is the single entry point the  
 surface/root/CLI/MCP calls. All consumer verbs live in the agent, which dispatches to  
 the rich protocol. A dump-all `execute(op, …)` aggregate is the violation here, not a  
 rich one.
5. **Allowed imports: taxonomy types and other contract types only.** Capabilities,  
 agents, surface, root invert the dependency arrow (`AES201`/`AES205`).
6. **Signatures use shared VOs** — no `String`/`i32`..`u64`/`f32`/`f64`/`Vec<String>` for  
 domain values. `bool` and `&str` (non-domain input) allowed with care. No union return  
 spanning several value shapes; when operations genuinely differ in return type, they  
 are separate methods. Trait object-safe (generic members `where Self: Sized`) and  
 `Send + Sync` bounded. All methods type-annotated.
7. **Register in shared** `mod.rs` so the pair is importable.

---

## Workflow

1. **Determine suffix** — `_protocol` (inward, rich) or `_aggregate` (outward, one method).
2. **Create file** → `contract_<concept>_<suffix>.rs`.
3. **Draft protocol** — one trait, one method per capability operation, each with a  
 concrete return type.
4. **Draft aggregate** — one trait, one method.
5. **Register** in shared `mod.rs`.
6. **Verify** → `lint-arwaky-cli scan <contract-dir>`.

## Template

Copy, fill, delete nothing.

### Protocol trait — rich, one named method per operation

```rust
use shared::<domain>::taxonomy_<domain>_vo::{ResultVO, VO};

pub trait I<Name>Protocol: Send + Sync {
    /// Capability contract for <domain>: every operation the capabilities expose.

    /// <What this operation does.>
    fn <operation_1>(&self, param: &VO) -> ResultVO;

    /// <What this operation does.>
    fn <operation_2>(&self, page: &PageVO, timeout: TimeoutVO) -> TextVO;

    /// Add one method per new capability operation.
    fn <operation_n>(&self, /* … */) -> ResultVO;
}
```

**One file → one trait → many named methods.** Each method is a real, typed operation.  
Violations: an `execute(op, …)` that dispatches several features behind one name, an  
untyped argument bag, or a return enum that papers over differing result shapes. A  
capability implements the whole trait, so it never carries unimplemented stubs for  
operations it does not own — if it does not own them, it does not implement this trait.

### Aggregate trait — one method

```rust
use shared::<domain>::taxonomy_<domain>_vo::{RequestVO, ResponseVO};

pub trait I<Name>Aggregate: Send + Sync {
    /// Single entry point over <domain>; the agent dispatches internally.
    fn execute(&self, request: RequestVO) -> ResponseVO;
}
```

**The aggregate is the one door consumers knock on.** Consumers never see the protocol's  
method list; the agent behind the aggregate routes the request to the right capability  
operation. Adding a consumer verb = a new variant on `RequestVO` + the agent's dispatch  
arm, not a new aggregate method.

### mod.rs

```rust
// <domain> — contract traits for <domain> operations
pub mod contract_<name>_protocol;
pub mod contract_<name>_aggregate;
```

---

## Section Contract

Every contract file is required to carry the rows that apply. Each exists for one reason.


| Section                            | Why it belongs here                                                            |
| ---------------------------------- | ------------------------------------------------------------------------------ |
| Module docstring (required)        | Names the contract's role: capability trait or entry-point trait.              |
| Suffix in file + trait name        | AES101/AES102 resolve `_protocol` vs `_aggregate` from the name.               |
| Protocol: rich, named methods      | Each operation stays typed and discoverable; no dispatch bag, no union return. |
| Protocol: one concrete return type | Callers know the result shape without narrowing an enum.                       |
| Aggregate: exactly one method      | Consumers depend on one stable entry point, not a shifting method list.        |
| Method signatures only (`;`)       | Outer layers depend on promises, not behaviour.                                |
| `Send + Sync` + object-safe        | Trait objects cross thread/task boundaries without surprises.                  |
| Shared VOs in signatures           | Domain values stay opaque across layers; no primitive leakage.                 |
| No impl-layer imports              | Keeps the dependency arrow (capabilities → trait ← agent).                     |
| Register in shared `mod.rs`        | Importable without reaching into private modules.                              |


---

## Verify

```bash
lint-arwaky-cli scan <contract-dir>
# Checks: AES101/AES102 (filename contract_<concept>_{protocol,aggregate}),
# AES201–AES205 (layer imports: no impl-layer imports; protocol ≠ aggregate import),
# AES402 (no primitives in signatures), role rules (contract ↔ capabilities/agent/surface).
# Manual (not machine-checked): protocol methods are all named and individually typed
# (no `execute(op, …)`, no untyped argument bag, no return enum spanning differing result
# shapes); aggregate declares exactly one method; traits object-safe, Send + Sync,
# no default bodies.
# Fallback compile gate: cargo check -p <crate-name>.
```

