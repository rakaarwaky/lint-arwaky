# Contract — Python

File: `contract_<concept>_<suffix>.py`. Contract = pure ABC definitions. No implementation.

## Import rules

**Allowed imports:** taxonomy types, other contract types.
**Forbidden:** capabilities, agents, surface, root.

## Rules (Python)

- ABC class only — `@abstractmethod`, body is `...` or `pass`.
- No private helper signatures.
- All methods type-annotated.
- Inherit `abc.ABC`.
- Signatures use shared VOs — no `str`/`int`/`float`/`list[str]`/`dict` for domain values.
- `bool` allowed for semantic toggles only.
- Register in shared `__init__.py`.

## Templates

### Protocol ABC

```python
from abc import ABC, abstractmethod
from shared.<domain>.taxonomy_<name>_vo import <VO>

class I<Name>Protocol(ABC):
    @abstractmethod
    def method_name(self, param: <VO>) -> None: ...
```

### Aggregate ABC

```python
from abc import ABC, abstractmethod
from shared.<domain>.taxonomy_<name>_vo import <VO>

class I<Name>Aggregate(ABC):
    @abstractmethod
    def execute(self, request: ScanRequest) -> list[LintResult]: ...
```

## Workflow

1. Which layer implements this? Capabilities → `_protocol`. Agent → `_aggregate`.
2. Golden Rule: only methods called by outer layers go in the contract.
3. Create `contract_<concept>_<suffix>.py` in shared domain.
4. Register in `__init__.py`.
5. `python -c "import <module>"`.

## Checklist

- [ ] Correct suffix `_protocol` or `_aggregate`.
- [ ] Only `@abstractmethod` definitions — no implementations.
- [ ] All methods type-annotated; inherit `abc.ABC`.
- [ ] No imports from capabilities, agents, surface.
- [ ] Signatures use shared VOs.
- [ ] Registered in shared `__init__.py`.
- [ ] `python -c "import <module>"` passes.
