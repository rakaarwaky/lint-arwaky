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
> **Length**: One file per feature. A `_protocol` file may declare **one trait per**  
> **capability seam**; each trait is **rich** — one method per operation, each with a  
> concrete return type. An `_aggregate` file declares **exactly one method**.

---

## Rules

Seven rules. Each one prevents a specific failure mode.

1. **Suffix is strictly** `_protocol` **or** `_aggregate`**.** Type names: `I<Name>Protocol`,  
 `I<Name>Aggregate`. File: `contract_<concept>_<suffix>.rs`.
2. `pub trait` **only — methods end with** `;`**, no bodies.** Never default  
 implementations, never private-helper signatures (`AES101`/`AES102`).
3. **One file per feature, one trait per capability seam, each trait rich.** A  
 `_protocol` file for a feature with several capabilities declares one `pub trait` per  
 seam — `I<Injector>Protocol`, `I<Sender>Protocol`, `I<Stream>Protocol` — side by side  
 in the same file. Each trait carries **every** method its capability implements, each  
 with its own named signature and **one concrete return type**. Never `execute(op, …)`  
 dispatch, never an untyped argument bag, never an enum return spanning several value  
 shapes.
4. **A capability implements exactly one trait, and all of it.** Because each trait is  
 scoped to one capability's own operations, a trait is never partially implemented: it  
 declares only the methods that capability owns, and that capability defines every one  
 of them. This is why a single file per feature is enough — no per-method splitting, and  
 no unimplemented stubs.
5. **Aggregate = exactly one method.** The aggregate is the single entry point the  
 surface/root/CLI/MCP calls. All consumer verbs live in the agent, which dispatches to  
 the rich protocol traits. A dump-all `execute(op, …)` aggregate is the violation here.
6. **Signatures use shared VOs for domain values.** Domain values must not be numeric
   primitives — `i32`–`u64`, `f32`/`f64` — or bare string types — `String`, `&str` —
   wrap them in a taxonomy-defined VO before they appear in a signature.
   `bool` is permitted for semantic toggles or predicates (e.g. `enabled: bool`), never for domain quantities. A domain value travelling as a bare `String`, `&str`, or
   `str` in a contract signature is a leak; the fix is always the same: introduce a
   newtype (`LanguageVO`, `ToolName`, `LayerNameVO`, etc.) and accept `&LanguageVO`
   instead of `&str`. The borrow checker guarantees the callee receives only a
   validated identifier it knows how to use. A `language: LanguageVO` parameter is the right pattern; a raw `language: &str` is not. Enums are not
   banned; the ban targets primitive leakage. A multi-variant response enum on the
   aggregate is correct and expected — each variant must hold VO-wrapped values where
   a domain value is carried, and primitive payloads are acceptable for identifiers,
   counts, and flags. Protocol methods keep one concrete VO return type; the aggregate
   response enum is the sanctioned way to carry heterogeneous results across one
   `execute()` entry point. Protocol traits are object-safe and `Send + Sync` bounded.
7. **Register in shared** `mod.rs` so the pair is importable.

---

## Workflow

1. **Determine suffix** — `_protocol` (inward, rich traits) or `_aggregate` (outward, one  
 method).
2. **List the feature's capability seams** — group the feature's operations by the  
 capability that owns them. One group becomes one trait.
3. **Create file** → `contract_<concept>_<suffix>.rs`, one trait per seam.
4. **Draft each trait** — every method its capability implements, one concrete return  
 type each.
5. **Register** in shared `mod.rs`.
6. **Verify** → `lint-arwaky-cli scan <contract-dir>`.

## Template

Copy, fill, delete nothing.

### Protocol file — one trait per capability seam, each rich

```rust
/// <Feature>-domain capability contracts (AES102 `_protocol`).
///
/// One file for the <feature> feature. Each trait below is one capability
/// seam: a trait carries every method that capability implements, with one
/// concrete return type each, so a capability implements its trait outright
/// and never carries unimplemented stubs.

use shared::<domain>::taxonomy_<domain>_vo::{ResultVO, VO};

pub trait I<Seam1>Protocol: Send + Sync {
    /// <Seam 1> capability: <what it owns>.

    /// <What this operation does.> Return <what it returns>.
    fn <operation_1>(&self, param: &VO) -> ResultVO;

    /// <What this operation does.> Return <what it returns>.
    fn <operation_2>(&self, page: &PageVO, timeout: TimeoutVO) -> TextVO;
}

pub trait I<Seam2>Protocol: Send + Sync {
    /// <Seam 2> capability: <what it owns>.

    /// <What this operation does.> Return <what it returns>.
    fn <operation_1>(&self, path: PathVO, content: TextVO) -> PathVO;
}

pub trait I<SeamN>Protocol: Send + Sync {
    /// <Seam N> capability: <what it owns>.

    /// <What this operation does.> Return <what it returns>.
    fn <operation_1>(&self, /* … */) -> ResultVO;
}
```

**One file → one feature → one trait per capability seam → many named methods per trait.**

Violations: an `execute(op, …)` that dispatches several features behind one name, an  
untyped argument bag, a return enum that papers over differing result shapes, a trait  
that declares methods its implementors do not own, or an implementor that leaves a  
declared method unimplemented.

### Aggregate file — one method

```rust
/// <Feature>-domain aggregate contract (AES101 `_aggregate`).
///
/// The single entry point over the <feature> feature. Consumers pass a
/// request; the agent behind the aggregate dispatches to the rich protocol
/// traits in `contract_<feature>_protocol.rs`.

use shared::<domain>::taxonomy_<domain>_vo::{RequestVO, ResponseVO};

pub trait I<Feature>Aggregate: Send + Sync {
    /// Single entry point over the <feature> feature.
    fn execute(&self, request: RequestVO) -> ResponseVO;
}
```

**The aggregate is the one door consumers knock on.** Consumers never see the protocol  
traits' method lists; the agent behind the aggregate routes the request to the right  
capability. Adding a consumer verb = a new variant on `RequestVO` + the agent's dispatch  
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

| Section                                  | Why it belongs here                                                                |
| ---------------------------------------- | ---------------------------------------------------------------------------------- |
| Module docstring (required)              | Names the feature and the seams in the file.                                       |
| Suffix in file + trait names             | AES101/AES102 resolve `_protocol` vs `_aggregate` from the name.                   |
| One file per feature                     | All seams of a feature live together; consumers import from one module.            |
| One trait per capability seam            | A trait lists only what its capability owns, so implementation is always complete. |
| Rich named methods, one return type each | Protocol traits: each method has one VO return; no dispatch bag.        |
| Aggregate: exactly one method            | One `execute()` whose response is a taxonomy-defined enum of VOs.        |
| Method signatures only (`;`)             | Outer layers depend on promises, not behaviour.                                    |
| `Send + Sync` + object-safe              | Trait objects cross thread/task boundaries without surprises.                      |
| Aggregate: exactly one method            | Consumers depend on one stable entry point, not a shifting method list.            |
| Shared VOs in signatures                 | Numeric and string domain values stay behind VOs; only `bool` is permitted bare. |
| No impl-layer imports                    | Keeps the dependency arrow (capabilities → trait ← agent).                         |
| Register in shared `mod.rs`              | Importable without reaching into private modules.                                  |

---

## Verify

```bash
lint-arwaky-cli scan <contract-dir>
# Checks: AES101/AES102 (filename contract_<concept>_{protocol,aggregate}),
# AES201–AES205 (layer imports: no impl-layer imports; protocol ≠ aggregate import),
# AES402 (no primitives in signatures), role rules (contract ↔ capabilities/agent/surface).
# Manual (not machine-checked):
#   - every method in every protocol trait is named and individually typed
#     (no `execute(op, …)`, no untyped argument bag, no return enum spanning differing
#     result shapes);
#   - each capability implements one trait from this file, and implements all of it
#     (a partial implementation cannot compile — build each capability to prove it);
#   - the aggregate declares exactly one `execute()` method; its response type is a
#     taxonomy-defined VO enum (each variant holds VO-wrapped values, not primitives).
#   - protocol traits are object-safe (`Box<dyn I<Seam>Protocol>` must compile), `Send + Sync`
#     bounded, with no default bodies. A `where Self: Sized` bound on a protocol method would
#     break object safety and the DI pattern — do not add it.
# Fallback compile gate: cargo check -p <crate-name>.
```

**Why `&str` / `str` is banned in contracts.** A bare `String`, `&str`, or `str` in
a contract signature is not a placeholder — it is a type hole. The compiler cannot tell
whether `name: &str` is a tool name, a layer name, a file path, a YAML blob, or a
symbol identifier; none of those are interchangeable, but the signature gives no clue.
A newtype (e.g. `ToolName`, `LayerNameVO`) makes the intent explicit at the call site,
so a caller cannot accidentally pass a path where a tool name is expected without the
compiler complaining. This is the same reasoning that drove the rename: signatures must
communicate domain structure, not just runtime shape.

**Boolean in signatures vs. taxonomy.** `bool` is treated as a primitive in
`taxonomy_*_entity` field declarations (no carve-out; see
[../../aes-taxonomy/references/HOW-TO-MAKE-RUST-TAXONOMY.md](../../aes-taxonomy/references/HOW-TO-MAKE-RUST-TAXONOMY.md)).
In contract signatures (`contract_*_protocol` / `contract_*_aggregate`), `bool`
is permitted only for semantic toggles or predicates (e.g. `enabled: bool`)
because the signature describes *when* a capability runs, not the domain value
it operates on. A taxonomy entity field named `enabled` would still need an
enum-like VO, not a raw `bool`.
