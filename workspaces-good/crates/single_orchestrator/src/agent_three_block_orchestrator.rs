// PURPOSE: three-block agent — the AES405 structure and delegation case
//
// The agent declares exactly three blocks: types and injected deps, the feature
// aggregate, then constructors and std traits. It implements the aggregate and
// nothing else — the protocol seam it uses is injected, never implemented here,
// because an agent that `impl`s a contract protocol duplicates a capability's
// work (AES405). `Default` is a std trait, not a protocol, so the std impl in
// Block 3 is fine.
//
// The header comment mentions the blocks in prose. Only a `Block <n>:` heading
// opens a block, so prose stays prose and this file remains a three-block agent.
use std::sync::Arc;

use calculator_shared::taxonomy_single_request::SingleRequest;
use calculator_shared::taxonomy_single_response::SingleResponse;

use calculator_shared::single_orchestrator::contract_single_protocol::{
    ISingleCheckerProtocol, ISingleRunnerAggregate,
};

// ─── Block 1: Struct Definitions ───────────────────
pub struct ThreeBlockOrchestrator {
    checker: Arc<dyn ISingleCheckerProtocol>,
}

// ─── Block 2: Aggregate Trait Implementation ───────
impl ISingleRunnerAggregate for ThreeBlockOrchestrator {
    fn execute(&self, request: SingleRequest) -> SingleResponse {
        self.checker.audit(request)
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────
impl Default for ThreeBlockOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl ThreeBlockOrchestrator {
    pub fn new(checker: Arc<dyn ISingleCheckerProtocol>) -> Self {
        Self { checker }
    }
}