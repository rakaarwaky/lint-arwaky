---
name: create-surface-typescript
description: "Create and validate TypeScript surface layer files following AES406: smart/utility/passive surfaces, strict import rules, delegate to aggregates, zero direct lower-layer imports, zero business logic, VO-based state, and explicit error handling."
metadata:
  tags: [typescript, aes, surface, smart, utility, passive, di, vo]
  triggers:
    - "create surface typescript"
    - "add surface typescript"
    - "fix surface structure typescript"
    - "create command typescript"
    - "create controller typescript"
    - "check surface typescript"
    - "audit surface typescript"
  dependencies: []
  related:
    - create-agent-typescript
    - create-taxonomy-typescript
    - create-contract-typescript
---

# create-surface-typescript

Surface layer = **entry points and UI adapters**. Three types with strict import rules. No business logic. Delegate to aggregates. File: `surface_<domain>_<role>.ts`.

## Three Surface Types (AES406)

| Type | Suffixes | Can Import From | Forbidden |
| --- | --- | --- | --- |
| Smart | `_command`, `_controller`, `_page`, `_entry` | taxonomy, `contract_*_aggregate` | capabilities, concrete agents, concrete smart surfaces |
| Utility | `_hook`, `_store`, `_action`, `_screen` | taxonomy, passive surfaces | smart surfaces, capabilities, agents |
| Passive | `_component`, `_view`, `_layout` | taxonomy only | all other layers, orchestration, business logic |

## Definition of Done

1. Correct surface type suffix.
2. Smart surfaces: delegate to aggregate via `I<Name>Aggregate` interface (DI).
3. Utility surfaces: thin adapter — map events/state to VOs.
4. Passive surfaces: pure rendering from shared VOs — no computation.
5. Zero business logic in any surface type.
6. Zero domain computation.
7. Error handling: never silently discard — return `Result<UiState, SurfaceError>` or update error state.
8. Shared VOs for all state fields.
9. No imports from capabilities, agents, or concrete implementations.
10. `npx tsc --noEmit` passes.

---

## Layer Boundaries

**Smart Surface** — receives user intent, maps events to requests, delegates to aggregate, updates UI state from result.

**Utility Surface** — maps low-level events to shared action/event VOs, holds lightweight UI state, composes passive components.

**Passive Surface** — renders from shared VOs, displays precomputed state. MUST NOT contain business logic, domain computation, or orchestrate aggregates.

---

## Error Handling

Never silently discard errors.

```text
Forbidden: const state = this.runner.run(request) ?? UiState.idle();
Preferred: return Ok(UiState.fromReport(report)) / Err(SurfaceError.execution(e))
Preferred: return UiState.error(ErrorMessage.fromErr(err))
```

---

## Helper vs Utility

**Keep in surface file** if ANY: accesses `this`, tightly coupled to this surface, static factory, surface-specific mapping logic, stateless but single-use.

**Extract to taxonomy utility** only if ALL: stateless (no `this`), pure, no side effects, domain-agnostic, reusable.

---

## Templates

| File | Purpose |
| --- | --- |
| `templates/surface_name_command.ts` | Smart surface (command/controller) |
| `templates/surface_name_component.ts` | Passive surface (component/view) |

---

## Workflow

1. **Determine type** — Smart / Utility / Passive? Choose correct suffix.
2. **Smart**: inject `I<Name>Aggregate` via constructor DI, delegate, return Result VO.
3. **Utility**: map events → VOs, hold minimal state, compose passive surfaces.
4. **Passive**: receive VO, render only — no logic.
5. **Enforce imports** — check forbidden imports per type.
6. **Error handling** — no silent discard.
7. **Verify** → `npx tsc --noEmit`.

---

## Verification Checklist

- [ ] Correct surface type suffix.
- [ ] Smart: imports only taxonomy + `contract_*_aggregate`. No capabilities/agents.
- [ ] Utility: imports only taxonomy + passive surfaces. No smart surfaces/capabilities.
- [ ] Passive: imports only taxonomy. No other layers.
- [ ] Smart: delegates to aggregate via injected interface.
- [ ] Zero business logic.
- [ ] Zero domain computation.
- [ ] No silent error discarding.
- [ ] All state fields use shared VOs.
- [ ] `npx tsc --noEmit` passes.
