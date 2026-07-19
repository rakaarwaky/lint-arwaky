use std::sync::Arc;

use shared::<domain>::taxonomy_<name>_vo::<VO>;
use shared::<domain>::contract_<name>_aggregate::I<Name>Aggregate;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct Agent<Name> {
    // DI fields use Arc<dyn Trait>
    // Value fields use shared VOs
}

// ─── Block 2: Public Contract (domain aggregate ONLY) ─────
impl I<Name>Aggregate for Agent<Name> {
    fn execute(&self, request: &ScanRequest) -> Vec<LintResult> {
        // orchestration only
        Vec::new()
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Agent<Name> {
    pub fn new(/* DI params */) -> Self {
        Self { /* ... */ }
    }
}
