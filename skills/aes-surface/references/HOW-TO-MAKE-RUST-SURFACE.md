# HOW TO MAKE SURFACE RUST

> **Purpose**: Drive the system from outside: map input events to VOs, call the aggregate, render results — no business logic.
>
> **Audience**: Agents and engineers scaffolding AES surface commands/controllers/components.
>
> **Scope**: Python, Rust, and TypeScript `surface_<domain>_<role>` files — Smart / Utility / Passive tiers with strict suffixes.
>
> **Location**: Feature domain package; Smart imports taxonomy + `_aggregate` + utility; Utility/Passive taxonomy-only.
>
> **Length**: One file per surface role; no `_entry` (that is root).

---

## Rules

- Smart: inject `Arc<dyn I<Name>Aggregate>` via DI, delegate, return `Result<State, SurfaceError>`.
- Utility: map events → VOs, hold minimal UI state, compose passive.
- Passive: render from VOs only — no computation, no orchestration.
- **Never silently discard errors:** forbidden `self.runner.run(&r).unwrap_or_default()`. Use `Ok/Err` or update error state VO.
- All state fields use shared VOs.

### Helper vs Utility

Keep in surface file if ANY: uses `&self`, surface-specific mapping, constructor.
Extract to taxonomy utility only if ALL: no `self`, pure, domain-agnostic, reusable.

### Workflow

1. Determine type (Smart/Utility/Passive), choose suffix.
2. Enforce import rules for that type.
3. No silent error discard.
4. `cargo check -p <crate-name>`.

---

## Template

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

---

## Section Contract

| Check | Why it belongs here |
| ----- | ------------------- |
| Correct suffix for surface type. | Required by AES layer rules and the linter; missing it is a defect. |
| Smart: only taxonomy + `contract_*_aggregate` imports. | Required by AES layer rules and the linter; missing it is a defect. |
| Utility: only taxonomy + passive surface imports. | Required by AES layer rules and the linter; missing it is a defect. |
| Passive: only taxonomy imports. | Required by AES layer rules and the linter; missing it is a defect. |
| Smart delegates via `Arc<dyn Trait>`. | Required by AES layer rules and the linter; missing it is a defect. |
| Zero business logic and computation. | Required by AES layer rules and the linter; missing it is a defect. |
| No silent error discarding. | Required by AES layer rules and the linter; missing it is a defect. |
| All state fields use shared VOs. | Required by AES layer rules and the linter; missing it is a defect. |
| `cargo check -p <crate-name>` passes. | Required by AES layer rules and the linter; missing it is a defect. |

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Manual (not machine-checked): surface tier suffix; zero business logic/computation; no silent error discard; state fields are VOs.
# Fallback compile gate: cargo check -p <crate-name>
```
