// PURPOSE: root container — doc-rules DI composition root
//
// Wires the capability into the agent behind the contract trait. The
// container is a constructor and a one-liner facade that returns the
// orchestrator; no business logic lives here.
use crate::agent_doc_orchestrator::DocOrchestrator;
use crate::capabilities_doc_checker::DocChecker;
use shared::doc_rules::contract_doc_aggregate::IDocRunnerAggregate;
use shared::doc_rules::contract_doc_protocol::IDocCheckerProtocol;
use std::sync::Arc;

/// Composition root for the doc-rules feature.
pub struct RootDocRulesContainer;

impl RootDocRulesContainer {
    /// Return a fresh orchestrator for this workspace.
    pub fn orchestrator() -> Arc<dyn IDocRunnerAggregate> {
        let checker: Arc<dyn IDocCheckerProtocol> = Arc::new(DocChecker {});
        Arc::new(DocOrchestrator::new(checker))
    }
}
