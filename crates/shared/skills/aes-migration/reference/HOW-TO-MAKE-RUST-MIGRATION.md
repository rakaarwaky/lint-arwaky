# HOW TO MAKE MIGRATION RUST

> **Purpose**: Guide phased migration of legacy Rust projects into AES layered architecture — taxonomy → contract → utility → capabilities → agent → surface → root.
>
> **Audience**: Agents and engineers executing a migration to AES.
>
> **Scope**: Phase-based migration workflow for Rust projects; references layer skills for execution.
>
> **Location**: Project root; each phase operates on a layer directory.
>
> **Length**: 9 phases (0–8); total duration depends on violation count.

---

## Rules



## Workflow

1. **Phase 0 — Audit** — run `lint-arwaky-cli scan .`; record baseline.
2. **Phase 1 — Taxonomy** — extract VOs, errors, constants.
3. **Phase 2 — Contract** — create protocol (1 method) + aggregate (many exports).
4. **Phase 3 — Utility** — extract stateless helpers to shared.
5. **Phase 4 — Capabilities** — implement protocols with business logic.
6. **Phase 5 — Agent** — implement aggregate, delegate to capabilities.
7. **Phase 6 — Surface** — map I/O, call aggregate.
8. **Phase 7 — Root** — wire containers, bootstrap entry.
9. **Phase 8 — Verify** — full scan → 0 violations; compile clean.

Nine rules. Each one governs one migration phase.

1. **Phase 0 first — audit before touching anything.** Run `lint-arwaky-cli scan .`; record baseline violations to choose strategy.
2. **Taxonomy before contract before capabilities.** VOs must exist before protocols can reference them.
3. **Protocol = one method per feature.** Never a mega `execute(op, …)` that dispatches multiple features.
4. **Aggregate = many methods, one per export.** Rich consumer surface; surface/root call specific verbs.
5. **Utility is stateless free functions only.** No struct, no impl, no upward imports (AES404).
6. **Capabilities implement protocol; agent implements aggregate.** No cross-layer imports (AES201).
7. **Surface calls aggregate; never imports agent or capabilities.** Dependency arrow points down (AES201 purpose).
8. **Root wires everything; never contains business logic.** Container constructs; entry bootstraps.
9. **Verify at every phase.** `lint-arwaky-cli scan <layer-dir>` → 0 before moving to next phase.

---



## Template

Copy, fill, delete nothing. Migration proceeds phase-by-phase; each phase outputs files matching the layer HOW-TO.

### Phase 1 — Taxonomy

```rust
// crates/shared/src/<domain>/taxonomy_<domain>_vo.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct <VOName>(pub String);

impl <VOName> {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}
```

### Phase 2 — Contract

```rust
// crates/shared/src/<domain>/contract_<domain>_protocol.rs
use super::taxonomy_<domain>_vo::*;

pub trait I<Domain>Protocol: Send + Sync {
    fn <feature>(&self, arg: &<VOName>) -> Result<<ResultVO>, <ErrorVO>>;
}

// crates/shared/src/<domain>/contract_<domain>_aggregate.rs
pub trait I<Domain>Aggregate: Send + Sync {
    fn list_<thing>(&self) -> Vec<<VOName>>;
    fn create_<thing>(&self, arg: &<VOName>) -> ExitCode;
}
```

### Phase 3 — Utility

```rust
// crates/shared/src/<domain>/utility_<domain>_<role>.rs
use super::taxonomy_<domain>_vo::*;

pub fn <helper>(arg: &<VOName>) -> <VOName> {
    // Stateless — no state, no struct, no impl
    <VOName>(arg.0.to_lowercase())
}
```

### Phase 4 — Capabilities

```rust
// crates/<domain>/src/capabilities_<domain>_<role>.rs
use shared::<domain>::contract_<domain>_protocol::I<Domain>Protocol;
use shared::<domain>::taxonomy_<domain>_vo::*;

pub struct <Capability> {
    dep: Arc<dyn SomeTrait>,
}

impl I<Domain>Protocol for <Capability> {
    fn <feature>(&self, arg: &<VOName>) -> Result<<ResultVO>, <ErrorVO>> {
        // Business logic here
        ...
    }
}

impl <Capability> {
    pub fn new(dep: Arc<dyn SomeTrait>) -> Self { Self { dep } }
}
```

### Phase 5 — Agent

```rust
// crates/<domain>/src/agent_<domain>_orchestrator.rs
use shared::<domain>::contract_<domain>_aggregate::I<Domain>Aggregate;
use shared::<domain>::contract_<domain>_protocol::I<Domain>Protocol;

pub struct <Domain>Orchestrator {
    proto: Arc<dyn I<Domain>Protocol>,
}

impl I<Domain>Aggregate for <Domain>Orchestrator {
    fn list_<thing>(&self) -> Vec<<VOName>> {
        self.proto.<feature>(...).unwrap_or_default()
    }
}

impl <Domain>Orchestrator {
    pub fn new(proto: Arc<dyn I<Domain>Protocol>) -> Self { Self { proto } }
}
```

### Phase 6 — Surface

```rust
// crates/<domain>/src/surface_<domain>_command.rs
use shared::<domain>::contract_<domain>_aggregate::I<Domain>Aggregate;

pub fn main(aggregate: &dyn I<Domain>Aggregate, args: &[String]) -> ExitCode {
    // Parse input, call aggregate, render output
    ...
}
```

### Phase 7 — Root

```rust
// crates/<domain>/src/root_<domain>_container.rs
use shared::<domain>::contract_<domain>_aggregate::I<Domain>Aggregate;
use crate::capabilities_<domain>_<role>::<Capability>;
use crate::agent_<domain>_orchestrator::<Domain>Orchestrator;

pub struct <Domain>Container {
    orchestrator: Arc<dyn I<Domain>Aggregate>,
}

impl <Domain>Container {
    pub fn new(dep: Arc<dyn SomeTrait>) -> Self {
        let proto: Arc<dyn I<Domain>Protocol> = Arc::new(<Capability>::new(dep));
        let orch: Arc<dyn I<Domain>Aggregate> = Arc::new(<Domain>Orchestrator::new(proto));
        Self { orchestrator: orch }
    }

    pub fn aggregate(&self) -> Arc<dyn I<Domain>Aggregate> {
        self.orchestrator.clone()
    }
}
```

---

## Section Contract

| Section | Why it belongs here |
| ------- | ------------------- |
| Phase number in heading | Makes the migration plan explicit; you know which layer you are on. |
| Rules as numbered list | Each rule prevents one failure mode; agents scan for the first violated rule. |
| Template per phase | Copy-paste-ready skeleton so the agent never guesses the structure. |
| Section Contract table | Justifies each required section; keeps the HOW-TO self-documenting. |
| Verify block | Machine check (lint) + manual check (reader) with exact commands. |

---

## Verify

```bash
# Phase 0 — baseline
lint-arwaky-cli scan .

# Per phase (replace <dir> with the layer directory being migrated)
lint-arwaky-cli scan crates/shared/src/<domain>  # taxonomy + contract + utility
lint-arwaky-cli scan crates/<domain>/src  # capabilities → agent → surface → root

# Final gate
lint-arwaky-cli scan .   # must be 0
cargo check --workspace
```

A pass means naming, layer imports, primitives, and roles are clean. Migration strategy
(phase order, skip conditions) is **manual** — see Rules §1–2.
