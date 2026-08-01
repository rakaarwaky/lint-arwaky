---
name: create-surface-rust
description: "Create and validate Rust surface layer files following AES406: smart/utility/passive surfaces, strict import rules, delegate to aggregates, zero direct lower-layer imports, zero business logic, VO-based state, and explicit error handling."
metadata:
  tags: [rust, aes, surface, smart, utility, passive, di, vo]
  triggers:
    - "create surface rust"
    - "add surface rust"
    - "fix surface structure rust"
    - "create command rust"
    - "create controller rust"
    - "check surface rust"
    - "audit surface rust"
  dependencies: []
  related:
    - create-agent-rust
    - create-taxonomy-rust
    - create-contract-rust
---
# create-surface-rust

Surface = entry points and UI adapters. No business logic. Delegate to aggregates. File: `surface_<domain>_<role>.rs`.

## Three Types (AES406)


| Type    | Suffixes                                     | Imports                          | Forbidden                            |
| --------- | ---------------------------------------------- | ---------------------------------- | -------------------------------------- |
| Smart   | `_command`, `_controller`, `_page`, `_entry` | taxonomy +`contract_*_aggregate` | capabilities, concrete agents        |
| Utility | `_hook`, `_store`, `_action`, `_screen`      | taxonomy + passive surfaces      | smart surfaces, capabilities, agents |
| Passive | `_component`, `_view`, `_layout`             | taxonomy only                    | all other layers                     |

## Rules

- Smart: inject `Arc<dyn I<Name>Aggregate>` via DI, delegate, return `Result<State, SurfaceError>`.
- Utility: map events → VOs, hold minimal UI state, compose passive.
- Passive: render from VOs only — no computation, no orchestration.
- **Never silently discard errors:** forbidden `self.runner.run(&r).unwrap_or_default()`. Use `Ok/Err` or update error state VO.
- All state fields use shared VOs.

## Helper vs Utility

Keep in surface file if ANY: uses `&self`, surface-specific mapping, constructor.
Extract to taxonomy utility only if ALL: no `self`, pure, domain-agnostic, reusable.

## Templates

```rust
use std::sync::Arc;

use shared::<domain>::taxonomy_<name>_vo::<VO>;
use shared::<domain>::contract_<name>_aggregate::I<Name>Aggregate;

pub struct Surface<Name> {
    aggregate: Arc<dyn I<Name>Aggregate>,
}

impl Surface<Name> {
    pub fn new(aggregate: Arc<dyn I<Name>Aggregate>) -> Self {
        Self { aggregate }
    }

    pub fn handle(&self, event: &TuiEvent) -> Result<UiState, SurfaceError> {
        // orchestration only
        Ok(UiState::idle())
    }
}
```

## Workflow

1. Determine type (Smart/Utility/Passive), choose suffix.
2. Enforce import rules for that type.
3. No silent error discard.
4. `cargo check -p <crate-name>`.

## Checklist

- [ ]  Correct suffix for surface type.
- [ ]  Smart: only taxonomy + `contract_*_aggregate` imports.
- [ ]  Utility: only taxonomy + passive surface imports.
- [ ]  Passive: only taxonomy imports.
- [ ]  Smart delegates via `Arc<dyn Trait>`.
- [ ]  Zero business logic and computation.
- [ ]  No silent error discarding.
- [ ]  All state fields use shared VOs.
- [ ]  `cargo check -p <crate-name>` passes.
