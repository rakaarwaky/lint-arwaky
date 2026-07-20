# The 3-Block Structure

Every implementation file MUST follow this order with mandatory block markers:

1. **Block 1 — Struct Definition**
2. **Block 2 — Aggregate Trait Implementation**
3. **Block 3 — Constructors, Std Traits, and Private Helpers**

Each block MUST be preceded by a block marker comment:

```rust
// ─── Block 1: Struct Definition ───────────────────────────
```

```rust
// ─── Block 2: Aggregate Trait Implementation ──────────────
```

```rust
// ─── Block 3: Constructors, Helpers, Private Methods ──────
```

## Block 1 — Struct Definition

```rust
// ─── Block 1: Struct Definition ───────────────────────────
pub struct <NameOrchestrator> {
    analyzer: Arc<dyn I<NameAnalyzer>Protocol>,
}
```

## Block 2 — Public Contract

Block 2 is RESERVED for the domain aggregate trait ONLY.

```rust
// ─── Block 2: Aggregate Trait Implementation ──────────────
impl I<NameOrchestrator>Aggregate for <NameOrchestrator> {
    fn execute(&self, request: &<ScanRequest>VO) -> Vec<<ResultVO>> {
        // orchestration only
    }
}
```

Do NOT put `Default`, `Clone`, `Debug`, `Display`, `From` impls in Block 2.

## Block 3 — Constructors, Std Traits, and Helpers

```rust
// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl <NameOrchestrator> {
    pub fn new(analyzer: Arc<dyn I<NameAnalyzer>Protocol>) -> Self {
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
