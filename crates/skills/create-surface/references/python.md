# Surface — Python

File: `surface_<domain>_<role>.py`. See `../SKILL.md` for the surface-type/import table.

## Rules (Python)

- Smart: inject `I<Name>Aggregate` via DI, delegate, return Result VO.
- Utility: map events → VOs, hold minimal UI state, compose passive.
- Passive: render from VOs only — no computation, no orchestration.
- **Never silently discard errors:** forbidden `result = self.runner.run(r) or None`. Use `Result.ok/err` or update error state VO.
- All state fields use shared VOs.

## Helper vs Utility

Keep in surface file if ANY: uses `self`, surface-specific mapping, factory.
Extract to taxonomy utility only if ALL: no `self`, pure, domain-agnostic, reusable.

## Templates

```python
from shared.<domain>.taxonomy_<name>_vo import <VO>
from shared.<domain>.contract_<name>_aggregate import I<Name>Aggregate

class Surface<Name>:
    def __init__(self, aggregate: I<Name>Aggregate):
        self._aggregate = aggregate

    def handle(self, event: TuiEvent) -> Result[UiState, SurfaceError]:
        # orchestration only
        return Ok(UiState.idle())
```

## Workflow

1. Determine type (Smart/Utility/Passive), choose suffix.
2. Enforce import rules for that type.
3. No silent error discard.
4. `python -c "import <module>"`.

## Checklist

- [ ]  Correct suffix for surface type.
- [ ]  Smart: only taxonomy + `contract_*_aggregate` imports.
- [ ]  Utility: only taxonomy + passive surface imports.
- [ ]  Passive: only taxonomy imports.
- [ ]  Smart delegates to aggregate via injected interface.
- [ ]  Zero business logic and computation.
- [ ]  No silent error discarding.
- [ ]  All state fields use shared VOs.
- [ ]  `python -c "import <module>"` passes.
