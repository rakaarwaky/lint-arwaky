# Primitive and VO Rules

Aggregate contracts should use shared VOs for domain data.

Bad:

```python
class IOrphanOrchestratorAggregate(ABC):
    @abstractmethod
    def execute(self, files: list[str]) -> list[str]: ...
```

Good:

```python
class IOrphanOrchestratorAggregate(ABC):
    @abstractmethod
    def execute(self, request: ScanRequest) -> list[LintResult]: ...
```

## Primitive Policy

| Primitive | Rule |
| --------- | ---- |
| `str`     | Forbidden for domain fields and public contract values. Use VO. |
| `int`     | Forbidden for domain values. Use VO. |
| `float`   | Forbidden for domain values. Use VO. |
| `bool`    | Allowed for semantic toggles when no richer VO is needed. |

Prefer VOs for: requests, reports, file paths, identifiers, execution results, violations, policies, thresholds.
