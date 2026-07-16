---
name: fix-cross-import-rust
version: 2.0.0
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
  - trait-consolidation
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

**NO!** Capabilities and Infrastructure are **PEER layers** — they CANNOT import from each other.

---

## Fix Strategy: Choose the Right Approach

When you encounter a cross-import violation, evaluate which fix strategy applies:

### Option A: Extract to Taxonomy Utility (Standalone Free Functions)

Use this approach when the code being imported is **stateless, pure logic** with no side effects.

**Extract to `taxonomy_*_utility.rs` when:**

| Condition | Example |
| --- | --- |
| Pure function — no `&self`, no struct state | `parse_path()`, `normalize_name()` |
| Stateless — all data via parameters | `fn compute_distance(a: &Point, b: &Point)` |
| No side effects — deterministic output | `fn sanitize_string(input: &str) -> String` |
| Universal utility — usable by any layer | Path parsing, string normalization, math helpers |
| No mocking needed — pure computation | Any function where testing doesn't require mocking |

**How it works:**

```rust
// taxonomy_path_utility.rs (TAXONOMY LAYER)
pub fn parse_path(path: &str) -> Option<String> {
    // Pure computation, no state, no side effects
    path.strip_prefix("/")
        .map(|s| s.to_string())
}

pub fn normalize_name(name: &str) -> String {
    name.trim().to_lowercase()
}
```

```rust
// capabilities_timeline_processor.rs (CONSUMER)
use crate::taxonomy_path_utility::{parse_path, normalize_name}; // ALLOWED: taxonomy import

pub fn process(path: &str) {
    let cleaned = parse_path(path).unwrap();
    let name = normalize_name(&cleaned);
    // ... use the results
}
```

**Benefits:**
- Zero wiring overhead — direct import, no constructors, no traits
- Simpler codebase for pure logic
- Any layer can consume (taxonomy, capabilities, infrastructure, agent)
- No runtime cost (no trait object dispatch)

---

### Option B: Dependency Injection via Traits (Port/Protocol Pattern)

Use this approach when the code requires **state, side effects, or layer-specific behavior**.

**Use DI when:**

| Condition | Example |
| --- | --- |
| Needs `&self` / struct state | Struct with fields for data/mutation |
| Has side effects / I/O | File operations, network calls, DB queries |
| Layer-specific implementation | Adapter that depends on concrete infrastructure |
| Needs mocking for testing | Interface where you want to swap implementations |
| Behavior varies per scenario | Different database adapters, file system backends |

**How it works:**

```rust
// 1. Define port/protocol in CONTRACT layer
// contract_frame_exporter_port.rs
pub trait IFrameExporterPort: Send + Sync {
    fn export(&self, frame: &Frame) -> PathBuf;
}

// 2. Capability implements the trait
// capabilities_frame_exporter.rs
pub struct FrameExporter {
    output_dir: PathBuf, // STATE
}
impl IFrameExporterPort for FrameExporter {
    fn export(&self, frame: &Frame) -> PathBuf {
        self.output_dir.join(format!("{}.png", frame.id))
    }
}

// 3. Consumer receives via DI (knows only the trait)
// capabilities_timeline_processor.rs
pub struct TimelineProcessor {
    exporter: Arc<dyn IFrameExporterPort>, // via DI, not direct import
}

// 4. Root container wires everything
// root_container.rs
let exporter: Arc<dyn IFrameExporterPort> = Arc::new(FrameExporter::new());
let processor = TimelineProcessor::new(exporter);
```

**Benefits:**
- Loose coupling — consumer doesn't know concrete type
- Testable — mock the trait, swap implementations
- Layer isolation — each layer only knows its own contracts

---

## Decision Tree: Which Option to Choose?

```
Encountered cross-import violation?
  │
  ├─ Does the code need &self / struct state?
  │   └─ YES → Option B: Dependency Injection
  │
  ├─ Does the code have side effects (I/O, file, network)?
  │   └─ YES → Option B: Dependency Injection
  │
  ├─ Is the behavior layer-specific (adapter, concrete impl)?
  │   └─ YES → Option B: Dependency Injection
  │
  └─ Is it pure, stateless, no &self?
      └─ YES → Option A: Extract to Taxonomy Utility
```

---

## Comparison: Option A vs Option B

| Aspect | Option A: Taxonomy Utility | Option B: DI (Traits) |
| --- | --- | --- |
| **Location** | `taxonomy_*_utility.rs` | `contract_*_port.rs` + layer impl |
| **Code pattern** | `pub fn foo()` standalone | `pub struct Foo; impl IFooPort for Foo { ... }` |
| **Importing** | Direct: `use crate::taxonomy_utility::foo;` | Via trait: `Arc<dyn IFooPort>` in constructor |
| **State** | None (stateless) | Can have state |
| **Side effects** | No (pure functions only) | Yes (I/O, file, network allowed) |
| **Scope** | Universal — any layer can import | Layer-specific — defined per peer relationship |
| **Wiring** | None needed | Root container wires implementations |
| **Testing** | Direct call, no mock needed | Mock trait for test isolation |

---

## Circular Dependencies: How to Break Cycles

Circular dependencies occur when Layer A imports Layer B, and Layer B imports Layer A — creating an unresolvable cycle.

### Pattern 1: Two-Way Dependency Between Capabilities

```rust
// VIOLATION: Cycle between capabilities
// capabilities_a.rs
use crate::capabilities_b::FunctionB;  // A imports B
pub fn function_a() { FunctionB(); }

// capabilities_b.rs
use crate::capabilities_a::FunctionA;  // B imports A
pub fn function_b() { FunctionA(); }
```

**Fix — Extract shared logic to Taxonomy Utility:**

```rust
// taxonomy_shared_logic_utility.rs (NEW)
pub fn shared_step() -> Result<(), Error> {
    // Move the common code here
}
```

```rust
// capabilities_a.rs
use crate::taxonomy_shared_logic_utility::shared_step;  // ALLOWED
pub fn function_a() { shared_step(); /* B-specific logic */ }

// capabilities_b.rs
use crate::taxonomy_shared_logic_utility::shared_step;  // ALLOWED
pub fn function_b() { shared_step(); /* A-specific logic */ }
```

**Fix — Use DI with a protocol trait:**

```rust
// contract_a_b_protocol.rs (NEW)
pub trait IABProtocol: Send + Sync {
    fn step(&self) -> Result<(), Error>;
}

// capabilities_a.rs
use crate::contract_a_b_protocol::IABProtocol;
pub struct CapabilityA { protocol: Arc<dyn IABProtocol> }
impl CapabilityA { pub fn function_a(&self) { self.protocol.step(); } }

// capabilities_b.rs
pub struct BImpl;  // implements IABProtocol
impl IABProtocol for BImpl { fn step(&self) -> Result<(), Error> { /* ... */ } }

// root_container.rs
let protocol: Arc<dyn IABProtocol> = Arc::new(BImpl);
let a = CapabilityA::new(protocol);  // inject into A too
```

### Pattern 2: Capability ↔ Infrastructure Cycle

```rust
// VIOLATION: Unresolvable cycle
// capabilities_analyzer.rs
use crate::infrastructure_db_adapter::DbConnection;  // Cap → Infra
pub fn analyze() { DbConnection::query(); }

// infrastructure_db_adapter.rs
use crate::capabilities_analyzer::AnalyzeResult;  // Infra → Cap
pub fn query() -> AnalyzeResult { /* ... */ }
```

**Fix — Break the cycle by moving shared types to Taxonomy:**

```rust
// taxonomy_analyze_result_utility.rs (NEW)
pub struct AnalyzeResult { data: Vec<String> }

// capabilities_analyzer.rs
use crate::taxonomy_analyze_result_utility::AnalyzeResult;

// infrastructure_db_adapter.rs
use crate::taxonomy_analyze_result_utility::AnalyzeResult;
```

**Fix — Use traits to decouple:**

```rust
// contract_db_port.rs (NEW)
pub trait IDbPort: Send + Sync {
    fn query(&self) -> AnalyzeResult;
}

// infrastructure_db_adapter.rs
impl IDbPort for DbAdapter { fn query(&self) -> AnalyzeResult { /* ... */ } }

// capabilities_analyzer.rs
use crate::contract_db_port::IDbPort;
pub struct Analyzer { db: Arc<dyn IDbPort> }  // depends on trait, not impl
```

### Pattern 3: Deep Cycle Chains (A → B → C → A)

When you have a longer cycle like `CapA → InfraB → CapC → CapA`:

1. **Identify the shared code** that creates the dependency
2. **Extract to taxonomy utility** if it's pure logic
3. **Break at the weakest link** — usually infrastructure ↔ capability boundary
4. **Use traits** for the remaining dependencies

```rust
// Before: CapA → InfraB → CapC → CapA (CYCLE)
// After:  CapA ──→ Taxonomy Utility ←── CapC
//                    ↑
//              InfraB uses trait from contract
```

### How to Detect Circular Dependencies

```rust
// Checklist for each cross-import:
// 1. File A imports from File B
// 2. Does File B also import from File A (directly or transitively)?
// 3. If YES → Circular dependency detected!
```

---

## Edge Cases and Tricky Scenarios

### Edge Case 1: Mostly-Pure Functions with One I/O Call

The function is mostly computation but has one file read/write:

```rust
// capabilities_data_processor.rs
pub fn process(data: &str) -> String {
    // 90% pure computation
    let parsed = parse_input(data);  // pure
    let computed = compute_result(parsed);  // pure
    // 10% I/O — breaks Option A
    std::fs::write("/tmp/output.txt", &computed);
    format!("Done: {}", computed)
}
```

**Decision:** Use **Option B (DI)** because of the I/O side effect.

```rust
// contract_file_writer_port.rs
pub trait IFileWriterPort: Send + Sync {
    fn write(&self, path: &str, content: &str) -> Result<(), Error>;
}

// capabilities_data_processor.rs
pub struct DataProcessor { writer: Arc<dyn IFileWriterPort> }

impl DataProcessor {
    pub fn process(&self, data: &str) -> String {
        let parsed = parse_input(data);  // pure logic stays here
        let computed = compute_result(parsed);
        self.writer.write("/tmp/output.txt", &computed).ok();  // I/O via trait
        format!("Done: {}", computed)
    }
}
```

---

### Edge Case 2: Mixed Struct (Some fields state, some pure functions)

```rust
// capabilities_data_processor.rs
pub struct DataProcessor {
    output_dir: PathBuf,           // STATE → needs DI
    cache: HashMap<String, String>, // STATE → needs DI
}

impl DataProcessor {
    pub fn parse_input(&self, input: &str) -> ParsedInput {  // NO side effects!
        // This function is pure — could be extracted to taxonomy
        let tokens = input.split_whitespace();
        // ...
    }
}
```

**Decision:** Split the struct.
- Pure functions (`parse_input`) → extract to `taxonomy_*_utility.rs`
- Stateful logic (`process`, `save`) → keep as DI trait

```rust
// taxonomy_parsing_utility.rs (extracted)
pub fn parse_input(input: &str) -> ParsedInput { ... }

// capabilities_data_processor.rs (simplified)
use crate::taxonomy_parsing_utility::parse_input;
pub struct DataProcessor {
    output_dir: PathBuf,
    cache: HashMap<String, String>,
}
impl DataProcessor {
    pub fn process(&self, input: &str) -> String {
        let parsed = parse_input(input);  // from taxonomy
        // ... rest uses state
    }
}
```

---

### Edge Case 3: Function That Appears Pure But Has Hidden Dependencies

```rust
// capabilities_analyzer.rs
pub fn detect_anomalies(data: &[f64]) -> Vec<usize> {
    // Looks pure, but...
    let threshold = get_threshold();  // <-- hidden dependency!
    data.iter().enumerate()
        .filter(|(_, v)| *v > threshold)
        .map(|(i, _)| i)
        .collect()
}

fn get_threshold() -> f64 {
    // Reads from environment, DB, or config — NOT pure!
    std::env::var("THRESHOLD").ok().and_then(|v| v.parse().ok())
        .unwrap_or(0.5)
}
```

**Decision:** Use **Option B (DI)** — the hidden dependency on environment/config makes it non-pure.

```rust
// contract_threshold_provider_port.rs
pub trait IThresholdProviderPort: Send + Sync {
    fn get_threshold(&self) -> f64;
}

// capabilities_analyzer.rs
pub struct AnomalyAnalyzer { threshold: Arc<dyn IThresholdProviderPort> }

impl AnomalyAnalyzer {
    pub fn detect_anomalies(&self, data: &[f64]) -> Vec<usize> {
        let threshold = self.threshold.get_threshold();  // injected
        data.iter().enumerate()
            .filter(|(_, v)| *v > threshold)
            .map(|(i, _)| i)
            .collect()
    }
}
```

---

### Edge Case 4: When to Prefer DI Even for Pure Functions

Use **Option B (DI)** even when the function is pure, if:

| Condition | Reason |
| --- | --- |
| Multiple implementations expected | Different algorithms, strategies |
| Testing requires mock behavior | Mock the trait to verify logic flow |
| Runtime configuration matters | Swap implementations based on config |
| Performance tuning needed | Hot path vs cold path variants |

```rust
// Even though sort is pure, you might want:
pub trait ISorter: Send + Sync {
    fn sort(&self, data: &mut Vec<String>);
}

// Quick sort for small datasets
pub struct QuickSort;  impl ISorter for QuickSort { ... }

// Merge sort for large datasets
pub struct MergeSort;  impl ISorter for MergeSort { ... }

// Config decides at runtime:
pub fn create_sorter(config: &Config) -> Arc<dyn ISorter> {
    if config.use_quick { Arc::new(QuickSort) } else { Arc::new(MergeSort) }
}
```

---

### Edge Case 5: Utility Functions That Should NOT Be Extracted

Some functions look pure but should stay in their layer:

| Function | Why Stay in Layer |
| --- | --- |
| `fn validate_import(imp: &Import)` | Domain-specific to that capability's logic |
| `fn build_query(filters: &[Filter])` | Tied to that infrastructure's DB schema |
| `fn format_report(report: &ReportData)` | Output format specific to that layer's needs |

**Rule:** If the function is **domain-specific to one layer** and won't be reused by other layers — keep it. Don't extract to taxonomy just for the sake of it.

---

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

---

## Cross-Import Patterns (DI Approach)

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

---

## Step-by-Step Fix

### Step 1: Find Violations

Read each file and ask:

- Does `capabilities_*.rs` use types from `capabilities_*.rs`? -> VIOLATION
- Does `infrastructure_*.rs` use types from `infrastructure_*.rs`? -> VIOLATION
- Does `infrastructure_*.rs` use types from `capabilities_*.rs`? -> VIOLATION
- Does `capabilities_*.rs` use types from `infrastructure_*.rs`? -> VIOLATION
- Does a capability directly instantiate an infrastructure type? -> VIOLATION (should use DI)

### Step 2: Choose Fix Strategy

Apply the decision tree above:

**Option A — Extract to Taxonomy Utility:**
If the imported code is pure, stateless, no `&self` needed:

```rust
// taxonomy_path_utility.rs
pub fn parse_path(path: &str) -> Option<String> { ... }
pub fn normalize_name(name: &str) -> String { ... }
```

Then update consumer imports to use taxonomy utility directly.

**Option B — Dependency Injection:**
If the code has state, side effects, or needs trait abstraction:

```rust
// 1. Create trait in contract layer
// contract_<concept>_port.rs
pub trait I<Concept>Port: Send + Sync {
    fn method(&self, args...) -> Result<Output, Error>;
}

// 2. Implement trait in the source layer
// capabilities_<concept>.rs or infrastructure_<concept>.rs
impl I<Concept>Port for <Concept> { ... }

// 3. Consumer receives via DI
pub struct Consumer {
    dep: Arc<dyn I<Concept>Port>,
}

// 4. Root container wires
```

### Step 3: Update Imports

Change imports to use the chosen approach:

```rust
// BEFORE (VIOLATION)
use crate::capabilities_other::OtherStruct;
use crate::infrastructure_provider::Provider;

// AFTER — Option A (Taxonomy Utility)
use crate::taxonomy_path_utility::{parse_path, normalize_name};

// AFTER — Option B (DI via Contract)
use crate::contract_other_port::IOtherPort;
use crate::contract_provider_port::IProviderPort;
```

### Step 4: Wire via DI (Option B only)

If using Dependency Injection:

1. Put implementation in ONE crate (the "owner")
2. Root container creates instance and wires to consumers
3. Consumers receive `Arc<dyn ITrait>` via constructor

---

## Common Violations and Fixes

| Violation | Fix Strategy |
| --- | --- |
| `capabilities_a.rs` uses `capabilities_b::Struct` | Option B: Create `contract_b_port.rs` with trait, use DI |
| `infrastructure_a.rs` uses `infrastructure_b::Struct` | Option B: Create `contract_b_port.rs` with trait, use DI |
| `infrastructure_a.rs` uses `capabilities_b::fn()` | Check: pure function? → Option A (taxonomy utility). Otherwise → Option B |
| `capabilities_a.rs` uses `infrastructure_b::fn()` | Check: pure function? → Option A (taxonomy utility). Otherwise → Option B |
| Capability creates `Infrastructure::new()` directly | Option B: Receive via `Arc<dyn ITrait>` in constructor |
| Infrastructure imports from capabilities | Check: pure function? → Option A (taxonomy utility). Otherwise → Option B |

---

## File Naming Convention

| Pattern Type    | Suffix          | Used By                                     | Implemented By            | Example                                     |
| --------------- | --------------- | ------------------------------------------- | ------------------------- | ------------------------------------------- |
| **Protocol**    | `_protocol.rs`  | Capabilities receive from capabilities      | Capabilities implements   | `contract_dummy_import_checker_protocol.rs` |
| **Port**        | `_port.rs`      | Infrastructure receives from infrastructure | Infrastructure implements | `contract_external_lint_port.rs`            |
| **Aggregate**   | `_aggregate.rs` | Agents receive from agents                  | Agents implements         | `contract_agent_role_aggregate.rs`          |
| **Utility**     | `_utility.rs`   | Any layer (direct import)                   | Taxonomy layer only       | `taxonomy_path_utility.rs`                  |

**Rule**: Contract layer contains ONLY trait definitions, NOT implementations or pure functions.

### When to Use Each Pattern

```rust
// Use _protocol when capabilities need to communicate with each other
pub trait I<Name>Protocol: Send + Sync { ... }  // capabilities_<name>.rs implements

// Use _port when infrastructure components need to communicate with each other
pub trait I<Name>Port: Send + Sync { ... }      // infrastructure_<name>.rs implements

// Use _aggregate when agents need to communicate with each other
pub trait I<Name>Aggregate: Send + Sync { ... }  // agent_<name>.rs implements

// Use _utility.rs (taxonomy layer) for pure, stateless functions
pub fn parse_path(path: &str) -> Option<String> { ... }  // taxonomy_path_utility.rs
```

---

## Quick Reference

| Layer          | Can Import From              | Cannot Import From                                           |
| -------------- | ---------------------------- | ------------------------------------------------------------ |
| taxonomy       | taxonomy                     | contract, capabilities, infrastructure, agent, surface, root |
| contract       | taxonomy, contract           | capabilities, infrastructure, agent, surface, root           |
| capabilities   | taxonomy, contract           | **infrastructure**, surface, agent, **capabilities**, root   |
| infrastructure | taxonomy, contract           | surface, **capabilities**, agent, **infrastructure**, root   |
| agent          | taxonomy, contract           | capabilities, infrastructure, surface, root                  |
| surface        | taxonomy, contract (limited) | capabilities, infrastructure, agent, root                    |
| root           | ALL layers                   | (none)                                                       |

**Key Rule**: Capabilities and Infrastructure are PEER layers — they CANNOT import from each other!
