# Examples

## BAD: Computation in Agent

```rust
impl OrphanOrchestrator {
    fn process(&self, files: &[FilePath]) {
        let total = files.len(); // BAD: domain/technical computation
        let sum: usize = files.iter().map(|f| f.size()).sum(); // BAD
    }
}
```

Fix: Move computation to capabilities.

## BAD: Business Logic in Agent

```rust
impl OrphanOrchestrator {
    fn analyze(&self, content: &FileContent) -> bool {
        content.value().contains("orphan") // BAD: business rule
    }
}
```

Fix: Move to capabilities.

## BAD: I/O in Agent

```rust
impl OrphanOrchestrator {
    fn execute(&self, path: &FilePath) {
        let content = std::fs::read_to_string(path.value()); // BAD
    }
}
```

Fix: Use an injected port.

## BAD: Dataclass Defined in Agent File

```rust
pub struct OrphanReport {
    results: Vec<String>,
}
```

Fix: Move to taxonomy.

## BAD: Concrete Service Field

```rust
pub struct OrphanOrchestrator {
    analyzer: OrphanAnalyzer, // BAD
}
```

Fix:

```rust
pub struct OrphanOrchestrator {
    analyzer: Arc<dyn IOrphanAnalyzerProtocol>,
}
```

## BAD: Std Trait in Block 2

```rust
impl Default for OrphanOrchestrator {
    fn default() -> Self { Self }
}

impl IOrphanOrchestratorAggregate for OrphanOrchestrator {
    fn execute(&self, request: &ScanRequest) -> Vec<LintResult> { Vec::new() }
}
```

Fix: Move `Default` to Block 3.

## GOOD: Correct 3-Block Order

```rust
use std::sync::Arc;

use shared::code_analysis::taxonomy_file_path_vo::FilePath;
use shared::code_analysis::taxonomy_lint_result_vo::LintResult;
use shared::orphan_detector::taxonomy_orphan_analyzer_protocol::IOrphanAnalyzerProtocol;
use shared::orphan_detector::taxonomy_orphan_orchestrator_aggregate::IOrphanOrchestratorAggregate;
use shared::scan::taxonomy_scan_request_vo::ScanRequest;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct OrphanOrchestrator {
    analyzer: Arc<dyn IOrphanAnalyzerProtocol>,
}

// ─── Block 2: Public Contract (domain aggregate ONLY) ─────
impl IOrphanOrchestratorAggregate for OrphanOrchestrator {
    fn execute(&self, request: &ScanRequest) -> Vec<LintResult> {
        let mut violations = Vec::new();
        for file in request.files() {
            match self.analyzer.analyze(file) {
                Ok(result) => violations.extend(result.into_violations()),
                Err(err) => violations.push(LintResult::from_analysis_error(file, err)),
            }
        }
        violations
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl OrphanOrchestrator {
    pub fn new(analyzer: Arc<dyn IOrphanAnalyzerProtocol>) -> Self {
        Self { analyzer }
    }
}
```
