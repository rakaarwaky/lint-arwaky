# Examples

## BAD: Computation in Agent

```python
class <Name>Orchestrator:
    def process(self, files: list[FilePath]):
        total = len(files)  # BAD: computation
        sum_val = sum(f.size for f in files)  # BAD
```

## BAD: Business Logic in Agent

```python
class <Name>Orchestrator:
    def evaluate(self, content: FileContent) -> bool:
        return "forbidden-marker" in content.value  # BAD: business rule
```

## BAD: I/O in Agent

```python
class <Name>Orchestrator:
    def execute(self, path: FilePath):
        content = open(path.value()).read()  # BAD
```

## BAD: Dataclass in Agent File

```python
@dataclass
class <Name>ReportVO:
    results: list[str]
```

## BAD: Concrete Service Field

```python
class <Name>Orchestrator:
    def __init__(self, analyzer: TextFormatter):  # BAD: concrete type
        self._analyzer = analyzer
```

## GOOD: Correct 3-Block Order

```python
from shared.<name_feature>.contract_<name>_protocol import I<Name>Protocol
from shared.<name_feature>.contract_<name>_aggregate import I<Name>Aggregate
from shared.<name_feature>.taxonomy_<name>_vo import <Name>VO
from shared.<name_feature>.taxonomy_<result>_vo import <ResultVO>

class <Name>Orchestrator(I<Name>Aggregate):
    def __init__(self, deps: <Name>Deps):
        self._deps = deps

    def execute(self, request: <RequestVO>) -> <ResultVO>:
        formatter: I<Name>Protocol = self._get_formatter(request)
        return formatter.process(request)

    def __repr__(self) -> str:
        return "<Name>Orchestrator()"

    def _get_formatter(self, request: <RequestVO>) -> I<Name>Protocol:
        match request.type:
            case RequestType.A: return self._deps.a
            case RequestType.B: return self._deps.b
```
