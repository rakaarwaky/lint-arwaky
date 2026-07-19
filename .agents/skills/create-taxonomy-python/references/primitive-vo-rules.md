# Primitive-to-VO Rules

## General Rule

Domain data MUST use VOs, not raw primitives.

Bad:

```python
@dataclass
class LintResult:
    file_path: str
    line: int
    severity: str
```

Good:

```python
@dataclass(frozen=True)
class LintResult:
    file_path: FilePath
    line: LineNumber
    severity: Severity
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
class LineNumber:
    _value: int

    def __post_init__(self) -> None:
        if self._value == 0:
            raise ValueError("LineNumber must be positive")
```

## Optional and Collection Primitives

Bad:

```python
@dataclass(frozen=True)
class RuleSet:
    patterns: list[str]
    description: str | None
```

Good:

```python
@dataclass(frozen=True)
class RuleSet:
    patterns: PatternList
    description: RuleDescription | None
```
