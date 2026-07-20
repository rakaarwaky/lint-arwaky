# Examples

## BAD: Dataclass Defined in Capabilities

```rust
// capabilities_<name-capability>.rs
pub struct <ResultVO> {
    is_valid: bool,
    reason: String,
}
```

Fix: Move to taxonomy.

```rust
// shared/<name-feature>/taxonomy_<name>_result_vo.rs
pub struct <ResultVO> {
    is_valid: <Flag>VO,
    reason: <Reason>VO,
}
```

## BAD: Taxonomy Importing Layer Code

```rust
// taxonomy_<name>_vo.rs
use crate::capabilities_<name-capability>::<NameAnalyzer>; // BAD
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
// shared/<name-feature>/taxonomy_<name>_analysis_result_vo.rs
pub struct <AnalysisResult>VO {
    is_valid: <Flag>VO,
    reason: <Reason>VO,
}

// capabilities_<name-capability>.rs
pub struct <NameCapability> {
    collaborator: Arc<dyn I<NameCollaborator>Protocol>,
    store: Arc<dyn I<NameStore>Protocol>,
}
```
