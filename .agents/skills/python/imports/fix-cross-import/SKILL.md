---
name: fix-cross-import-python
version: 2.0.0
category: refactoring
tags: [aes, import, separation, shared, taxonomy, aes201, python]
triggers:
  - "fix cross import python"
  - "cross capability import python"
  - "cross infrastructure import python"
  - "fix aes201 python"
dependencies: []
related:
  - fix-capability-structure
  - module_logic_validator
  - protocol-consolidation
---
# fix-cross-import-python

## Rules

- Capabilities MUST NOT import from capabilities (AES201 FORBIDDEN)
- Infrastructure MUST NOT import from infrastructure (AES201 FORBIDDEN)
- Infrastructure MUST NOT import from capabilities (AES201 FORBIDDEN)
- Capabilities MUST NOT import from infrastructure (AES201 FORBIDDEN)
- Use protocols/ports in contract layer for cross-peer dependencies
- ALWAYS import from contract layer, NEVER from peer layer
- Root container wires implementations — capabilities receive via DI

## Purpose

Fix AES201 violations where:

- Capability imports from another capability (FORBIDDEN)
- Infrastructure imports from another infrastructure (FORBIDDEN)
- Infrastructure imports from capabilities (FORBIDDEN)
- Capabilities imports from infrastructure (FORBIDDEN)

## When to Use

- Any layer file uses types/functions from another layer file directly (not via protocol)
- `capabilities_*.py` uses types from `capabilities_*.py` (peer-to-peer import)
- `infrastructure_*.py` uses types from `infrastructure_*.py` (peer-to-peer import)
- Infrastructure imports from capabilities (cross-layer import)
- Capabilities import from infrastructure (cross-layer import)
- Any file directly instantiates a concrete type instead of using DI

## The Fundamental Question

> **"Can peer layers import from each other?"**

**NO!** Capabilities and Infrastructure are **PEER layers** - they CANNOT import from each other.

## AES201 Import Rules (Critical)

### Capabilities Layer

```
ALLOWED: taxonomy_*, contract_*
FORBIDDEN: infrastructure_*, surface_*, agent_*, capabilities_*, root_*
```

### Infrastructure Layer

```
ALLOWED: taxonomy_*, contract_*
FORBIDDEN: surface_*, capabilities_*, agent_*, infrastructure_*, root_*
```

## Cross-Import Patterns

### Pattern 1: Capabilities importing from Capabilities [FORBIDDEN]

```python
# WRONG:
# capabilities_timeline_processor.py
from capabilities_frame_exporter import FrameExporter      # FORBIDDEN
from capabilities_keyframe_calculator import Calculator    # FORBIDDEN

# ALSO WRONG (even for pure functions!):
# capabilities_layer_detection_analyzer.py
from capabilities_layer_prefix_extractor import extract_layer_from_prefix  # FORBIDDEN!

# CORRECT:
# 1. Create protocol in contract layer (ONLY protocols, no implementations!)
# contract_frame_exporter_protocol.py
from abc import ABC, abstractmethod
from pathlib import Path

class FrameExporterProtocol(ABC):
    @abstractmethod
    def export(self, frame: 'Frame') -> Path:
        ...

# contract_keyframe_calculator_protocol.py
class KeyframeCalculatorProtocol(ABC):
    @abstractmethod
    def calculate(self, keyframes: list['Keyframe']) -> list['MotionPath']:
        ...

# 2. Capability implements protocol
# capabilities_frame_exporter.py
class FrameExporter(FrameExporterProtocol):
    def export(self, frame: 'Frame') -> Path:
        ...

# 3. Other capability receives via DI (knows only the protocol)
# capabilities_timeline_processor.py
from contract_frame_exporter_protocol import FrameExporterProtocol      # ALLOWED
from contract_keyframe_calculator_protocol import KeyframeCalculatorProtocol  # ALLOWED

class TimelineProcessor:
    def __init__(
        self,
        exporter: FrameExporterProtocol,      # via DI
        calculator: KeyframeCalculatorProtocol, # via DI
    ):
        self._exporter = exporter
        self._calculator = calculator

# 4. Root container wires implementation
# root_container.py
exporter: FrameExporterProtocol = FrameExporter()
calculator: KeyframeCalculatorProtocol = Calculator()
processor = TimelineProcessor(exporter, calculator)
```

**Note**: Contract layer contains ONLY protocols (interfaces), NOT pure functions or implementations.

### Pattern 2: Infrastructure importing from Infrastructure [FORBIDDEN]

```python
# WRONG:
# infrastructure_psd_reader.py
from infrastructure_psd_parser import PSDParser  # FORBIDDEN

# CORRECT:
# 1. Create port in contract layer
# contract_psd_parser_port.py
from abc import ABC, abstractmethod
from pathlib import Path

class PSDParserPort(ABC):
    @abstractmethod
    def parse(self, path: Path) -> 'PSDData':
        ...

# 2. Infrastructure imports from contract (ALLOWED)
# infrastructure_psd_reader.py
from contract_psd_parser_port import PSDParserPort  # ALLOWED

class PSDReader:
    def __init__(self, parser: PSDParserPort):  # via DI
        self._parser = parser
```

### Pattern 3: Infrastructure importing from Capabilities [FORBIDDEN]

```python
# WRONG:
# infrastructure_import_parser_adapter.py
from capabilities_dummy_analyzer import symbol_used_real  # FORBIDDEN
from capabilities_unused_analyzer import extract_imported_aliases  # FORBIDDEN

# CORRECT:
# 1. Create protocol in contract layer (ONLY protocol, no implementation)
# contract_import_analyzer_port.py
from abc import ABC, abstractmethod

class ImportAnalyzerPort(ABC):
    @abstractmethod
    def analyze(self, content: str) -> 'AnalysisResult':
        ...

# 2. Capability implements protocol
# capabilities_import_analyzer.py
class ImportAnalyzer(ImportAnalyzerPort):
    def analyze(self, content: str) -> 'AnalysisResult':
        # All computation here
        ...

# 3. Infrastructure receives via DI (knows only the protocol)
# infrastructure_import_parser_adapter.py
class ImportParserAdapter:
    def __init__(self, analyzer: ImportAnalyzerPort):  # via DI
        self._analyzer = analyzer

# 4. Root container wires implementation
# root_container.py
analyzer: ImportAnalyzerPort = ImportAnalyzer()
parser = ImportParserAdapter(analyzer)
```

### Pattern 4: Shared Infrastructure Implementation [DI WIRING]

When multiple modules need the same infrastructure implementation:

```python
# WRONG:
# capabilities_a.py
from code_analysis import FileCollectorProvider  # FORBIDDEN - imports implementation

# capabilities_b.py
from code_analysis import FileCollectorProvider  # FORBIDDEN - imports implementation

# CORRECT:
# 1. Contract defines interface (in shared)
# contract_scanner_provider_port.py
from abc import ABC, abstractmethod

class ScannerProviderPort(ABC):
    @abstractmethod
    def scan_directory(self, path: 'DirectoryPath') -> list['FilePath']:
        ...

# 2. ONE module owns the implementation
# code_analysis/infrastructure_file_collector_provider.py
class FileCollectorProvider(ScannerProviderPort):
    def scan_directory(self, path: 'DirectoryPath') -> list['FilePath']:
        ...

# 3. Root container wires implementation to all consumers
# root_container.py
scanner: ScannerProviderPort = FileCollectorProvider()

# 4. Capabilities receive via DI (know only the protocol)
# capabilities_lint_executor.py
class LintExecutor:
    def __init__(self, scanner: ScannerProviderPort):  # via DI, not direct import
        self._scanner = scanner
```

## Step-by-Step Fix

### Step 1: Find Violations

Read each file and ask:

- Does `capabilities_*.py` use types from `capabilities_*.py`? -> VIOLATION
- Does `infrastructure_*.py` use types from `infrastructure_*.py`? -> VIOLATION
- Does `infrastructure_*.py` use types from `capabilities_*.py`? -> VIOLATION
- Does `capabilities_*.py` use types from `infrastructure_*.py`? -> VIOLATION
- Does a capability directly instantiate an infrastructure type? -> VIOLATION (should use DI)

### Step 2: Create Protocol in Contract Layer

Create protocol (port/protocol) in contract layer for needed functionality:

```python
# contract_<concept>_protocol.py or contract_<concept>_port.py
# ONLY protocols, NO implementations or pure functions!
from abc import ABC, abstractmethod

class I<Concept>Protocol(ABC):
    @abstractmethod
    def method(self, args...) -> Result:
        ...
```

### Step 3: Update Imports

Change imports to use contract layer:

```python
# BEFORE (VIOLATION)
from capabilities_other import OtherStruct
from infrastructure_provider import Provider

# AFTER (CORRECT)
from contract_other_protocol import IOtherProtocol
from contract_provider_port import IProviderPort
```

### Step 4: Wire via DI (if needed)

If the implementation is shared across modules:

1. Put implementation in ONE module (the "owner")
2. Root container creates instance and wires to consumers
3. Consumers receive protocol via constructor

## Common Violations and Fixes

| Violation | Fix |
|-----------|-----|
| `capabilities_a.py` uses `capabilities_b.Struct` | Create `contract_b_protocol.py` with protocol, use DI |
| `infrastructure_a.py` uses `infrastructure_b.Struct` | Create `contract_b_port.py` with protocol, use DI |
| `infrastructure_a.py` uses `capabilities_b.fn()` | Move computation to capabilities, infra only does I/O |
| `capabilities_a.py` uses `infrastructure_b.fn()` | Receive via `I<Name>Protocol` in constructor |
| Capability creates `Infrastructure()` directly | Receive via `I<Name>Protocol` in constructor |
| Infrastructure imports from capabilities | Create protocol in contract layer |

## File Naming Convention

| Protocol Type | Suffix | Used By | Implemented By | Example |
|---------------|--------|---------|----------------|---------|
| **Protocol** | `_protocol.py` | Capabilities receive from capabilities | Capabilities implements | `contract_dummy_import_checker_protocol.py` |
| **Port** | `_port.py` | Infrastructure receives from infrastructure | Infrastructure implements | `contract_external_lint_port.py` |
| **Aggregate** | `_aggregate.py` | Agents receive from agents | Agents implements | `contract_agent_role_aggregate.py` |

**Rule**: Contract layer contains ONLY protocol definitions, NOT implementations or pure functions.

### When to Use Each

```python
# Use _protocol when capabilities need to communicate with each other
class I<Name>Protocol(ABC): ...  # capabilities_<name>.py implements

# Use _port when infrastructure components need to communicate with each other
class I<Name>Port(ABC): ...      # infrastructure_<name>.py implements

# Use _aggregate when agents need to communicate with each other
class I<Name>Aggregate(ABC): ...  # agent_<name>.py implements
```

## Quick Reference

| Layer | Can Import From | Cannot Import From |
|-------|-----------------|-------------------|
| taxonomy | taxonomy | contract, capabilities, infrastructure, agent, surface, root |
| contract | taxonomy, contract | capabilities, infrastructure, agent, surface, root |
| capabilities | taxonomy, contract | **infrastructure**, surface, agent, **capabilities**, root |
| infrastructure | taxonomy, contract | surface, **capabilities**, agent, **infrastructure**, root |
| agent | taxonomy, contract | capabilities, infrastructure, surface, root |
| surface | taxonomy, contract (limited) | capabilities, infrastructure, agent, root |
| root | ALL layers | (none) |

**Key Rule**: Capabilities and Infrastructure are PEER layers — they CANNOT import from each other!
