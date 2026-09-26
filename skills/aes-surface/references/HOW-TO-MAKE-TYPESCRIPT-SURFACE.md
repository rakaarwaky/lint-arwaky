# HOW TO MAKE SURFACE TYPESCRIPT

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

- Smart: inject `I<Name>Aggregate` via constructor DI, delegate, return `Result<UiState, SurfaceError>`.
- Utility: map events → VOs, hold minimal UI state, compose passive.
- Passive: render from VOs only — no computation, no orchestration.
- **Never silently discard errors:** forbidden `this.runner.run(r) ?? UiState.idle()`. Use `Ok/Err` or update error state VO.
- All state fields use shared VOs.

### Helper vs Utility

Keep in surface file if ANY: uses `this`, surface-specific mapping, static factory.
Extract to taxonomy utility only if ALL: no `this`, pure, domain-agnostic, reusable.

### Workflow

1. Determine type (Smart/Utility/Passive), choose suffix.
2. Enforce import rules for that type.
3. No silent error discard.
4. `npx tsc --noEmit`.

---

## Template

```typescript
import { <VO> } from '../shared/<domain>/taxonomy_<name>_vo';
import { I<Name>Aggregate } from '../shared/<domain>/contract_<name>_aggregate';

export class Surface<Name> {
    constructor(private readonly aggregate: I<Name>Aggregate) {}

    handle(event: TuiEvent): Result<UiState, SurfaceError> {
        // orchestration only
        return Ok(UiState.idle());
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
| Smart delegates to aggregate via injected interface. | Required by AES layer rules and the linter; missing it is a defect. |
| Zero business logic and computation. | Required by AES layer rules and the linter; missing it is a defect. |
| No silent error discarding. | Required by AES layer rules and the linter; missing it is a defect. |
| All state fields use shared VOs. | Required by AES layer rules and the linter; missing it is a defect. |
| `npx tsc --noEmit` passes. | Required by AES layer rules and the linter; missing it is a defect. |

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Manual (not machine-checked): surface tier suffix; zero business logic/computation; no silent error discard; state fields are VOs.
# Fallback compile gate: npx tsc --noEmit
```
