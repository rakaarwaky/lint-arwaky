# The 3-Block Structure

1. **Block 1 — Struct Definition**
2. **Block 2 — Aggregate Trait Implementation**
3. **Block 3 — Constructors, Std Traits, and Private Helpers**

## Block 1 — Struct Definition

```rust
pub struct OrphanOrchestrator {
    analyzer: Arc<dyn IOrphanAnalyzerProtocol>,
}
```

## Block 2 — Public Contract

Block 2 is RESERVED for the domain aggregate trait ONLY.

```rust
impl IOrphanOrchestratorAggregate for OrphanOrchestrator {
    fn execute(&self, request: &ScanRequest) -> Vec<LintResult> {
        // orchestration only
    }
}
```

Do NOT put `Default`, `Clone`, `Debug`, `Display`, `From` impls in Block 2.

## Block 3 — Constructors, Std Traits, and Helpers

```rust
impl OrphanOrchestrator {
    pub fn new(analyzer: Arc<dyn IOrphanAnalyzerProtocol>) -> Self {
        Self { analyzer }
    }

    fn should_skip_file(&self, file: &FilePath) -> bool {
        self.policy.is_excluded(file)
    }
}
```

## Trait Placement Decision Rule

```text
Trait impl found in an agent file?
  │
  ├─ Is it the domain aggregate? (I<Name>Aggregate)
  │   └─ YES → Block 2
  │
  └─ Is it a std/derive/utility trait? (Default, Clone, Debug, Display, From, etc.)
      └─ YES → Block 3
```
