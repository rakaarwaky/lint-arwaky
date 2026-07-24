use std::sync::Arc;

use shared::<domain>::taxonomy_<name>_vo::<VO>;
use shared::<domain>::contract_<name>_aggregate::I<Name>Aggregate;
use shared::<domain>::contract_<protocol>_protocol::I<Protocol>Protocol;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct <Name>Orchestrator {
    // DI fields use Arc<dyn Trait>
    // Value fields use shared VOs
    service: Arc<dyn I<Protocol>Protocol>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl I<Name>Aggregate for <Name>Orchestrator {
    fn execute(&self, request: &<RequestVO>) -> <ResultVO> {
        // orchestration only - delegate to protocol
        self.service.process(request)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl <Name>Orchestrator {
    pub fn new(service: Arc<dyn I<Protocol>Protocol>) -> Self {
        Self { service }
    }
}
