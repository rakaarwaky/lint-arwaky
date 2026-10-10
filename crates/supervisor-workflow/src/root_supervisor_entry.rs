// PURPOSE: Supervisor entry point — reaches the agent layer so AES505
// reachability from an `*_entry` file is satisfied (the agent is not
// orphaned).
//
// This file is intentionally thin: it exists to keep the agent layer
// reachable from an entry point in the reachability graph. It does not
// duplicate the composition root's wiring — callers that need a ready-made
// orchestrator use `SupervisorContainer` from `root_supervisor_container`.

use std::sync::Arc;

use shared_supervisor::contract_supervisor_aggregate::ISupervisorAggregate;
use shared_supervisor::taxonomy_supervisor_request::SupervisorRequest;
use shared_supervisor::taxonomy_supervisor_response::SupervisorResponse;

/// Build the default mock-wired supervisor aggregate (one composition root
/// call, no I/O beyond what the mock seams carry — which is none).
pub fn default_supervisor() -> Arc<dyn ISupervisorAggregate> {
    crate::root_supervisor_container::SupervisorContainer::new().orchestrator()
}

/// Run one full supervisor cycle with the default mock wiring.
pub fn run_default_cycle(request: SupervisorRequest) -> SupervisorResponse {
    default_supervisor().execute(request)
}
