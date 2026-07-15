---
name: fix-cross-import
version: 1.5.0
category: refactoring
tags: [aes, import, separation, shared, taxonomy, aes201]
triggers:
  - "fix cross import"
  - "cross capability import"
  - "cross infrastructure import"
dependencies: []
related:
  - fix-capability-structure
  - module_logic_validator
---
# fix-cross-import

## Rules

- Capabilities MUST NOT import from capabilities (AES201 FORBIDDEN)
- Infrastructure MUST NOT import from infrastructure (AES201 FORBIDDEN)
- Use protocols/ports in contract layer for cross-peer dependencies
- ALWAYS import from contract layer, NEVER from peer layer

## Purpose

Fix AES201 violations where:

- Capability imports from another capability (FORBIDDEN)
- Infrastructure imports from another infrastructure (FORBIDDEN)

## When to Use

- `capabilities_*.py` imports from `capabilities_*.py`
- `infrastructure_*.py` imports from `infrastructure_*.py`

## The Fundamental Question

> **"Can peer layers import from each other?"**

**NO!** Capabilities and Infrastructure are **PEER layers** - they CANNOT import from each other.

## AES201 Import Rules (Critical)

### Capabilities Layer

```
ALLOWED: taxonomy, contract
FORBIDDEN: infrastructure*, surface*, agent*, capabilities*, root
```

### Infrastructure Layer

```
ALLOWED: taxonomy, contract
FORBIDDEN: surface*, capabilities*, agent*, infrastructure*, root
```

## Cross-Import Patterns

### Pattern 1: Capabilities importing from Capabilities [FORBIDDEN]

```
WRONG:
  capabilities_timeline_processor.py
    from capabilities_frame_exporter import FrameExporter  # FORBIDDEN
    from capabilities_keyframe_calculator import Calculator  # FORBIDDEN
  
CORRECT:
  # 1. Create protocol in contract layer
  contract_frame_exporter_protocol.py
    class FrameExporterProtocol(ABC):
      @abstractmethod
      def export(self, frame: Frame) -> Path: ...
  
  contract_keyframe_calculator_protocol.py
    class KeyframeCalculatorProtocol(ABC):
      @abstractmethod
      def calculate(self, keyframes: List[Keyframe]) -> List[MotionPath]: ...
  
  # 2. Capability imports from contract (ALLOWED)
  capabilities_timeline_processor.py
    from contract_frame_exporter_protocol import FrameExporterProtocol  # ALLOWED
    from contract_keyframe_calculator_protocol import KeyframeCalculatorProtocol  # ALLOWED
```

### Pattern 2: Infrastructure importing from Infrastructure [FORBIDDEN]

```
WRONG:
  infrastructure_psd_reader.py
    from infrastructure_psd_parser import PSDParser  # FORBIDDEN
  
CORRECT:
  # 1. Create port in contract layer
  contract_psd_parser_port.py
    class PSDParserPort(ABC):
      @abstractmethod
      def parse(self, path: Path) -> PSDData: ...
  
  # 2. Infrastructure imports from contract (ALLOWED)
  infrastructure_psd_reader.py
    from contract_psd_parser_port import PSDParserPort  # ALLOWED
```

### Step 1: Find Violations

Read each file and ask:

- Does `capabilities_*.py` import from `capabilities_*.py`? -> VIOLATION
- Does `infrastructure_*.py` import from `infrastructure_*.py`? -> VIOLATION

### Step 2: Create Protocol/Port

Create protocol/port in contract layer for needed functionality.

### Step 3: Update Imports

Change imports to use contract layer.

## Summary


| Violation                        | Solution                          |
| ---------------------------------- | ----------------------------------- |
| capabilities -> capabilities     | Create protocol in contract layer |
| infrastructure -> infrastructure | Create port in contract layer     |
