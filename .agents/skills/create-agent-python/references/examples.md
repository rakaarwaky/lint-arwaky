# Examples

## BAD: Computation in Agent

```python
class OrphanOrchestrator:
    def process(self, files: list[FilePath]):
        total = len(files)  # BAD: computation
        sum_val = sum(f.size for f in files)  # BAD
```

## BAD: Business Logic in Agent

```python
class OrphanOrchestrator:
    def analyze(self, content: FileContent) -> bool:
        return "orphan" in content.value  # BAD: business rule
```

## BAD: I/O in Agent

```python
class OrphanOrchestrator:
    def execute(self, path: FilePath):
        content = open(path.value()).read()  # BAD
```

## BAD: Dataclass in Agent File

```python
@dataclass
class OrphanReport:
    results: list[str]
```

## BAD: Concrete Service Field

```python
class OrphanOrchestrator:
    def __init__(self, analyzer: OrphanAnalyzer):  # BAD
        self._analyzer = analyzer
```

## GOOD: Correct 3-Block Order

```python
from shared.orphan_detector.contract_orphan_protocol import IOrphanAnalyzerProtocol
from shared.orphan_detector.contract_orphan_aggregate import IOrphanOrchestratorAggregate
from shared.code_analysis.taxonomy_result_vo import LintResult

class OrphanOrchestrator(IOrphanOrchestratorAggregate):
    def __init__(self, analyzer: IOrphanAnalyzerProtocol):
        self._analyzer = analyzer

    def execute(self, request: ScanRequest) -> list[LintResult]:
        violations: list[LintResult] = []
        for file in request.files():
            try:
                result = self._analyzer.analyze(file)
                violations.extend(result.into_violations())
            except Exception as e:
                violations.append(LintResult.from_analysis_error(file, e))
        return violations

    def __repr__(self) -> str:
        return "OrphanOrchestrator()"

    @classmethod
    def create_default(cls) -> "OrphanOrchestrator":
        return cls(analyzer=CapabilitiesOrphanAnalyzer())
```
