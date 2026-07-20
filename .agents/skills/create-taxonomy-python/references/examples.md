# Examples

## BAD: Dataclass Defined in Capabilities

```python
# capabilities_<name-capability>.py
@dataclass
class <ResultVO>:
    is_valid: bool
    reason: str
```

Fix: Move to taxonomy.

```python
# shared/<name-feature>/taxonomy_<name>_result_vo.py
@dataclass(frozen=True)
class <ResultVO>:
    is_valid: <Flag>VO
    reason: <Reason>VO
```

## BAD: Taxonomy Importing Layer Code

```python
# taxonomy_<name>_vo.py
from capabilities_<name-capability> import <NameAnalyzer>  # BAD
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
# shared/<name-feature>/taxonomy_<name>_analysis_result_vo.py
@dataclass(frozen=True)
class <AnalysisResult>VO:
    is_valid: <Flag>VO
    reason: <Reason>VO

# capabilities_<name-capability>.py
class <NameCapability>:
    def __init__(self, collaborator: I<NameCollaborator>Protocol):
        self._collaborator = collaborator
```
