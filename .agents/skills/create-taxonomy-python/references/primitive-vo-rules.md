# Primitive-to-VO Rules

## General Rule

Domain data MUST use VOs, not raw primitives.

Bad:

```python
@dataclass
class <ResultVO>:
    target: str
    position: int
    level: str
```

Good:

```python
@dataclass(frozen=True)
class <ResultVO>:
    target: <Target>VO
    position: <LineNumber>VO
    level: <Severity>VO
```

## Primitive Policy

| Primitive | Rule                                                                                |
| --------- | ----------------------------------------------------------------------------------- |
| `str`     | Forbidden for domain fields and public contract return values. Use VO.              |
| `int`     | Forbidden. Use domain VO.                                                           |
| `float`   | Forbidden. Use domain VO.                                                           |
| `bool`    | Allowed for semantic toggles when no richer VO is needed.                           |

Prefer VOs for: file paths, symbol names, messages, line numbers, column numbers, severity, durations, counts, thresholds, identifiers.

## VO Construction Rules

VOs MUST validate on construction when the domain has invariants.

```python
@dataclass(frozen=True)
class <LineNumber>VO:
    _value: int

    def __post_init__(self) -> None:
        if self._value == 0:
            raise ValueError("<LineNumber>VO must be positive")
```

## Optional and Collection Primitives

Bad:

```python
@dataclass(frozen=True)
class <RuleSet>VO:
    patterns: list[str]
    description: str | None
```

Good:

```python
@dataclass(frozen=True)
class <RuleSet>VO:
    patterns: <PatternList>VO
    description: <RuleDescription>VO | None
```
