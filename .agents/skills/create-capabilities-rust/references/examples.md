# Examples

## BAD: Capability Without Trait (AES403)

```rust
pub struct FrameComposer;

impl FrameComposer {
    pub fn compose_frame(&self) {
        // public behavior without protocol trait
    }
}
```

Fix:

```rust
pub struct FrameComposer;

impl IFrameComposerProtocol for FrameComposer {
    fn compose_frame(&self) {
        // contract implementation
    }
}
```

## BAD: I/O in Capabilities (AES404)

```rust
impl MyCapability {
    fn process(&self) {
        let content = std::fs::read_to_string("file.txt"); // FORBIDDEN
    }
}
```

Fix: Move I/O to utility.

## BAD: Data Class Defined in Layer File

```rust
pub struct OrphanResult {
    is_orphan: bool,
    reason: String,
}
```

Fix: Move to shared taxonomy, then import.

## BAD: Concrete Service Field

```rust
pub struct CapabilitiesOrphanAnalyzer {
    extractor: FilenameExtractor, // BAD
}
```

Fix:

```rust
pub struct CapabilitiesOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
}
```

## BAD: Std Trait in Block 2

```rust
pub struct ArchLineChecker;

impl Default for ArchLineChecker {
    fn default() -> Self { Self }
}

impl ILineCheckerProtocol for ArchLineChecker {
    fn check_line_counts(&self, ...) { // ...
    }
}
```

Fix: Move `Default` to Block 3.

## GOOD: Capability with DI and Shared VO

```rust
use std::sync::Arc;

use shared::orphan_detector::taxonomy_orphan_analysis_policy_vo::OrphanAnalysisPolicy;
use shared::orphan_detector::taxonomy_orphan_file_cache_port::IOrphanFileCachePort;
use shared::orphan_detector::taxonomy_orphan_filename_extractor_protocol::IOrphanFilenameExtractorProtocol;
use shared::orphan_detector::taxonomy_capabilities_orphan_protocol::ICapabilitiesOrphanProtocol;

pub struct CapabilitiesOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCachePort>,
    policy: OrphanAnalysisPolicy,
}

impl ICapabilitiesOrphanProtocol for CapabilitiesOrphanAnalyzer {
    // public contract methods only
}
```

## GOOD: Correct 3-Block Structure

```rust
use std::sync::Arc;

use shared::code_analysis::taxonomy_file_path_vo::FilePath;
use shared::code_analysis::taxonomy_layer_definition_vo::LayerDefinition;
use shared::code_analysis::taxonomy_line_checker_protocol::ILineCheckerProtocol;
use shared::code_analysis::taxonomy_line_checker_utility::is_barrel_file;
use shared::code_analysis::taxonomy_lint_result_vo::LintResult;
use shared::code_analysis::taxonomy_source_vo::SourceContentVO;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ArchLineChecker;

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl ILineCheckerProtocol for ArchLineChecker {
    fn check_line_counts(
        &self,
        file: &FilePath,
        definition: Option<&LayerDefinition>,
        source: &SourceContentVO,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = file.basename();
        if is_barrel_file(basename) {
            return;
        }
        // Remaining domain logic...
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for ArchLineChecker {
    fn default() -> Self { Self }
}

impl ArchLineChecker {
    pub fn new() -> Self { Self }

    fn is_layer_relevant(&self, definition: &LayerDefinition) -> bool {
        // Private helper specific to this checker.
        true
    }
}
```
