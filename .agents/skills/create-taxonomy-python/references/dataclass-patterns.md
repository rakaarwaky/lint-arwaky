# Dataclass Patterns

## Value Objects (`_vo.py`)

Prefer frozen dataclasses.

Bad:

```python
@dataclass
class FilePath:
    value: str
```

Good:

```python
@dataclass(frozen=True)
class FilePath:
    _value: str

    def __post_init__(self) -> None:
        if not self._value.strip():
            raise ValueError("FilePath cannot be empty")

    @property
    def value(self) -> str:
        return self._value
```

## Composite Value Objects

Use other VOs as fields, not raw primitives.

```python
@dataclass(frozen=True)
class ImportRuleVO:
    pattern: RulePattern
    message: RuleMessage
```

## Entities (`_entity.py`)

```python
@dataclass(frozen=True)
class SymbolEntity:
    id: SymbolId
    name: SymbolName
```

## Error Types (`_error.py`)

Use Python exceptions.

```python
class ConfigError(Exception):
    def __init__(self, key: ConfigKey, message: ErrorMessage):
        self._key = key
        self._message = message
        super().__init__(f"Config error for {key.value}: {message.value}")
```

## Event Types (`_event.py`)

```python
@dataclass(frozen=True)
class ScanCompletedEvent:
    scan_id: ScanId
```

## Constants (`_constant.py`)

```python
FPS_DEFAULT: float = 24.0
MIN_REVEAL_SECONDS: float = 0.5
MANIFEST_FILENAME: str = "manifest.json"
```

Rules: no functions, no I/O, no external layer imports, no mutable state.
