---
name: fix-cross-import-rust
version: 1.0.0
category: refactoring
tags: [aes, import, separation, shared, taxonomy, aes201, rust]
triggers:
  - "fix cross import rust"
  - "cross capability import rust"
  - "cross infrastructure import rust"
  - "fix aes201 rust"
dependencies: []
related:
  - fix-capability-structure
  - module_logic_validator
---
# fix-cross-import-rust

## Rules

- Capabilities MUST NOT import from capabilities (AES201 FORBIDDEN)
- Infrastructure MUST NOT import from infrastructure (AES201 FORBIDDEN)
- Infrastructure MUST NOT import from capabilities (AES201 FORBIDDEN)
- Capabilities MUST NOT import from infrastructure (AES201 FORBIDDEN)
- Use traits (ports/protocols) in contract layer for cross-peer dependencies
- ALWAYS import from contract layer, NEVER from peer layer
- Root container wires implementations — capabilities receive via DI

## Purpose

Fix AES201 violations where:

- Capability imports from another capability (FORBIDDEN)
- Infrastructure imports from another infrastructure (FORBIDDEN)
- Infrastructure imports from capabilities (FORBIDDEN)
- Capabilities imports from infrastructure (FORBIDDEN)

## When to Use

- Any layer file uses types/functions from another layer file directly (not via trait)
- `capabilities_*.rs` uses types from `capabilities_*.rs` (peer-to-peer import)
- `infrastructure_*.rs` uses types from `infrastructure_*.rs` (peer-to-peer import)
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

```rust
// WRONG:
// capabilities_timeline_processor.rs
use crate::capabilities_frame_exporter::FrameExporter;      // FORBIDDEN
use crate::capabilities_keyframe_calculator::Calculator;    // FORBIDDEN

// ALSO WRONG (even for pure functions!):
// capabilities_layer_detection_analyzer.rs
use crate::capabilities_layer_prefix_extractor::extract_layer_from_prefix;  // FORBIDDEN!

// CORRECT:
// 1. Create trait in contract layer (ONLY traits, no functions!)
// contract_frame_exporter_port.rs
pub trait IFrameExporterPort: Send + Sync {
    fn export(&self, frame: &Frame) -> PathBuf;
}

// contract_keyframe_calculator_port.rs
pub trait IKeyframeCalculatorPort: Send + Sync {
    fn calculate(&self, keyframes: &[Keyframe]) -> Vec<MotionPath>;
}

// 2. Capability implements trait
// capabilities_frame_exporter.rs
pub struct FrameExporter;
impl IFrameExporterPort for FrameExporter {
    fn export(&self, frame: &Frame) -> PathBuf { ... }
}

// 3. Other capability receives via DI (knows only the trait)
// capabilities_timeline_processor.rs
use crate::contract_frame_exporter_port::IFrameExporterPort;      // ALLOWED
use crate::contract_keyframe_calculator_port::IKeyframeCalculatorPort;  // ALLOWED

pub struct TimelineProcessor {
    exporter: Arc<dyn IFrameExporterPort>,      // via DI
    calculator: Arc<dyn IKeyframeCalculatorPort>, // via DI
}

// 4. Root container wires implementation
// root_container.rs
let exporter: Arc<dyn IFrameExporterPort> = Arc::new(FrameExporter::new());
let calculator: Arc<dyn IKeyframeCalculatorPort> = Arc::new(Calculator::new());
let processor = TimelineProcessor::new(exporter, calculator);
```

**Note**: Contract layer contains ONLY traits (interfaces), NOT pure functions or implementations.

### Pattern 2: Infrastructure importing from Infrastructure [FORBIDDEN]

```rust
// WRONG:
// infrastructure_psd_reader.rs
use crate::infrastructure_psd_parser::PSDParser;  // FORBIDDEN

// CORRECT:
// 1. Create port in contract layer
// contract_psd_parser_port.rs
pub trait IPSDParserPort: Send + Sync {
    fn parse(&self, path: &Path) -> Result<PSDData, ParseError>;
}

// 2. Infrastructure imports from contract (ALLOWED)
// infrastructure_psd_reader.rs
use crate::contract_psd_parser_port::IPSDParserPort;  // ALLOWED

pub struct PSDReader {
    parser: Arc<dyn IPSDParserPort>,  // via DI
}
```

### Pattern 3: Infrastructure importing from Capabilities [FORBIDDEN]

```rust
// WRONG:
// infrastructure_import_parser_adapter.rs
use crate::capabilities_dummy_analyzer::symbol_used_real;  // FORBIDDEN
use crate::capabilities_unused_analyzer::extract_imported_aliases;  // FORBIDDEN

// CORRECT:
// 1. Create trait in contract layer (ONLY trait, no implementation)
// contract_import_analyzer_port.rs
pub trait IImportAnalyzerPort: Send + Sync {
    fn analyze(&self, content: &str) -> AnalysisResult;
}

// 2. Capability implements trait
// capabilities_import_analyzer.rs
pub struct ImportAnalyzer;
impl IImportAnalyzerPort for ImportAnalyzer {
    fn analyze(&self, content: &str) -> AnalysisResult {
        // All computation here
    }
}

// 3. Infrastructure receives via DI (knows only the trait)
// infrastructure_import_parser_adapter.rs
pub struct ImportParserAdapter {
    analyzer: Arc<dyn IImportAnalyzerPort>,  // via DI
}

// 4. Root container wires implementation
// root_container.rs
let analyzer: Arc<dyn IImportAnalyzerPort> = Arc::new(ImportAnalyzer::new());
let parser = ImportParserAdapter::new(analyzer);
```

### Pattern 4: Shared Infrastructure Implementation [DI WIRING]

When multiple crates need the same infrastructure implementation:

```rust
// WRONG:
// capabilities_a.rs
use code_analysis::FileCollectorProvider::new();  // FORBIDDEN - imports implementation

// capabilities_b.rs
use code_analysis::FileCollectorProvider::new();  // FORBIDDEN - imports implementation

// CORRECT:
// 1. Contract defines interface (in shared)
// contract_scanner_provider_port.rs
pub trait IScannerProviderPort: Send + Sync {
    fn scan_directory(&self, path: &DirectoryPath) -> Result<FilePathList, FileSystemError>;
}

// 2. ONE crate owns the implementation
// code-analysis/src/infrastructure_file_collector_provider.rs
pub struct FileCollectorProvider;
impl IScannerProviderPort for FileCollectorProvider { ... }

// 3. Root container wires implementation to all consumers
// root_container.rs
let scanner: Arc<dyn IScannerProviderPort> = Arc::new(
    code_analysis::FileCollectorProvider::new()
);

// 4. Capabilities receive via DI (know only the trait)
// capabilities_lint_executor.rs
pub struct LintExecutor {
    scanner: Arc<dyn IScannerProviderPort>,  // via DI, not direct import
}
```

## Step-by-Step Fix

### Step 1: Find Violations

Read each file and ask:

- Does `capabilities_*.rs` use types from `capabilities_*.rs`? -> VIOLATION
- Does `infrastructure_*.rs` use types from `infrastructure_*.rs`? -> VIOLATION
- Does `infrastructure_*.rs` use types from `capabilities_*.rs`? -> VIOLATION
- Does `capabilities_*.rs` use types from `infrastructure_*.rs`? -> VIOLATION
- Does a capability directly instantiate an infrastructure type? -> VIOLATION (should use DI)

### Step 2: Create Trait in Contract Layer

Create trait (port/protocol) in contract layer for needed functionality:

```rust
// contract_<concept>_port.rs or contract_<concept>_protocol.rs
// ONLY traits, NO implementations or pure functions!
pub trait I<Concept>Port: Send + Sync {
    fn method(&self, args...) -> Result<Output, Error>;
}
```

### Step 3: Update Imports

Change imports to use contract layer:

```rust
// BEFORE (VIOLATION)
use crate::capabilities_other::OtherStruct;
use crate::infrastructure_provider::Provider;

// AFTER (CORRECT)
use crate::contract_other_port::IOtherPort;
use crate::contract_provider_port::IProviderPort;
```

### Step 4: Wire via DI (if needed)

If the implementation is shared across crates:

1. Put implementation in ONE crate (the "owner")
2. Root container creates instance and wires to consumers
3. Consumers receive `Arc<dyn ITrait>` via constructor

## Common Violations and Fixes

| Violation | Fix |
|-----------|-----|
| `capabilities_a.rs` uses `capabilities_b::Struct` | Create `contract_b_port.rs` with trait, use DI |
| `infrastructure_a.rs` uses `infrastructure_b::Struct` | Create `contract_b_port.rs` with trait, use DI |
| `infrastructure_a.rs` uses `capabilities_b::fn()` | Move computation to capabilities, infra only does I/O |
| `capabilities_a.rs` uses `infrastructure_b::fn()` | Receive via `Arc<dyn ITrait>` in constructor |
| Capability creates `Infrastructure::new()` directly | Receive via `Arc<dyn ITrait>` in constructor |
| Infrastructure imports from capabilities | Create protocol in contract layer |

## File Naming Convention

| Trait Type | Suffix | Used By | Implemented By | Example |
|-----------|--------|---------|----------------|---------|
| **Protocol** | `_protocol.rs` | Capabilities receive from capabilities | Capabilities implements | `contract_dummy_import_checker_protocol.rs` |
| **Port** | `_port.rs` | Infrastructure receives from infrastructure | Infrastructure implements | `contract_external_lint_port.rs` |
| **Aggregate** | `_aggregate.rs` | Agents receive from agents | Agents implements | `contract_agent_role_aggregate.rs` |

**Rule**: Contract layer contains ONLY trait definitions, NOT implementations or pure functions.

### When to Use Each

```rust
// Use _protocol when capabilities need to communicate with each other
pub trait I<Name>Protocol: Send + Sync { ... }  // capabilities_<name>.rs implements

// Use _port when infrastructure components need to communicate with each other  
pub trait I<Name>Port: Send + Sync { ... }      // infrastructure_<name>.rs implements

// Use _aggregate when agents need to communicate with each other
pub trait I<Name>Aggregate: Send + Sync { ... }  // agent_<name>.rs implements
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
