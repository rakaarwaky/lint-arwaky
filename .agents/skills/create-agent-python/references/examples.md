# Examples

## BAD: Computation in Agent

```python
class <NameOrchestrator>:
    def process(self, files: list[FilePath]):
        total = len(files)  # BAD: computation
        sum_val = sum(f.size for f in files)  # BAD
```

## BAD: Business Logic in Agent

```python
class <NameOrchestrator>:
    def evaluate(self, content: FileContent) -> bool:
        return "forbidden-marker" in content.value  # BAD: business rule
```

## BAD: I/O in Agent

```python
class <NameOrchestrator>:
    def execute(self, path: FilePath):
        content = open(path.value()).read()  # BAD
```

## BAD: Dataclass in Agent File

```python
@dataclass
class <Report>VO:
    results: list[str]
```

## BAD: Concrete Service Field

```python
class <NameOrchestrator>:
    def __init__(self, analyzer: <NameAnalyzer>):  # BAD
        self._analyzer = analyzer
```

## GOOD: Correct 3-Block Order

```python
from shared.<name-feature>.contract_analyzer_protocol import I<NameAnalyzer>Protocol
from shared.<name-feature>.contract_orchestrator_aggregate import I<NameOrchestrator>Aggregate
from shared.<name-feature>.taxonomy_result_vo import <ResultVO>

class <NameOrchestrator>(I<NameOrchestrator>Aggregate):
    def __init__(self, analyzer: I<NameAnalyzer>Protocol):
        self._analyzer = analyzer

    def execute(self, request: <ScanRequest>VO) -> list[<ResultVO>]:
        results: list[<ResultVO>] = []
        for file in request.files():
            try:
                result = self._analyzer.analyze(file)
                results.extend(result.into_results())
            except Exception as e:
                results.append(<ResultVO>.from_analysis_error(file, e))
        return results

    def __repr__(self) -> str:
        return "<NameOrchestrator>()"

    @classmethod
    def create_default(cls) -> "<NameOrchestrator>":
        return cls(analyzer=Capabilities<NameCapability>())
```
