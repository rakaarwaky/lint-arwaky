# Primitive and VO Rules

Aggregate contracts should use shared VOs for domain data.

Bad:

```python
class I<NameOrchestrator>Aggregate(ABC):
    @abstractmethod
    def execute(self, files: list[str]) -> list[str]: ...
```

Good:

```python
class I<NameOrchestrator>Aggregate(ABC):
    @abstractmethod
    def execute(self, request: <ScanRequest>VO) -> list[<ResultVO>]: ...
```

## Primitive Policy

| Primitive | Rule                                                            |
| --------- | --------------------------------------------------------------- |
| `str`     | Forbidden for domain fields and public contract values. Use VO. |
| `int`     | Forbidden for domain values. Use VO.                            |
| `float`   | Forbidden for domain values. Use VO.                            |
| `bool`    | Allowed for semantic toggles when no richer VO is needed.       |

Prefer VOs for: requests, reports, file paths, identifiers, execution results, results, policies, thresholds.
