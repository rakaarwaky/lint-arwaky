# Examples

## BAD: Computation in Agent

```rust
impl <NameOrchestrator> {
    fn process(&self, files: &[FilePath]) {
        let total = files.len(); // BAD: domain/technical computation
        let sum: usize = files.iter().map(|f| f.size()).sum(); // BAD
    }
}
```

Fix: Move computation to capabilities.

## BAD: Business Logic in Agent

```rust
impl <NameOrchestrator> {
    fn evaluate(&self, content: &FileContent) -> bool {
        content.value().contains("forbidden-marker") // BAD: business rule
    }
}
```

Fix: Move to capabilities.

## BAD: I/O in Agent

```rust
impl <NameOrchestrator> {
    fn execute(&self, path: &FilePath) {
        let content = std::fs::read_to_string(path.value()); // BAD
    }
}
```

Fix: Use an injected port.

## BAD: Dataclass Defined in Agent File

```rust
pub struct <Report>VO {
    results: Vec<String>,
}
```

Fix: Move to taxonomy.

## BAD: Concrete Service Field

```rust
pub struct <NameOrchestrator> {
    analyzer: <NameAnalyzer>, // BAD
}
```

Fix:

```rust
pub struct <NameOrchestrator> {
    analyzer: Arc<dyn I<NameAnalyzer>Protocol>,
}
```

## BAD: Std Trait in Block 2

```rust
impl Default for <NameOrchestrator> {
    fn default() -> Self { Self }
}

impl I<NameOrchestrator>Aggregate for <NameOrchestrator> {
    fn execute(&self, request: &<ScanRequest>VO) -> Vec<<ResultVO>> { Vec::new() }
}
```

Fix: Move `Default` to Block 3.

## GOOD: Correct 3-Block Order

```rust
use std::sync::Arc;

use shared::<name-feature>::taxonomy_file_path_vo::FilePath;
use shared::<name-feature>::taxonomy_result_vo::<ResultVO>;
use shared::<name-feature>::contract_analyzer_protocol::I<NameAnalyzer>Protocol;
use shared::<name-feature>::contract_orchestrator_aggregate::I<NameOrchestrator>Aggregate;
use shared::<name-feature>::taxonomy_scan_request_vo::<ScanRequest>VO;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct <NameOrchestrator> {
    analyzer: Arc<dyn I<NameAnalyzer>Protocol>,
}

// ─── Block 2: Public Contract (domain aggregate ONLY) ─────
impl I<NameOrchestrator>Aggregate for <NameOrchestrator> {
    fn execute(&self, request: &<ScanRequest>VO) -> Vec<<ResultVO>> {
        let mut results = Vec::new();
        for file in request.files() {
            match self.analyzer.analyze(file) {
                Ok(result) => results.extend(result.into_results()),
                Err(err) => results.push(<ResultVO>::from_analysis_error(file, err)),
            }
        }
        results
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl <NameOrchestrator> {
    pub fn new(analyzer: Arc<dyn I<NameAnalyzer>Protocol>) -> Self {
        Self { analyzer }
    }
}
```
