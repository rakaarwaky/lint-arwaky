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

Surface layer = **entry points and UI adapters**. Three types with strict import rules. No business logic. Delegate to aggregates. File: `surface_<domain>_<role>.rs`.

## Three Surface Types (AES406)

| Type | Suffixes | Can Import From | Forbidden |
| --- | --- | --- | --- |
| Smart | `_command`, `_controller`, `_page`, `_entry` | taxonomy, `contract_*_aggregate` | capabilities, concrete agents, concrete smart surfaces |
| Utility | `_hook`, `_store`, `_action`, `_screen` | taxonomy, passive surfaces | smart surfaces, capabilities, agents |
| Passive | `_component`, `_view`, `_layout` | taxonomy only | all other layers, orchestration, business logic |

## Definition of Done

1. Correct surface type suffix.
2. Smart surfaces: delegate to aggregate via `dyn I<Name>Aggregate` (DI via `Arc<dyn Trait>`).
3. Utility surfaces: thin adapter — map events/state to VOs.
4. Passive surfaces: pure rendering from shared VOs — no computation.
5. Zero business logic in any surface type.
6. Zero domain computation.
7. Error handling: never silently discard — return `Result<State, SurfaceError>` or update error state.
8. Shared VOs for all state fields.
9. No imports from capabilities, agents, or concrete implementations.
10. `cargo check -p <crate-name>` passes.

---

## Layer Boundaries

**Smart Surface** — receives user intent, maps events to requests, delegates to aggregate, updates UI state from result.

**Utility Surface** — maps low-level events to shared action/event VOs, holds lightweight UI state, composes passive components.

**Passive Surface** — renders from shared VOs, displays precomputed state. MUST NOT contain business logic, domain computation, or orchestrate aggregates.

---

## Error Handling

Never silently discard errors.

```text
Forbidden: let state = self.runner.run(&request).unwrap_or_default();
Preferred: return Ok(UiState::from_report(report)) / Err(SurfaceError::execution(e))
Preferred: return UiState::error(ErrorMessage::from_err(e))
```

---

## Helper vs Utility

**Keep in surface file** if ANY: accesses `&self`, tightly coupled to this surface, constructor, surface-specific mapping logic, stateless but single-use.

**Extract to taxonomy utility** only if ALL: stateless (no `self`), pure, no side effects, domain-agnostic, reusable.

---

## Templates

| File | Purpose |
| --- | --- |
| `templates/surface_name_command.rs` | Smart surface (command/controller) |
| `templates/surface_name_component.rs` | Passive surface (component/view) |

---

## Workflow

1. **Determine type** — Smart / Utility / Passive? Choose correct suffix.
2. **Smart**: inject `Arc<dyn I<Name>Aggregate>` via DI, delegate, return `Result` VO.
3. **Utility**: map events → VOs, hold minimal state, compose passive surfaces.
4. **Passive**: receive VO, render only — no logic.
5. **Enforce imports** — check forbidden imports per type.
6. **Error handling** — no silent discard.
7. **Verify** → `cargo check -p <crate-name>`.

---

## Verification Checklist

- [ ] Correct surface type suffix.
- [ ] Smart: imports only taxonomy + `contract_*_aggregate`. No capabilities/agents.
- [ ] Utility: imports only taxonomy + passive surfaces. No smart surfaces/capabilities.
- [ ] Passive: imports only taxonomy. No other layers.
- [ ] Smart: delegates to aggregate via `Arc<dyn Trait>`.
- [ ] Zero business logic.
- [ ] Zero domain computation.
- [ ] No silent error discarding.
- [ ] All state fields use shared VOs.
- [ ] `cargo check -p <crate-name>` passes.
