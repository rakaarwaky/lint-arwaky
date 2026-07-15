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

- `capabilities_*.rs` uses types from `capabilities_*.rs`
- `infrastructure_*.rs` uses types from `infrastructure_*.rs`
- `infrastructure_*.rs` uses types from `capabilities_*.rs`
- `capabilities_*.rs` uses types from `infrastructure_*.rs`

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
// 1. For pure functions: move to contract layer
// contract_layer_prefix_port.rs
pub fn extract_layer_from_prefix(filename: &str) -> Option<String> {
    // pure computation, no I/O
}

// 2. For traits: create trait in contract layer
// contract_frame_exporter_port.rs
pub trait IFrameExporterPort: Send + Sync {
    fn export(&self, frame: &Frame) -> PathBuf;
}

// 3. Capability imports from contract (ALLOWED)
// capabilities_timeline_processor.rs
use crate::contract_frame_exporter_port::IFrameExporterPort;      // ALLOWED
use crate::contract_keyframe_calculator_port::IKeyframeCalculatorPort;  // ALLOWED

pub struct TimelineProcessor {
    exporter: Arc<dyn IFrameExporterPort>,      // via DI
    calculator: Arc<dyn IKeyframeCalculatorPort>, // via DI
}
```

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
// Option A: Move computation to capabilities, infrastructure only does I/O
// capabilities_import_analyzer.rs (NEW)
pub struct ImportAnalyzer;
impl ImportAnalyzer {
    pub fn analyze(content: &str) -> AnalysisResult {
        // All computation here
    }
}

// infrastructure_import_parser_adapter.rs
// Only does I/O (file reading), returns raw content
pub struct ImportParserAdapter;
impl IImportParserPort for ImportParserAdapter {
    fn read_file(&self, path: &Path) -> Result<String, io::Error> {
        fs::read_to_string(path)  // I/O only
    }
}

// Option B: If infrastructure MUST call capabilities, use protocol in contract
// contract_import_analyzer_protocol.rs
pub trait IImportAnalyzerProtocol: Send + Sync {
    fn analyze(&self, content: &str) -> AnalysisResult;
}

// infrastructure_import_parser_adapter.rs
pub struct ImportParserAdapter {
    analyzer: Arc<dyn IImportAnalyzerProtocol>,  // via DI
}
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

```
contract_<concept>_port.rs      // Outbound interface (implemented by infrastructure)
contract_<concept>_protocol.rs  // Inbound interface (implemented by capabilities)
contract_<concept>_aggregate.rs // Facade interface
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
