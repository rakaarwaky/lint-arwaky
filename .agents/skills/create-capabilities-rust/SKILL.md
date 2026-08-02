---
name: create-capabilities-rust
description: "Create and validate Rust capabilities layer files following AES rules: concrete implementation of behavior (business logic + external adaptation), 3-block structure, max 3 types per file, protocol trait contracts, DI for service dependencies, and shared VOs for domain data."
metadata:
  tags: [rust, aes, capabilities, protocol, 3-block-structure, di, vo]
  triggers:
    - "create capabilities rust"
    - "add capabilities rust"
    - "fix capabilities structure rust"
    - "create protocol rust"
    - "capabilities missing protocol rust"
    - "validate capabilities logic rust"
    - "check capabilities rust"
    - "audit capabilities rust"
  dependencies: []
  related:
    - create-agent-rust
    - create-taxonomy-rust
    - create-contract-rust
    - create-utility-rust
---
# create-capabilities-rust

Capabilities = concrete protocol trait implementation. File: `capabilities_<domain>_<role>.rs`.

**Allowed imports:** Taxonomy, Contract (`_protocol` only), Utility.
**Forbidden:** `agent_*`, other `capabilities_*`, `surface_*`, local domain models, magic constants.

## Examples Role Naming

**Internal:** validator, assessor, calculator, resolver, classifier, selector, mapper, transformer, policy, enricher, evaluator, analyzer, scorer, grader, ranker, filter, checker, reviewer, approver, rejector
**External:** repository, gateway, client, provider, fetcher, reader, writer, scanner, executor, publisher, subscriber, adapter, connector, uploader, downloader, sender, receiver, dispatcher, watcher, monitor

## Structure Rules

- Rule 1: Internal helper structs without trait impl → ALLOWED.
- Rule 2: ≥1 struct implements a protocol trait.
- Rule 3: Total struct + enum ≤ 3.

## 3-Block Structure

```text
// Block 1: Struct Definition
// Block 2: Protocol Trait Implementation
// Block 3: Constructors, Std Traits, Helpers
```

## Helper vs Utility Decision Matrix

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

## Templates

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

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
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

## Workflow

1. Confirm implements protocol behavior (not orchestration/data/mechanics).
2. File `use shared::..._protocol::I<Name>` — if missing → flag `CapabilityNoProtocol`.
3. Create `contract_<name>_protocol.rs` if missing.
4. Enforce 3-Block with explicit `// Block 1:`, `// Block 2:`, `// Block 3:` comments.
5. AES403: ≥1 trait implementor, ≤3 types, `Arc<dyn Trait>` for DI, shared VOs.
6. No forbidden imports, no inter-capability deps, no local domain models.
7. `cargo check -p <crate-name>`.

## Checklist

- [ ]  Block 1 → 2 → 3 order followed with explicit comments.
- [ ]  Block 2: ONLY `impl I<Name>Protocol for ...`.
- [ ]  ≥1 struct implements protocol trait; ≤3 total struct+enum.
- [ ]  Imports from `_protocol` module or Utility only.
- [ ]  No local domain models, no agent/capability imports.
- [ ]  `Arc<dyn Trait>` for DI; shared VOs for fields and trait signatures.
- [ ]  Constants → `taxonomy_<domain>_constant.rs`.
- [ ]  Helper functions in Block 3 are `private` or `pub(crate)` (not fully `pub` unless justified).
- [ ]  Low-level, reusable, stateless ops → moved to Utility.
- [ ]  `cargo check -p <crate-name>` passes.

```
```
