# HOW TO MAKE MIGRATION PYTHON

> **Purpose**: Guide phased migration of legacy Python projects into AES layered architecture — taxonomy → contract → utility → capabilities → agent → surface → root.
>
> **Audience**: Agents and engineers executing a migration to AES.
>
> **Scope**: Phase-based migration workflow for Python projects; references layer skills for execution.
>
> **Location**: Project root; each phase operates on a layer directory.
>
> **Length**: 9 phases (0–8); total duration depends on violation count.

---

## Rules

Nine rules. Each one governs one migration phase.

1. **Phase 0 first — audit before touching anything.** Run `lint-arwaky-cli scan .`; record baseline violations to choose strategy.
2. **Taxonomy before contract before capabilities.** VOs must exist before protocols can reference them.
3. **Protocol = one file per feature, one trait per capability seam, each trait rich.** One trait declares every method its capability owns, each with a concrete return type. Never a mega `execute(op, …)` that dispatches multiple features behind one name.
4. **Aggregate = exactly one method.** The single `execute(request)` entry point; the response is a taxonomy-defined enum of VOs. New consumer verbs are new variants on the request enum, not new aggregate methods.
5. **Utility is stateless free functions only.** No class, no state, no upward imports (AES404).
6. **Capabilities implement protocol; agent implements aggregate.** No cross-layer imports (AES201).
7. **Surface calls aggregate; never imports agent or capabilities.** Dependency arrow points down (AES201 purpose).
8. **Root wires everything; never contains business logic.** Container constructs; entry bootstraps.
9. **Verify at every phase.** `lint-arwaky-cli scan <layer-dir>` → 0 before moving to next phase.

## Workflow

1. **Phase 0 — Audit** — run `lint-arwaky-cli scan .`; record baseline.
2. **Phase 1 — Taxonomy** — extract VOs, errors, constants.
3. **Phase 2 — Contract** — create protocol (one file per feature, one trait per capability seam) + aggregate (exactly one `execute()` method).
4. **Phase 3 — Utility** — extract stateless helpers to shared.
5. **Phase 4 — Capabilities** — implement protocols with business logic.
6. **Phase 5 — Agent** — implement aggregate, delegate to capabilities.
7. **Phase 6 — Surface** — map I/O, call aggregate.
8. **Phase 7 — Root** — wire containers, bootstrap entry.
9. **Phase 8 — Verify** — full scan → 0 violations; compile clean.

---

## Template

Copy, fill, delete nothing. Migration proceeds phase-by-phase; each phase outputs files matching the layer HOW-TO.

### Phase 1 — Taxonomy

```python
# modules/shared/src/taxonomy_<domain>_vo.py
from __future__ import annotations
from dataclasses import dataclass

@dataclass(frozen=True)
class <VOName>:
    """Domain value object — wraps a primitive to prevent leakage."""
    value: str  # or int, bool, etc. as appropriate

# Register in modules/shared/src/__init__.py
```

### Phase 2 — Contract

```python
# modules/shared/src/contract_<domain>_protocol.py
from abc import ABC, abstractmethod
from modules.shared.src.taxonomy_<domain>_request import RequestVO
from modules.shared.src.taxonomy_<domain>_response import ResponseVO
from modules.shared.src.taxonomy_<domain>_vo import <VO>

# One file per feature, one ABC per capability seam, each ABC rich.
class I<Seam1>Protocol(ABC):
    """<What this seam owns.>"""

    @abstractmethod
    def <operation_1>(self, arg: <VO>) -> <ResultVO>:
        ...

    @abstractmethod
    def <operation_2>(self, arg: <VO>) -> <ResultVO>:
        ...

class I<Seam2>Protocol(ABC):
    """<What this seam owns.>"""

    @abstractmethod
    def <operation_1>(self, arg: <VO>) -> <ResultVO>:
        ...

# modules/shared/src/contract_<domain>_aggregate.py
class I<Domain>Aggregate(ABC):
    """The single entry point — exactly one method."""

    @abstractmethod
    def execute(self, request: RequestVO) -> ResponseVO:
        ...
```

### Phase 3 — Utility

```python
# modules/shared/src/utility_<domain>_<role>.py
from modules.shared.src.taxonomy_<domain>_vo import <VO>

def <helper>(arg: <VO>) -> <VO>:
    """Stateless helper — no state, no class, no upward imports."""
    ...
```

### Phase 4 — Capabilities

```python
# modules/<domain>/src/capabilities_<domain>_<role>.py
from modules.shared.src.contract_<domain>_protocol import I<Domain>Protocol
from modules.shared.src.taxonomy_<domain>_vo import <VO>

class <Capability>(I<Seam1>Protocol):
    """Block 1: Module DeclarationStruct with DI deps typed against a contract protocol."""
    def __init__(self, dep: I<SeamDependency>Protocol) -> None:
        self._dep = dep

    """Block 2: Module ContractProtocol implementation — every declared method, no stubs."""
    def <operation_1>(self, arg: <VO>) -> <ResultVO>:
        # Business logic here
        ...

    def <operation_2>(self, arg: <VO>) -> <ResultVO>:
        ...

    """Block 3: Module HelpersFactory."""
    @classmethod
    def create(cls, dep: I<SeamDependency>Protocol) -> I<Seam1>Protocol:
        return cls(dep)
```

### Phase 5 — Agent

```python
# modules/<domain>/src/agent_<domain>_orchestrator.py
from modules.shared.src.contract_<domain>_aggregate import I<Domain>Aggregate
from modules.shared.src.contract_<domain>_protocol import I<Domain>Protocol

class <Domain>Orchestrator(I<Domain>Aggregate):
    """Block 1: Module DeclarationStruct with protocol deps."""
    def __init__(self, protocol: I<Domain>Protocol) -> None:
        self._protocol = protocol

    """Block 2: Module ContractAggregate implementation — one method, dispatch to protocols."""
    def execute(self, request: RequestVO) -> ResponseVO:
        # Match the request, delegate to the owning protocol trait. Never compute here.
        if isinstance(request, RequestVO.<Variant1>):
            return ResponseVO.<Result1>(self._protocol.<operation_1>(..))
        return ResponseVO.<Result2>(self._protocol.<operation_2>(..))
```

### Phase 6 — Surface

```python
# modules/<domain>/src/surface_<domain>_command.py
from modules.shared.src.contract_<domain>_aggregate import I<Domain>Aggregate

def main(aggregate: I<Domain>Aggregate, argv: list[str]) -> int:
    """Parse input, call aggregate, render output — no business logic."""
    ...
```

### Phase 7 — Root

```python
# modules/<domain>/src/root_<domain>_container.py
from modules.<domain>.src.capabilities_<domain>_<role> import <Capability>
from modules.<domain>.src.agent_<domain>_orchestrator import <Domain>Orchestrator
from modules.shared.src.contract_<domain>_aggregate import I<Domain>Aggregate

class <Domain>Container:
    def __init__(self, dep: I<SeamDependency>Protocol) -> None:
        proto = <Capability>.create(dep)
        self._orchestrator = <Domain>Orchestrator(proto)

    @property
    def aggregate(self) -> I<Domain>Aggregate:
        return self._orchestrator
```

---

## Section Contract

| Section | Why it belongs here|
| ------- | -------------------|
| Phase number in heading | Makes the migration plan explicit; you know which layer you are on.|
| Rules as numbered list | Each rule prevents one failure mode; agents scan for the first violated rule.|
| Template per phase | Copy-paste-ready skeleton so the agent never guesses the structure.|
| Section Contract table | Justifies each required section; keeps the HOW-TO self-documenting.|
| Verify block | Machine check (lint) + manual check (reader) with exact commands.|

---

## Verify

```bash
# Phase 0 — baseline
lint-arwaky-cli scan .

# Per phase (replace <dir> with the layer directory being migrated)
lint-arwaky-cli scan modules/shared/src  # taxonomy + contract + utility
lint-arwaky-cli scan modules/<domain>/src  # capabilities → agent → surface → root

# Final gate
lint-arwaky-cli scan .   # must be 0
python -m compileall -q modules/
```

A pass means naming, layer imports, primitives, and roles are clean. Migration strategy
(phase order, skip conditions) is **manual** — see Rules §1–2.

---

## Python Lint (Verify Pipeline)

> Shared commands, MCP tools, exit codes, and AES101 naming live in `references/lint-rules.md` (Lint Rules — Global).
> This file only holds Python deltas: adapters, scan session, Python-specific fix routing, and role boundaries.

### Adapters & external tooling

`lint-arwaky-cli install` installs external adapters; `lint-arwaky-cli adapters` lists the active ones (Ruff, Mypy, Bandit, …). `get_config` accepts `"language": "python"`.

| Adapter | Standalone command | What it covers |
| :----- | :----- | :----- |
| Ruff | `ruff check src/` | lint + style gate |
| Mypy | `mypy src/` | static type checking |
| Bandit | `lint-arwaky-cli security modules/` | security issues |
| CVE check | `lint-arwaky-cli dependencies modules/` | library vulnerabilities |

`lint-arwaky-cli external modules/` runs only the external linters (ruff).

### Scan session

```bash
# Pre-flight
python3 -c "import sys; print(sys.version)"
python3 -c "import ruff; print('ruff ok')" 2>/dev/null || echo "ruff not installed"

# Scan (text / json / by rule / by member)
lint-arwaky-cli scan workspaces-bad/modules
lint-arwaky-cli scan workspaces-bad/modules --format json
lint-arwaky-cli scan workspaces-bad/modules --filter AES201
lint-arwaky-cli orphan modules/ --member animator
lint-arwaky-cli ci modules/ --threshold 80
lint-arwaky-cli scan modules/ --format junit -o reports/

# Reports → XDG data dir
lint-arwaky-cli scan modules/ --format json \
  --output-dir ~/.local/share/aes-lint-arwaky/reports
lint-arwaky-cli scan modules/ --format sarif \
  > ~/.local/share/aes-lint-arwaky/reports/scan_python.sarif
```

### Python-specific fix routing

- **AES101 rename** → after `git mv`, update `__init__.py` imports (barrel is the registration point; missing re-export = orphan AES501–506).
- **AES201 forbidden import** → for `capabilities_*`, allowed: `taxonomy`, `contract`, `utility` (mandatory: `taxonomy` + `contract(*_protocol)`); forbidden: other `capabilities_*`, `agent`, `surface`, `root`. Break the dependency through a contract protocol/aggregate — `aes-contract`.
- **AES204** → remove the dummy import and its stub (`aes-{layer}`).
- **AES304** → auto-fixable via `lint-arwaky-cli fix <path> --filter AES304`, or fix the root cause and delete the suppression manually.
- **AES403** → every capability class MUST inherit its protocol ABC.
- **AES405** → every agent class MUST inherit its aggregate ABC, no direct `capabilities_*` import, no `: Any` on aggregate signatures.
- **AES404** → `utility_*` files: stateless functions only; import `taxonomy` and nothing else.
- **AES303** → add the missing `class` / `def` (skipped for `utility`/`surface` layers).

### Role boundaries

| Layer | Can contain | Cannot contain |
| :----- | :----- | :----- |
| capabilities | Pure computation, validation | I/O, network, database |
| utility | Stateless pure functions | State, classes with fields, non-taxonomy imports |
| agent | Orchestration flow | Computation, I/O, business |

### Quick fix recipes

```bash
# AES101/102 rename
git mv src/capabilities_scanner.py src/capabilities_file_scanner.py
# then update __init__.py imports

# AES203 unused import
lint-arwaky-cli fix src/file.py --filter AES203

# AES304 bypass
grep -rn 'noqa\|type: ignore\|pragma: no cover\|FIXME\|TODO' src/
# fix root cause, delete suppression

# AES501–506 orphan
lint-arwaky-cli orphan modules/ --format json
# truly dead → delete; should be wired → container / import chain

# AES401–406 role
lint-arwaky-cli role modules/ --filter AES403
# I/O out of capabilities → utility; business out of surface → capabilities;
# orchestration out of capabilities → agent
```

### Common issues

| Issue | Fix strategy |
| :----- | :----- |
| Cross-layer imports | Contract protocols via DI (`aes-contract`) |
| Missing protocol inheritance | Create protocol ABC and inherit (`aes-contract` + `aes-capabilities`) |
| Mixed layer responsibilities | Move code to the correct layer (`aes-*`) |
| Magic constants | Extract to taxonomy constants (`aes-taxonomy`) |
| Surface importing capabilities | Use aggregate contracts instead (`aes-contract`) |

---

## Python Section Contract

| Section | Why it belongs here |
| :----- | :----- |
| Pre-flight | A scan on a broken interpreter is noise, not signal. |
| Scan with path + format | Findings must be machine-triageable (json) or human-scannable (text). |
| Triage order | CRITICAL (AES201/205/304) blocks merge; fix before HIGH/MEDIUM cosmetics. |
| Fix routing by AES code | Wrong fix layer recreates the violation; route via lint-routing in `lint-rules.md` (`## AES fix routing`). |
| Barrel update after rename | AES101 without `__init__.py` re-export becomes AES501–506. |
| Role-boundary check | Capabilities/utility/agent rows above must hold after fixes. |
| Re-scan to 0 | Done means zero AES findings, not "mostly clean". |
| Language verify | `py_compile` / import / ruff / mypy still pass — AES clean ≠ code healthy. |

---

## Python Verify

```bash
# 1. Re-scan — must be 0 AES violations
lint-arwaky-cli scan <target-path>

# 2. Syntax
python3 -m py_compile src/<module>.py

# 3. Import
python3 -c "import <module>"

# 4. Ruff (if available)
ruff check src/

# 5. Mypy (if available)
mypy src/
```

Exit codes (shared, `lint-rules.md` (`## Exit codes`)): `0` pass · `1` violations · `2` config/parse error.

Manual:

- [ ] Layer imports follow AES201; `python3 -c "import <module>"` succeeds.
- [ ] Capability classes inherit their protocol ABC (AES403).
- [ ] Utility files are stateless and import `taxonomy` only (AES404).
- [ ] Agent classes inherit their aggregate ABC; no `Any` on aggregate signatures (AES405).
- [ ] Surface files follow role-based imports (AES406).
- [ ] `ruff check src/` and `mypy src/` clean.
- [ ] `lint-arwaky-cli scan modules/` reports 0 violations.
