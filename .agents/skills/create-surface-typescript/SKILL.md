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

Surface = entry points and UI adapters. No business logic. Delegate to aggregates. File: `surface_<domain>_<role>.ts`.

## Three Types (AES406)


| Type    | Suffixes                                     | Imports                          | Forbidden                            |
| --------- | ---------------------------------------------- | ---------------------------------- | -------------------------------------- |
| Smart   | `_command`, `_controller`, `_page`, `_entry` | taxonomy +`contract_*_aggregate` | capabilities, concrete agents        |
| Utility | `_hook`, `_store`, `_action`, `_screen`      | taxonomy + passive surfaces      | smart surfaces, capabilities, agents |
| Passive | `_component`, `_view`, `_layout`             | taxonomy only                    | all other layers                     |

## Rules

- Smart: inject `I<Name>Aggregate` via constructor DI, delegate, return `Result<UiState, SurfaceError>`.
- Utility: map events → VOs, hold minimal UI state, compose passive.
- Passive: render from VOs only — no computation, no orchestration.
- **Never silently discard errors:** forbidden `this.runner.run(r) ?? UiState.idle()`. Use `Ok/Err` or update error state VO.
- All state fields use shared VOs.

## Helper vs Utility

Keep in surface file if ANY: uses `this`, surface-specific mapping, static factory.
Extract to taxonomy utility only if ALL: no `this`, pure, domain-agnostic, reusable.

## Templates

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

## Workflow

1. Determine type (Smart/Utility/Passive), choose suffix.
2. Enforce import rules for that type.
3. No silent error discard.
4. `npx tsc --noEmit`.

## Checklist

- [ ]  Correct suffix for surface type.
- [ ]  Smart: only taxonomy + `contract_*_aggregate` imports.
- [ ]  Utility: only taxonomy + passive surface imports.
- [ ]  Passive: only taxonomy imports.
- [ ]  Smart delegates to aggregate via injected interface.
- [ ]  Zero business logic and computation.
- [ ]  No silent error discarding.
- [ ]  All state fields use shared VOs.
- [ ]  `npx tsc --noEmit` passes.
