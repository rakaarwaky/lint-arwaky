# Taxonomy — Python

Source layout: `modules/shared/src/<domain>/`. File: `taxonomy_<domain>_<type>.py`.

## Import rules

**Allowed imports:** other taxonomy types, stdlib.
**Forbidden:** capabilities, agents, surface, root, contracts, I/O (in VOs/entities/errors/events/constants).

## File-name suffix table

| Suffix         | Content                | Key constraint                             |
| ---------------- | ------------------------ | -------------------------------------------- |
| `_vo.py`       | Value Objects          | Validate in`__init__`, immutable, no I/O   |
| `_entity.py`   | Entities with identity | Identity VO field required                 |
| `_error.py`    | Domain errors          | Extend`Exception`, VO fields only          |
| `_event.py`    | Domain events          | Immutable, VO payload fields               |
| `_constant.py` | Compile-time constants | Pure literals only — no functions, no I/O |
| `_utility.py`  | Stateless helpers      | No class, no`self`, domain-agnostic        |

## VO primitive rules (AES401)

Forbidden for domain fields: `str`, `int`, `float`, `list[str]`, `dict`.
`bool` allowed for semantic toggles only.

## Templates

### Value Object

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class <Name>:
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

### Entity

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class <Name>:
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
- **`error_id`** — a stable numeric id such as `1001`, for API/DB/monitoring correlation
- **`error_code`** — a stable string name such as `ORDER_NOT_FOUND`, readable in logs
- **`message`** — a human-readable description derived from the id, code, and VOs

Both the `error_id` and `error_code` stay stable across releases. Renaming or
reformatting `message` is a compatible change; changing either id or code is
breaking. Allocate ids in per-domain blocks (order from 1000, billing from 2000)
to avoid cross-domain collisions.


### Constants

```python
# Default value description.
<NAME>_DEFAULT: float = 24.0

# Minimum value description.
<NAME>_MIN: float = 0.5

# Filename constant.
<NAME>_FILENAME: str = "file.json"
```

## Workflow

1. Determine type (VO/Entity/Error/Event/Constant).
2. Create `taxonomy_<domain>_<type>.py` in `shared/src/<domain>/`.
3. VOs: validate in `__init__`, use `@dataclass(frozen=True)` or manual.
4. Errors: extend `Exception`; expose an `error_id`, an `error_code`, and a
   `message` property derived from the stored VOs.
5. Constants: pure literals only.
6. Register in `__init__.py`.
7. `python -c "import <module>"`.

## Checklist

- [ ]  Correct suffix.
- [ ]  VOs validate on construction; composite VOs use other VOs (no raw primitives).
- [ ]  Errors extend `Exception`, expose `error_id`, `error_code`, and `message`.
- [ ]  Constants are pure literal values.
- [ ]  No import from capabilities, agents, surface, root, contracts.
- [ ]  No I/O, network, or database in taxonomy files.
- [ ]  Registered in shared `__init__.py`.
- [ ]  `python -c "import <module>"` passes.
