# The 3-Block Structure

Every implementation file MUST follow this order:

1. **Block 1 — Struct Definition**
2. **Block 2 — Domain Protocol Trait Implementation**
3. **Block 3 — Constructors, Std Traits, and Private Helpers**

## Block 1 — Struct Definition

```rust
pub struct ArchLineChecker;
```

Or with dependencies:

```rust
pub struct CapabilitiesOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCachePort>,
    policy: OrphanAnalysisPolicy,
}
```

## Block 2 — Public Contract

Block 2 is RESERVED for the domain protocol trait ONLY.

```rust
impl ILineCheckerProtocol for ArchLineChecker {
    fn check_line_counts(
        &self,
        file: &FilePath,
        definition: Option<&LayerDefinition>,
        source: &SourceContentVO,
        violations: &mut Vec<LintResult>,
    ) {
        // domain behavior
    }
}
```

Do NOT put these in Block 2:

```rust
impl Default for ArchLineChecker
impl Clone for ArchLineChecker
impl Debug for ArchLineChecker
impl Display for ArchLineChecker
impl From<...> for ArchLineChecker
```

Those belong in Block 3.

## Block 3 — Constructors, Std Traits, and Helpers

Block 3 contains:

- `new()`
- builders
- `Default`, `Clone`, `Debug`, `Display`
- other std trait impls
- private helper methods
- domain-specific associated functions used only by this struct

```rust
impl Default for ArchLineChecker {
    fn default() -> Self {
        Self
    }
}

impl ArchLineChecker {
    pub fn new() -> Self {
        Self
    }

    fn effective_threshold(&self, layer: &LayerDefinition) -> LineCountThreshold {
        // private helper
    }
}
```

## Trait Placement Decision Rule

```text
Trait impl found in a capabilities file?
  │
  ├─ Is it the domain protocol? (I<Name>Protocol)
  │   └─ YES → Block 2
  │
  └─ Is it a std/derive/utility trait? (Default, Clone, Debug, Display, From, etc.)
      └─ YES → Block 3
```

## Method Placement Decision Rule

```text
Method / function found in a capabilities file?
  │
  ├─ Is it defined as the domain protocol trait method?
  │   └─ YES → Block 2
  │
  ├─ Is it a std/derive trait impl?
  │   └─ YES → Block 3
  │
  ├─ Is it a factory method? (new(), builder)
  │   └─ YES → Block 3
  │
  └─ Is it a private helper?
      └─ YES → Block 3
```
