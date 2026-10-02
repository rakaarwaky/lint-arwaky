# HOW TO MAKE CAPABILITIES PYTHON

> **Purpose**: Implement concrete protocol behaviour: domain rules and external adaptation as protocol implementations.
>
> **Audience**: Agents and engineers scaffolding AES capability files.
>
> **Scope**: Python, Rust, and TypeScript `capabilities_<domain>_<role>` files — 3-block structure, ≥1 protocol implementor, ≤3 types.
>
> **Location**: Feature domain package; depends only on taxonomy, `_protocol` contracts, and utility.
>
> **Length**: At most 3 types per file; Block 1 → 2 → 3 order.

---

## Rules

### Capability vs Utility Boundary

A capability file **implements a protocol** — it has state (via DI), business rules, and concrete behaviour.
A utility file **performs a pure operation** — no state, no business rules, just a narrow task.

| Decision | → Capability | → Utility |
|---|---|---|
| Has `self` / instance state / injected dependencies | Yes | No |
| Contains business rules or protocol logic | Yes | No |
| Stateless, domain-agnostic, ≥2 consumers | — | Yes |
| Implements `_protocol` interface/ABC/trait | Yes | No |
| Pure free function with no business logic | No | Yes |

**Rule**: If a method or helper in Block 3 is stateless, domain-agnostic, and reusable across modules — extract it to a `utility_*` file. It does not belong in the capability layer.

### Import rules

**Allowed imports:** Taxonomy, Contract (`_protocol` only), Utility.
**Forbidden:** `agent_*`, other `capabilities_*`, `surface_*`, local domain models, magic constants.

### Structure rules (Python)

- Rule 1: Internal helper classes without ABC → ALLOWED.
- Rule 2: ≥1 class inherits a protocol ABC.
- Rule 3: Total class count ≤ 3.

### 3-Block Structure

```text
# Block 1: Class Definition & Constructor
# Block 2: Protocol Method Implementation
# Block 3: Dunder Methods, Factories, Helpers
```

Method placement: `@abstractmethod` → Block 2. Dunder/factory/private → Block 3. Stateless free function → extract to `*utility_.py`.

### Helper vs Utility

Keep in Block 3 if ANY: uses `self`, domain-specific, single consumer, factory.
Extract to utility only if ALL: no `self`, pure, no side effects, domain-agnostic, ≥2 consumers.
I/O: stateless + I/O + domain-agnostic = utility OK.

### Workflow

1. Confirm implements protocol behavior (not orchestration/data/mechanics).
2. File imports from `_protocol` module — if missing → flag `CapabilityNoProtocol`.
3. Create `contract_<name>_protocol.py` if missing.
4. Enforce 3-Block with explicit `# ─── Block 1:`, `Block 2:`, `Block 3:` comments — AES403 `CapabilityBlockMarkers` reports a file whose banners are missing, out of order, or above 3.
5. AES403: ≥1 protocol inheritor, ≤3 classes, DI via protocols, shared VOs.
6. No forbidden imports, no inter-capability deps, no local domain models.
7. `python -c "import <module>"`.

---

## Template

### 3-block implementation

```python
from shared.<domain>.taxonomy_<name>_vo import <VO>
from shared.<domain>.contract_<name>_protocol import I<Name>Protocol

# ─── Block 1: Class Definition & Constructor ──────────────
class Capabilities<Name>(I<Name>Protocol):
    def __init__(self, /* DI params */) -> None:
        # DI fields use protocol interfaces
        # Value fields use shared VOs
        ...

    # ─── Block 2: Protocol Method Implementation ──────────────
    def method_name(self, param: <VO>) -> None:
        # domain behavior
        ...

    # ─── Block 3: Dunder Methods, Factories & Helpers ─────────
    def __repr__(self) -> str:
        return "Capabilities<Name>()"

    @classmethod
    def create_default(cls) -> "Capabilities<Name>":
        return cls()
```

### Protocol ABC

```python
from abc import ABC, abstractmethod
from shared.<domain>.taxonomy_<name>_vo import <VO>

class I<Name>Protocol(ABC):
    @abstractmethod
    def method_name(self, param: <VO>) -> None: ...
```

---

## Section Contract

The `Enforced` column says which rule reports a violation, so you know what the
linter will catch and what stays a review responsibility. `Manual` means no rule
checks it — the shape is still the contract, but a violation is caught by review
or by the interpreter, not by `scan`.

| Check                                                             | Enforced                                                          |
| ----------------------------------------------------------------- | ----------------------------------------------------------------- |
| All three `# ─── Block 1:` / `Block 2:` / `Block 3:` banners present, in order, none above 3. | **AES403** `CapabilityBlockMarkers` (MEDIUM)   |
| Block 1 (class) precedes Block 2 (protocol methods).              | **AES403** `CapabilityBlockOrder` (HIGH)                          |
| ≥1 class inherits a protocol ABC.                                 | **AES403** `CapabilityNoImplementor` (MEDIUM)                     |
| Exactly 1 protocol ABC per file.                                  | **AES403** `CapabilityMultiProtocol` (MEDIUM)                     |
| ≤3 total classes.                                                 | **AES403** `CapabilityTooManyTypes` (HIGH)                        |
| Imports from `_protocol` module only.                             | **AES201**–**AES205** (import rules)                              |
| No agent / surface / root imports.                                | **AES201** `FORBIDDEN_IMPORT`                                     |
| Block 2: ONLY protocol ABC method implementations.                | Manual — no rule reads Block 2's contents                        |
| DI via protocol interfaces; shared VOs for fields and signatures. | Manual                                                            |
| Constants → `taxonomy_<domain>_constant.py`.                     | **AES403** `CapabilityLocalConstant` (MEDIUM)                     |
| Block 3 public helpers with no production caller are private (`def _name`). | **AES403** `CapabilityPublicHelper` (MEDIUM)       |
| Test classes (`class Test*`, `def test_*`) live in `tests/`, never inline. | **AES403** `CapabilityEmbeddedTest` (LOW)          |
| Low-level ops → Utility.                                          | Manual — the helper-vs-utility matrix is a judgement call         |
| `python -c "import <module>"` passes.                             | The interpreter, not `scan`                                       |

**On the block banners.** A banner is `Block <digits>:` standing as its own word
inside a comment, so the Python comment sigil is `#`. The colon is load-bearing:
prose such as `Block 1 (types) -> Block 2` is not a marker, and neither is
`Sub-Block 4:`. Put the banner above the block it heads, at the same indent as
the code it introduces.

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# AES403 machine-enforced: class budget ≤3; protocol inheritor present;
#   protocol method defined before the first private helper; module-level
#   constants in taxonomy file; no inline test class or test function;
#   public methods after the Block 2 boundary with no production caller flagged.
# Manual (not machine-checked): helper-vs-utility matrix; role naming lists.
# Fallback compile gate: python -c "import <shared_package>.<module>"
```
