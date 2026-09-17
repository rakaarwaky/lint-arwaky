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
    def __init__(self, message: str):
        self._message = message
        super().__init__(message)

    @property
    def message(self) -> str:
        return self._message
```

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

1. Determine type (VO/Entity/Error/Event/Constant/Utility).
2. Create `taxonomy_<domain>_<type>.py` in `shared/src/<domain>/`.
3. VOs: validate in `__init__`, use `@dataclass(frozen=True)` or manual.
4. Errors: extend `Exception`.
5. Constants: pure literals only.
6. Register in `__init__.py`.
7. `python -c "import <module>"`.

## Checklist

- [ ]  Correct suffix.
- [ ]  VOs validate on construction; composite VOs use other VOs (no raw primitives).
- [ ]  Errors extend `Exception`.
- [ ]  Constants are pure literal values.
- [ ]  No import from capabilities, agents, surface, root, contracts.
- [ ]  No I/O, network, or database in taxonomy files.
- [ ]  Registered in shared `__init__.py`.
- [ ]  `python -c "import <module>"` passes.
