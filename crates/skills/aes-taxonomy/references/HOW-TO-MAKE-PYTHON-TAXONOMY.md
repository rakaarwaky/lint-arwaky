# HOW TO MAKE TAXONOMY PYTHON

> **Purpose**: Define the stable language of the domain: value objects, entities, errors, events, and constants.
>
> **Audience**: Agents and engineers scaffolding AES taxonomy files in the shared domain.
>
> **Scope**: Python taxonomy files named `taxonomy_<domain>_<concept>_<suffix>.py` using suffixes `_vo`, `_entity`, `_error`, `_event`, `_constant` only.
>
> **Location**: Python shared domain source root next to contracts:
>
> ```text
> modules/shared/src/<domain>/
> ```
>
> Python taxonomy files must be registered in the domain `__init__.py`, which is the Python shared barrel.
>
> **Length**: One taxonomy type per file. Constants files must be at least 5 lines per AES302. No I/O, no upward imports, no raw primitives for public/domain fields.

---

## Rules

### Import rules

**Allowed imports:**

```text
Other taxonomy types
Pure stdlib modules, such as dataclasses, enum, typing, re
```

**Forbidden imports:**

```text
capabilities
agents
surface
root
contracts
I/O modules
network modules
filesystem modules
environment modules
randomness modules
time-retrieval modules
any module with side effects
```

Taxonomy files must not perform I/O, network access, database access, filesystem access, environment access, randomness, or time retrieval.

---

### File-name pattern

Use:

```text
taxonomy_<domain>_<concept>_<suffix>.py
```

Where:

```text
<domain>  = snake_case domain name
<concept> = snake_case concept name; may contain multiple underscores
<suffix>  = one of the allowed taxonomy suffixes
```

The pattern has three placeholders after `taxonomy`, but because `<concept>` may itself be multi-word, the final filename may contain more than three underscore-separated segments.

Examples:

```text
taxonomy_order_order_id_vo.py
taxonomy_order_money_vo.py
taxonomy_order_order_entity.py
taxonomy_order_order_not_found_error.py
taxonomy_order_order_created_event.py
taxonomy_order_order_constant.py
```

The filename must match the taxonomy type it contains.

Example mapping:

```text
taxonomy_order_order_id_vo.py           -> OrderId
taxonomy_order_order_entity.py          -> Order
taxonomy_order_order_not_found_error.py -> OrderNotFoundError
taxonomy_order_order_created_event.py   -> OrderCreated
taxonomy_order_order_constant.py        -> ORDER_* constants
```

Allowed suffixes are exactly:

```text
vo
entity
error
event
constant
```

Anything else fails AES102.

---

### File-name suffix table

| Suffix | Content | Key constraint |
|---|---|---|
| `_vo.py` | Value Objects | Validate in `__init__` or `__post_init__`, immutable, no I/O |
| `_entity.py` | Entities with identity | Identity VO field required; domain fields must be VOs |
| `_error.py` | Domain errors | Extend `Exception`; store VO fields only |
| `_event.py` | Domain events | Immutable, VO payload fields only |
| `_constant.py` | Compile-time constants | Pure literals only — no functions, no I/O |

---

### VO primitive rules (AES401)

AES401 forbids raw primitives as public/domain fields.

There is no `bool` carve-out. `bool` is treated as a primitive.

The implementation primitive set is broader than the most common examples. It includes at least:

```text
str
int
float
bool
list
dict
tuple
set
bytes
None
Any
Optional
Union
List
Dict
Tuple
Set
FrozenSet
```

Public/domain fields in entities, events, errors, and composite VOs must be taxonomy VOs, not primitives.

A leaf VO may privately encapsulate a primitive value for validation and representation. That internal encapsulation is a modeling convention and is not machine-checked for `_vo` files.

A leaf VO may expose a read-only accessor such as `.value` for its wrapped primitive, but domain boundaries must pass the VO itself, not the primitive.

Constants are the explicit exception for literal values because constant files contain compile-time literals, not domain fields.

---

### File length (AES302)

Every taxonomy file must be at least **5 lines** or the linter reports:

```text
[AES302] FILE_TOO_SHORT: File contains fewer than the required minimum lines.
FIX: Expand the component or merge this logic into a related module. (min: 5).
```

Docstrings and comments count. For constants, each constant should have a descriptive comment line above it — this keeps the file above the minimum while also documenting intent.

---

## Workflow

1. Determine type:
   ```text
   VO
   Entity
   Error
   Event
   Constant
   ```

2. Create the file:
   ```text
   taxonomy_<domain>_<concept>_<suffix>.py
   ```
   inside:
   ```text
   modules/shared/src/<domain>/
   ```

3. For VOs:
   ```text
   Validate on construction.
   Use @dataclass(frozen=True) or an equivalent immutable manual implementation.
   No I/O.
   No side effects.
   ```

4. For Entities:
   ```text
   Include an identity VO field.
   Use VO fields only for domain state.
   Do not use raw primitives for public/domain fields.
   ```

5. For Errors:
   ```text
   Extend Exception.
   Store VO fields only — no raw str, int, float, dict.
   Provide an error_id property (stable numeric id) for fast machine branching.
   Provide an error_code property (plain string identifier) for human-readable
     error classification.
   Provide a message property that derives a human-readable description
     from the stored VOs, error id, and error code.
   Do not store raw str as a domain payload field.
   ```

6. For Events:
   ```text
   Use @dataclass(frozen=True).
   Use VO payload fields only.
   Name events as past-tense domain facts.
   ```

7. For Constants:
   ```text
   Use module-level literal assignments only.
   No functions.
   No classes.
   No I/O.
   No computed values.
   ```

8. Register the public type in the domain `__init__.py`.

9. Verify the module imports:
   ```bash
   python -c "import modules.shared.src.<domain>.taxonomy_<domain>_<concept>_<suffix>"
   ```

Example:

```bash
python -c "import modules.shared.src.order.taxonomy_order_order_id_vo"
```

If your Python package root differs from `modules.shared`, substitute the correct package root while preserving the actual source path layout.

---

## Template

### Value Object

```python
from dataclasses import dataclass


@dataclass(frozen=True)
class <Name>:
    # Private primitive encapsulation is allowed only inside leaf VOs.
    _value: str

    def __post_init__(self) -> None:
        if not self._value.strip():
            raise ValueError("<Name> cannot be empty")

    @property
    def value(self) -> str:
        return self._value

    def __str__(self) -> str:
        return self._value
```

Concrete example:

```python
from dataclasses import dataclass


@dataclass(frozen=True)
class OrderId:
    _value: str

    def __post_init__(self) -> None:
        if not self._value.strip():
            raise ValueError("OrderId cannot be empty")

    @property
    def value(self) -> str:
        return self._value

    def __str__(self) -> str:
        return self._value
```

---

### Entity

```python
from dataclasses import dataclass

from .taxonomy_<domain>_<concept>_id_vo import <Name>Id


@dataclass(frozen=True)
class <Name>:
    id: <Name>Id
```

Concrete example:

```python
from dataclasses import dataclass

from .taxonomy_order_order_id_vo import OrderId


@dataclass(frozen=True)
class Order:
    id: OrderId
```

Entity fields must be taxonomy VOs.

Example with additional VO fields:

```python
from dataclasses import dataclass

from .taxonomy_order_customer_id_vo import CustomerId
from .taxonomy_order_money_vo import Money
from .taxonomy_order_order_id_vo import OrderId
from .taxonomy_order_order_status_vo import OrderStatus


@dataclass(frozen=True)
class Order:
    id: OrderId
    customer_id: CustomerId
    total: Money
    status: OrderStatus
```

Do not do this:

```python
@dataclass(frozen=True)
class Order:
    id: str
    customer_id: str
    total: float
    status: str
```

The linter may not catch every bare primitive field in a dataclass, but it is still a taxonomy defect.

---

### Error

```python
class <Name>Error(Exception):
    def __init__(self, <field_name>: <FieldVO>) -> None:
        self._<field_name> = <field_name>
        super().__init__()

    @property
    def <field_name>(self) -> <FieldVO>:
        return self._<field_name>

    @property
    def error_id(self) -> int:
        """Stable numeric id. Callers branch on this, never on the message."""
        return <NNNN>

    @property
    def error_code(self) -> str:
        """Stable machine-readable name. Callers branch on this, never on the message."""
        return "<DOMAIN>_<REASON>"

    @property
    def message(self) -> str:
        return f"{self.error_id} {self.error_code}: <Description>: {self._<field_name>}"

    def __str__(self) -> str:
        return self.message
```

Concrete example:

```python
from .taxonomy_order_order_id_vo import OrderId


class OrderNotFoundError(Exception):
    def __init__(self, order_id: OrderId) -> None:
        self._order_id = order_id
        super().__init__()

    @property
    def order_id(self) -> OrderId:
        return self._order_id

    @property
    def error_id(self) -> int:
        """Stable numeric id. Callers branch on this, never on the message."""
        return 1001

    @property
    def error_code(self) -> str:
        """Stable machine-readable name. Callers branch on this, never on the message."""
        return "ORDER_NOT_FOUND"

    @property
    def message(self) -> str:
        return f"{self.error_id} {self.error_code}: order not found: {self._order_id}"

    def __str__(self) -> str:
        return self.message
```

Every error must expose all four:
- **Field VO** — programmatic access to the domain data that caused the error
- **Error id** — a stable numeric id such as `1001`, so APIs, database rows,
  and monitoring queries can correlate on a compact value
- **Error code** — a stable string name such as `ORDER_NOT_FOUND`, readable in
  logs without a lookup table
- **Message** — a human-readable description derived from the error id, the
  error code, and the stored VOs

Both the error id and the error code stay stable across releases. Renaming or
reformatting the message is a compatible change; changing the id or the code
is a breaking one. Allocate ids in blocks per domain (order errors from 1000,
billing errors from 2000) so two domains cannot collide on the same number.

Good:

```python
order_id: OrderId
customer_id: CustomerId
amount: Money
```

Bad:

```python
message: str
order_id: str
amount: float
payload: dict
```

---

### Event

```python
from dataclasses import dataclass


@dataclass(frozen=True)
class <Name>:
    <field_name>: <FieldVO>
```

Concrete example:

```python
from dataclasses import dataclass

from .taxonomy_order_order_id_vo import OrderId


@dataclass(frozen=True)
class OrderCreated:
    order_id: OrderId
```

Events must be immutable and use VO payload fields only.

Good:

```python
order_id: OrderId
customer_id: CustomerId
total: Money
```

Bad:

```python
order_id: str
customer_id: str
total: float
payload: dict
```

---

### Constants

Use the singular `_constant` suffix:

```text
taxonomy_<domain>_<concept>_constant.py
```

Example:

```text
taxonomy_order_order_constant.py
```

Template:

```python
# Default value description.
<NAME>_DEFAULT: float = 24.0

# Minimum value description.
<NAME>_MIN: float = 0.5

# Filename constant.
<NAME>_FILENAME: str = "file.json"
```

Concrete example:

```python
# Default order refresh interval in hours.
ORDER_REFRESH_INTERVAL_DEFAULT: float = 24.0

# Minimum order refresh interval in hours.
ORDER_REFRESH_INTERVAL_MIN: float = 0.5

# Order snapshot filename.
ORDER_SNAPSHOT_FILENAME: str = "order_snapshot.json"
```

Constants must be pure literals.

Allowed:

```python
MAX_ITEMS: int = 100
DEFAULT_CURRENCY: str = "USD"
MIN_AMOUNT: float = 0.01
```

Forbidden:

```python
def get_max_items() -> int:
    return 100

MAX_ITEMS = get_max_items()
```

> The comment lines above each constant are not decoration — they keep the file
> above the AES302 five-line minimum. A bare three-constant file fails.

---

### Registration

Register each public taxonomy type in the domain `__init__.py`.

Example:

```python
# modules/shared/src/order/__init__.py

from .taxonomy_order_order_id_vo import OrderId
from .taxonomy_order_order_entity import Order
from .taxonomy_order_order_not_found_error import OrderNotFoundError
from .taxonomy_order_order_created_event import OrderCreated

__all__ = [
    "OrderId",
    "Order",
    "OrderNotFoundError",
    "OrderCreated",
]
```

---

## Section Contract

| Check | Enforcement | Why it belongs here |
|---|---|---|
| Correct file pattern: `taxonomy_<domain>_<concept>_<suffix>.py`. | Machine-checked: AES101. | Required by AES layer rules and the linter; missing it is a defect. |
| Correct suffix: `_vo`, `_entity`, `_error`, `_event`, `_constant`. | Machine-checked: AES102. | Required by AES layer rules and the linter; missing it is a defect. |
| File is at least 5 lines. | Machine-checked: AES302. | Required by AES layer rules and the linter; missing it is a defect. |
| No forbidden layer imports from capabilities, agents, surface, root, contracts. | Partially machine-checked: AES201–AES205. See Verify. | Required by AES layer rules; missing it is a defect. |
| Registered in shared `__init__.py`. | Machine-checked: AES501 orphan check. See orphan note in Verify. | Required by AES layer rules and the linter; missing it is a defect. |
| Constant file contains only constant material, not classes/functions. | Machine-checked: AES401 on constant files. | Required by AES layer rules and the linter; missing it is a defect. |
| One taxonomy type per file. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| VOs validate on construction. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| VOs are immutable. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Leaf VOs encapsulate only the primitive needed for validation. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Composite VOs use other VOs, not raw primitives. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Public/domain fields avoid primitives. | Convention — not machine-checked reliably. The reader verifies this; the linter may miss bare dataclass fields. | Required by AES taxonomy convention; missing it is a defect. |
| Entities contain an identity VO field. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Entities use VO fields for domain state. | Convention — not machine-checked reliably. The reader verifies this. | Required by AES taxonomy convention; missing it is a defect. |
| Events are immutable. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Events use VO payload fields only. | Convention — not machine-checked reliably. The reader verifies this. | Required by AES taxonomy convention; missing it is a defect. |
| Errors extend `Exception`. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Errors store VO fields only. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Errors expose an `error_id` property (stable numeric id), an `error_code` property (stable string name), and a `message` property derived from their VOs. | Best practice — not machine-checked. Enables callers to branch on `error_id`/`error_code` and to read the description without parsing `__str__`. | Required by AES best practice; missing it is a defect. |
| Constants are pure literal values. | Convention for literal purity; structural violations may be machine-checked. | Required by AES taxonomy convention; missing it is a defect. |
| No I/O, network, database, filesystem, environment, randomness, or time retrieval. | Convention — not machine-checked fully. The reader verifies this; the linter does not guarantee it. | Required by AES taxonomy convention; missing it is a defect. |
| `python -c "import ..."` passes. | Manual fallback gate. | Required by the Python packaging check; missing it is a defect. |

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
```

Machine-checked areas include:

```text
AES101: filename pattern
AES102: suffix correctness
AES203: unused imports
AES204: dummy functions
AES205: circular dependencies
AES301/AES302: file length bounds
AES401: non-constant declarations inside _constant files
AES501–AES506: orphan files and reachability
```

### What is convention, not enforcement

```text
Import boundary rules (AES201/AES202) — the import matrix above is the target
  architecture. Do not treat a clean scan as proof the boundaries hold.

Primitive rules (AES401 on fields) — the auditor only inspects lines that end in
  punctuation (, ; } ) :) or contain an arrow (-> ). A conventional dataclass
  field written as `name: str` is skipped entirely. VO files are not scanned for
  primitives at all, since the check is gated on the _entity/_error/_event
  suffixes. Wrap domain fields in VOs by discipline, not by expectation of a
  violation.

Class/inheritance rules (AES403/AES405) — verify these by reading.
```

A pass means the mechanically-checkable rules are satisfied. The boundary and
primitive rules above are the reader's responsibility.

### Orphan reachability — read this before Phase 2

A taxonomy file that is only referenced by barrels and other taxonomy files
is reported as an orphan:

```
[AES501] 'taxonomy_user_vo' is not reachable and not imported by higher layers.
WHY? only imported by lower-layer files (__init__.py, taxonomy_user_error.py)
FIX: Import 'taxonomy_user_vo' from a _entry file AND a contract_* or higher-layer file.
```

Barrel registration alone does **not** clear this. The shared barrel satisfies
the "has importers" half; you also need a `contract_*` file to import the
taxonomy type. The fix is structural, not per-file: wire the full chain through
root, then rescan. Individual layer phases are expected to show orphan
violations — they clear once the complete dependency graph is in place. Do not
treat an isolated taxonomy scan as the final gate.

Fallback compile gate:

```bash
python -c "import modules.shared.src.<domain>.taxonomy_<domain>_<concept>_<suffix>"
```

Example:

```bash
python -c "import modules.shared.src.order.taxonomy_order_order_id_vo"
```

Related: [HOW-TO-MAKE-RUST-TAXONOMY.md](HOW-TO-MAKE-RUST-TAXONOMY.md), [HOW-TO-MAKE-TYPESCRIPT-TAXONOMY.md](HOW-TO-MAKE-TYPESCRIPT-TAXONOMY.md)
