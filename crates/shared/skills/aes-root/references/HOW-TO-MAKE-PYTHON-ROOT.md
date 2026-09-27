# HOW TO MAKE ROOT PYTHON

> **Purpose**: Compose the system: containers wire capabilities to contracts; entries bootstrap and start the process.
>
> **Audience**: Agents and engineers scaffolding AES composition roots.
>
> **Scope**: Python, Rust, and TypeScript `root_<concept>_<container|entry>` files (plus documented barrel/entry exceptions).
>
> **Location**: Feature or app package top; the only layer that may construct implementations.
>
> **Length**: One container per feature; one (or few) entry files; no business logic, no orchestration policy.

---

## Rules

1. **Suffix is strictly `_container` or `_entry`.** File: `root_<concept>_<suffix>.py`.
   Documented exceptions: `main.py`, `__init__.py`, barrel/entry files (AES101/AES102).
2. **Container = wire one feature only.** Instantiate capabilities, bind them to
   contract protocol/aggregate types, expose the aggregate. Never business logic.
3. **Entry = bootstrap the app.** Parse argv (if needed), compose feature container
   factories, start the surface/CLI loop. Never construct capabilities directly.
4. **Allowed imports: everything below root** — agent, capabilities, surface, contract,
   taxonomy, utility. Nothing below root may import root (AES201–AES205).
5. **No business logic, no orchestration policy, no technical parsing, no UI behaviour** —
   those live in capabilities / agent / utility / surface.
6. **Signatures use shared VOs** where domain values appear (AES402). `bool` for
   semantic toggles only.
7. **Register** in package `__init__.py` when the container/entry is public.

### Workflow

1. **Determine role** — Container (wire one feature) or Entry (bootstrap all)?
2. **Create file** → `root_<concept>_<suffix>.py`.
3. **Wire deps** → instantiate capabilities; bind to contract interfaces/aggregates.
4. **Verify** → `lint-arwaky-cli scan <layer-path>` then `python -c "import <module>"`.

---

## Template

Copy, fill, delete nothing. File: `root_<concept>_<suffix>.py` where
`<suffix>` is `_container` or `_entry`.

### Container — wire one feature

```python
"""<Concept> composition root — wires <N> capabilities into the orchestrator."""
from __future__ import annotations

from modules.<feature>.src.agent_<concept>_orchestrator import <Concept>Orchestrator
from modules.<feature>.src.capabilities_<concept>_<role> import <Capability>
from modules.shared.src.contract_<concept>_aggregate import I<Concept>Aggregate
from modules.shared.src.contract_<concept>_protocol import I<Concept>Protocol


class <Concept>Container:
    """Construct the capabilities and the orchestrator (wiring only)."""

    def __init__(self, /* shared deps */) -> None:
        runners: list[I<Concept>Protocol] = [
            <Capability>(/* deps */),
        ]
        self._orchestrator = <Concept>Orchestrator(runners)

    @property
    def aggregate(self) -> I<Concept>Aggregate:
        return self._orchestrator


def create_<concept>_feature(/* shared deps */) -> I<Concept>Aggregate:
    """Fully-wired <concept> feature aggregate (factory for entry / DI)."""
    return <Concept>Container(/* deps */).aggregate
```

**Container rules:** import capabilities + agent + contract types only; expose
the aggregate (contract), never a concrete capability; no business/orchestration
logic — construction and binding only.

### Entry — bootstrap the application

```python
"""CLI entry — parse argv, compose feature containers, start the surface."""
from __future__ import annotations

import sys

from modules.<feature>.src.root_<concept>_container import create_<concept>_feature
from modules.<feature>.src.surface_<concept>_command import main as surface_main


def main(argv: list[str] | None = None) -> int:
    argv = sys.argv[1:] if argv is None else argv
    aggregate = create_<concept>_feature()
    return int(surface_main(aggregate, argv))


if __name__ == "__main__":
    raise SystemExit(main())
```

**Entry rules:** may parse argv (bootstrap only) and compose containers; start
the surface/CLI loop; never import concrete capabilities directly — go through
a container factory; no business logic.

---

## Section Contract

| Section | Why it belongs here |
| ------- | ------------------- |
| Module docstring | Names the role: composition container or app entry. |
| Correct filename + suffix | AES101/AES102 resolve the layer and role from the name. |
| Layer-legal imports | Keeps the dependency arrow pointed down (AES201–AES205). |
| Contract types in signatures | Expose aggregate/protocol, not concretions (AES402). |
| Container: construct + bind only | Wiring is the whole job; logic belongs below. |
| Entry: compose containers + start | Bootstrap only; never new capabilities directly. |
| Registered in package barrel | Dead code otherwise; composition needs the export. |

---

## Anti-Patterns

- **Business / orchestration / parsing / UI logic in root** — move down a layer.
- **Entry importing `capabilities_*` directly** — go through a container factory.
- **Container exposing a concrete capability type** — return the aggregate contract.
- **Wrong suffix or undocumented name** — only `_container` / `_entry` (or listed exceptions).

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Manual (not machine-checked): correct role suffix; no business/orchestration/parsing/UI logic; nothing below root imports root.
# Fallback compile gate: python -c "import <shared_package>.<module>"
```
