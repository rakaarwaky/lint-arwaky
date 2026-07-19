# Examples

## BAD: Dataclass Defined in Capabilities

```python
# capabilities_orphan_analyzer.py
@dataclass
class OrphanResult:
    is_orphan: bool
    reason: str
```

Fix: Move to taxonomy.

```python
# shared/orphan_detector/taxonomy_orphan_result_vo.py
@dataclass(frozen=True)
class OrphanResult:
    is_orphan: OrphanFlag
    reason: OrphanReason
```

## BAD: Taxonomy Importing Layer Code

```python
# taxonomy_orphan_vo.py
from capabilities_orphan_analyzer import OrphanAnalyzer  # BAD
```

Taxonomy must not import from layers.

## BAD: Domain Rule Inside Utility

```python
def is_port_trait_name(name: str) -> bool:
    return name.endswith("Port")
```

If this knows AES naming conventions, it belongs in capabilities as a helper.

## GOOD: Dataclass in Taxonomy + Implementor with DI

```python
# shared/orphan_detector/taxonomy_orphan_analysis_result_vo.py
@dataclass(frozen=True)
class OrphanAnalysisResult:
    is_orphan: OrphanFlag
    reason: OrphanReason

# capabilities_orphan_analyzer.py
class CapabilitiesOrphanAnalyzer:
    def __init__(self, extractor: IOrphanFilenameExtractorProtocol):
        self._extractor = extractor
```
