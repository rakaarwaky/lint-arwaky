# Examples

## BAD: Capability Without Protocol (AES403)

```python
class FrameComposer:
    def compose_frame(self) -> None:
        # public behavior without protocol ABC
        ...
```

Fix:

```python
class FrameComposer(IFrameComposerProtocol):
    def compose_frame(self) -> None:
        # contract implementation
        ...
```

## BAD: I/O in Capabilities (AES404)

```python
class MyCapability:
    def process(self) -> None:
        content = open("file.txt").read()  # FORBIDDEN
```

## BAD: Data Class Defined in Layer File

```python
@dataclass
class OrphanResult:
    is_orphan: bool
    reason: str
```

Fix: Move to shared taxonomy, then import.

## BAD: Concrete Service Field

```python
class CapabilitiesOrphanAnalyzer:
    def __init__(self, extractor: FilenameExtractor):  # BAD
        self._extractor = extractor
```

Fix:

```python
class CapabilitiesOrphanAnalyzer:
    def __init__(self, extractor: IOrphanFilenameExtractorProtocol):
        self._extractor = extractor
```

## BAD: Orchestration Inside Capability (No Orchestration, §8)

```python
class MyPipeline(IImportCheckerProtocol):
    def run(self) -> None:
        a = self.step_a()          # calls another capability's behavior
        if a.is_ok():
            self.step_b()          # branching between capabilities
        else:
            self.escalate()        # error-escalation policy
```

Fix: remove flow control and cross-capability calls. Let the Agent layer compose the pipeline. The capability executes one responsibility and returns a result.

## BAD: Domain Model Defined in Capability (No Domain Definition, §8)

```python
@dataclass
class OrphanResult:    # domain model defined here = forbidden
    is_orphan: bool
    reason: str
```

Fix: define `OrphanResult` as a Taxonomy VO; the capability only consumes and produces it.

## BAD: Dunder Methods in Block 2

```python
class ArchLineChecker(ILineCheckerProtocol):
    def __init__(self) -> None: ...

    def __repr__(self) -> str:           # ← Block 2 position, NOT a protocol method
        return "ArchLineChecker()"

    def check_line_counts(self, ...) -> None:  # ← pushed down
        ...
```

Fix: Move `__repr__` to Block 3.

## GOOD: Capability with DI and Shared VO

```python
from shared.orphan_detector.taxonomy_orphan_analysis_policy_vo import OrphanAnalysisPolicy
from shared.orphan_detector.contract_orphan_file_cache_port import IOrphanFileCachePort
from shared.orphan_detector.contract_orphan_filename_extractor_protocol import IOrphanFilenameExtractorProtocol
from shared.orphan_detector.contract_capabilities_orphan_protocol import ICapabilitiesOrphanProtocol

class CapabilitiesOrphanAnalyzer(ICapabilitiesOrphanProtocol):
    def __init__(
        self,
        extractor: IOrphanFilenameExtractorProtocol,
        cache: IOrphanFileCachePort,
        policy: OrphanAnalysisPolicy,
    ):
        self._extractor = extractor
        self._cache = cache
        self._policy = policy
```

## GOOD: Correct 3-Block Structure

```python
from shared.code_analysis.taxonomy_file_path_vo import FilePath
from shared.code_analysis.taxonomy_layer_definition_vo import LayerDefinition
from shared.code_analysis.taxonomy_line_checker_protocol import ILineCheckerProtocol
from shared.code_analysis.taxonomy_line_checker_utility import is_barrel_file
from shared.code_analysis.taxonomy_lint_result_vo import LintResult
from shared.code_analysis.taxonomy_source_vo import SourceContentVO


# ─── Block 1: Class Definition & Constructor ──────────────
class ArchLineChecker(ILineCheckerProtocol):
    def __init__(self) -> None:
        pass

    # ─── Block 2: Public Contract (domain protocol ONLY) ──
    def check_line_counts(
        self,
        file: FilePath,
        definition: LayerDefinition | None,
        source: SourceContentVO,
        violations: list[LintResult],
    ) -> None:
        basename = file.basename()
        if is_barrel_file(basename):
            return
        # Remaining domain logic...

    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return "ArchLineChecker()"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, ArchLineChecker)

    @classmethod
    def create_default(cls) -> "ArchLineChecker":
        return cls()
```
