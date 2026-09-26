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
4. Enforce 3-Block with explicit `// Block 1:`, `// Block 2:`, `// Block 3:` comments.
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

    // HELPERS: Should be `private` (no `pub`) or `pub(crate)` for testing.
    // If a helper needs to be fully `pub` and reusable across modules, extract it to Utility.
    fn helper_method(&self) -> bool {
        // internal logic
        true
    }
}
```

---

## Section Contract


| Check                                                                                         | Why it belongs here                                                 |
| --------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| Block 1 → 2 → 3 order followed with explicit comments.                                        | Required by AES layer rules and the linter; missing it is a defect. |
| Block 2: ONLY `impl I<Name>Protocol for ...`.                                                 | Required by AES layer rules and the linter; missing it is a defect. |
| ≥1 struct implements protocol trait; ≤3 total struct+enum.                                    | Required by AES layer rules and the linter; missing it is a defect. |
| Imports from `_protocol` module or Utility only.                                              | Required by AES layer rules and the linter; missing it is a defect. |
| No local domain models, no agent/capability imports.                                          | Required by AES layer rules and the linter; missing it is a defect. |
| `Arc<dyn Trait>` for DI; shared VOs for fields and trait signatures.                          | Required by AES layer rules and the linter; missing it is a defect. |
| Constants → `taxonomy_<domain>_constant.rs`.                                                  | Required by AES layer rules and the linter; missing it is a defect. |
| Helper functions in Block 3 are `private` or `pub(crate)` (not fully `pub` unless justified). | Required by AES layer rules and the linter; missing it is a defect. |
| Low-level, reusable, stateless ops → moved to Utility.                                        | Required by AES layer rules and the linter; missing it is a defect. |
| `cargo check -p <crate-name>` passes.                                                         | Required by AES layer rules and the linter; missing it is a defect. |


---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Manual (not machine-checked): 3-block order; Block 2 only protocol methods; helper-vs-utility matrix; role naming lists.
# Fallback compile gate: cargo check -p <crate-name>
```

