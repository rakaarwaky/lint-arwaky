# HOW TO MAKE CAPABILITIES RUST

> **Purpose**: Implement concrete protocol behaviour: domain rules and external adaptation as protocol implementations.
>
> **Audience**: Agents and engineers scaffolding AES capability files.
>
> **Scope**: Python, Rust, and TypeScript `capabilities_<domain>_<role>` files — 3-block structure, ≥1 protocol implementor, ≤3 types.
>
> **Location**: Feature domain package; depends only on taxonomy, `_protocol` contracts, and utility.
>
> **Length**: At most 3 types per file; Block 1 → 2 → 3 order.

---

## Rules

### Capability vs Utility Boundary

A capability file **implements a protocol** — it has state (via DI), business rules, and concrete behaviour.
A utility file **performs a pure operation** — no state, no business rules, just a narrow task.

| Decision | → Capability | → Utility |
|---|---|---|
| Has `&self` / instance state / injected dependencies | Yes | No |
| Contains business rules or protocol logic | Yes | No |
| Stateless, domain-agnostic, ≥2 consumers | — | Yes |
| Implements `_protocol` trait | Yes | No |
| Pure free function with no business logic | No | Yes |

**Rule**: If a method in Block 3 is stateless (`fn` without `&self`), domain-agnostic, and reusable across modules — extract it to a `utility_*` file. It does not belong in the capability layer.

### Import rules

**Allowed imports:** Taxonomy, Contract (`_protocol` only), Utility.
**Forbidden:** `agent_*`, other `capabilities_*`, `surface_*`, local domain models, magic constants.

### Structure rules (Rust)

- Rule 1: Internal helper structs without trait impl → ALLOWED.
- Rule 2: ≥1 struct implements a protocol trait.
- Rule 3: Total struct + enum ≤ 3.

### 3-Block Structure

```text
// Block 1: Struct Definition
// Block 2: Protocol Trait Implementation
// Block 3: Constructors, Std Traits, Helpers
```

### Helper vs Utility Decision Matrix

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

### Workflow

1. Confirm implements protocol behavior (not orchestration/data/mechanics).
2. File `use shared::..._protocol::I<Name>` — if missing → flag `CapabilityNoProtocol`.
3. Create `contract_<name>_protocol.rs` if missing.
4. Enforce 3-Block with explicit `// Block 1:`, `// Block 2:`, `// Block 3:` comments — AES403 `CapabilityBlockMarkers` reports a file whose banners are missing, out of order, or above 3.
5. AES403: ≥1 trait implementor, ≤3 types, `Arc<dyn Trait>` for DI, shared VOs.
6. No forbidden imports, no inter-capability deps, no local domain models.
7. `cargo check -p <crate-name>`.

---

## Template

```rust
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

// ─── Block 2: Protocol Trait Implementation ───────────────
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

    // HELPERS: `pub fn` is only for genuine API. A helper with no caller in
    // production code should be `fn` (private) or `pub(crate)` if a `#[cfg(test)]`
    // module in the same file calls it. Integration tests under `tests/` compile
    // as a separate crate, so a helper they call must stay `pub`.
    fn helper_method(&self) -> bool {
        // internal logic
        true
    }
}
```

---

## Section Contract

The `Enforced` column says which rule reports a violation, so you know what the
linter will catch and what stays a review responsibility. `Manual` means no rule
checks it — the shape is still the contract, but a violation is caught by review
or by `cargo`, not by `scan`.

| Check                                                        | Enforced                                                          |
| ------------------------------------------------------------ | ----------------------------------------------------------------- |
| All three `// Block 1:` / `// Block 2:` / `// Block 3:` banners present, in order, none above 3. | **AES403** `CapabilityBlockMarkers` (MEDIUM)                      |
| Block 2 (protocol impl) precedes Block 3 (inherent impl).    | **AES403** `CapabilityBlockOrder` (HIGH)                          |
| ≥1 struct implements a protocol trait.                       | **AES403** `CapabilityNoImplementor` (MEDIUM)                     |
| Exactly 1 protocol trait per file.                           | **AES403** `CapabilityMultiProtocol` (MEDIUM)                     |
| ≤3 total struct + enum.                                      | **AES403** `CapabilityTooManyTypes` (HIGH)                        |
| Imports from `_protocol` or Utility only.                    | **AES201**–**AES205** (import rules)                              |
| No agent / surface / root imports.                           | **AES201** `FORBIDDEN_IMPORT`                                     |
| Block 2 holds ONLY `impl I<Name>Protocol for ...`.           | Manual — no rule reads Block 2's contents                        |
| `Arc<dyn Trait>` for DI; shared VOs for fields and signatures. | Manual                                                          |
| Constants → `taxonomy_<domain>_constant.rs`.                 | **AES403** `CapabilityLocalConstant` (MEDIUM)                     |
| Block 3 helpers with no production caller are `fn` or `pub(crate)`, not `pub`. | **AES403** `CapabilityPublicHelper` (MEDIUM)       |
| `#[cfg(test)] mod tests` lives in `tests/`, never inline.    | **AES403** `CapabilityEmbeddedTest` (LOW)                         |
| Low-level, reusable, stateless ops → moved to Utility.       | Manual — the helper-vs-utility matrix below is a judgement call   |
| `cargo check -p <crate-name>` passes.                        | `cargo`, not `scan`                                               |

**On the block banners.** A banner is `Block <digits>:` standing as its own word
inside a comment. The colon is load-bearing: prose such as
`Block 1 (types) -> Block 2` is not a marker, and neither is `Sub-Block 4:`. Put
the banner above the item it heads — and above that item's own `///` doc comment,
never between the doc comment and the item, which would detach the documentation.

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# AES403 machine-enforced: type budget ≤3; exactly 1 protocol trait; protocol
#   implementor present; block order (protocol impl before inherent impl);
#   all three block banners present, in order, none above 3; local constants
#   in taxonomy file, not in capabilities file; no inline #[cfg(test)] mod
#   tests; pub helpers without production callers flagged.
# Manual (not machine-checked): Block 2 contents; helper-vs-utility matrix;
#   Arc<dyn Trait> for DI; role naming lists.
# Fallback compile gate: cargo check -p <crate-name>
```
