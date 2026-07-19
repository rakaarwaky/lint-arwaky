# Examples

## BAD: Dataclass Defined in Capabilities

```rust
// capabilities_orphan_analyzer.rs
pub struct OrphanResult {
    is_orphan: bool,
    reason: String,
}
```

Fix: Move to taxonomy.

```rust
// shared/orphan_detector/taxonomy_orphan_result_vo.rs
pub struct OrphanResult {
    is_orphan: OrphanFlag,
    reason: OrphanReason,
}
```

## BAD: Taxonomy Importing Layer Code

```rust
// taxonomy_orphan_vo.rs
use crate::capabilities_orphan_analyzer::OrphanAnalyzer; // BAD
```

Taxonomy must not import from layers.

## BAD: Domain Rule Inside Utility

```rust
pub fn is_port_trait_name(name: &str) -> bool {
    name.ends_with("Port")
}
```

If this knows AES naming conventions, it belongs in capabilities as a helper.

## GOOD: Dataclass in Taxonomy + Implementor with DI

```rust
// shared/orphan_detector/taxonomy_orphan_analysis_result_vo.rs
pub struct OrphanAnalysisResult {
    is_orphan: OrphanFlag,
    reason: OrphanReason,
}

// capabilities_orphan_analyzer.rs
pub struct CapabilitiesOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCachePort>,
}
```
