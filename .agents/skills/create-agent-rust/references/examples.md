# Examples

## BAD: Computation in Agent

```rust
impl <Name>Orchestrator {
    fn process(&self, files: &[FilePath]) {
        let total = files.len(); // BAD: domain/technical computation
        let sum: usize = files.iter().map(|f| f.size()).sum(); // BAD
    }
}
```

Fix: Move computation to capabilities.

## BAD: Business Logic in Agent

```rust
impl <Name>Orchestrator {
    fn evaluate(&self, content: &FileContent) -> bool {
        content.value().contains("forbidden-marker") // BAD: business rule
    }
}
```

Fix: Move to capabilities.

## BAD: I/O in Agent

```rust
impl <Name>Orchestrator {
    fn execute(&self, path: &FilePath) {
        let content = std::fs::read_to_string(path.value()); // BAD
    }
}
```

Fix: Use an injected protocol.

## BAD: Dataclass Defined in Agent File

```rust
pub struct <Name>ReportVO {
    results: Vec<String>,
}
```

Fix: Move to taxonomy.

## BAD: Concrete Service Field

```rust
pub struct <Name>Orchestrator {
    service: <ServiceType>, // BAD: concrete type
}
```

Fix:

```rust
pub struct <Name>Orchestrator {
    service: Arc<dyn I<Service>Protocol>,
}
```

## BAD: Std Trait in Block 2

```rust
impl Default for <Name>Orchestrator {
    fn default() -> Self { Self }
}

impl I<Name>Aggregate for <Name>Orchestrator {
    fn execute(&self, request: &<RequestVO>) -> <ResultVO> { <ResultVO>::default() }
}
```

Fix: Move `Default` to Block 3.

## GOOD: Correct 3-Block Order

```rust
use std::sync::Arc;

use shared::<domain>::taxonomy_<name>_vo::<VO>;
use shared::<domain>::contract_<name>_aggregate::I<Name>Aggregate;
use shared::<domain>::contract_<protocol>_protocol::I<Protocol>Protocol;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct <Name>Orchestrator {
    deps: <Name>Deps,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl I<Name>Aggregate for <Name>Orchestrator {
    fn execute(&self, request: &<RequestVO>) -> <ResultVO> {
        let formatter: &dyn I<Protocol>Protocol = match request.type() {
            RequestType::A => self.deps.a.as_ref(),
            RequestType::B => self.deps.b.as_ref(),
        };
        formatter.process(request)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl <Name>Orchestrator {
    pub fn new(deps: <Name>Deps) -> Self {
        Self { deps }
    }
}
```
